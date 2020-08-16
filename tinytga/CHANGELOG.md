# Changelog

[`tinytga`](https://crates.io/crates/tinytga) is a no_std, low memory footprint TGA loading library for embedded applications.

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Changed

- **(breaking)** [#407](https://github.com/jamwaffles/embedded-graphics/pull/407) The `image_descriptor` in `TgaHeader` was replaced by `image_origin` and `alpha_channel_bits`.
- **(breaking)** [#420](https://github.com/jamwaffles/embedded-graphics/pull/420) To support the new embedded-graphics 0.7 image API a color type parameter was added to `Tga`.
- **(breaking)** [#430](https://github.com/jamwaffles/embedded-graphics/pull/430) The `graphics` feature was removed and the `embedded-graphics` dependency is now non optional.
- **(breaking)** [#430](https://github.com/jamwaffles/embedded-graphics/pull/430) `Tga` does no longer implement `IntoIterator`. Pixel iterators can now be created using the `pixels` an `raw_pixels` methods.
- **(breaking)** [#430](https://github.com/jamwaffles/embedded-graphics/pull/430) `Tga::from_slice` now checks that the specified color type matches the bit depth of the image.
- **(breaking)** [#430](https://github.com/jamwaffles/embedded-graphics/pull/430) The `TgaFooter` struct was replaced by the `raw_developer_dictionary` and `raw_extension_area` methods in `Tga`.
- **(breaking)** [#430](https://github.com/jamwaffles/embedded-graphics/pull/430) `Tga::width` and `Tga::height` were replaced by `Tga::size` which requires `embedded_graphics::geometry::OriginDimensions` to be in scope (also included in the embedded-graphics `prelude`).
- **(breaking)** [#430](https://github.com/jamwaffles/embedded-graphics/pull/430) The color map can now be accessed using the new `ColorMap` type.

### Added

- [#407](https://github.com/jamwaffles/embedded-graphics/pull/407) Added support for bottom-left origin images to `TgaIterator`.
- [#430](https://github.com/jamwaffles/embedded-graphics/pull/430) The image ID can now be accessed using `Tga::image_id`.

### Fixed

- [#407](https://github.com/jamwaffles/embedded-graphics/pull/407) Additional data in `pixel_data`, beyond `width * height` pixels, is now discarded by `TgaIterator`.
- [#430](https://github.com/jamwaffles/embedded-graphics/pull/430) Images with unsupported BPP values in the header no longer cause panics. Instead an error is returned by `from_slice`.
- [#430](https://github.com/jamwaffles/embedded-graphics/pull/430) Errors during the execution of a pixel iterator no longer cause panics. Instead the corrupted portion of the image is filled with black pixels.

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
