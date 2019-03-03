# Embedded graphics

[![Build Status](https://travis-ci.org/jamwaffles/embedded-graphics.svg?branch=master)](https://travis-ci.org/jamwaffles/embedded-graphics)
[![Crates.io](https://img.shields.io/crates/v/embedded-graphics.svg)](https://crates.io/crates/embedded-graphics)
[![Docs.rs](https://docs.rs/embedded-graphics/badge.svg)](https://docs.rs/embedded-graphics)

## [Documentation](https://docs.rs/embedded-graphics)

A small 2D graphics library to draw things on embedded graphical LCDs, like the SSD1306 OLED display.

This crate aims to make drawing 2D graphics primitives super easy. It currently supports the
following:

* 1 bit-per-pixel images
* 8 bit-per-pixel images
* 16 bit-per-pixel images
* [BMP format (`.bmp`)](https://en.wikipedia.org/wiki/BMP_file_format) images at 1, 8 or 16BPP
* Primitives
    * Lines
    * Rectangles (and squares)
    * Circles
* Text with [multiple bitmap fonts](src/fonts)

A core goal is to do the above without using any buffers; the crate should work without a
dynamic memory allocator and without pre-allocating large chunks of memory. To achieve this, it
takes an `Iterator` based approach, where pixel values and positions are calculated on the fly,
with the minimum of saved state. This allows the consuming application to use far less RAM at
little to no performance penalty.

To use this crate in a driver, you only need to implement the `Drawing` trait to start drawing
things.

You can also add your own objects by implementing `IntoIterator<Item = Pixel>` to create an
iterator that `Drawable#draw()` can consume.

Example usage can be found [in the simulator](./simulator/examples):

```rust
use embedded_graphics::coord::Coord;
use embedded_graphics::fonts::Font6x8;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line};

// Create a display object to draw into
// This will be whichever display driver you decide to use, like the SSD1306, SSD1351, etc
let mut display = Display::new();

display.draw(
    Circle::new(Coord::new(64, 64), 64)
        .with_stroke(Some(1u8.into()))
        .into_iter(),
);

display.draw(
    Line::new(Coord::new(64, 64), Coord::new(0, 64))
        .with_stroke(Some(1u8.into()))
        .into_iter(),
);
display.draw(
    Line::new(Coord::new(64, 64), Coord::new(80, 80))
        .with_stroke(Some(1u8.into()))
        .into_iter(),
);

display.draw(
    Font6x8::render_str("Hello World!")
        .with_stroke(Some(1u8.into()))
        .translate(Coord::new(5, 50))
        .into_iter(),
);
```

## Features

* `nalgebra_support` - use the [Nalgebra](https://crates.io/crates/nalgebra) crate with `no_std` support to use as the `Coord` type. This should allow you to use most Nalgebra methods on objects rendered by embedded_graphics.

## TODO

* [ ] General matrix transforms

## Attribution

All source font PNGs are taken from the excellent [Uzebox Wiki page](http://uzebox.org/wiki/Font_Bitmaps).

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
