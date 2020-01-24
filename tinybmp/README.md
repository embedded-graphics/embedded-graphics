# TinyBMP

[![Build Status](https://circleci.com/gh/jamwaffles/embedded-graphics/tree/master.svg?style=shield)](https://circleci.com/gh/jamwaffles/embedded-graphics/tree/master)
[![Crates.io](https://img.shields.io/crates/v/tinybmp.svg)](https://crates.io/crates/tinybmp)
[![Docs.rs](https://docs.rs/tinybmp/badge.svg)](https://docs.rs/tinybmp)

## [Documentation](https://docs.rs/tinybmp)

A small BMP parser designed for embedded, no-std environments but usable anywhere. Beyond parsing the image header, no other allocations are made. A reference to the input image data is kept and slices are returned from it.

## Example

```rust
use tinybmp::{Bmp, FileType, Header};

let bmp = Bmp::from_slice(include_bytes!("../tests/chessboard-8px-24bit.bmp"))
    .expect("Failed to parse BMP image");

// Read the BMP header
assert_eq!(
    bmp.header,
    Header {
        file_type: FileType::BM,
        file_size: 314,
        reserved_1: 0,
        reserved_2: 0,
        image_data_start: 122,
        bpp: 24,
        image_width: 8,
        image_height: 8,
        image_data_len: 192
    }
);

// Check that raw image data slice is the correct length (according to parsed header)
assert_eq!(bmp.image_data().len(), bmp.header.image_data_len as usize);

// Get an iterator over the pixel coordinates and colors in this image and collect into a vec
let pixels: Vec<((u32, u32), u32)> = bmp.into_iter().collect();

// Loaded example image is 8x8px
assert_eq!(pixels.len(), 8 * 8);
```

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
