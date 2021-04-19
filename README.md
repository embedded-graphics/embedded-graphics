<p align="center">
    <img width="150" src="https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/191fe7f8a0fedc713f9722b9dc59208dacadee7e/assets/logo.svg?sanitize=true" alt="Embedded graphics logo">
</p>
<p align="center">
    <a href="https://circleci.com/gh/embedded-graphics/embedded-graphics/tree/master"><img src="https://circleci.com/gh/embedded-graphics/embedded-graphics/tree/master.svg?style=shield" alt="Build Status"></a>
    <a href="https://crates.io/crates/embedded-graphics"><img src="https://img.shields.io/crates/v/embedded-graphics.svg" alt="Crates.io"></a>
    <a href="https://docs.rs/embedded-graphics"><img src="https://docs.rs/embedded-graphics/badge.svg" alt="Docs.rs"></a>
    <a href="https://matrix.to/#/#rust-embedded-graphics:matrix.org"><img src="https://img.shields.io/matrix/rust-embedded-graphics:matrix.org" alt="embedded-graphics on Matrix"></a>
</p>

# Embedded graphics

Embedded-graphics is a 2D graphics library that is focused on memory constrained embedded devices.

A core goal of embedded-graphics is to draw graphics without using any buffers; the crate is
`no_std` compatible and works without a dynamic memory allocator, and without pre-allocating
large chunks of memory. To achieve this, it takes an `Iterator` based approach, where pixel
colors and positions are calculated on the fly, with the minimum of saved state. This allows the
consuming application to use far less RAM at little to no performance penalty.

It contains built in items that make it easy to draw 2D graphics primitives:

