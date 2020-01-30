# TinyTGA

[![Build Status](https://circleci.com/gh/jamwaffles/embedded-graphics/tree/master.svg?style=shield)](https://circleci.com/gh/jamwaffles/embedded-graphics/tree/master)
[![Crates.io](https://img.shields.io/crates/v/tinytga.svg)](https://crates.io/crates/tinytga)
[![Docs.rs](https://docs.rs/tinytga/badge.svg)](https://docs.rs/tinytga)

## [Documentation](https://docs.rs/tinytga)

A small TGA parser designed for embedded, no-std environments but usable anywhere. Beyond parsing the image header, no other allocations are made. A reference to the input image data is kept and slices are returned from it.

Call `Tga.into_iter()` to get an iterator over individual pixels in the image.

## Example

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

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
