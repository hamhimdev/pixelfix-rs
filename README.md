
# pixelfix-rs

**pixelfix-rs** is a Rust implementation of [Corecii’s Transparent Pixel Fix (pixelfix)](https://github.com/Corecii/Transparent-Pixel-Fix).

Here's some statistics compared to Corecii's project:
* **100–120x or 95%+ faster** in my testing (shown below)
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

## Installation
I don't have prebuilt binaries for MacOS yet, but I do for Windows and Linux. If you know how to, you probably can build the program on MacOS yourself and it should work fine. Windows and Linux users can get it from the [Releases page.](https://codeberg.org/SystemSniper/Rooftop/releases)

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
