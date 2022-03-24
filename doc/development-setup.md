# Development setup

## Ubuntu/Linux Mint

```bash
# Update to latest stable version of Rust
rustup update

# Ensure rustfmt is installed
rustup component add rustfmt

# Install `cargo-readme`, `cargo-deadlinks` and `lychee`
cargo install cargo-readme cargo-deadlinks lychee

# Install `just`
cargo install just

# Install SDL2 for simulator for build script
sudo apt install libsdl2-dev

```

## Generating readmes

The various `README.md` files in this project are generated from each crate's `lib.rs` comment. To
regenerate a readme, ensure [`cargo-readme`](https://crates.io/crates/cargo-readme) and
[`just`](https://github.com/casey/just) are installed then run:

```bash
just generate-readme <crate>

# e.g.
just generate-readme simulator
```

All readmes can be generated at the same time by running `just generate-readmes` in the project
root.

Running `just build` will check if the readme was successfully updated. The updated `README.md`
should be committed into git.
