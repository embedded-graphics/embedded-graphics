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
//! You can also add your own objects by implementing [`Drawable`] on them. Additionally,
//! all iterators over pixels (`Iterator<Item = Pixel<C>>`) have a default [`Drawable`]
//! implementation already created.
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
//! use embedded_graphics::fonts::{Text, FONT6X8};
//! use embedded_graphics::pixelcolor::Rgb565;
//! use embedded_graphics::style::{PrimitiveStyle, TextStyle};
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display = MockDisplay::default();
//!
//! let c = Circle::new(Point::new(20, 20), 8).into_styled(PrimitiveStyle::with_fill(Rgb565::RED));
//! let t = Text::new("Hello Rust!", Point::new(20, 16)).into_styled(TextStyle::with_text_color(FONT6X8, Rgb565::GREEN));
//!
//! c.draw(&mut display);
//! t.draw(&mut display);
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
//! use embedded_graphics::fonts::FONT6X8;
//! use embedded_graphics::{egtext, egcircle};
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display = MockDisplay::default();
//!
//! let c = egcircle!((20, 20), 8, fill_color = Some(Rgb565::RED));
//! let t = egtext!("Hello Rust!", font = FONT6X8, text_color = Some(Rgb565::GREEN)).translate(Point::new(20, 16));
//!
//! c.draw(&mut display);
//! t.draw(&mut display);
//! ```
//!
//! ## Chaining
//!
//! Items can be chained to build more complex graphics objects.
//!
//! ```rust
//! use embedded_graphics::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//! use embedded_graphics::fonts::FONT6X8;
//! use embedded_graphics::{egtext, egcircle, egrectangle};
//! # use embedded_graphics::mock_display::MockDisplay;
//!
//! fn build_thing(text: &'static str) -> impl Iterator<Item = Pixel<Rgb565>> {
//!     egrectangle!((0, 0), (40, 40)).into_iter()
//!         .chain(&egcircle!((20, 20), 8, fill_color = Some(Rgb565::RED)))
//!         .chain(&egtext!(text, font = FONT6X8, text_color = Some(Rgb565::GREEN)).translate(Point::new(20, 16)))
//! }
//!
//! fn main() {
//!     # let mut display = MockDisplay::default();
//!     build_thing("Hello Rust!").draw(&mut display);
//! }
//! ```
//!
//! # Implementing `embedded_graphics` in a driver
//!
//! To add support for embedded_graphics to a display driver, [`DrawTarget`] should be implemented.
//! This allows all embedded_graphics objects to be rendered by the display. See the [`DrawTarget`]
//! documentation for implementation details.
//!
//! [`Circle`]: ./primitives/circle/struct.Circle.html
//! [`Point`]: ./geometry/struct.Point.html
//! [`Size`]: ./geometry/struct.Size.html
//! [`Font6x8`]: ./fonts/type.Font6x8.html
//! [`DrawTarget`]: ./trait.DrawTarget.html
//! [`Drawable`]: ./drawable/trait.Drawable.html

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

use crate::drawable::Drawable;
use crate::geometry::{Dimensions, Point, Size};
use crate::pixelcolor::PixelColor;
use crate::primitives::Primitive;
use crate::style::{PrimitiveStyle, Styled};

