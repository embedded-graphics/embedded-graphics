# Development setup

## Minimum supported Rust version

The minimum supported Rust version for embedded-graphics is `1.40.0` or greater.
Ensure you have the latest stable version of Rust installed, preferably through <https://rustup.rs>.

## Ubuntu/Linux Mint

```bash
# Update to latest stable version of Rust
rustup update

# Ensure rustfmt is installed
rustup component add rustfmt

# Install `cargo-readme`
cargo install cargo-readme

# Install SDL2 for simulator and linkchecker for build script

# Python 2 systems (Ubuntu older than 20.04, Linux Mint 19, etc)
sudo apt install libsdl2-dev linkchecker

# OR

# Python 3 systems (Ubuntu 20.04+, Linux Mint 20, etc)
sudo apt install python3-pip
sudo pip3 install git+https://github.com/linkchecker/linkchecker.git
```

## Generating readmes

The various `README.md` files in this project are generated from each crate's `lib.rs` comment. To
regenerate a readme, ensure [`cargo-readme`](https://crates.io/crates/cargo-readme) is installed
then run:

```bash
./readme.sh <crate>

# e.g.
./readme.sh simulator
```

All readmes can be generated at the same time by running the `./generate_readmes.sh` script in the project root.

Running `./build.sh` will check if the readme was successfully updated. The updated `README.md`
should be committed into git.