* [Raw data images](https://docs.rs/embedded-graphics/latest/embedded_graphics/image/struct.ImageRaw.html)
* [Primitives](https://docs.rs/embedded-graphics/latest/embedded_graphics/primitives/index.html)
    * [Lines](https://docs.rs/embedded-graphics/latest/embedded_graphics/primitives/line/struct.Line.html)
    * [Rectangles (and squares)](https://docs.rs/embedded-graphics/latest/embedded_graphics/primitives/rectangle/struct.Rectangle.html)
    * [Circles](https://docs.rs/embedded-graphics/latest/embedded_graphics/primitives/circle/struct.Circle.html)
    * [Ellipses](https://docs.rs/embedded-graphics/latest/embedded_graphics/primitives/ellipse/struct.Ellipse.html)
    * [Arcs](https://docs.rs/embedded-graphics/latest/embedded_graphics/primitives/arc/struct.Arc.html)
    * [Sectors](https://docs.rs/embedded-graphics/latest/embedded_graphics/primitives/sector/struct.Sector.html)
    * [Triangles](https://docs.rs/embedded-graphics/latest/embedded_graphics/primitives/triangle/struct.Triangle.html)
    * [Polylines](https://docs.rs/embedded-graphics/latest/embedded_graphics/primitives/polyline/struct.Polyline.html)
    * [Rounded rectangles](https://docs.rs/embedded-graphics/latest/embedded_graphics/primitives/rounded_rectangle/struct.RoundedRectangle.html)
* [Text](https://docs.rs/embedded-graphics/latest/embedded_graphics/text/index.html)
* [Monospaced fonts](https://docs.rs/embedded-graphics/latest/embedded_graphics/mono_font/index.html)

## Additional functions provided by external crates

Embedded-graphics is designed to be extended by the application or other crates. Examples of
this are adding support for different image formats or implementing custom fonts.

* [BMP images - `tinybmp`](https://crates.io/crates/tinybmp)
* [TGA images - `tinytga`](https://crates.io/crates/tinytga)
* [ProFont monospace font - `profont`](https://crates.io/crates/profont)
* [Picofont Pico8 font - `embedded-picofont`](https://crates.io/crates/embedded_picofont)
* [IBM437 font - `ibm437`](https://crates.io/crates/ibm437)
* [Simple layout/alignment functions - `embedded-layout`](https://crates.io/crates/embedded-layout)
* [TextBox with text alignment options - `embedded-text`](https://crates.io/crates/embedded-text)
* [Heapless plotting library for small embedded targets - `embedded-plots`](https://crates.io/crates/embedded-plots)

Note that some of these crates may not support the latest version of embedded-graphics.

If you know of a crate that is not in this list, please [open an
issue](https://github.com/embedded-graphics/embedded-graphics/issues/new) to add it.

## Display drivers

To support many different kinds of display, embedded-graphics doesn't include any drivers
directly but provides the [`DrawTarget`] API in [`embedded-graphics-core`] that can be
implemented by external crates. In addition to the drivers for real displays, the
[simulator](https://docs.rs/embedded-graphics-simulator/) can be used to test code during
development.

![Photographs showing embedded-graphics running on physical display hardware.](https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/4c680418cc4c37b8f7ed92d8b26edfde880d4c96/assets/banner-photo.jpg)

These are just some of the displays the community has added embedded-graphics support to. This
list is taken from the [dependent crates
list](https://crates.io/crates/embedded-graphics/reverse_dependencies) on crates.io so might be
missing some unpublished entries. Please [open an
issue](https://github.com/embedded-graphics/embedded-graphics/issues/new) if there's a display driver
that should be added to this list.

Note that some drivers may not support the latest version of embedded-graphics.

* [embedded-graphics-web-simulator](https://crates.io/crates/embedded-graphics-web-simulator): Simulated display in your browser via Webassembly
* [epd-waveshare](https://crates.io/crates/epd-waveshare) Driver for various ePaper displays (EPD) from Waveshare
* [hub75](https://crates.io/crates/hub75): A rust driver for hub75 rgb matrix displays
* [ili9341](https://crates.io/crates/ili9341): A platform agnostic driver to interface with the ILI9341 (and ILI9340C) TFT LCD display
* [ls010b7dh01](https://crates.io/crates/ls010b7dh01): A platform agnostic driver for the LS010B7DH01 memory LCD display
* [push2_display](https://crates.io/crates/push2_display): Ableton Push2 embedded-graphics display driver
* [sh1106](https://crates.io/crates/sh1106): I2C driver for the SH1106 OLED display
* [ssd1306](https://crates.io/crates/ssd1306): I2C and SPI (4 wire) driver for the SSD1306 OLED display
* [ssd1309](https://crates.io/crates/ssd1309): I2C/SPI driver for the SSD1309 OLED display written in 100% Rust.
* [ssd1322](https://crates.io/crates/ssd1322): Pure Rust driver for the SSD1322 OLED display chip
* [ssd1331](https://crates.io/crates/ssd1331): SPI (4 wire) driver for the SSD1331 OLED display
* [ssd1351](https://crates.io/crates/ssd1351): SSD1351 driver
* [ssd1675](https://crates.io/crates/ssd1675): Rust driver for the Solomon Systech SSD1675 e-Paper display (EPD) controller
* [st7735-lcd](https://crates.io/crates/st7735-lcd): Rust library for displays using the ST7735 driver
* [st7789](https://crates.io/crates/st7789): A Rust driver library for ST7789 displays
* [st7920](https://crates.io/crates/st7920): ST7920 LCD driver in Rust

## Simulator

Embedded graphics comes with a [simulator]! The simulator can be used to test and debug
embedded graphics code, or produce examples and interactive demos to show off embedded graphics
features.

![A screenshot of embedded-graphics running in its simulator.](https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/4c680418cc4c37b8f7ed92d8b26edfde880d4c96/assets/simulator-demo.png)

Take a look at the [examples repository](https://github.com/embedded-graphics/examples) to see what
embedded-graphics can do, and how it might look on a display. You can run the examples like
this:

```bash
git clone https://github.com/embedded-graphics/examples.git
cd examples

cargo run --example hello-world
```

## Crate features

Additional features can be enabled by adding the following features to your `Cargo.toml`.

* `nalgebra_support` - use the [Nalgebra](https://crates.io/crates/nalgebra) crate with `no_std`
support to enable conversions from `nalgebra::Vector2` to [`Point`] and [`Size`].

* `fixed_point` - use fixed point arithmetic instead of floating point for all trigonometric
calculation.

## Migrating from older versions

* [Migration guide from 0.5 to 0.6](https://github.com/embedded-graphics/embedded-graphics/blob/master/MIGRATING-0.5-0.6.md).
* [Migration guide from 0.6 to 0.7](https://github.com/embedded-graphics/embedded-graphics/blob/master/MIGRATING-0.5-0.6.md).

## Implementing `embedded_graphics` support for a display driver

To add support for embedded-graphics to a display driver, [`DrawTarget`] from
[`embedded-graphics-core`] must be implemented. This allows all embedded-graphics items to be
rendered by the display. See the [`DrawTarget`] documentation for implementation details.

## Examples

### Drawing examples

[![A grid of screenshots showing primitives, text and other items that can be drawn using embedded-graphics.](https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/4c680418cc4c37b8f7ed92d8b26edfde880d4c96/doc/assets/all_drawing_ops.png)](https://docs.rs/embedded-graphics/latest/embedded_graphics/examples/index.html)

Example usage of drawing primitives, text and images with embedded-graphics can be found [here](https://docs.rs/embedded-graphics/latest/embedded_graphics/examples/index.html).

### Shapes and text

The following example draws some shapes and text to a [`MockDisplay`] in place of target
hardware. The [simulator](https://docs.rs/embedded-graphics-simulator/) can also be used for
debugging, development or if hardware is not available.

```rust
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{
        Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, Triangle,
    },
    text::{Alignment, Text},
    mock_display::MockDisplay,
};

fn main() -> Result<(), std::convert::Infallible> {
    // Create a new mock display
    let mut display: MockDisplay<BinaryColor> = MockDisplay::new();

    // Create styles used by the drawing operations.
    let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let thick_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 3);
    let border_stroke = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(3)
        .stroke_alignment(StrokeAlignment::Inside)
        .build();
    let fill = PrimitiveStyle::with_fill(BinaryColor::On);
    let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

    let yoffset = 10;

    // Draw a 3px wide outline around the display.
    display
        .bounding_box()
        .into_styled(border_stroke)
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
    Text::with_alignment(
        text,
        display.bounding_box().center() + Point::new(0, 15),
        character_style,
        Alignment::Center,
    )
    .draw(&mut display)?;

    Ok(())
}
```

This example is also included in the [examples](https://github.com/embedded-graphics/examples) repository and
can be run using `cargo run --example hello-world`. It produces this output:

![Embedded Graphics Simulator example screenshot](https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/4c680418cc4c37b8f7ed92d8b26edfde880d4c96/assets/hello-world-simulator.png)

Additional examples can be found in the [examples](https://github.com/embedded-graphics/examples) repository.

[`Circle`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/primitives/circle/struct.Circle.html
[`MockDisplay`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/mock_display/struct.MockDisplay.html
[`Point`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/geometry/struct.Point.html
[`Size`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/geometry/struct.Size.html
[`DrawTarget`]: https://docs.rs/embedded-graphics-core/latest/embedded_graphics_core/draw_target/trait.DrawTarget.html
[`embedded-graphics-core`]: https://docs.rs/embedded-graphics-core/
[`Drawable`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/drawable/trait.Drawable.html
[simulator]: https://github.com/embedded-graphics/simulator
[simulator examples]: https://github.com/embedded-graphics/simulator/tree/master/examples

## Minimum supported Rust version

The minimum supported Rust version for embedded-graphics is `1.40.0` or greater.
Ensure you have the correct version of Rust installed, preferably through <https://rustup.rs>.

## Development setup

Please see the [development setup guide](https://github.com/embedded-graphics/embedded-graphics/blob/master/doc/development-setup.md).

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
