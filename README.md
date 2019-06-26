# Embedded graphics

[![Build Status](https://circleci.com/gh/jamwaffles/embedded-graphics/tree/master.svg?style=shield)](https://circleci.com/gh/jamwaffles/embedded-graphics/tree/master)
[![Crates.io](https://img.shields.io/crates/v/embedded-graphics.svg)](https://crates.io/crates/embedded-graphics)
[![Docs.rs](https://docs.rs/embedded-graphics/badge.svg)](https://docs.rs/embedded-graphics)

## [Documentation](https://docs.rs/embedded-graphics)

A small 2D graphics library to draw things on embedded graphical LCDs, like the SSD1306 OLED display.

This crate aims to make drawing 2D graphics primitives super easy. It currently supports the
following:

* 1 bit-per-pixel images
* 8 bit-per-pixel images
* 16 bit-per-pixel images
* [BMP format (`.bmp`)](https://en.wikipedia.org/wiki/BMP_file_format) images at 1, 8 or 16BPP (requires `bmp` feature)
* [TGA format (`.tga`)](https://en.wikipedia.org/wiki/Truevision_TGA) images (requires `tga` feature)
* Primitives
    * Lines
    * Rectangles (and squares)
    * Circles
    * Triangles
* Text with [multiple bitmap fonts](src/fonts)

A core goal is to do the above without using any buffers; the crate should work without a
dynamic memory allocator and without pre-allocating large chunks of memory. To achieve this, it
takes an `Iterator` based approach, where pixel values and positions are calculated on the fly,
with the minimum of saved state. This allows the consuming application to use far less RAM at
little to no performance penalty.

More information and up to date docs can be found on [docs.rs](https://docs.rs/embedded-graphics).

Example usage can be found [in the simulator](./simulator/examples):

```rust
use embedded_graphics::coord::Coord;
use embedded_graphics::fonts::Font6x8;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line};

fn main() {
    // Create a display object to draw into
    // This will be whichever display driver you decide to use, like the SSD1306, SSD1351, etc
    let mut display = Display::new();

    display.draw(Circle::new(Coord::new(64, 64), 64).with_stroke(Some(1u8)));
    display.draw(Line::new(Coord::new(64, 64), Coord::new(0, 64)).with_stroke(Some(1u8)));
    display.draw(Line::new(Coord::new(64, 64), Coord::new(80, 80)).with_stroke(Some(1u8)));

    display.draw(
        Font6x8::render_str("Hello World!")
            .with_stroke(Some(1u8))
            .translate(Coord::new(5, 50)),
    );
}
```

Macros are also supported for text and primitives:

```rust
use embedded_graphics::prelude::*;
use embedded_graphics::{circle, icoord, line, rect, text_6x8, triangle};

fn main() {
    // Create a display object to draw into
    // This will be whichever display driver you decide to use, like the SSD1306, SSD1351, etc
    let mut display = Display::new();

    display.draw(egcircle!((64, 64), 64, stroke = Some(1u8)));
    display.draw(egline!((64, 64), (0, 64), stroke = Some(1u8)));
    display.draw(egline!((64, 64), (80, 80), stroke = Some(1u8)));
    display.draw(egrectangle!((64, 64), (80, 80), stroke = None, fill = Some(2u8)));
    display.draw(text_6x8!("Hello world!", stroke = Some(1u8)).translate(icoord!(5, 50)));
}
```

## Cargo Features

* `nalgebra_support` - use the [Nalgebra](https://crates.io/crates/nalgebra) crate with `no_std` support to use as the `Coord` type. This should allow you to use most Nalgebra methods on objects rendered by embedded_graphics.
* `bmp` - use the [TinyBMP](https://crates.io/crates/tinybmp) crate for BMP image support.
* `tga` - use the [TinyTGA](https://crates.io/crates/tinytga) crate for TGA image support.

## Display drivers with embedded-graphics support

* [ili9341](https://crates.io/crates/ili9341): A platform agnostic driver to interface with the ILI9341 (and ILI9340C) TFT LCD display
* [ls010b7dh01](https://crates.io/crates/ls010b7dh01): A platform agnostic driver for the LS010B7DH01 memory LCD display
* [sh1106](https://crates.io/crates/sh1106): I2C driver for the SH1106 OLED display
* [ssd1306](https://crates.io/crates/ssd1306): I2C and SPI (4 wire) driver for the SSD1306 OLED display
* [ssd1322](https://crates.io/crates/ssd1322): Pure Rust driver for the SSD1322 OLED display chip
* [ssd1331](https://crates.io/crates/ssd1331): SPI (4 wire) driver for the SSD1331 OLED display
* [ssd1351](https://crates.io/crates/ssd1351): SSD1351 driver
* [ssd1675](https://crates.io/crates/ssd1675): Rust driver for the Solomon Systech SSD1675 e-Paper display (EPD) controller
* [st7735-lcd](https://crates.io/crates/st7735-lcd): Rust library for displays using the ST7735 driver

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
