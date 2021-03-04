//! Embedded-graphics is a 2D graphics library that is focused on memory constrained embedded devices.
//!
//! A core goal of embedded-graphics is to draw graphics without using any buffers; the crate is
//! `no_std` compatible and works without a dynamic memory allocator, and without pre-allocating
//! large chunks of memory. To achieve this, it takes an `Iterator` based approach, where pixel
//! colors and positions are calculated on the fly, with the minimum of saved state. This allows the
//! consuming application to use far less RAM at little to no performance penalty.
//!
//! It contains built in items that make it easy to draw 2D graphics primitives:
//!
//! * [Raw data images](./image/struct.ImageRaw.html)
//! * [Primitives](./primitives/index.html)
//!     * [Lines](./primitives/line/struct.Line.html)
//!     * [Rectangles (and squares)](./primitives/rectangle/struct.Rectangle.html)
//!     * [Circles](./primitives/circle/struct.Circle.html)
//!     * [Ellipses](./primitives/ellipse/struct.Ellipse.html)
//!     * [Arcs](./primitives/arc/struct.Arc.html)
//!     * [Sectors](./primitives/sector/struct.Sector.html)
//!     * [Triangles](./primitives/triangle/struct.Triangle.html)
//!     * [Polylines](./primitives/polyline/struct.Polyline.html)
//!     * [Rounded rectangles](./primitives/rounded_rectangle/struct.RoundedRectangle.html)
//! * [Text](./text/index.html)
//! * [Monospaced fonts](./mono_font/index.html)
//!
//! # Additional functions provided by external crates
//!
//! Embedded-graphics is designed to be extended by the application or other crates. Examples of
//! this are adding support for different image formats or implementing custom fonts.
//!
//! * [BMP images - `tinybmp`](https://crates.io/crates/tinybmp)
//! * [TGA images - `tinytga`](https://crates.io/crates/tinytga)
//! * [ProFont monospace font - `profont`](https://crates.io/crates/profont)
//! * [Picofont Pico8 font - `embedded-picofont`](https://crates.io/crates/embedded_picofont)
//! * [IBM437 font - `ibm437`](https://crates.io/crates/ibm437)
//! * [Simple layout/alignment functions - `embedded-layout`](https://crates.io/crates/embedded-layout)
//! * [TextBox with text alignment options - `embedded-text`](https://crates.io/crates/embedded-text)
//! * [Heapless plotting library for small embedded targets - `embedded-plots`](https://crates.io/crates/embedded-plots)
//!
//! Note that some of these crates may not support the latest version of embedded-graphics.
//!
//! If you know of a crate that is not in this list, please [open an
//! issue](https://github.com/embedded-graphics/embedded-graphics/issues/new) to add it.
//!
//! # Display drivers
//!
//! To support many different kinds of display, embedded-graphics doesn't include any drivers
//! directly but provides the [`DrawTarget`] API in [`embedded-graphics-core`] that can be
//! implemented by external crates. In addition to the drivers for real displays, the
//! [simulator](https://docs.rs/embedded-graphics-simulator/) can be used to test code during
//! development.
//!
//! ![Embedded graphics on real hardware](https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/master/assets/banner-photo.jpg)
//!
//! These are just some of the displays the community has added embedded-graphics support to. This
//! list is taken from the [dependent crates
//! list](https://crates.io/crates/embedded-graphics/reverse_dependencies) on crates.io so might be
//! missing some unpublished entries. Please [open an
//! issue](https://github.com/embedded-graphics/embedded-graphics/issues/new) if there's a display driver
//! that should be added to this list.
//!
//! Note that some drivers may not support the latest version of embedded-graphics.
//!
//! * [embedded-graphics-web-simulator](https://crates.io/crates/embedded-graphics-web-simulator): Simulated display in your browser via Webassembly
//! * [epd-waveshare](https://crates.io/crates/epd-waveshare) Driver for various ePaper displays (EPD) from Waveshare
//! * [hub75](https://crates.io/crates/hub75): A rust driver for hub75 rgb matrix displays
//! * [ili9341](https://crates.io/crates/ili9341): A platform agnostic driver to interface with the ILI9341 (and ILI9340C) TFT LCD display
//! * [ls010b7dh01](https://crates.io/crates/ls010b7dh01): A platform agnostic driver for the LS010B7DH01 memory LCD display
//! * [push2_display](https://crates.io/crates/push2_display): Ableton Push2 embedded-graphics display driver
//! * [sh1106](https://crates.io/crates/sh1106): I2C driver for the SH1106 OLED display
//! * [ssd1306](https://crates.io/crates/ssd1306): I2C and SPI (4 wire) driver for the SSD1306 OLED display
//! * [ssd1309](https://crates.io/crates/ssd1309): I2C/SPI driver for the SSD1309 OLED display written in 100% Rust.
//! * [ssd1322](https://crates.io/crates/ssd1322): Pure Rust driver for the SSD1322 OLED display chip
//! * [ssd1331](https://crates.io/crates/ssd1331): SPI (4 wire) driver for the SSD1331 OLED display
//! * [ssd1351](https://crates.io/crates/ssd1351): SSD1351 driver
//! * [ssd1675](https://crates.io/crates/ssd1675): Rust driver for the Solomon Systech SSD1675 e-Paper display (EPD) controller
//! * [st7735-lcd](https://crates.io/crates/st7735-lcd): Rust library for displays using the ST7735 driver
//! * [st7789](https://crates.io/crates/st7789): A Rust driver library for ST7789 displays
//! * [st7920](https://crates.io/crates/st7920): ST7920 LCD driver in Rust
//!
//! # Simulator
//!
//! Embedded graphics comes with a [simulator]! The simulator can be used to test and debug
//! embedded graphics code, or produce examples and interactive demos to show of embedded graphics
//! features.
//!
//! ![It can display all sorts of embedded-graphics test code.](https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/master/assets/simulator-demo.png)
//!
//! Take a look at the [simulator examples] to see what
//! embedded-graphics can do, and how it might look on a display. You can run the examples like
//! this:
//!
//! ```bash
//! git clone https://github.com/embedded-graphics/embedded-graphics.git
//! cd embedded-graphics
//!
//! cargo run -p embedded-graphics-simulator --example hello
//! ```
//!
//! [simulator]: https://github.com/embedded-graphics/simulator
//! [simulator examples]: https://github.com/embedded-graphics/simulator/tree/master/examples
//!
//! # Crate features
//!
//! Additional features can be enabled by adding the following features to your `Cargo.toml`.
//!
//! * `nalgebra_support` - use the [Nalgebra](https://crates.io/crates/nalgebra) crate with `no_std`
//! support to enable conversions from `nalgebra::Vector2` to [`Point`] and [`Size`].
//!
//! * `fixed_point` - use fixed point arithmetic instead of floating point for all trigonometric
//! calculation.
//!
//! # Migrating from 0.5 to 0.6
//!
//! Please read [the migration guide](https://github.com/embedded-graphics/embedded-graphics/blob/master/MIGRATING-0.5-0.6.md).
//!
//! # Implementing `embedded_graphics` support for a display driver
//!
//! To add support for embedded-graphics to a display driver, [`DrawTarget`] from
//! [`embedded-graphics-core`] must be implemented. This allows all embedded-graphics items to be
//! rendered by the display. See the [`DrawTarget`] documentation for implementation details.
//!
//! # Examples
//!
//! ## Drawing examples
//!
//! [![Collage of drawing examples](https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/master/doc/assets/all_drawing_ops.png)](https://github.com/embedded-graphics/embedded-graphics/blob/master/doc/drawing-examples.md)
//!
//! Example usage of drawing primitives, text and images with embedded-graphics can be found [here](https://github.com/embedded-graphics/embedded-graphics/blob/master/doc/drawing-examples.md).
//!
//! ## Shapes and text
//!
//! The following example draws some shapes and text to a [`MockDisplay`] in place of target
//! hardware. The [simulator](https://docs.rs/embedded-graphics-simulator/) can also be used for
//! debugging, development or if hardware is not available.
//!
//! ```rust,no_run
//! use embedded_graphics::{
//!     mono_font::{ascii::Font6x9, MonoTextStyle},
//!     pixelcolor::BinaryColor,
//!     prelude::*,
//!     primitives::{Circle, Rectangle, Triangle, PrimitiveStyle},
//!     text::Text,
//!     mock_display::MockDisplay,
//! };
//!
//! fn main() -> Result<(), std::convert::Infallible> {
//!     // Create a new mock display
//!     let mut display: MockDisplay<BinaryColor> = MockDisplay::new();
//!     # display.set_allow_overdraw(true);
//!
//!     // Create styles used by the drawing operations.
//!     let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
//!     let thick_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 3);
//!     let fill = PrimitiveStyle::with_fill(BinaryColor::On);
//!     let text_style = MonoTextStyle::new(Font6x9, BinaryColor::On);
//!
//!     let yoffset = 10;
//!
//!     // Draw a 3px wide outline around the display.
//!     let display_size = display.size() - Size::new(1, 1);
//!     Rectangle::new(Point::zero(), display_size)
//!         .into_styled(thick_stroke)
//!         .draw(&mut display)?;
//!
//!     // Draw a triangle.
//!     Triangle::new(
//!         Point::new(16, 16 + yoffset),
//!         Point::new(16 + 16, 16 + yoffset),
//!         Point::new(16 + 8, yoffset),
//!     )
//!     .into_styled(thin_stroke)
//!     .draw(&mut display)?;
//!
//!     // Draw a filled square
//!     Rectangle::new(Point::new(52, yoffset), Size::new(16, 16))
//!         .into_styled(fill)
//!         .draw(&mut display)?;
//!
//!     // Draw a circle with a 3px wide stroke.
//!     Circle::new(Point::new(88, yoffset), 17)
//!         .into_styled(thick_stroke)
//!         .draw(&mut display)?;
//!
//!     // Draw centered text.
//!     let text = "embedded-graphics";
//!     let width = text.len() as i32 * 6;
//!     Text::new(text, Point::new(64 - width / 2, 40))
//!         .into_styled(text_style)
//!         .draw(&mut display)?;
//!
//!     Ok(())
//! }
//! ```
//!
//! This example is also included in the [simulator](https://github.com/embedded-graphics/simulator/tree/master/examples) crate and
//! can be run using `cargo run --example hello-world`. It produces this output:
//!
//! ![Embedded Graphics Simulator example screenshot](https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/master/assets/hello-world-simulator.png)
//!
//! Additional examples can be found in the [simulator](https://github.com/embedded-graphics/simulator) crate.
//!
//! [`Circle`]: ./primitives/circle/struct.Circle.html
//! [`MockDisplay`]: ./mock_display/struct.MockDisplay.html
//! [`Point`]: ./geometry/struct.Point.html
//! [`Size`]: ./geometry/struct.Size.html
//! [`DrawTarget`]: https://docs.rs/embedded-graphics-core/latest/embedded_graphics_core/draw_target/trait.DrawTarget.html
//! [embedded-graphics-core]: https://docs.rs/embedded-graphics-core/
//! [`Drawable`]: ./drawable/trait.Drawable.html

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/191fe7f8a0fedc713f9722b9dc59208dacadee7e/assets/logo.svg?sanitize=true"
)]
#![no_std]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unsafe_code)]
#![deny(unstable_features)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]

