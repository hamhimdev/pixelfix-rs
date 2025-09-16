use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use image::{Rgba, RgbaImage};
use indicatif::{ProgressBar, ProgressStyle};
use kiddo::SquaredEuclidean;
use kiddo::float::kdtree::KdTree;
use rayon::prelude::*;

const NEIGHBOR_OFFSETS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
];

const JITTER_SCALE: f64 = 0.00001;
const JITTER_RANGE: f64 = 0.001;

#[derive(Debug)]
pub enum ProcessResult {
    Success,
    Skipped,
}

pub struct BorderPixel {
    pub position: [f64; 2],
    pub color: [u8; 3],
}

pub struct ProcessingStats {
    pub completed: AtomicUsize,
    pub skipped: AtomicUsize,
    pub errors: AtomicUsize,
}

impl ProcessingStats {
    pub fn new() -> Self {
        Self {
            completed: AtomicUsize::new(0),
            skipped: AtomicUsize::new(0),
            errors: AtomicUsize::new(0),
        }
    }

    pub fn print_summary(&self) {
        let completed = self.completed.load(Ordering::Relaxed);
        let skipped = self.skipped.load(Ordering::Relaxed);
        let errors = self.errors.load(Ordering::Relaxed);

        println!("Successfully processed: {}", completed);
        if skipped > 0 {
            println!(
                "Skipped (no transparent pixels or already processed): {}",
                skipped
            );
        }
        if errors > 0 {
            println!("Errors: {}", errors);
        }
    }
}

pub struct LargeImagePolicy {
    pub allow_large_images: AtomicBool,
    pub user_has_decided: AtomicBool,
}

impl LargeImagePolicy {
    pub fn new() -> Self {
        Self {
            allow_large_images: AtomicBool::new(false),
            user_has_decided: AtomicBool::new(false),
        }
    }

    pub fn should_process_large_image(
        &self,
        width: u32,
        height: u32,
        file_path: &str,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        if width <= 4096 && height <= 4096 {
            return Ok(true);
        }

        if self.user_has_decided.load(Ordering::Relaxed) {
            return Ok(self.allow_large_images.load(Ordering::Relaxed));
        }

        let megapixels = (width as f64 * height as f64) / 1_000_000.0;
        println!();
        println!(
            "⚠️ Large image detected: {}x{} ({:.1} MP)",
            width, height, megapixels
        );
        println!("File: {}", extract_filename(file_path));
        println!("Processing large images will take significant time and system memory.");

        let allow = get_user_confirmation("Continue processing large images?")?;

        self.allow_large_images.store(allow, Ordering::Relaxed);
        self.user_has_decided.store(true, Ordering::Relaxed);

        if allow {
            println!("Processing all images...");
        } else {
            println!("Good idea. Skipping all large images in processing.");
        }
        println!();

        Ok(allow)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = std::time::Instant::now();
    let (file_paths, debug_mode) = parse_arguments()?;

    if file_paths.is_empty() {
        print_usage();
        return Ok(());
    }

    println!("Found {} files to process", file_paths.len());

    let progress_bar = create_progress_bar(file_paths.len());
    let stats = Arc::new(ProcessingStats::new());
    let large_image_policy = Arc::new(LargeImagePolicy::new());

    process_files_parallel(
        &file_paths,
        debug_mode,
        &progress_bar,
        &stats,
        &large_image_policy,
    );

    progress_bar.finish_with_message("Complete!");
    stats.print_summary();

    let duration = start_time.elapsed();
    println!("Done in: {:.2}s", duration.as_secs_f64());

    wait_for_any_key();
    Ok(())
}

fn parse_arguments() -> Result<(Vec<String>, bool), Box<dyn std::error::Error>> {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let mut debug_mode = false;

    args.retain(|arg| {
        if arg == "-d" {
            debug_mode = true;
            false
        } else {
            true
        }
    });

    if args.is_empty() {
        return Ok((Vec::new(), debug_mode));
    }

    println!("Collecting files...");
    let file_paths = collect_file_paths(args)?;

    Ok((file_paths, debug_mode))
}

fn collect_file_paths(args: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut file_paths = Vec::new();

    for arg in args {
        let path = Path::new(&arg);
        if path.is_dir() {
            match collect_png_files_recursive(path) {
                Ok(mut png_files) => file_paths.append(&mut png_files),
                Err(e) => eprintln!("Error reading directory {}: {}", arg, e),
            }
        } else if path.is_file() {
            file_paths.push(arg);
        } else {
            eprintln!("Path not found: {}", arg);
        }
    }

    Ok(file_paths)
}

fn collect_png_files_recursive(dir: &Path) -> Result<Vec<String>, std::io::Error> {
    let mut png_files = Vec::new();

    fn visit_directory(dir: &Path, files: &mut Vec<String>) -> std::io::Result<()> {
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
                visit_directory(&path, files)?;
            }
        }
        Ok(())
    }

    visit_directory(dir, &mut png_files)?;
    png_files.sort();
    Ok(png_files)
}

