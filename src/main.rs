use std::env;
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use image::{Rgba, RgbaImage};
use indicatif::{ProgressBar, ProgressStyle};
use kiddo::float::kdtree::KdTree;
use kiddo::SquaredEuclidean;
use rayon::prelude::*;

const NEIGHBOR_LOCATIONS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
];

const MAX_BORDER_PIXELS: usize = 100_000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = std::time::Instant::now();

    let mut args: Vec<String> = env::args().skip(1).collect();
    let mut dbg_mode = false;

    args.retain(|arg| {
        if arg == "-d" {
            dbg_mode = true;
            false
        } else {
            true
        }
    });

    if args.is_empty() {
        print_usage();
        return Ok(());
    }

    println!("Collecting files...");
    let mut file_paths = Vec::new();

    for arg in args {
        let path = Path::new(&arg);
        if path.is_dir() {
            match collect_png_files(path) {
                Ok(mut png_files) => file_paths.append(&mut png_files),
                Err(e) => eprintln!("Error reading directory {}: {}", arg, e),
            }
        } else if path.is_file() {
            file_paths.push(arg);
        } else {
            eprintln!("Path not found: {}", arg);
        }
    }

    if file_paths.is_empty() {
        println!("No files to process.");
        wait_for_any_key();
        return Ok(());
    }

    println!("Found {} files to process", file_paths.len());

    let pb = ProgressBar::new(file_paths.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.red} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} files ({eta})\nProcessing: {msg}")
            .unwrap()
            .progress_chars("#>-")
    );

    let completed_files = Arc::new(AtomicUsize::new(0));
    let error_count = Arc::new(AtomicUsize::new(0));
    let skipped_files = Arc::new(AtomicUsize::new(0));

    let results: Vec<_> = file_paths
        .into_par_iter()
        .map(|file_path| {
            let filename = Path::new(&file_path)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or(&file_path);
            
            pb.set_message(filename.to_string());
            let result = process_image_optimized(&file_path, dbg_mode);
            pb.inc(1);
            (file_path, result)
        })
        .collect();

    for (file_path, result) in results {
        match result {
            Ok(ProcessResult::Success) => {
                completed_files.fetch_add(1, Ordering::Relaxed);
            }
            Ok(ProcessResult::Skipped) => {
                skipped_files.fetch_add(1, Ordering::Relaxed);
            }
            Err(e) => {
                eprintln!("Error processing {}: {}", file_path, e);
                error_count.fetch_add(1, Ordering::Relaxed);
            }
        }
    }

    pb.finish_with_message("Complete!");

    let final_errors = error_count.load(Ordering::Relaxed);
    let final_completed = completed_files.load(Ordering::Relaxed);
    let final_skipped = skipped_files.load(Ordering::Relaxed);

    println!("Successfully processed: {}", final_completed);
    if final_skipped > 0 {
        println!(
            "Skipped (no transparent pixels or already processed): {}",
            final_skipped
        );
    }
    if final_errors > 0 {
        println!("Errors: {}", final_errors);
    }

    let duration = start_time.elapsed();
    let seconds = duration.as_secs_f64();
    println!("Done in: {:.2}s", seconds);

    wait_for_any_key();
    Ok(())
}

#[derive(Debug)]
enum ProcessResult {
    Success,
    Skipped,
}

fn process_image_optimized(
    file_path: &str,
    dbg_mode: bool,
) -> Result<ProcessResult, Box<dyn std::error::Error + Send + Sync>> {
    let mut img = image::open(file_path)?.into_rgba8();
    let (width, height) = img.dimensions();

    if width > 16384 || height > 16384 {
        return Err(format!(
            "Image too large: {}x{} - skipping for safety",
            width, height
        )
        .into());
    }

    let border_pixels = find_border_pixels_parallel(&img)?;

    if border_pixels.is_empty() {
        return Ok(ProcessResult::Skipped);
    }

    let tree = build_kdtree(&border_pixels)?;

    process_transparent_pixels_parallel(&tree, &border_pixels, &mut img, dbg_mode)?;

    img.save(file_path)?;
    Ok(ProcessResult::Success)
}

