# pixelfix-rs

**pixelfix-rs** is a Rust implementation of [Corecii’s Transparent Pixel Fix (pixelfix)](https://github.com/Corecii/Transparent-Pixel-Fix).

Here's some statistics compared to Corecii's project:

* **100–120x or 95%+ faster** in my testing (shown below)
* About **10x smaller** (due to being a compiled binary)
* Works as a **drop-in replacement**

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

I don't have prebuilt binaries for macOS yet, but I do for Windows and Linux. If you are comfortable with manually building the program for use in macOS, or if you wish to build the program to use yourself, a quick guide is provided below. Windows and Linux users can get a binary built by me from the [Releases page](https://codeberg.org/hamhim/pixelfix-rs/releases).

With the binaries, you can use it directly by defining it's path, or set it up to be accessible globally.  

### Windows

#### Method 1 - Add pixelfix to the right click context menu "send to" feature on explorer

1. Have `pixelfix.exe` downloaded
2. Open the Run dialogue with Windows Key + R
3. Insert `shell:sendto` as shown below

    ![Windows' Run dialogue, with shell:sendto inserted](https://codeberg.org/hamhim/pixelfix-rs/raw/branch/main/repository/shellsendto.webp)

4. Press `OK` or the Enter key, it should open a File Explorer window
5. Move the `pixelfix.exe` binary to that directory
6. Done! You can use pixelfix on folders and files by right clicking on them and hovering over the "Send to" option, then selecting `pixelfix.exe`. On Windows 11, you may have to click on "Show more options" to bring back the Windows 10 context menu.

### Linux

#### Using an installation script

You can directly run the following command in your terminal to automatically install pixelfix and make it globally accessible.

```sh
curl -s https://cutely.strangled.net/pixelfix.sh | bash
```

### Building

Follow the instructions [on the Rust language website](https://www.rust-lang.org/tools/install) to install Rust. You will also need Git; if you don't have it, [get it here](https://git-scm.com/downloads). Alternatively, you can download [an archive from Codeberg](https://codeberg.org/hamhim/pixelfix-rs/archive/main.zip).

1. **Get the source code** using one of the following methods:

      * **a. Git (Recommended):** Clone the repository.

        ```sh
        git clone https://codeberg.org/hamhim/pixelfix-rs
        ```

      * **b. Direct Download:** Download and extract the `.zip` archive from Codeberg.

2. **Navigate into the project directory:**

    ```sh
    cd pixelfix-rs
    ```

    *(Note: Codeberg should append the branch name of the repository if you download a zip or tarball archive, in this case you need to use `pixelfix-rs-main`)*.

3. **Build the project** using Cargo:

    ```sh
    cargo build --release
    ```

4. **Done\!** The executable will be in the `target/release/` directory. For example: `target/release/pixelfix`.

5. **(Linux (and maybe macOS?) Only)** You shouldn't have to, but you may need to make it executable using one of the following methods:

      * **a. Using `chmod` in the terminal**

        ```sh
        chmod +x target/release/pixelfix
        ```

      * **b. Using your file manager**

        Most Linux file managers can often make a file executable. To do this, right-click the file, go to its **Properties** or **Permissions**, and check the box to **Allow executing file as a program** or a similar option.

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

The program is under the MIT License. [The repository has the clause attached here](https://codeberg.org/hamhim/pixelfix-rs/src/branch/main/LICENSE).

##### This repository is mirrored to [Clickette Git](https://git.clickette.org/hamhim/pixelfix-rs) and worked on [Codeberg](https://codeberg.org/hamhim/pixelfix-rs).