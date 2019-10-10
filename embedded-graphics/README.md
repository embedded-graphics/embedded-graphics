# Embedded graphics

[![Build Status](https://circleci.com/gh/jamwaffles/embedded-graphics/tree/master.svg?style=shield)](https://circleci.com/gh/jamwaffles/embedded-graphics/tree/master)
[![Crates.io](https://img.shields.io/crates/v/embedded-graphics.svg)](https://crates.io/crates/embedded-graphics)
[![Docs.rs](https://docs.rs/embedded-graphics/badge.svg)](https://docs.rs/embedded-graphics)

## [Documentation](https://docs.rs/embedded-graphics)

A small 2D graphics library to draw things on embedded graphical LCDs, like the SSD1306 OLED display.

This crate aims to make drawing 2D graphics primitives super easy. It currently supports the
following:

- 1 bit-per-pixel images
- 8 bit-per-pixel images
- 16 bit-per-pixel images
- [BMP format (`.bmp`)](https://en.wikipedia.org/wiki/BMP_file_format) images at 1, 8 or 16BPP (requires `bmp` feature)
- [TGA format (`.tga`)](https://en.wikipedia.org/wiki/Truevision_TGA) images (requires `tga` feature)
- Primitives
  - Lines
  - Rectangles (and squares)
  - Circles
  - Triangles
- Text with [multiple bitmap fonts](src/fonts)

A core goal is to do the above without using any buffers; the crate should work without a
dynamic memory allocator and without pre-allocating large chunks of memory. To achieve this, it
takes an `Iterator` based approach, where pixel values and positions are calculated on the fly,
with the minimum of saved state. This allows the consuming application to use far less RAM at
little to no performance penalty.

More information and up to date docs can be found on [docs.rs](https://docs.rs/embedded-graphics).

Example usage can be found [in the simulator](./simulator/examples):

```rust
use embedded_graphics::fonts::Font6x8;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line};

// Only used for examples - this would be replaced by the driver for your chosen display
use embedded_graphics::mock_display::MockDisplay as Display;

fn main() {
    // Create a display object to draw into
    // This will be whichever display driver you decide to use, like the SSD1306, SSD1351, etc
    let mut display = Display::new();

    Circle::new(Point::new(64, 64), 64).stroke_color(Some(BinaryColor::On)).draw(&mut display);
    Line::new(Point::new(64, 64), Point::new(0, 64)).stroke_color(Some(BinaryColor::On)).draw(&mut display);
    Line::new(Point::new(64, 64), Point::new(80, 80)).stroke_color(Some(BinaryColor::On)).draw(&mut display);

    Font6x8::render_str("Hello World!")
        .stroke_color(Some(BinaryColor::On))
        .translate(Point::new(5, 50))
        .draw(&mut display);
}
```

Macros are also supported for text and primitives:

```rust
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::{egcircle, egline, egrectangle, egtriangle, text_6x8};

// Only used for examples - this would be replaced by the driver for your chosen display
use embedded_graphics::mock_display::MockDisplay as Display;

fn main() {
    // Create a display object to draw into
    // This will be whichever display driver you decide to use, like the SSD1306, SSD1351, etc
    let mut display = Display::new();

    egcircle!(
        (64, 64),
        64,
        stroke_color = Some(BinaryColor::On)
    ).draw(&mut display);
    egline!(
        (64, 64),
        (0, 64),
        stroke_color = Some(BinaryColor::On)
    ).draw(&mut display);
    egline!(
        (64, 64),
        (80, 80),
        stroke_color = Some(BinaryColor::On)
    ).draw(&mut display);
    egrectangle!(
        (64, 64),
        (80, 80),
        stroke_color = None,
        fill_color = Some(BinaryColor::Off)
    ).draw(&mut display);
    text_6x8!("Hello world!", stroke_color = Some(BinaryColor::On))
        .translate(Point::new(5, 50))
        .draw(&mut display);
}
```

## Cargo Features

- `nalgebra_support` - use the [Nalgebra](https://crates.io/crates/nalgebra) crate with `no_std`
  support to enable conversions from `nalgebra::Vector2` to `Coord` and `UnsignedCoord`.
- `bmp` - use the [TinyBMP](https://crates.io/crates/tinybmp) crate for BMP image support.
- `tga` - use the [TinyTGA](https://crates.io/crates/tinytga) crate for TGA image support.

## Display drivers with embedded-graphics support

- [ili9341](https://crates.io/crates/ili9341): A platform agnostic driver to interface with the ILI9341 (and ILI9340C) TFT LCD display
- [ls010b7dh01](https://crates.io/crates/ls010b7dh01): A platform agnostic driver for the LS010B7DH01 memory LCD display
- [sh1106](https://crates.io/crates/sh1106): I2C driver for the SH1106 OLED display
- [ssd1306](https://crates.io/crates/ssd1306): I2C and SPI (4 wire) driver for the SSD1306 OLED display
- [ssd1322](https://crates.io/crates/ssd1322): Pure Rust driver for the SSD1322 OLED display chip
- [ssd1331](https://crates.io/crates/ssd1331): SPI (4 wire) driver for the SSD1331 OLED display
- [ssd1351](https://crates.io/crates/ssd1351): SSD1351 driver
- [ssd1675](https://crates.io/crates/ssd1675): Rust driver for the Solomon Systech SSD1675 e-Paper display (EPD) controller
- [st7735-lcd](https://crates.io/crates/st7735-lcd): Rust library for displays using the ST7735 driver

There may be other drivers out there we don't know about yet. If you know of a driver to add to this list, please open [an issue](https://github.com/jamwaffles/embedded-graphics/issues/new)!

## Development setup

Ensure you have the latest stable version of Rust installed, preferably through <https://rustup.rs>.

### Ubuntu/Linux Mint

```bash
# Update to latest stable version of Rust
rustup update

# Ensure rustfmt is installed
rustup component add rustfmt

# Install SDL2 for simulator and PIP to install linkchecker
sudo apt install libsdl2-dev python-pip

# Install linkchecker so doc links can be checked
sudo pip install linkchecker
```

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
