# rs-Heart-Strings

###### Unarchived 2024-11-17: to learn to publish this thing into crates.io properly, i might as well make a few more changes.

###### ~~ARCHIVED 2023-06-13: i've considered this project done for now. but also, turns out that money is a thing and that i won't have an internet connection after 15/june/2023, which will massively hinder my ability to study. send [help](https://liberapay.com/wilker/), please.~~

## Make hearts to your hearts content!

This is my second ever project finished in my process of learning Rust! (if you consider a [hello-world](/Wilker-uwu/rs-hello-world) a project at all).

## Installation:

All listed methods requires `cargo` to be installed. You can get it with the Rustup toolchain at <https://rustup.rs/>.

### From Cargo:

You can simply install from the listed crate at `crates.io` using

```bash
cargo install heart-strings
```

### From source:

After cloning the repo with git, simply build and have at it!

```bash
git clone https://github.com/Wilker-uwu/rs-heart-strings.git
cd rs-heart-strings
cargo build --release
cargo install --path .
```

## Usage:

Throw a bunch of numbers and see hearts fly!

```bash
hearts [HEARTS...]
```
...where HEARTS is just a number of hearts that you choose! Multiple numbers means multiple lines of hearts!

#### Example:

```bash
$ hearts 5 32 16 54
💕💗💓💖💖
💖💞💖💞💓💞💝❣️💕❤️💘❤️💞💘💓💝💓❣️💝💝💝❤️❣️💕💞❣️💗💖❣️💞❤️❣️
💗💖❤️💝💗💗💞❣️💓💝💗❣️💘💗💓💞
💖❤️💓💕💘❣️💞💓💘💘💓💘💗❣️💘💕💗💘💝💝💓💕💗❤️💞💕💘💞💝💞💗💞💝❤️💞💞💗💞❤️💘💓💕❤️💝❣️❤️💝❣️💘💗❣️💖❤️💕
```
###### IMPORTANT: anything else other than numbers will NOT work!