fn create_progress_bar(total_files: usize) -> ProgressBar {
    let pb = ProgressBar::new(total_files as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.red} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} files ({eta})\nProcessing: {msg}")
            .unwrap()
            .progress_chars("#>-")
    );
    pb
}

fn process_files_parallel(
    file_paths: &[String],
    debug_mode: bool,
    progress_bar: &ProgressBar,
    stats: &Arc<ProcessingStats>,
    large_image_policy: &Arc<LargeImagePolicy>,
) {
    let results: Vec<_> = file_paths
        .par_iter()
        .map(|file_path| {
            let filename = extract_filename(file_path);
            progress_bar.set_message(filename.to_string());

            let result = process_single_image(file_path, debug_mode, large_image_policy);
            progress_bar.inc(1);

            (file_path.clone(), result)
        })
        .collect();

    for (file_path, result) in results {
        match result {
            Ok(ProcessResult::Success) => {
                stats.completed.fetch_add(1, Ordering::Relaxed);
            }
            Ok(ProcessResult::Skipped) => {
                stats.skipped.fetch_add(1, Ordering::Relaxed);
            }
            Err(e) => {
                eprintln!("Error processing {}: {}", file_path, e);
                stats.errors.fetch_add(1, Ordering::Relaxed);
            }
        }
    }
}

