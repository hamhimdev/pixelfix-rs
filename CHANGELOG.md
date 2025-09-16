# Changelog

## v0.1.4 (17 September 2025)

#### More freedom!

For the large image warning, it will now remember your choice for the full batch of processing. Good luck to your CPU. I'm not responsible for any damage caused by pixelfix.

![Pixelfix](https://codeberg.org/hamhim/pixelfix-rs/raw/branch/main/repository/changelog/0.1.4/meow.webp)

#### Realistically:

Pixelfix should be able to handle ~4k images perfectly fine without killing your computer, it's the parallel processing that will since if you have multiple ~4k images... maybe ask your exporter of choice to not do this?

#### By the way

I forgot to make binary builds for 0.1.1/2 and 3, there will be ones for v0.1.4 now that I have realised. I was too busy with getting the Nix flake working.

## v0.1.3 (16 September 2025)

Wrong hash in flake, oops. Also, turns out, if you don't have ipv6 (and this is from my little testing) you can't use codeberg as the flake source. I also didn't mention a pretty major step if you wanted to install pixelfix from configuration.nix. Second minor readme update!

## v0.1.2 (16 September 2025)

Wrong cargo command in readme. I don't think you can edit publishes on crates.io.

## v0.1.1 (16 September 2025)

Most of the changes were just refactoring of the code. Here are key updates that isn't refactoring:

#### Cargo

Pixelfix is now on https://crates.io, which means on any device that can run Rust and Cargo, you can simply run the following assuming you have them installed:

```sh
cargo install pixelfix
```

#### NixOS & Nix

Since I use NixOS (btw), I wanted to make a flake to use pixelfix with. It's here now, instructions to use it is in the readme.

#### GitHub mirror

Pixelfix is now mirrored to GitHub. Only reason is so people can use it in their flake, since Codeberg can experience downtime because they dont have Microsoft backing them.

#### Limits

Increased limit of image processing from `16384x16384` to `65536x65536`. I don't know why you would want to go larger. If you're trying to pixelfix an image larger than `4096x4096`, pixelfix will now put a warning telling you "what the fuck are you doing? are you ok?"

<video src="https://codeberg.org/hamhim/pixelfix-rs/raw/branch/main/repository/changelog/0.1.1/meow.webm" width="600" controls loop muted></video>

https://codeberg.org/hamhim/pixelfix-rs/raw/branch/main/repository/changelog/0.1.1/meow.webm

#### Improved Readme

I spent some time writing documentation for pixelfix, making everything clear and easy to read. I might have put a little bit too much time, but a cool thing came out of it! Pixelfix logo! It isn't anything special but it'll do.

<p align="center">
    <img src="https://codeberg.org/hamhim/pixelfix-rs/raw/branch/main/repository/pixelfix.webp" height="128" alt="pixelfix logo" style="vertical-align: middle;"/>
</p>

Some changes I made include the following:
- Nix/NixOS Install guide
- Raw video urls since GitHub doesn't support video tags apparently?
- Organized the list a bit better
- Cargo install instructions
- idk what else i did a lot of stuff

#### Stuff that I'm going to do

- Get pixelfix on winget
- Make MacOS builds
- More optimizations (it needs to be even more unecessarily fast!!!)

## v0.1.0 (Initial Release)

- The actual thing
