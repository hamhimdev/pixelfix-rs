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

## Installing

I don't have prebuilt binaries for MacOS yet, but I do for Windows and Linux. If you are comfortable with manually building the program for use in MacOS, or if you wish to build the program to use yourself, a quick guide is provided below. Windows and Linux users can get a binary built by me from the [Releases page.](https://codeberg.org/hamhim/pixelfix-rs/releases)

With the binaries, you can use it directly by defining it's path, or set it up to be accessible globally.  

### Windows

#### Method 1 - Add pixelfix to the right click send to feature on explorer

0. Have `pixelfix.exe` downloaded
1. Open the Run dialogue with Windows Key + R
2. Insert `shell:sendto` as shown below

![Windows' Run dialogue, with shell:sendto inserted](https://codeberg.org/hamhim/pixelfix-rs/raw/branch/main/repository/shellsendto.webp)

3. Press `OK` or the Enter key, it should open a File Explorer window
4. Move the `pixelfix.exe` binary to that directory
5. Done! You can use pixelfix on folders and files by right clicking on them and hovering over the "Share To" option, then selecting `pixelfix.exe`. On Windows 11, you may have to click on "Show more options" to bring back the Windows 10 context menu.

### Building

Follow the instructions [on the Rust language website](https://www.rust-lang.org/learn/get-started) to install Rust and you optionally need git, if you don't have git download a zip from Codeberg.  

1. Download the repository using git: `git clone https://codeberg.org/hamhim/pixelfix-rs`
2. Change the directory to be git: `cd pixelfix-rs`
3. Build using cargo: `cargo build # For native installation`
4. Done! It'll be in /target/release/pixelfix.
5. On linux you _may_ need to make it executable, you can do this through your file manager or by running: `chmod +x thePathToThePixelfixBinary`

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

## Licensing

The program is under the MIT License. [The repository has the clause attached here.](https://codeberg.org/hamhim/pixelfix-rs/src/branch/main/LICENSE)
