# Changelog

[`tinytga`](https://crates.io/crates/tinytga) is a no_std, low memory footprint TGA loading library for embedded applications.

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Changed

- **(breaking)** [#407](https://github.com/jamwaffles/embedded-graphics/pull/407) The `image_descriptor` in `TgaHeader` was replaced by `image_origin` and `alpha_channel_bits`.

### Added

- [#407](https://github.com/jamwaffles/embedded-graphics/pull/407) Added support for bottom-left origin images to `TgaIterator`.

### Fixed

- [#407](https://github.com/jamwaffles/embedded-graphics/pull/407) Additional data in `pixel_data`, beyond `width * height` pixels, is now discarded by `TgaIterator`.

## [0.3.2] - 2020-03-20

## [0.3.1] - 2020-02-17

- **(breaking)** [#247](https://github.com/jamwaffles/embedded-graphics/pull/247) "reverse" integration of tinytga into [`embedded-graphics`](https://crates.io/crates/embedded-graphics). tinytga now has a `graphics` feature that must be turned on to enable embedded-graphics support. The `tga` feature from embedded-graphics is removed.

  **Before**

  `Cargo.toml`

  ```toml
  [dependencies]
  embedded-graphics = { version = "0.6.0-alpha.3", features = [ "tga" ]}
  ```

  Your code

  ```rust
  use embedded_graphics::prelude::*;
  use embedded_graphics::image::ImageTga;

  let image = ImageTga::new(include_bytes!("../../../assets/patch.tga")).unwrap();
  display.draw(&image);
  ```

  **After**

  `Cargo.toml`

  ```toml
  [dependencies]
  embedded-graphics = "0.6.0"
  tinytga = { version = "*", features = [ "graphics" ]}
  ```

  Your code

  ```rust
  use embedded_graphics::{prelude::*, image::Image};
  use tinytga::Tga;

  let image = Tga::new(include_bytes!("../../../assets/patch.tga")).unwrap();
  let image = Image::new(&image);
  display.draw(&image);
  ```

## 0.2.0

### Added

- [#217](https://github.com/jamwaffles/embedded-graphics/pull/217) Added support for TGA files with color map.

### Fixed

- [#217](https://github.com/jamwaffles/embedded-graphics/pull/217) Images without a TGA footer are now parsed correctly.
- [#216](https://github.com/jamwaffles/embedded-graphics/pull/216) Fixed integer overflow for some RLE compressed TGA files.
- [#218](https://github.com/jamwaffles/embedded-graphics/pull/218) Test README examples in CI and update them to work with latest crate versions.

<!-- next-url -->
[unreleased]: https://github.com/jamwaffles/tinytga/compare/tinytga-v0.3.2...HEAD
[0.3.2]: https://github.com/jamwaffles/tinytga/compare/tinytga-v0.3.0...tinytga-v0.3.2

[0.3.1]: https://github.com/jamwaffles/embedded-graphics/compare/tinytga-v0.2.0...tinytga-v0.3.1
