# TinyBMP

[![Build Status](https://circleci.com/gh/jamwaffles/embedded-graphics/tree/master.svg?style=svg)](https://circleci.com/gh/jamwaffles/embedded-graphics/tree/master)
[![Crates.io](https://img.shields.io/crates/v/tinybmp.svg)](https://crates.io/crates/tinybmp)
[![Docs.rs](https://docs.rs/tinybmp/badge.svg)](https://docs.rs/tinybmp)

## [Documentation](https://docs.rs/tinybmp)

A small BMP parser designed for embedded, no-std environments but usable anywhere. Beyond parsing the image header, no other allocations are made. A reference to the input image data is kept and slices are returned from it.

## Example

```rust
use tinybmp::{Bmp, FileType, Header};

fn main() {
    let bmp =
        Bmp::from_slice(include_bytes!("../tests/chessboard-8px-24bit.bmp")).expect("Failed to parse");

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

    let image_data: &[u8] = bmp.image_data();

    // Render, process or iterate on `image_data` here
}
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
