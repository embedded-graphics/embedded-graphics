//! This crate aims to make drawing 2D graphics primitives super easy. It currently supports the
//! following built in items:
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
//! * [Text with multiple fonts](./fonts/index.html#types)
//!
//! Additional functionality provided by external crates:
//!
//! * [BMP images - `tinybmp`](https://crates.io/crates/tinybmp)
//! * [TGA images - `tinytga`](https://crates.io/crates/tinytga)
//! * [ProFont monospace font - `profont`](https://crates.io/crates/profont)
//! * [Picofont Pico8 font - `embedded-picofont`](https://crates.io/crates/embedded_picofont)
//! * [IBM437 font - `ibm437`](https://crates.io/crates/ibm437)
//! * [Simple layout/alignment functions - `embedded-layout`](https://crates.io/crates/embedded-layout)
//!
//! If you know of a crate that is not in this list, please [open an
//! issue](https://github.com/jamwaffles/embedded-graphics/issues/new).
//!
//! Note that some of these crates may not support the latest version of embedded-graphics.
//!
//! You can also add your own objects by implementing [`Drawable`] on them. Additionally, all
//! iterators over pixels (`Iterator<Item = Pixel<C>>`) have a default [`Drawable`] implementation
//! already created.
//!
//! A core goal of embedded-graphics is to draw graphics without using any buffers; the crate is
//! `no_std` compatible and works without a dynamic memory allocator, and without pre-allocating
//! large chunks of memory. To achieve this, it takes an `Iterator` based approach, where pixel
//! values and positions are calculated on the fly, with the minimum of saved state. This allows the
//! consuming application to use far less RAM at little to no performance penalty.
//!
//! # Supported displays
//!
//! These are just some of the displays the community has added embedded_graphics support to. This
//! list is taken from the [dependent crates
//! list](https://crates.io/crates/embedded-graphics/reverse_dependencies) on crates.io so might be
//! missing some unpublished entries. Please [open an
//! issue](https://github.com/jamwaffles/embedded-graphics/issues/new) if there's a display driver
//! that should be added to this list.
//!
//! Note that some drivers may not support the latest version of embedded-graphics.
//!
//! * [embedded-graphics-web-simulator](https://crates.io/crates/embedded-graphics-web-simulator): Simulated display in your browser via Webassembly
//! * [epd-waveshare](https://crates.io/crates/epd-waveshare) Driver for various ePaper displays (EPD) from Waveshare
//! * [hub75](https://crates.io/crates/hub75): A rust driver for hub75 rgb matrix displays
//! * [ili9341](https://crates.io/crates/ili9341): A platform agnostic driver to interface with the ILI9341 (and ILI9340C) TFT LCD display
//! * [ls010b7dh01](https://crates.io/crates/ls010b7dh01): A platform agnostic driver for the LS010B7DH01 memory LCD display
//! * [sh1106](https://crates.io/crates/sh1106): I2C driver for the SH1106 OLED display
//! * [ssd1306](https://crates.io/crates/ssd1306): I2C and SPI (4 wire) driver for the SSD1306 OLED display
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
//! Embedded graphics comes with a [simulator]!
//!
//! ![It can display all sorts of embedded-graphics test code.](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/assets/simulator-demo.png)
//!
//! Take a look at the [simulator examples] to see what
//! embedded_graphics can do, and how it might look on a display. You can run the examples like
//! this:
//!
//! ```bash
//! git clone https://github.com/jamwaffles/embedded-graphics.git
//! cd embedded-graphics
//!
//! cargo run -p embedded-graphics-simulator --example hello
//! ```
//!
//! [simulator]: https://github.com/jamwaffles/embedded-graphics/tree/c4f74c12dae9f0a0193fa48192f905a002bf8c9d/simulator
//! [simulator examples]: https://github.com/jamwaffles/embedded-graphics/tree/c4f74c12dae9f0a0193fa48192f905a002bf8c9d/simulator/examples
//!
//! # Crate features
//!
//! Add these to your `Cargo.toml` to turn on extra bits of functionality.
//!
//! * `nalgebra_support` - use the [Nalgebra](https://crates.io/crates/nalgebra) crate with `no_std`
//! support to enable conversions from `nalgebra::Vector2` to [`Point`] and [`Size`].
//!
//! * `fixed_point` - use fixed point arithmetic instead of floating point for all trigonometric
//! calculation.
//!
//! # Implementing `embedded_graphics` in a driver
//!
//! To add support for embedded_graphics to a display driver, [`DrawTarget`] should be implemented.
//! This allows all embedded_graphics objects to be rendered by the display. See the [`DrawTarget`]
//! documentation for implementation details.
//!
//! # Examples
//!
//! ## Drawing examples
//!
//! [![Collage of drawing examples](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/doc/assets/all_drawing_ops.png)](https://github.com/jamwaffles/embedded-graphics/blob/master/doc/drawing-examples.md)
//!
//! Example usage of drawing primitives, text and images with embedded-graphics can be found [here](https://github.com/jamwaffles/embedded-graphics/blob/master/doc/drawing-examples.md).
//!
//! ## Draw a circle and some text
//!
//! This example uses the [`Circle`] primitive and the [`Font6x8`] font to draw a filled circle and  some text over it on the screen.
//!
//! ```rust
//! use embedded_graphics::{
//!     fonts::{Font6x8, Text},
//! #   mock_display::MockDisplay,
//!     pixelcolor::Rgb565,
//!     prelude::*,
//!     primitives::Circle,
//!     style::{PrimitiveStyle, TextStyle},
//! };
//!
//! # let mut display = MockDisplay::default();
//! # display.set_allow_overdraw(true);
//! # display.set_allow_out_of_bounds_drawing(true);
//!
//! let c = Circle::new(Point::new(12, 12), 17).into_styled(PrimitiveStyle::with_fill(Rgb565::RED));
//! let t = Text::new("Hello Rust!", Point::new(20, 16))
//!     .into_styled(TextStyle::new(Font6x8, Rgb565::GREEN));
//!
//! // The `display` variable contains a `DrawTarget` implementation provided by the display driver
//! // crate. See the driver crate documentation for more information about how it is constructed.
//! c.draw(&mut display)?;
//! t.draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//! ## Chaining
//!
//! Items can be chained to build more complex graphics objects.
//!
//! ```rust
//! use embedded_graphics::{
//!     fonts::{Font6x8, Text},
//!     mock_display::MockDisplay,
//!     pixelcolor::Rgb565,
//!     prelude::*,
//!     primitives::{Circle, Rectangle},
//!     style::{PrimitiveStyle, TextStyle},
//! };
//!
//! fn build_thing(text: &'static str) -> impl Iterator<Item = Pixel<Rgb565>> {
//!     Rectangle::new(Point::new(0, 0), Size::new(40, 40))
//!         .into_styled(PrimitiveStyle::with_stroke(Rgb565::CYAN, 1))
//!         .into_pixels()
//!         .chain(
//!             Circle::new(Point::new(12, 12), 17)
//!                 .into_styled(PrimitiveStyle::with_fill(Rgb565::RED))
//!                 .into_pixels(),
//!         )
//!         .chain(
//!             Text::new(text, Point::new(20, 16))
//!                 .into_styled(TextStyle::new(Font6x8, Rgb565::GREEN))
//!                 .into_pixels(),
//!         )
//! }
//!
//! # let mut display = MockDisplay::default();
//! # display.set_allow_overdraw(true);
//! # display.set_allow_out_of_bounds_drawing(true);
//! build_thing("Hello Rust!").draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//!
//! [`Circle`]: ./primitives/circle/struct.Circle.html
//! [`Point`]: ./geometry/struct.Point.html
//! [`Size`]: ./geometry/struct.Size.html
//! [`Font6x8`]: ./fonts/struct.Font6x8.html
//! [`DrawTarget`]: ./draw_target/trait.DrawTarget.html
//! [`Drawable`]: ./drawable/trait.Drawable.html

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/jamwaffles/embedded-graphics/191fe7f8a0fedc713f9722b9dc59208dacadee7e/assets/logo.svg?sanitize=true"
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

#[cfg(feature = "nalgebra_support")]
extern crate nalgebra;

mod draw_target;
mod drawable;
pub mod fonts;
pub mod geometry;
pub mod image;
pub mod mock_display;
pub mod pixel_iterator;
pub mod pixelcolor;
pub mod prelude;
pub mod primitives;
pub mod style;
pub mod transform;

pub use crate::{
    draw_target::DrawTarget,
    drawable::{Drawable, Pixel},
};
