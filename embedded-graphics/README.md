<p align="center">
    <img width="150" src="https://raw.githubusercontent.com/jamwaffles/embedded-graphics/191fe7f8a0fedc713f9722b9dc59208dacadee7e/assets/logo.svg?sanitize=true" alt="Embedded graphics logo">
</p>
<p align="center">
    <a href="https://circleci.com/gh/jamwaffles/embedded-graphics/tree/master"><img src="https://circleci.com/gh/jamwaffles/embedded-graphics/tree/master.svg?style=shield" alt="Build Status"></a>
    <a href="https://crates.io/crates/embedded-graphics"><img src="https://img.shields.io/crates/v/embedded-graphics.svg" alt="Crates.io"></a>
    <a href="https://docs.rs/embedded-graphics"><img src="https://docs.rs/embedded-graphics/badge.svg" alt="Docs.rs"></a>
    <a href="https://matrix.to/#/#rust-embedded-graphics:matrix.org"><img src="https://img.shields.io/matrix/rust-embedded-graphics:matrix.org" alt="embedded-graphics on Matrix"></a>
</p>

# Embedded graphics

Embedded-graphics is a 2D graphics library that is focused on memory constrained embedded devices.

A core goal of embedded-graphics is to draw graphics without using any buffers; the crate is
`no_std` compatible and works without a dynamic memory allocator, and without pre-allocating
large chunks of memory. To achieve this, it takes an `Iterator` based approach, where pixel
values and positions are calculated on the fly, with the minimum of saved state. This allows the
consuming application to use far less RAM at little to no performance penalty.

It contains built in items that make it easy to draw 2D graphics primitives:

* Raw data images
* Primitives
    * Lines
    * Rectangles (and squares)
    * Circles
    * Ellipses
    * Arcs
    * Sectors
    * Triangles
    * Polylines
    * Rounded rectangles
* Text with multiple fonts

## Additional functions provided by external crates

Embedded-graphics is designed to be extended by the application or other crates. Examples of
this are adding support for different image formats or implementing custom fonts.

