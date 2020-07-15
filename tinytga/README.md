# TinyTGA

[![Build Status](https://circleci.com/gh/jamwaffles/embedded-graphics/tree/master.svg?style=shield)](https://circleci.com/gh/jamwaffles/embedded-graphics/tree/master)
[![Crates.io](https://img.shields.io/crates/v/tinytga.svg)](https://crates.io/crates/tinytga)
[![Docs.rs](https://docs.rs/tinytga/badge.svg)](https://docs.rs/tinytga)

## [Documentation](https://docs.rs/tinytga)

A small TGA parser designed for embedded, no-std environments but usable anywhere. Beyond
parsing the image header, no other allocations are made.

To access the individual pixels in an image, the `Tga` struct implements `IntoIterator`. It is
also possible to access the unaltered raw image data by reading the `pixel_data` field. This
data will need to be interpreted according to the `image_type` specified in the header.

## Features

* `graphics` - enables `embedded-graphics` integration.

## Examples

### Load a Run Length Encoded (RLE) TGA image

```rust
use tinytga::{ImageType, Pixel, Tga, TgaFooter, TgaHeader};

// Include an image from a local path as bytes
let data = include_bytes!("../tests/chessboard_4px_rle.tga");

// Create a TGA instance from a byte slice
let img = Tga::from_slice(data).unwrap();

// Take a look at the header
assert_eq!(
    img.header,
    TgaHeader {
        id_len: 0,
        has_color_map: false,
        image_type: ImageType::RleTruecolor,
        color_map_start: 0,
        color_map_len: 0,
        color_map_depth: 0,
        x_origin: 0,
        y_origin: 4,
        width: 4,
        height: 4,
        pixel_depth: 24,
        image_descriptor: 32,
    }
);

// Take a look at the footer
assert_eq!(
    img.footer,
    Some(TgaFooter {
        extension_area_offset: 0,
        developer_directory_offset: 0
    })
);

// Collect pixels into a `Vec<Pixel>`
let pixels = img.into_iter().collect::<Vec<Pixel>>();
```

### Use with `embedded-graphics`

This example demonstrates `embedded-graphics` support by rendering a TGA image to a mock
display.

The `graphics` feature of `tinytga` needs to be enabled in `Cargo.toml` to use the `Tga` object
with embedded-graphics.

```rust
use embedded_graphics::{image::Image, pixelcolor::Rgb888, prelude::*};
use tinytga::Tga;

let tga = Tga::from_slice(include_bytes!("../tests/rust-rle-bw-topleft.tga")).unwrap();

let image: Image<Tga, Rgb888> = Image::new(&tga, Point::zero());

image.draw(&mut display)?;
```

[`embedded-graphics`]: https://docs.rs/embedded-graphics

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
