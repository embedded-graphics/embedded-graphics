# Changelog

[`tinytga`](https://crates.io/crates/tinytga) is a no_std, low memory footprint TGA loading library for embedded applications.

## Unreleased

- **(breaking)** #247 "reverse" integration of tinytga into [`embedded-graphics`](https://crates.io/crates/embedded-graphics). tinytga now has a `graphics` feature that must be turned on to enable embedded-graphics support. The `tga` feature from embedded-graphics is removed.

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

- #217 Added support for TGA files with color map.

### Fixed

- #217 Images without a TGA footer are now parsed correctly.
- #216 Fixed integer overflow for some RLE compressed TGA files.
- #218 Test README examples in CI and update them to work with latest crate versions.
