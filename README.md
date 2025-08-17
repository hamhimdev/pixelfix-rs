# pixelfix-rs

**pixelfix-rs** is a Rust implementation of [Corecii’s Transparent Pixel Fix (pixelfix)](https://github.com/Corecii/Transparent-Pixel-Fix).

Here's some statistics compared to Corecii's project:
* **100–120x or 95%+ faster** in my testing (below)
* About **10x smaller** (due to being a compiled binary)
* Works as a **drop-in replacement** for pixelfix

<video src="https://codeberg.org/hamhim/pixelfix-rs/raw/branch/main/repository/showcase.webm" 
       width="600" 
       controls 
       loop 
       muted>
</video>

## Features
* Fix one image, many images, or an entire folder (recursive)
* Progress bar with ETA
* FAST!!!

## Usage

```sh
# Fix a single file
pixelfix image.png

# Fix multiple files
pixelfix image1.png image2.png

# Fix every PNG in a folder (recursive)
pixelfix path/to/folder

# Debug mode (shows replaced pixels instead of leaving them transparent)
pixelfix -d image.png
```

## Why is this needed?

I would recommend reading the last section of [Corecii's repository](https://github.com/Corecii/Transparent-Pixel-Fix#more-info), they have a great explaination for this.