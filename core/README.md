<p align="center">
    <img width="150" src="https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/191fe7f8a0fedc713f9722b9dc59208dacadee7e/assets/logo.svg?sanitize=true" alt="Embedded graphics logo">
</p>
<p align="center">
    <a href="https://circleci.com/gh/embedded-graphics/embedded-graphics/tree/master"><img src="https://circleci.com/gh/embedded-graphics/embedded-graphics/tree/master.svg?style=shield" alt="Build Status"></a>
    <a href="https://crates.io/crates/embedded-graphics-core"><img src="https://img.shields.io/crates/v/embedded-graphics-core.svg" alt="Crates.io"></a>
    <a href="https://docs.rs/embedded-graphics-core"><img src="https://docs.rs/embedded-graphics-core/badge.svg" alt="Docs.rs"></a>
    <a href="https://matrix.to/#/#rust-embedded-graphics:matrix.org"><img src="https://img.shields.io/matrix/rust-embedded-graphics:matrix.org" alt="embedded-graphics on Matrix"></a>
</p>

# Embedded graphics core

embedded-graphics-core contains the core components of [embedded-graphics] that are required to
add embedded-graphics support to display drivers, image libraries, text renderers and other
third party crates.

This crate should only be used by crates that extend embedded-graphics.
Applications should instead depend on [embedded-graphics] itself.

Like any other crate, embedded-graphics-core will change over time, however it will change at a
much slower rate than embedded-graphics itself, and will likely release fewer breaking changes.
This will provide more stability and compatability for the weider embedded-graphics ecosystem,
whilst allowing non-core features of embedded-graphics to evolve at a faster pace. The same
version of embedded-graphics-core may be used for multiple major versions of embedded-graphics.

### Core functionality

* `Pixel` - A struct that defines the position and color of a single pixel.
* `Drawable` - A trait that should be implemented for anything that is drawable to a
  `DrawTarget`. Examples include shapes, text, UI elements, etc.
* Geometry - `Point`, `Size` and `Rectangle` provide ways of defining positions,
  dimensions and rectangular areas respectively.
* Dimensions - the `Dimensions` and `OriginDimensions` traits allow the size and position of
  an object to be defined.
* The `prelude` reexports useful items to reduce boilerplate.

## Colors

The `pixelcolor` module provides various standard color types, from `BinaryColor` to
`Rgb888`. See the `pixelcolor` module documentation for the complete list of color depths
and formats available.

## Display drivers

See the `DrawTarget` documentation for examples on how to integrate embedded-graphics with a
display driver using the `DrawTarget` trait.

## Images

The `ImageDrawable` trait should be implemented for any image or image-like item, for example
a spritemap.

[embedded-graphics]: https://docs.rs/embedded-graphics

## Minimum supported Rust version

The minimum supported Rust version for embedded-graphics-core is `1.40.0` or greater.
Ensure you have the latest stable version of Rust installed, preferably through <https://rustup.rs>.

## Development setup

Please see the [development setup guide](../doc/development-setup.md).

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