* [BMP images - `tinybmp`](https://crates.io/crates/tinybmp)
* [TGA images - `tinytga`](https://crates.io/crates/tinytga)
* [ProFont monospace font - `profont`](https://crates.io/crates/profont)
* [Picofont Pico8 font - `embedded-picofont`](https://crates.io/crates/embedded_picofont)
* [IBM437 font - `ibm437`](https://crates.io/crates/ibm437)
* [Simple layout/alignment functions - `embedded-layout`](https://crates.io/crates/embedded-layout)

Note that some of these crates may not support the latest version of embedded-graphics.

If you know of a crate that is not in this list, please [open an
issue](https://github.com/jamwaffles/embedded-graphics/issues/new) to add it.

## Display drivers

To support many different kinds of display, embedded-graphics doesn't include any drivers
directly but provides the `DrawTarget` API that can be implemented by external crates. In
addition to the drivers for real displays, the
[simulator](https://docs.rs/embedded-graphics-simulator/) can be used to test code during
development.

![Embedded graphics on real hardware](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/assets/banner-photo.jpg)

These are just some of the displays the community has added embedded-graphics support to. This
list is taken from the [dependent crates
list](https://crates.io/crates/embedded-graphics/reverse_dependencies) on crates.io so might be
missing some unpublished entries. Please [open an
issue](https://github.com/jamwaffles/embedded-graphics/issues/new) if there's a display driver
that should be added to this list.

Note that some drivers may not support the latest version of embedded-graphics.

* [embedded-graphics-web-simulator](https://crates.io/crates/embedded-graphics-web-simulator): Simulated display in your browser via Webassembly
* [epd-waveshare](https://crates.io/crates/epd-waveshare) Driver for various ePaper displays (EPD) from Waveshare
* [hub75](https://crates.io/crates/hub75): A rust driver for hub75 rgb matrix displays
* [ili9341](https://crates.io/crates/ili9341): A platform agnostic driver to interface with the ILI9341 (and ILI9340C) TFT LCD display
* [ls010b7dh01](https://crates.io/crates/ls010b7dh01): A platform agnostic driver for the LS010B7DH01 memory LCD display
* [sh1106](https://crates.io/crates/sh1106): I2C driver for the SH1106 OLED display
* [ssd1306](https://crates.io/crates/ssd1306): I2C and SPI (4 wire) driver for the SSD1306 OLED display
* [ssd1322](https://crates.io/crates/ssd1322): Pure Rust driver for the SSD1322 OLED display chip
* [ssd1331](https://crates.io/crates/ssd1331): SPI (4 wire) driver for the SSD1331 OLED display
* [ssd1351](https://crates.io/crates/ssd1351): SSD1351 driver
* [ssd1675](https://crates.io/crates/ssd1675): Rust driver for the Solomon Systech SSD1675 e-Paper display (EPD) controller
* [st7735-lcd](https://crates.io/crates/st7735-lcd): Rust library for displays using the ST7735 driver
* [st7789](https://crates.io/crates/st7789): A Rust driver library for ST7789 displays
* [st7920](https://crates.io/crates/st7920): ST7920 LCD driver in Rust

## Simulator

Embedded graphics comes with a [simulator]! The simulator can be used to test and debug
embedded graphics code, or produce examples and interactive demos to show of embedded graphics
features.

![It can display all sorts of embedded-graphics test code.](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/assets/simulator-demo.png)

Take a look at the [simulator examples] to see what
embedded-graphics can do, and how it might look on a display. You can run the examples like
this:

```bash
git clone https://github.com/jamwaffles/embedded-graphics.git
cd embedded-graphics

cargo run -p embedded-graphics-simulator --example hello
```

[simulator]: https://github.com/jamwaffles/embedded-graphics/tree/master/simulator
[simulator examples]: https://github.com/jamwaffles/embedded-graphics/tree/master/simulator/examples

## Crate features

Additional features can be enabled by adding the following features to your `Cargo.toml`.

* `nalgebra_support` - use the [Nalgebra](https://crates.io/crates/nalgebra) crate with `no_std`
support to enable conversions from `nalgebra::Vector2` to `Point` and `Size` 

* `fixed_point` - use fixed point arithmetic instead of floating point for all trigonometric
calculation.

## Migrating from 0.5 to 0.6

Please read [the migration guide](https://github.com/jamwaffles/embedded-graphics/blob/master/embedded-graphics/MIGRATING-0.5-0.6.md).

## Implementing `embedded_graphics` supprot for a driver

To add support for embedded-graphics to a display driver, `DrawTarget` should be implemented.
This allows all embedded-graphics objects to be rendered by the display. See the `DrawTarget` 
documentation for implementation details.

## Examples

### Drawing examples

[![Collage of drawing examples](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/doc/assets/all_drawing_ops.png)](https://github.com/jamwaffles/embedded-graphics/blob/master/doc/drawing-examples.md)

Example usage of drawing primitives, text and images with embedded-graphics can be found [here](https://github.com/jamwaffles/embedded-graphics/blob/master/doc/drawing-examples.md).

### Shapes and text

The following example draws some shapes and text to a `MockDisplay` in place of target
hardware. The [simulator](https://docs.rs/embedded-graphics-simulator/) can also be used for
debugging, development or if hardware is not available.

```rust
use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Rectangle, Triangle},
    style::{PrimitiveStyle, TextStyle},
    mock_display::MockDisplay,
};

fn main() -> Result<(), std::convert::Infallible> {
    // Create a new mock display
    let mut display: MockDisplay<BinaryColor> = MockDisplay::new();
    # display.set_allow_overdraw(true);

    // Create styles used by the drawing operations.
    let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let thick_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 3);
    let fill = PrimitiveStyle::with_fill(BinaryColor::On);
    let text_style = TextStyle::new(Font6x8, BinaryColor::On);

    let yoffset = 10;

    // Draw a 3px wide outline around the display.
    let display_size = display.size() - Size::new(1, 1);
    Rectangle::new(Point::zero(), display_size)
        .into_styled(thick_stroke)
        .draw(&mut display)?;

    // Draw a triangle.
    Triangle::new(
        Point::new(16, 16 + yoffset),
        Point::new(16 + 16, 16 + yoffset),
        Point::new(16 + 8, yoffset),
    )
    .into_styled(thin_stroke)
    .draw(&mut display)?;

    // Draw a filled square
    Rectangle::new(Point::new(52, yoffset), Size::new(16, 16))
        .into_styled(fill)
        .draw(&mut display)?;

    // Draw a circle with a 3px wide stroke.
    Circle::new(Point::new(88, yoffset), 17)
        .into_styled(thick_stroke)
        .draw(&mut display)?;

    // Draw centered text.
    let text = "embedded-graphics";
    let width = text.len() as i32 * 6;
    Text::new(text, Point::new(64 - width / 2, 40))
        .into_styled(text_style)
        .draw(&mut display)?;

    Ok(())
}
```

This example is also included in the [simulator](https://github.com/jamwaffles/embedded-graphics/tree/master/simulator/examples) crate and
can be run using `cargo run --example hello-world`. It produces this output:

![Embedded Graphics Simulator example screenshot](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/assets/hello-world-simulator.png)

Additional examples can be found in the [simulator](https://github.com/jamwaffles/embedded-graphics/tree/master/simulator) crate.

### Chaining

Items can be chained to build more complex graphics objects.

```rust
use embedded_graphics::{
    fonts::{Font6x8, Text},
    mock_display::MockDisplay,
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, Rectangle},
    style::{PrimitiveStyle, TextStyle},
};

fn build_thing(text: &'static str) -> impl Iterator<Item = Pixel<Rgb565>> {
    Rectangle::new(Point::new(0, 0), Size::new(40, 40))
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::CYAN, 1))
        .into_pixels()
        .chain(
            Circle::new(Point::new(12, 12), 17)
                .into_styled(PrimitiveStyle::with_fill(Rgb565::RED))
                .into_pixels(),
        )
        .chain(
            Text::new(text, Point::new(20, 16))
                .into_styled(TextStyle::new(Font6x8, Rgb565::GREEN))
                .into_pixels(),
        )
}

build_thing("Hello Rust!").draw(&mut display)?;
```


## Generating readmes

The various `README.md` files in this project are generated from each crate's `lib.rs` comment. To
regenerate a readme, ensure [`cargo-readme`](https://crates.io/crates/cargo-readme) is installed
then run:

```bash
./readme.sh <crate>

# e.g.
./readme.sh simulator
```

All readmes can be generated at the same time by running the `./generate_readmes.sh` script in the project root.

Running `./build.sh` will check if the readme was successfully updated. The updated `README.md`
should be committed into git.

## Development setup

### Minimum supported Rust version

The minimum supported Rust version for embedded-graphics is `1.40.0` or greater.
Ensure you have the latest stable version of Rust installed, preferably through <https://rustup.rs>.

### Ubuntu/Linux Mint

```bash
# Update to latest stable version of Rust
rustup update

# Ensure rustfmt is installed
rustup component add rustfmt

# Install `cargo-readme`
cargo install cargo-readme

# Install SDL2 for simulator and linkchecker for build script

# Python 2 systems (Ubuntu older than 20.04, Linux Mint 19, etc)
sudo apt install libsdl2-dev linkchecker

# OR

# Python 3 systems (Ubuntu 20.04+, Linux Mint 20, etc)
sudo apt install python3-pip
sudo pip3 install git+https://github.com/linkchecker/linkchecker.git
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
