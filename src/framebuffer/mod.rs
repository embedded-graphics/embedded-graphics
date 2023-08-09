//! Framebuffer.
//!
//! This module contains two generic framebuffer implementations which can be used for a multitude
//! of applications. The examples sections shows some common uses for the framebuffers.
//!
//! The two implementations differ in the way the pixel data is stored. The [`ArrayFramebuffer`]
//! uses an owned array to store the pixel data and [`SliceFramebuffer`] uses a borrowed mutable
//! slice instead.
//!
//! # Examples
//!
//! ## Use a framebuffer to implement a display driver
//!
//! ```rust
//! use embedded_graphics::{
//!     common::{ColorType, Horizontal, buffer_size},
//!     draw_target::DrawTarget,
//!     framebuffer::{ArrayFramebuffer, Framebuffer},
//!     pixelcolor::{raw::order::LittleEndian, Rgb565, RgbColor},
//!     prelude::*,
//!     primitives::PrimitiveStyle,
//! };
//!
//! const DISPLAY_SIZE: Size = Size::new(320, 240);
//! type DisplayFramebuffer = ArrayFramebuffer<
//!     { buffer_size::<Rgb565, Horizontal>(DISPLAY_SIZE) },
//!     Rgb565,
//!     LittleEndian,
//!     Horizontal,
//! >;
//!
//! pub struct Display {
//!     framebuffer: DisplayFramebuffer,
//! }
//!
//! impl Display {
//!     pub const fn new() -> Self {
//!         Self {
//!             framebuffer: DisplayFramebuffer::new(DISPLAY_SIZE),
//!         }
//!     }
//!
//!     pub fn flush(&mut self) {
//!         let data = self.framebuffer.data();
//!
//!         // The data could now be transmitted to the display.
//!     }
//! }
//!
//! impl DrawTarget for Display {
//!     type Error = core::convert::Infallible;
//!
//!     fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
//!     where
//!         I: IntoIterator<Item = Pixel<Self::Color>>,
//!     {
//!         self.framebuffer.draw_iter(pixels)
//!     }
//!
//!     // A real implementation should also forward the other draw target methods.
//! }
//!
//! impl ColorType for Display {
//!     type Color = Rgb565;
//! }
//!
//! impl OriginDimensions for Display {
//!     fn size(&self) -> Size {
//!         self.framebuffer.size()
//!     }
//! }
//!
//! // The display driver can be used like any other embedded-graphics draw target:
//!
//! let mut display = Display::new();
//!
//! display.clear(Rgb565::BLUE).unwrap();
//! display
//!     .bounding_box()
//!     .into_styled(PrimitiveStyle::with_stroke(Rgb565::YELLOW, 1))
//!     .draw(&mut display)
//!     .unwrap();
//!
//! display.flush();
//! # assert_eq!(display.framebuffer.data()[0..2], Rgb565::YELLOW.to_le_bytes());
//! # assert_eq!(display.framebuffer.data()[(320 * 2) * 3 / 2..][0..2], Rgb565::BLUE.to_le_bytes());
//! ```
//!
//! ## Caching of complex drawing operations
//!
//! ```rust
//! use embedded_graphics::{
//!     common::{Horizontal, buffer_size},
//!     framebuffer::{ArrayFramebuffer, Framebuffer},
//!     image::Image,
//!     pixelcolor::{raw::order::LittleEndian, Gray8},
//!     prelude::*,
//! };
//!
//! const CACHE_SIZE: Size = Size::new(32, 32);
//! type Cache = ArrayFramebuffer<
//!     { buffer_size::<Gray8, Horizontal>(CACHE_SIZE) },
//!     Gray8,
//!     LittleEndian,
//!     Horizontal,
//! >;
//!
//! # fn draw_something<D: DrawTarget>(target: &mut D) {}
//! let mut cache: Cache = Cache::new(CACHE_SIZE);
//!
//! // Draw to the cache instead of directly to the display.
//! draw_something(&mut cache);
//!
//! // Use an `Image` to specify the position and draw the cache to the display.
//! # let mut display = embedded_graphics::mock_display::MockDisplay::<Gray8>::new();
//! Image::new(&cache.as_image(), Point::new(10, 10)).draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//!
//! ## Implement image filters
//!
//! ```rust,
//! use embedded_graphics::{
//!     framebuffer::{Framebuffer, SliceFramebuffer},
//!     pixelcolor::{raw::order::LittleEndian, Gray8},
//!     prelude::*,
//! };
//!
//! /// A basic implementation of a horizontal 1D box blur.
//! /// `pixel` and `set_pixel` are used to access and modify the image data.
//! fn blur_horizontal(framebuffer: &mut impl Framebuffer<Color = Gray8>) {
//!     let bounding_box = framebuffer.bounding_box();
//!     for y in bounding_box.rows() {
//!         let mut left = framebuffer.pixel(Point::new(0, y)).unwrap();
//!         for x in bounding_box.columns() {
//!             let center = framebuffer.pixel(Point::new(x, y)).unwrap();
//!             let right = framebuffer.pixel(Point::new(x + 1, y)).unwrap_or(center);
//!
//!             let sum =
//!                 u16::from(left.luma()) + u16::from(center.luma()) + u16::from(right.luma());
//!             let average = (sum / 3) as u8;
//!
//!             framebuffer.set_pixel(Point::new(x, y), Gray8::new(average));
//!
//!             left = center;
//!         }
//!     }
//! }
//!
//! #[rustfmt::skip]
//! let mut data = [
//!     0,   0,   0,   0, 0,
//!     0, 255, 255, 255, 0,
//!     0, 255,   0, 255, 0,
//!     0, 255, 255, 255, 0,
//!     0,   0,   0,   0, 0,
//! ];
//!
//! let mut framebuffer =
//!     SliceFramebuffer::<Gray8, LittleEndian>::new(&mut data, Size::new(5, 5)).unwrap();
//! blur_horizontal(&mut framebuffer);
//!
//! #[rustfmt::skip]
//! assert_eq!(framebuffer.data(), &[
//!      0,   0,   0,   0,  0,
//!     85, 170, 255, 170, 85,
//!     85,  85, 170,  85, 85,
//!     85, 170, 255, 170, 85,
//!      0,   0,   0,   0,  0,
//! ]);
//! ```

use crate::{
    common::{GetPixel, PixelArrangement, SetPixel},
    draw_target::DrawTarget,
    geometry::OriginDimensions,
    image::ImageRaw,
    pixelcolor::{raw::order::DataOrder, StorablePixelColor},
};

#[cfg(test)]
#[macro_use]
mod test_common;

mod array;
mod slice;

pub use array::ArrayFramebuffer;
pub use slice::SliceFramebuffer;

/// Trait with functions common to all framebuffer implementation.
pub trait Framebuffer: OriginDimensions + GetPixel + SetPixel + DrawTarget
where
    Self::Color: StorablePixelColor,
{
    /// The bit or byte order.
    type DataOrder: DataOrder<<Self::Color as StorablePixelColor>::Raw>;
    /// The pixel arrangement.
    type PixelArrangement: PixelArrangement;

    /// Returns a reference to the raw framebuffer data.
    fn data(&self) -> &[u8];

    /// Returns a mutable reference to the raw framebuffer data.
    fn data_mut(&mut self) -> &mut [u8];

    /// Returns an immutable view of the framebuffer.
    fn as_image(&self) -> ImageRaw<'_, Self::Color, Self::DataOrder, Self::PixelArrangement>;
}