pub mod draw_target;
pub mod geometry;
pub mod image;
pub mod iterator;
pub mod mock_display;
pub mod mono_font;
pub mod prelude;
pub mod primitives;
mod styled;
pub mod text;
pub mod transform;

pub use embedded_graphics_core::{pixelcolor, Drawable, Pixel};
pub use styled::Styled;

/// Trait to convert unsigned into signed integer.
trait SaturatingCast<T> {
    /// Casts a unsigned integer into a positive value.
    ///
    /// If the value is too large the maximum positive value is returned instead.
    fn saturating_cast(self) -> T;

    /// Casts a unsigned integer into a negative value.
    ///
    /// If the value is too large the minimum negative value is returned instead.
    fn saturating_cast_neg(self) -> T;
}

impl SaturatingCast<i32> for u32 {
    fn saturating_cast(self) -> i32 {
        if self < 0x8000_0000 {
            self as i32
        } else {
            i32::max_value()
        }
    }

    fn saturating_cast_neg(self) -> i32 {
        if self < 0x8000_0000 {
            -(self as i32)
        } else {
            i32::min_value()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saturating_cast() {
        assert_eq!(0u32.saturating_cast(), 0i32);
        assert_eq!(0u32.saturating_cast_neg(), 0i32);

        assert_eq!(1u32.saturating_cast(), 1i32);
        assert_eq!(1u32.saturating_cast_neg(), -1i32);

        assert_eq!(0x7FFF_FFFFu32.saturating_cast(), 0x7FFF_FFFFi32);
        assert_eq!(0x7FFF_FFFFu32.saturating_cast_neg(), -0x7FFF_FFFFi32);

        assert_eq!(0x8000_0000u32.saturating_cast(), 0x7FFF_FFFFi32);
        assert_eq!(0x8000_0000u32.saturating_cast_neg(), -0x8000_0000i32);

        assert_eq!(u32::max_value().saturating_cast(), 0x7FFF_FFFFi32);
        assert_eq!(u32::max_value().saturating_cast_neg(), -0x8000_0000i32);
    }
}
