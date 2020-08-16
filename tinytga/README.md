# TinyTGA

[![Build Status](https://circleci.com/gh/jamwaffles/embedded-graphics/tree/master.svg?style=shield)](https://circleci.com/gh/jamwaffles/embedded-graphics/tree/master)
[![Crates.io](https://img.shields.io/crates/v/tinytga.svg)](https://crates.io/crates/tinytga)
[![Docs.rs](https://docs.rs/tinytga/badge.svg)](https://docs.rs/tinytga)
[![embedded-graphics on Matrix](https://img.shields.io/matrix/rust-embedded-graphics:matrix.org)](https://matrix.to/#/#rust-embedded-graphics:matrix.org)

## [Documentation](https://docs.rs/tinytga)

A small TGA parser designed for use with [embedded-graphics] targetting no-std environments but
usable anywhere. Beyond parsing the image header, no other allocations are made.

tinytga provides two methods of accessing the pixel data inside a TGA file. The most convenient
way is to use a color type provided by [embedded-graphics] to define the format stored inside
the TGA file. But it is also possible to directly access the raw pixel representation instead.

## Examples

### Load a Run Length Encoded (RLE) TGA image

```rust
use embedded_graphics::{prelude::*, pixelcolor::Rgb888};
use tinytga::{Bpp, ImageOrigin, ImageType, RawPixel, Tga, TgaHeader};

// Include an image from a local path as bytes
let data = include_bytes!("../tests/chessboard_4px_rle.tga");

// Create a TGA instance from a byte slice.
// The color type is set by defining the type of the `img` variable.
let img: Tga<Rgb888> = Tga::from_slice(data).unwrap();

// Check the size of the image.
assert_eq!(img.size(), Size::new(4, 4));

// Collect pixels into a vector.
let pixels: Vec<_> = img.pixels().collect();
```

### Drawing an image using `embedded-graphics`

This example demonstrates how a TGA image can be drawn to a [embedded-graphics] draw target.

```rust
use embedded_graphics::{image::Image, pixelcolor::Rgb888, prelude::*};
use tinytga::Tga;

// Include an image from a local path as bytes
let data = include_bytes!("../tests/chessboard_4px_rle.tga");
let tga: Tga<Rgb888> = Tga::from_slice(data).unwrap();

let image = Image::new(&tga, Point::zero());

image.draw(&mut display)?;
```

### Accessing raw pixel data

If you do not want to use the color types provided by [embedded-graphics] you can also access
the raw image data.

```rust
use embedded_graphics::{prelude::*, pixelcolor::Rgb888};
use tinytga::{Bpp, ImageOrigin, ImageType, RawPixel, Tga, TgaHeader};

// Include an image from a local path as bytes.
let data = include_bytes!("../tests/chessboard_4px_rle.tga");

// Create a TGA instance from a byte slice.
let img = Tga::from_slice_raw(data).unwrap();

// Take a look at the raw image header.
assert_eq!(
    img.raw_header(),
    TgaHeader {
        id_len: 0,
        has_color_map: false,
        image_type: ImageType::RleTruecolor,
        color_map_start: 0,
        color_map_len: 0,
        color_map_depth: None,
        x_origin: 0,
        y_origin: 4,
        width: 4,
        height: 4,
        pixel_depth: Bpp::Bits24,
        image_origin: ImageOrigin::TopLeft,
        alpha_channel_depth: 0,
    }
);

// Collect raw pixels into a vector.
let pixels: Vec<_> = img.raw_pixels().collect();
```

## Embedded-graphics drawing performance

`tinytga` uses different code paths to draw images with different `ImageOrigin` .
The performance difference between the origins will depend on the display driver, but using
images with the origin at the top left corner will generally result in the best performance.

[embedded-graphics]: https://docs.rs/embedded-graphics

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
