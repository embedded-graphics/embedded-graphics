//! # Embedded graphics
//!
//! This crate aims to make drawing 2D graphics primitives super easy. It currently supports the
//! following:
//!
//! * [raw data images](./image/struct.Image.html)
//! * [BMP-format images](./image/struct.ImageBmp.html) (with `bmp` feature enabled)
//! * [TGA-format images](./image/struct.ImageTga.html) (with `tga` feature enabled)
//! * [Primitives](./primitives/index.html)
//!     * [Lines](./primitives/line/struct.Line.html)
//!     * [Rectangles (and squares)](./primitives/rectangle/struct.Rectangle.html)
//!     * [Circles](./primitives/circle/struct.Circle.html)
//!     * [Triangles](./primitives/triangle/struct.Triangle.html)
//! * [Text with multiple fonts](./fonts/index.html#types)
//!
//! You can also add your own objects by implementing `IntoIterator<Item = Pixel<C>>` to create an
//! iterator that [`Drawing#draw()`][`Drawing`] can consume.
//!
//! A core goal is to do the above without using any buffers; the crate should work without a
//! dynamic memory allocator and without pre-allocating large chunks of memory. To achieve this, it
//! takes an `Iterator` based approach, where pixel values and positions are calculated on the fly,
//! with the minimum of saved state. This allows the consuming application to use far less RAM at
//! little to no performance penalty.
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
//! * [ili9341](https://crates.io/crates/ili9341): A platform agnostic driver to interface with the ILI9341 (and ILI9340C) TFT LCD display
//! * [ls010b7dh01](https://crates.io/crates/ls010b7dh01): A platform agnostic driver for the LS010B7DH01 memory LCD display
//! * [sh1106](https://crates.io/crates/sh1106): I2C driver for the SH1106 OLED display
//! * [ssd1306](https://crates.io/crates/ssd1306): I2C and SPI (4 wire) driver for the SSD1306 OLED display
//! * [ssd1322](https://crates.io/crates/ssd1322): Pure Rust driver for the SSD1322 OLED display chip
//! * [ssd1331](https://crates.io/crates/ssd1331): SPI (4 wire) driver for the SSD1331 OLED display
//! * [ssd1351](https://crates.io/crates/ssd1351): SSD1351 driver
//! * [ssd1675](https://crates.io/crates/ssd1675): Rust driver for the Solomon Systech SSD1675 e-Paper display (EPD) controller
//! * [st7735-lcd](https://crates.io/crates/st7735-lcd): Rust library for displays using the ST7735 driver
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
//! * `bmp` - use the [TinyBMP](https://crates.io/crates/tinybmp) crate for BMP image support.
//! * `tga` - use the [TinyTGA](https://crates.io/crates/tinytga) crate for TGA image support.
//!
//! # Examples
//!
//! ## Draw a circle and some text
//!
//! This example uses the [`Circle`] primitive and the [`Font6x8`] font to draw a filled circle and  some text over it on the screen.
//!
//! ```rust
//! use embedded_graphics::prelude::*;
//! use embedded_graphics::primitives::Circle;
//! use embedded_graphics::fonts::Font6x8;
//! use embedded_graphics::pixelcolor::Rgb565;
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display = MockDisplay::default();
//!
//! let c = Circle::new(Point::new(20, 20), 8).fill(Some(Rgb565::RED));
//! let t = Font6x8::render_str("Hello Rust!").fill(Some(Rgb565::GREEN)).translate(Point::new(20, 16));
//!
//! display.draw(c);
//! display.draw(t);
//! ```
//!
//! ## Draw a circle and some text
//!
//! To make life even easier, some handy [macros](#macros) are provided for drawing styled
//! primitives and text. Converting the example above, we get this:
//!
//! ```rust
//! use embedded_graphics::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//! use embedded_graphics::{text_6x8, egcircle};
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display = MockDisplay::default();
//!
//! let c = egcircle!((20, 20), 8, fill = Some(Rgb565::RED));
//! let t = text_6x8!("Hello Rust!", fill = Some(Rgb565::GREEN)).translate(Point::new(20, 16));
//!
//! display.draw(c);
//! display.draw(t);
//! ```
//!
//! ## Chaining
//!
//! Items can be chained to build more complex graphics objects.
//!
//! ```rust
//! use embedded_graphics::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//! use embedded_graphics::{text_6x8, egcircle, egrectangle};
//! # use embedded_graphics::mock_display::MockDisplay;
//!
//! fn build_thing(text: &'static str) -> impl Iterator<Item = Pixel<Rgb565>> {
//!     egrectangle!((0, 0), (40, 40)).into_iter()
//!         .chain(egcircle!((20, 20), 8, fill = Some(Rgb565::RED)))
//!         .chain(text_6x8!(text, fill = Some(Rgb565::GREEN)).translate(Point::new(20, 16)))
//! }
//!
//! fn main() {
//!     # let mut display = MockDisplay::default();
//!     display.draw(build_thing("Hello Rust!"));
//! }
//! ```
//!
//! # Implementing `embedded_graphics` in a driver
//!
//! To add support for embedded_graphics to a display driver, [`Drawing`] (and if possible
//! [`SizedDrawing`]) should be implemented. This allows all embedded_graphics objects to be
//! rendered by the display. See their [respective][`Drawing`] [docs][`SizedDrawing`] for
//! implementation details.
//!
//! [`Circle`]: ./primitives/circle/struct.Circle.html
//! [`Point`]: ./geometry/struct.Point.html
//! [`Size`]: ./geometry/struct.Size.html
//! [`Font6x8`]: ./fonts/type.Font6x8.html
//! [`Drawing`]: ./trait.Drawing.html
//! [`SizedDrawing`]: ./trait.SizedDrawing.html

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/jamwaffles/embedded-graphics/01d2ea6e7053f9f79cab19d0c193a00dbdaea321/assets/logo.png"
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