/// Defines a display that can be used to render [`Drawable`] objects.
///
/// To use this crate in a driver, `DrawTarget` must be implemented. This trait defines how a
/// display draws pixels, and optionally provides a way to define accelerated drawing methods for
/// graphical primitives such as lines, rectangles, triangles, and circles.
///
/// Once a `DrawTarget` is defined, it can be used to render [`Drawable`]s. Note that any iterator
/// over [`Pixel`]s has a default implementation for the [`Drawable`] trait. See the [`Drawable`]
/// trait documentation for more details.
///
/// [`Drawable`]: ./drawable/trait.Drawable.html
/// [`Pixel`]: ./drawable/struct.Pixel.html
///
/// Here's an example for an imaginary display that has a 64x64px framebuffer of 8 bit values that
/// communicates over a (simplified) SPI interface:
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::DrawTarget;
/// use embedded_graphics::egcircle;
/// use embedded_graphics::pixelcolor::{Gray8, GrayColor};
/// use embedded_graphics::drawable::Pixel;
/// use embedded_graphics::geometry::Size;
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
/// impl DrawTarget<Gray8> for ExampleDisplay {
///     /// Draw a `Pixel` that has a color defined as `Gray8`.
///     fn draw_pixel(&mut self, pixel: Pixel<Gray8>) {
///         let Pixel(coord, color) = pixel;
///         // Place an (x, y) pixel at the right index in the framebuffer
///         let index = coord.x + coord.y * 64;
///         self.framebuffer[index as usize] = color.luma();
///     }
///
///     fn size(&self) -> Size {
///         Size::new(64, 64)
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
///     let circle = egcircle!((32, 32), 10, stroke_color = Some(Gray8::WHITE));
///     circle.draw(&mut display);
///
///     // Update the display
///     display.flush().expect("Failed to send data to display");
/// }
/// ```
///
/// ## Hardware Acceleration
///
/// In addition to defining [`draw_pixel`], an implementation of [`DrawTarget`] can also provide
/// alternative implementations of drawing methods for graphical primitives. Here is an example of
/// how a display with accelerated methods can implement [`DrawTarget`]:
///
/// [`draw_pixel`]: ./trait.DrawTarget.html#method.draw_pixel
/// [`DrawTarget`]: ./trait.DrawTarget.html
///
/// ```rust
/// # use embedded_graphics::prelude::*;
/// # use embedded_graphics::DrawTarget;
/// # use embedded_graphics::egrectangle;
/// # use embedded_graphics::primitives::rectangle::Rectangle;
/// # use embedded_graphics::pixelcolor::{Gray8, GrayColor};
/// # use embedded_graphics::drawable::Pixel;
/// # use embedded_graphics::style::{PrimitiveStyle, Styled};
/// #
/// # struct SPI1;
/// #
/// # impl SPI1 {
/// #     pub fn send_bytes(&self, buf: &[u8]) -> Result<(), ()> {
/// #         Ok(())
/// #     }
/// # }
/// #
/// /// A fake display 64px x 64px where each pixel is stored as a single `u8`
/// struct FastExampleDisplay {
///     framebuffer: [u8; 64 * 64],
///     iface: SPI1,
/// }
///
/// impl FastExampleDisplay {
///     /// Send buffer to the display
///     pub fn flush(&self) -> Result<(), ()> {
///         self.iface.send_bytes(&self.framebuffer)
///     }
///
///     /// A HW-accelerated method for drawing rectangles
///     pub fn fast_rectangle(&self, rect: &Styled<Rectangle, PrimitiveStyle<Gray8>>) {
///         // Does some speedy drawing
///     }
/// }
///
/// impl DrawTarget<Gray8> for FastExampleDisplay {
///     /// Draw a `pixel` that has a colour defined as `Gray8`
///     fn draw_pixel(&mut self, pixel: Pixel<Gray8>) {
///         let Pixel(coord, color) = pixel;
///         // Place an (x, y) pixel at the right index in the framebuffer
///         let index = coord.x + coord.y * 64;
///         self.framebuffer[index as usize] = color.luma();
///     }
///
///     fn size(&self) -> Size {
///         Size::new(64, 64)
///     }
///
///     /// Use the accelerated method when drawing rectangles
///     fn draw_rectangle(&mut self, item: &Styled<Rectangle, PrimitiveStyle<Gray8>>) {
///         self.fast_rectangle(item);
///     }
/// }
///
/// fn main() {
///     let mut display = FastExampleDisplay {
///         framebuffer: [0; 4096],
///         iface: SPI1
///     };
///
///     // Draw a rectangle from (10, 20) to (30, 40) with a white stroke
///     let rect = egrectangle!((10, 20), (30, 40), stroke_color = Some(Gray8::WHITE));
///     rect.draw(&mut display); // Uses the accelerated draw_rectangle function
///
///     // Update the display
///     display.flush().expect("Failed to send data to display");
/// }
///
/// ```
pub trait DrawTarget<C>
where
    C: PixelColor,
{
    /// Draws a pixel on the display.
    ///
    /// Note that some displays require a "flush" operation
    /// to actually write changes to the framebuffer.
    fn draw_pixel(&mut self, item: drawable::Pixel<C>);

    /// Draws an object from an iterator over its pixels.
    fn draw_iter<T>(&mut self, item: T)
    where
        T: IntoIterator<Item = drawable::Pixel<C>>,
    {
        for pixel in item {
            self.draw_pixel(pixel);
        }
    }

    /// Returns the dimensions of the `DrawTarget` in pixels.
    fn size(&self) -> Size;

    /// Clears the display with the supplied color.
    ///
    /// This default implementation should be replaced if the implementing driver provides an
    /// accelerated clearing method.
    fn clear(&mut self, color: C)
    where
        Self: Sized,
    {
        primitives::Rectangle::new(Point::zero(), Point::zero() + self.size())
            .into_styled(PrimitiveStyle::with_fill(color))
            .draw(self);
    }

    /// Flushes changes to the framebuffer.
    ///
    /// Note that some displays operate in "immediate mode", which does not require any flushing.
    /// Because of this, the default implementation of this method is a noop. If the implementing
    /// display requires flushing, this method should be overriden in the trait impl.
    fn flush(&mut self) {}

    /// Draws a line primitive.
    ///
    /// This default trait method should be overridden if a display provides hardware-accelerated
    /// methods for drawing lines.
    ///
    /// # Caution
    ///
    /// This method should not be called directly from application code. It is used to define the
    /// internals of the [`draw`] method used for the [`Line`] primitive. To draw a line, call
    /// [`draw`] on a [`Line`] primitive object.
    ///
    /// [`Line`]: ./primitives/line/struct.Line.html
    /// [`draw`]: ./trait.DrawTarget.html#method.draw
    fn draw_line(&mut self, item: &Styled<primitives::Line, PrimitiveStyle<C>>) {
        self.draw_iter(item);
    }

    /// Draws a triangle primitive.
    ///
    /// This default trait method should be overridden if a display provides hardware-accelerated
    /// methods for drawing triangles.
    ///
    /// # Caution
    ///
    /// This method should not be called directly from application code. It is used to define the
    /// internals of the [`draw`] method used for the [`Triangle`] primitive. To draw a triangle, call
    /// [`draw`] on a [`Triangle`] primitive object.
    ///
    /// [`Triangle`]: ./primitives/triangle/struct.Triangle.html
    /// [`draw`]: ./trait.DrawTarget.html#method.draw
    fn draw_triangle(&mut self, item: &Styled<primitives::Triangle, PrimitiveStyle<C>>) {
        self.draw_iter(item);
    }

    /// Draws a rectangle primitive.
    ///
    /// This default trait method should be overridden if a display provides hardware-accelerated
    /// methods for drawing rectangle.
    ///
    /// # Caution
    ///
    /// This method should not be called directly from application code. It is used to define the
    /// internals of the [`draw`] method used for the [`Rectangle`] primitive. To draw a rectangle, call
    /// [`draw`] on a [`Rectangle`] primitive object.
    ///
    /// [`Rectangle`]: ./primitives/rectangle/struct.Rectangle.html
    /// [`draw`]: ./trait.DrawTarget.html#method.draw
    fn draw_rectangle(&mut self, item: &Styled<primitives::Rectangle, PrimitiveStyle<C>>) {
        self.draw_iter(item);
    }

    /// Draws a circle primitive.
    ///
    /// This default trait method should be overridden if a display provides hardware-accelerated
    /// methods for drawing circles.
    ///
    /// # Caution
    ///
    /// This method should not be called directly from application code. It is used to define the
    /// internals of the [`draw`] method used for the [`Circle`] primitive. To draw a circle, call
    /// [`draw`] on a [`Circle`] primitive object.
    ///
    /// [`Circle`]: ./primitives/circle/struct.Circle.html
    /// [`draw`]: ./trait.DrawTarget.html#method.draw
    fn draw_circle(&mut self, item: &Styled<primitives::Circle, PrimitiveStyle<C>>) {
        self.draw_iter(item);
    }
}