fn find_border_pixels_parallel(
    img: &RgbaImage,
) -> Result<Vec<([f64; 2], [u8; 3])>, Box<dyn std::error::Error + Send + Sync>> {
    let (width, height) = img.dimensions();

    let mut border_pixels: Vec<_> = (0..height)
        .into_par_iter()
        .flat_map(|y| {
            let mut row_borders = Vec::new();

            for x in 0..width {
                let pixel = img.get_pixel(x, y);
                if pixel[3] > 0 {
                    let is_border = NEIGHBOR_LOCATIONS.iter().any(|&(dx, dy)| {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        nx >= 0
                            && nx < width as i32
                            && ny >= 0
                            && ny < height as i32
                            && img.get_pixel(nx as u32, ny as u32)[3] == 0
                    });

                    if is_border {
                        row_borders.push(([x as f64, y as f64], [pixel[0], pixel[1], pixel[2]]));
                    }
                }
            }

            row_borders
        })
        .collect();

    border_pixels.truncate(MAX_BORDER_PIXELS);

    Ok(border_pixels)
}

fn build_kdtree(
    border_pixels: &[([f64; 2], [u8; 3])],
) -> Result<KdTree<f64, usize, 2, 64, u32>, Box<dyn std::error::Error + Send + Sync>> {
    use std::collections::HashMap;

    let mut position_map: HashMap<(i32, i32), usize> = HashMap::new();
    let mut jittered_positions: Vec<([f64; 2], usize)> = Vec::new();

    for (i, (pos, _)) in border_pixels.iter().enumerate() {
        let grid_key = (pos[0] as i32, pos[1] as i32);

        if let std::collections::hash_map::Entry::Vacant(e) = position_map.entry(grid_key) {
            e.insert(i);

            let jitter_x = (i as f64 * 0.00001) % 0.001;
            let jitter_y = ((i * 7) as f64 * 0.00001) % 0.001;

            jittered_positions.push(([pos[0] + jitter_x, pos[1] + jitter_y], i));
        }
    }

    let mut tree: KdTree<f64, usize, 2, 64, u32> = KdTree::new();

    for (jittered_pos, original_index) in jittered_positions {
        tree.add(&jittered_pos, original_index);
    }

    Ok(tree)
}

fn process_transparent_pixels_parallel(
    tree: &KdTree<f64, usize, 2, 64, u32>,
    border_pixels: &[([f64; 2], [u8; 3])],
    img: &mut RgbaImage,
    dbg_mode: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    img.enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, y, pixel)| {
            if pixel[3] == 0 {
                let nearest = tree.nearest_one::<SquaredEuclidean>(&[x as f64, y as f64]);
                if nearest.item < border_pixels.len() {
                    let color = border_pixels[nearest.item].1;
                    *pixel = Rgba([color[0], color[1], color[2], if dbg_mode { 255 } else { 0 }]);
                }
            }
        });

    Ok(())
}

fn collect_png_files(dir: &Path) -> Result<Vec<String>, std::io::Error> {
    let mut png_files = Vec::new();

    fn visit_dir(dir: &Path, files: &mut Vec<String>) -> std::io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension.to_ascii_lowercase() == "png" {
                        if let Some(path_str) = path.to_str() {
                            files.push(path_str.to_string());
                        }
                    }
                }
            } else if path.is_dir() {
                visit_dir(&path, files)?;
            }
        }
        Ok(())
    }

    visit_dir(dir, &mut png_files)?;
    png_files.sort();
    Ok(png_files)
}

fn print_usage() {
    println!("pixelfix \"path to file\" to fix transparent pixels in file");
    println!("pixelfix \"path to file\" \"path to file 2\" to fix multiple files");
    println!("pixelfix \"path/to/folder\" to fix all PNG files in a folder (recursive)");
    println!("pixelfix -d \"path to file\" to view debug output");
}

#[cfg(windows)]
fn wait_for_any_key() {
    use std::thread;
    use std::time::Duration;
    use winapi::um::winuser::{GetAsyncKeyState, VK_LBUTTON};

    println!("Press any key to exit");
    loop {
        for vk_code in 1..=254 {
            if vk_code != VK_LBUTTON as i32 {
                unsafe {
                    if GetAsyncKeyState(vk_code) as u16 & 0x8000 != 0 {
                        return;
                    }
                }
            }
        }
        thread::sleep(Duration::from_millis(50));
    }
}

#[cfg(not(windows))]
fn wait_for_any_key() {
    println!("Exiting...");
}