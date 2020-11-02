# TinyBMP

[![Build Status](https://circleci.com/gh/jamwaffles/embedded-graphics/tree/master.svg?style=shield)](https://circleci.com/gh/jamwaffles/embedded-graphics/tree/master)
[![Crates.io](https://img.shields.io/crates/v/tinybmp.svg)](https://crates.io/crates/tinybmp)
[![Docs.rs](https://docs.rs/tinybmp/badge.svg)](https://docs.rs/tinybmp)
[![embedded-graphics on Matrix](https://img.shields.io/matrix/rust-embedded-graphics:matrix.org)](https://matrix.to/#/#rust-embedded-graphics:matrix.org)

## [Documentation](https://docs.rs/tinybmp)

A small BMP parser designed for embedded, no-std environments but usable anywhere. Beyond
parsing the image header, no other allocations are made.

To use `tinybmp` without `embedded-graphics` the raw data for individual pixels in an image
can be accessed using the methods provided by the `RawBmp` struct.

## Examples

### Using `Bmp` to draw a BMP image

If the color format inside the BMP file is known at compile time the `Bmp` type can be used
to draw an image to an `embedded-graphics` draw target. The BMP file used in this example
uses 16 bits per pixel with a RGB565 format.

```rust
use embedded_graphics::{image::Image, prelude::*};
use tinybmp::Bmp;

let bmp_data = include_bytes!("../tests/chessboard-8px-color-16bit.bmp");

// Load 16 BPP 8x8px image.
// Note: The color type is specified explicitly to match the format used by the BMP image.
let bmp = Bmp::<Rgb565>::from_slice(bmp_data).unwrap();

// Draw the image with the top left corner at (10, 20) by wrapping it in
// an embedded-graphics `Image`.
Image::new(&bmp, Point::new(10, 20)).draw(&mut display)?;
```

### Using `DynamicBmp` to draw a BMP image

If the exact color format used in the BMP file isn't known at compile time, for example to read
user supplied images, the `DynamicBmp` can be used. Because automatic color conversion will
be used the drawing performance might be degraded in comparison to `Bmp`.

```rust
use embedded_graphics::{image::Image, prelude::*};
use tinybmp::DynamicBmp;

let bmp_data = include_bytes!("../tests/chessboard-8px-color-16bit.bmp");

// Load BMP image with unknown color format.
// Note: There is no need to explicitly specify the color type.
let bmp = DynamicBmp::from_slice(bmp_data).unwrap();

// Draw the image with the top left corner at (10, 20) by wrapping it in
// an embedded-graphics `Image`.
Image::new(&bmp, Point::new(10, 20)).draw(&mut display)?;
```

### Accessing the raw image data

The `RawBmp` struct provides methods to access lower level information about a BMP file,
like the BMP header or the raw image data. An instance of this type can be created by using
`from_slice` or by accessing the underlying raw object of a `Bmp` or `DynamicBmp` object
by using `as_raw`.

```rust
use embedded_graphics::prelude::*;
use tinybmp::{RawBmp, Bpp, Header, RawPixel};

let bmp = RawBmp::from_slice(include_bytes!("../tests/chessboard-8px-24bit.bmp"))
    .expect("Failed to parse BMP image");

// Read the BMP header
assert_eq!(
    bmp.header(),
    &Header {
        file_size: 314,
        image_data_start: 122,
        bpp: Bpp::Bits24,
        image_size: Size::new(8, 8),
        image_data_len: 192,
        channel_masks: None,
    }
);

// Check that raw image data slice is the correct length (according to parsed header)
assert_eq!(bmp.image_data().len(), bmp.header().image_data_len as usize);

// Get an iterator over the pixel coordinates and values in this image and load into a vec
let pixels: Vec<RawPixel> = bmp.pixels().collect();

// Loaded example image is 8x8px
assert_eq!(pixels.len(), 8 * 8);
```

[`embedded-graphics`]: https://crates.io/crates/embedded-graphics

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
