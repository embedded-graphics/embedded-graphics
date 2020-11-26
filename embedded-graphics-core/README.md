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

## Embedded Graphics Core

embedded-graphics-core contains the common components of [embedded-graphics] that can be used to
integrate embedded-graphics into display drivers, image libraries and other third party crates.

Display driver crates, image decoding crates and other crates that wish to integrate
embedded-graphics should use embedded-graphics-core to provid embedded-graphics integration for
their crate. Conversely, consumers of embedded-graphics and these crates should use
[embedded-graphics] itself.

Like any other crate, embedded-graphics-core will change over time, however it will change at a
much slower rate than embedded-graphics itself, and will likely release fewer breaking changes.
This will provide more stability and compatability for the weider embedded-graphics ecosystem,
whilst allowing non-core features of embedded-graphics to evolve at a faster pace.

It is strongly recommended to use as much of embedded-graphics-core as possible when
implementing an integration.

### Common functionality

* `Pixel` - A simple struct containing a `Point` and color defining a single pixel.
* `Drawable` - A trait that should be implemented for anything that is drawable to a
  `DrawTarget`. Examples include shapes, text, UI elements, etc.
* Geometry - `Point`, `Size` and `Rectangle` provide ways of defining positions,
  dimensions and areas respectively.
* Dimensions - the `Dimensions` and `OriginDimensions` traits allow sizing of objects.
* The `prelude` reexports useful items to reduce boilerplate.

## Colors

The `pixelcolor` module provides various standard colors, from `BinaryColor` to `Rgb888`.
See the `pixelcolor` module documentation for the complete list of color depths and formats
available.

## Display driver authors

See the `DrawTarget` documentation for examples on how to integrate embedded-graphics with a
display driver using the `DrawTarget` trait.

## Images

The `ImageDrawable` trait should be implemented for any image or image-like item, like a font
bitmap or spritemap.

[embedded-graphics]: https://docs.rs/embedded-graphics

## Minimum supported Rust version

The minimum supported Rust version for embedded-graphics is `1.40.0` or greater.
Ensure you have the latest stable version of Rust installed, preferably through <https://rustup.rs>.

## Development setup

Please see the [development setup guide](./doc/development-setup.md).

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
