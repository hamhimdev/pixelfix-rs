<p align="center">
  <img src="https://codeberg.org/hamhim/pixelfix-rs/raw/branch/main/repository/pixelfix.webp" height="128" alt="pixelfix logo" style="vertical-align: middle;"/>
</p>

<h1 align="center">Pixelfix</h1>

---

**pixelfix-rs** is a Rust implementation of [Corecii’s Transparent Pixel Fix (pixelfix)](https://github.com/Corecii/Transparent-Pixel-Fix).

Here's some statistics compared to Corecii's project:

- **100–120x or 95%+ faster** in my testing (shown below)
- About **10x smaller** in file size (due to being a compiled binary)
- Works as a **drop-in replacement**

<video src="https://codeberg.org/hamhim/pixelfix-rs/raw/branch/main/repository/showcase.webm" width="600" controls loop muted></video>

https://codeberg.org/hamhim/pixelfix-rs/raw/branch/main/repository/showcase.webm

## Features

- Fix one image, many images, or an entire folder (recursive)
- Progress bar with ETA
- FAST!!!

## Installing

Currently, only Windows and Linux (and NixOS) is formally supported. Pixelfix should theorethically work on MacOS as it is platform agnostic, you can look at the build instructions a little below if you want to use the program or install cargo to install pixelfix through it.

With the binaries, you can use it directly by defining it's path, or set it up to be accessible globally. Windows and Linux users can get a binary built by me from the [Releases page on Codeberg](https://codeberg.org/hamhim/pixelfix-rs/releases).

### Cargo (All platforms)

Cargo is a package manager for Rust that installs packages and software ("crates") from [crates.io](https://crates.io/). You can find instructions to install Cargo [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

After installation, you can run the following on any platform to install pixelfix globally:

```sh
cargo install pixelfix
```

This is really only recommended on MacOS because this also requires installing Rust alongside and build dependencies. Use other methods if you can.

### Windows

#### Release Binary

Builds of pixelfix for Windows is provided in the [Releases page on Codeberg](https://codeberg.org/hamhim/pixelfix-rs/releases).

#### Cargo

See [Cargo (All platforms)](#cargo-all-platforms).

#### Usage

##### Method 1 - Add pixelfix to the right click context menu "send to" feature on explorer

1. Have `pixelfix.exe` downloaded
2. Open the Run dialogue with Windows Key + R
3. Insert `shell:sendto` as shown below

   ![Windows' Run dialogue, with shell:sendto inserted](https://codeberg.org/hamhim/pixelfix-rs/raw/branch/main/repository/shellsendto.webp)

4. Press `OK` or the Enter key, it should open a File Explorer window
5. Move the `pixelfix.exe` binary to that directory
6. Done! You can use pixelfix on folders and files by right clicking on them and hovering over the "Send to" option, then selecting `pixelfix.exe`. On Windows 11, you may have to click on **Show more options** to bring back the Windows 10 context menu.

### MacOS

I don't have builds for MacOS. You'll need to [build](#building) yourself or install with cargo.

#### Cargo

See [Cargo (All platforms)](#cargo-all-platforms).

### Linux

#### Using an installation script (Recommended)

You can directly run the following command in your terminal to automatically install pixelfix and make it globally accessible.

```sh
curl -s https://cutely.strangled.net/pixelfix.sh | bash
```

#### Cargo

See [Cargo (All platforms)](#cargo-all-platforms).

#### Nix

If you're just using the Nix package manager (not NixOS), use the installation script above for the easiest setup.

For Nix users who prefer the flake approach:

```bash
# Install permanently to user profile
nix profile install git+https://codeberg.org/hamhim/pixelfix-rs

# Use once without installing
nix run git+https://codeberg.org/hamhim/pixelfix-rs -- image.png

# Replace with github:hamhimdev/pixelfix-rs/main if you prefer to use GitHub
```

#### NixOS

A flake is provided that you can use to install the program on NixOS systems:

`flake.nix`

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    pixelfix.url = "git+https://codeberg.org/hamhim/pixelfix-rs";
    # You can also use https://git.clickette.org/hamhim/pixelfix-rs, but Codeberg will be faster and more reliable.
    # If you prefer, you can replace this with a GitHub mirror by using "github:hamhimdev/pixelfix-rs/main"
  };

  outputs = { self, nixpkgs, pixelfix }: {
    nixosConfigurations.yourhostname = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        ./configuration.nix
        {
          environment.systemPackages = [
            pixelfix.packages.x86_64-linux.default
          ];
        }
      ];
    };
  };
}
```

Optionally, you can also add the system package directly in your `configuration.nix`. To do so you will need to define the pixelfix input in the file parameters. On the top:

`configuration.nix`

```nix
{
  config,
  pkgs,
  pixelfix,
  ...
}:
```

And then add it to your packages:

`configuration.nix`

```nix
{
  environment.systemPackages = [
    pixelfix.packages.${pkgs.system}.default
    # ...
  ];
}
```

#### Home Manager

For Home Manager users:

`home.nix`

```nix
{ config, pkgs, ... }:
{
  home.packages = [
    inputs.pixelfix.packages.${pkgs.system}.default
  ];
}
```

### Building

Follow the instructions [on the Rust language website](https://www.rust-lang.org/tools/install) to install Rust. You will also need Git; if you don't have it, [get it here](https://git-scm.com/downloads). Alternatively, you can download [an archive file from Codeberg](https://codeberg.org/hamhim/pixelfix-rs/archive/main.zip) or [GitHub](https://github.com/hamhimdev/pixelfix-rs/archive/refs/heads/main.zip).

1. **Get the source code** using one of the following methods:

   - **a. Git (Recommended):** Clone the repository.

     ```sh
     git clone https://codeberg.org/hamhim/pixelfix-rs
     ```

     A mirror exists on https://git.clickette.org/hamhim/pixelfix-rs and https://github.com/hamhimdev/pixelfix-rs. You can replace the url with this for the same result.

   - **b. Direct Download:** Download and extract the `.zip` archive from Codeberg.

2. **Navigate into the project directory:**

   ```sh
   cd pixelfix-rs
   ```

   _(Note: Codeberg should append the branch name of the repository if you download a zip or tarball archive, and GitHub should only use the name of the branch, in this case you need to use `pixelfix-rs-main` or `main` respectively.)_.

3. **Build the project** using Cargo:

   ```sh
   cargo build --release
   ```

   With Nix or NixOS you can also use:

   ```sh
   nix build
   ```

4. **Done\!** The executable will be in the `target/release/` directory. For example: `target/release/pixelfix`. If you built with nix, it should also be in `result/bin/pixelfix` (symlink to nix directory of where it built pixelfix).

5. **(Linux (and maybe macOS?) Only)** You shouldn't have to, but you may need to make it executable using one of the following methods:

   - **a. Using `chmod` in the terminal**

     ```sh
     chmod +x target/release/pixelfix
     ```

   - **b. Using your file manager**

     Most Linux file managers can often make a file executable. To do this, right-click the file, go to its **Properties** or **Permissions**, and check the box to **Allow executing file as a program** or a similar option.

     #### Dolphin (KDE Plasma)

     On Dolphin, you can find the option by going into the file's properties and going to permissions to check "Allow executing file as program".

     ![Dolphin inside KDE Plasma on the properties menu of pixelfix](https://codeberg.org/hamhim/pixelfix-rs/raw/branch/main/repository/kdePlasma.webp)

     #### Nemo (Cinnamon)

     On Nemo, you can find the option by going into the file's properties and going to permissions to check "Allow executing file as program".

     ![Nemo inside KDE Plasma on the properties menu of pixelfix](https://codeberg.org/hamhim/pixelfix-rs/raw/branch/main/repository/cinnamon.webp)

## Usage

```sh
# Fix a single file
pixelfix image.png

# Fix multiple files
pixelfix image1.png image2.png

# Fix every PNG in a folder (recursive)
pixelfix path/to/folder

# Fix every PNG in multiple folders (recursive)
pixelfix path/to/folder1 path/to/folder2

# Debug mode (shows replaced pixels instead of leaving them transparent)
pixelfix -d image.png
```

## Why is this needed?

I would recommend reading the last section of [Corecii's repository](https://github.com/Corecii/Transparent-Pixel-Fix#more-info), they have a great explaination for this.

## Licensing

The program is under the MIT License. [The repository has the clause attached here](./src/branch/main/LICENSE).