mod check_readme;
pub mod drawable;
pub mod fonts;
pub mod geometry;
pub mod image;
#[doc(hidden)]
pub mod mock_display;
pub mod pixelcolor;
pub mod prelude;
pub mod primitives;
pub mod style;
pub mod transform;

use crate::geometry::Dimensions;
use crate::pixelcolor::PixelColor;

/// To use this crate in a driver, `Drawing` must be implemented. This allows display drivers to
/// support all embedded_graphics objects through the `draw()` method.
///
/// Note that you should also implement [`SizedDrawing`] if the display supports partial updates.
///
/// Here's an example for an imaginary display that has a 64x64px framebuffer of 8 bit values that
/// communicates over a (simplified) SPI interface:
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::Drawing;
/// use embedded_graphics::egcircle;
/// use embedded_graphics::pixelcolor::{Gray8, GrayColor};
///
/// # struct SPI1;
/// #
/// # impl SPI1 {
/// #     pub fn send_bytes(&self, buf: &[u8]) -> Result<(), ()> {
/// #         Ok(())
/// #     }
/// # }
/// #
/// /// A fake display 64px x 64px where each pixel is stored as a single `u8`
/// struct ExampleDisplay {
///     framebuffer: [u8; 64 * 64],
///     iface: SPI1,
/// }
///
/// impl ExampleDisplay {
///     /// Send buffer to the display
///     pub fn flush(&self) -> Result<(), ()> {
///         self.iface.send_bytes(&self.framebuffer)
///     }
/// }
///
/// impl Drawing<Gray8> for ExampleDisplay {
///     /// Draw any item that can produce an iterator of `Pixel`s that have a colour defined as a `Gray8`
///     fn draw<T>(&mut self, item: T)
///     where
///         T: IntoIterator<Item = Pixel<Gray8>>,
///     {
///         for Pixel(coord, color) in item {
///             // Place an (x, y) pixel at the right index in the framebuffer
///             let index = coord[0] + (coord[1] * 64);
///
///             self.framebuffer[index as usize] = color.luma();
///         }
///     }
/// }
///
/// fn main() {
///     let mut display = ExampleDisplay {
///         framebuffer: [0; 4096],
///         iface: SPI1
///     };
///
///     // Draw a circle centered around `(32, 32)` with a radius of `10` and a white stroke
///     display.draw(egcircle!((32, 32), 10, stroke = Some(Gray8::WHITE)));
///
///     // Update the display
///     display.flush().expect("Failed to send data to display");
/// }
/// ```
///
/// [`SizedDrawing`]: ./trait.SizedDrawing.html
pub trait Drawing<C>
where
    C: PixelColor,
{
    /// Draw an object from an iterator over its pixels
    fn draw<T>(&mut self, item: T)
    where
        T: IntoIterator<Item = drawable::Pixel<C>>;
}

