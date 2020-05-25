# Changelog

[`tinybmp`](https://crates.io/crates/tinybmp) is a no_std, low memory footprint BMP loading library for embedded applications.

<!-- next-header -->

## [Unreleased] - ReleaseDate

## [0.2.2] - 2020-03-20

## [0.2.1] - 2020-02-17

### Added

- #352 Added support for decoding 1 bit pixel depth BMP images.

- #244 Added `.into_iter()` support to the `Bmp` struct to get an iterator over every pixel in the image.

### Changed

- **(breaking)** #247 "reverse" integration of tinybmp into [`embedded-graphics`](https://crates.io/crates/embedded-graphics). tinybmp now has a `graphics` feature that must be turned on to enable embedded-graphics support. The `bmp` feature from embedded-graphics is removed.

  **Before**

  `Cargo.toml`

  ```toml
  [dependencies]
  embedded-graphics = { version = "0.6.0-alpha.3", features = [ "bmp" ]}
  ```

  Your code

  ```rust
  use embedded_graphics::prelude::*;
  use embedded_graphics::image::ImageBmp;

  let image = ImageBmp::new(include_bytes!("../../../assets/patch.bmp")).unwrap();
  display.draw(&image);
  ```

  **After**

  `Cargo.toml`

  ```toml
  [dependencies]
  embedded-graphics = "0.6.0"
  tinybmp = { version = "*", features = [ "graphics" ]}
  ```

  Your code

  ```rust
  use embedded_graphics::{prelude::*, image::Image};
  use tinybmp::Bmp;

  let image = Bmp::new(include_bytes!("../../../assets/patch.bmp")).unwrap();
  let image = Image::new(&image);
  display.draw(&image);
  ```

## 0.1.1

### Fixed

- #218 Test README examples in CI and update them to work with latest crate versions.

### Changed

- #228 Upgraded to nom 5 internally. No user-facing changes.

## 0.1.0

### Added

- Release `tinybmp` crate to crates.io

<!-- next-url -->
[unreleased]: https://github.com/jamwaffles/tinybmp/compare/tinybmp-v0.2.2...HEAD
[0.2.2]: https://github.com/jamwaffles/tinybmp/compare/tinybmp-v0.2.0...tinybmp-v0.2.2

[0.2.1]: https://github.com/jamwaffles/embedded-graphics/compare/tinybmp-v0.1.1...tinybmp-v0.2.1
