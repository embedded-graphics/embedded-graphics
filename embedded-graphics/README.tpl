<p align="center">
    <img width="150" src="https://raw.githubusercontent.com/jamwaffles/embedded-graphics/191fe7f8a0fedc713f9722b9dc59208dacadee7e/assets/logo.svg?sanitize=true" alt="Embedded graphics logo">
</p>
<p align="center">
    <a href="https://circleci.com/gh/jamwaffles/embedded-graphics/tree/master"><img src="https://circleci.com/gh/jamwaffles/embedded-graphics/tree/master.svg?style=shield" alt="Build Status"></a>
    <a href="https://crates.io/crates/embedded-graphics"><img src="https://img.shields.io/crates/v/embedded-graphics.svg" alt="Crates.io"></a>
    <a href="https://docs.rs/embedded-graphics"><img src="https://docs.rs/embedded-graphics/badge.svg" alt="Docs.rs"></a>
    <a href="https://matrix.to/#/#rust-embedded-graphics:matrix.org"><img src="https://img.shields.io/matrix/rust-embedded-graphics:matrix.org" alt="embedded-graphics on Matrix"></a>
</p>

# Embedded graphics

{{readme}}

## Development setup

### Minimum supported Rust version

The minimum supported Rust version for embedded-graphics is `1.40.0` or greater.
Ensure you have the latest stable version of Rust installed, preferably through <https://rustup.rs>.

### Ubuntu/Linux Mint

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

## Attribution

All source font PNGs are taken from the excellent [Uzebox Wiki page](http://uzebox.org/wiki/Font_Bitmaps).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