/// Very similar to the [`Drawing`] trait, but accepts drawable objects which have a known size
///
/// If the device used supports partial updates where only a given range of pixels is updated, you
/// should also implement `SizedDrawing` alongside [`Drawing`]. This trait is similar to `Drawing`,
/// but has a bound on [`Dimensions`] which provides methods for getting the bounding box of the
/// passed item to draw.
///
/// The example below shows a contrived implementation for a display that doesn't require a
/// framebuffer. It sends pixels one by one to over the SPI bus which isn't very efficient, but that
/// could be fixed by using a fixed length chunked buffering scheme.
///
/// ```rust
/// use embedded_graphics::egcircle;
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::SizedDrawing;
/// use embedded_graphics::pixelcolor::{Gray8, GrayColor};
///
/// # struct SPI1;
/// #
/// # impl SPI1 {
/// #     pub fn send_bytes(&self, buf: &[u8]) -> Result<(), ()> {
/// #         Ok(())
/// #     }
/// #
/// #     pub fn send_command(&self, cmd: &[u8]) -> Result<(), ()> {
/// #         Ok(())
/// #     }
/// # }
/// #
/// /// A fake display 64px x 64px where each pixel is stored as a single `u8`
/// struct ExampleBufferlessDisplay {
///     iface: SPI1,
/// }
///
/// impl ExampleBufferlessDisplay {
///     /// Set draw area
///     pub fn set_draw_area(&self, x1: i32, y1: i32, x2: i32, y2: i32) -> Result<(), ()> {
///         // Some magic incantation to set a sub-area of the display to update
///         self.iface
///             .send_command(&[0xff, x1 as u8, y1 as u8, x2 as u8, y2 as u8])
///     }
/// }
///
/// impl SizedDrawing<Gray8> for ExampleBufferlessDisplay {
///     fn draw_sized<T>(&mut self, item: T)
///     where
///         T: IntoIterator<Item = Pixel<Gray8>> + Dimensions,
///     {
///         // Get bounding box `Point`s as `(u32, u32)`
///         let tl = item.top_left();
///         let br = item.bottom_right();
///
///         // Set a sub-area of the display to update
///         self.set_draw_area(tl[0], tl[1], br[0], br[1]);
///
///         // Send updated pixel one at a time. Could use a chunked buffer to make this more efficient.
///         // `coord` isn't used as the update area is the same as the item's bounding box which
///         // wraps the bytes automatically
///         for Pixel(_coord, color) in item {
///             self.iface.send_bytes(&[color.luma()]);
///         }
///     }
/// }
///
/// fn main() {
///     let mut display = ExampleBufferlessDisplay {
///         iface: SPI1
///     };
///
///     // Draw a circle centered around `(32, 32)` with a radius of `10` and a white stroke
///     display.draw_sized(egcircle!((32, 32), 10, stroke = Some(Gray8::WHITE)));
///
///     // No `flush()` is required as `draw_sized()` sends the bytes directly
/// }
/// ```
///
/// [`Drawing`]: ./trait.Drawing.html
/// [`Dimensions`]: ./geometry/trait.Dimensions.html
/// [`SizedDrawing`]: ./trait.SizedDrawing.html
pub trait SizedDrawing<C>
where
    C: PixelColor + Clone,
{
    /// Draw an object from an iterator over its pixels
    fn draw_sized<T>(&mut self, item: T)
    where
        T: IntoIterator<Item = drawable::Pixel<C>> + Dimensions;
}