fn extract_filename(file_path: &str) -> &str {
    Path::new(file_path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(file_path)
}

fn process_single_image(
    file_path: &str,
    debug_mode: bool,
    large_image_policy: &Arc<LargeImagePolicy>,
) -> Result<ProcessResult, Box<dyn std::error::Error + Send + Sync>> {
    let mut image = load_and_validate_image(file_path, large_image_policy)?;

    let border_pixels = find_border_pixels(&image)?;
    if border_pixels.is_empty() {
        return Ok(ProcessResult::Skipped);
    }

    let spatial_index = build_spatial_index(&border_pixels)?;
    fill_transparent_pixels(&spatial_index, &border_pixels, &mut image, debug_mode)?;

    image.save(file_path)?;
    Ok(ProcessResult::Success)
}

fn load_and_validate_image(
    file_path: &str,
    large_image_policy: &Arc<LargeImagePolicy>,
) -> Result<RgbaImage, Box<dyn std::error::Error + Send + Sync>> {
    let image = image::open(file_path)?.into_rgba8();
    let (width, height) = image.dimensions();

    if !large_image_policy.should_process_large_image(width, height, file_path)? {
        return Err("Skipped: Answered \"no\" to processing large images".into());
    }

    // why?
    if width > 65536 || height > 65536 {
        return Err(format!(
            "Image too large: {}x{} - maximum supported size is 65536x65536 (why are you working with such huge images?)",
            width, height
        ).into());
    }

    Ok(image)
}

fn find_border_pixels(
    image: &RgbaImage,
) -> Result<Vec<BorderPixel>, Box<dyn std::error::Error + Send + Sync>> {
    let (width, height) = image.dimensions();

    let border_pixels: Vec<BorderPixel> = (0..height)
        .into_par_iter()
        .flat_map(|y| {
            let mut row_borders = Vec::new();

            for x in 0..width {
                let pixel = image.get_pixel(x, y);

                if pixel[3] > 0 && is_border_pixel(image, x, y, width, height) {
                    row_borders.push(BorderPixel {
                        position: [x as f64, y as f64],
                        color: [pixel[0], pixel[1], pixel[2]],
                    });
                }
            }

            row_borders
        })
        .collect();

    Ok(border_pixels)
}

fn is_border_pixel(image: &RgbaImage, x: u32, y: u32, width: u32, height: u32) -> bool {
    NEIGHBOR_OFFSETS.iter().any(|&(dx, dy)| {
        let neighbor_x = x as i32 + dx;
        let neighbor_y = y as i32 + dy;

        neighbor_x >= 0
            && neighbor_x < width as i32
            && neighbor_y >= 0
            && neighbor_y < height as i32
            && image.get_pixel(neighbor_x as u32, neighbor_y as u32)[3] == 0
    })
}

fn build_spatial_index(
    border_pixels: &[BorderPixel],
) -> Result<KdTree<f64, usize, 2, 64, u32>, Box<dyn std::error::Error + Send + Sync>> {
    let mut position_deduplication: HashMap<(i32, i32), usize> = HashMap::new();
    let mut unique_positions: Vec<([f64; 2], usize)> = Vec::new();

    for (index, border_pixel) in border_pixels.iter().enumerate() {
        let grid_key = (
            border_pixel.position[0] as i32,
            border_pixel.position[1] as i32,
        );

        if let std::collections::hash_map::Entry::Vacant(entry) =
            position_deduplication.entry(grid_key)
        {
            entry.insert(index);

            let jittered_position = apply_position_jitter(&border_pixel.position, index);
            unique_positions.push((jittered_position, index));
        }
    }

    let mut spatial_tree: KdTree<f64, usize, 2, 64, u32> = KdTree::new();
    for (position, original_index) in unique_positions {
        spatial_tree.add(&position, original_index);
    }

    Ok(spatial_tree)
}

fn apply_position_jitter(position: &[f64; 2], seed: usize) -> [f64; 2] {
    let jitter_x = (seed as f64 * JITTER_SCALE) % JITTER_RANGE;
    let jitter_y = ((seed * 7) as f64 * JITTER_SCALE) % JITTER_RANGE;

    [position[0] + jitter_x, position[1] + jitter_y]
}

fn fill_transparent_pixels(
    spatial_index: &KdTree<f64, usize, 2, 64, u32>,
    border_pixels: &[BorderPixel],
    image: &mut RgbaImage,
    debug_mode: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    image
        .enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, y, pixel)| {
            if pixel[3] == 0 {
                fill_transparent_pixel(spatial_index, border_pixels, x, y, pixel, debug_mode);
            }
        });

    Ok(())
}

fn fill_transparent_pixel(
    spatial_index: &KdTree<f64, usize, 2, 64, u32>,
    border_pixels: &[BorderPixel],
    x: u32,
    y: u32,
    pixel: &mut Rgba<u8>,
    debug_mode: bool,
) {
    let query_point = [x as f64, y as f64];
    let nearest_result = spatial_index.nearest_one::<SquaredEuclidean>(&query_point);

    if nearest_result.item < border_pixels.len() {
        let nearest_color = border_pixels[nearest_result.item].color;
        let alpha = if debug_mode { 255 } else { 0 };
        *pixel = Rgba([nearest_color[0], nearest_color[1], nearest_color[2], alpha]);
    }
}

fn get_user_confirmation(prompt: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    use std::io::{self, Write};

    print!("   {} (y/n): ", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let response = input.trim().to_lowercase();
    Ok(response == "y" || response == "yes")
}

fn print_usage() {
    println!("pixelfix \"path/to/file.png\" - Fix single file");
    println!("pixelfix \"file1.png\" \"file2.png\" - Fix multiple files");
    println!(
        "pixelfix \"path/to/folder\" \"path/to/folder2\" - Fix all PNG files in folder(s) (recursive)"
    );
    println!(
        "pixelfix -d \"path/to/file.png\" - Enable debug mode (makes transparent pixels visible)"
    );
}

#[cfg(windows)]
fn wait_for_any_key() {
    use std::thread;
    use std::time::Duration;
    use winapi::um::winuser::{GetAsyncKeyState, VK_LBUTTON};

    println!("Press any key to exit...");
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
    println!("Process completed.");
}
