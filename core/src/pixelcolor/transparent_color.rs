//! Transparent colors
//!
//! # Usage example
//!
//! TODO we need some implementation for a working usage.
//!
//! # Implementing transparent color types
//!
//! Transparent color types can be added by implementing the [`ColorBlend<C>`] trait with
//! an existing [`PixelColor`] as the underling color.
//! Please note that the transparent color may have the same [`PixelColor::Raw`] type as
//! its underlying color.
//!
//! A transparent color SHOULD implement [`From<C>`] so that a color converted
//! target can easily be created. This makes sure it is always possible to draw with an opaque color
//! over a `DrawTarget` that supports transparency.
//!
//! If a transparent color is implemented with an alpha channel (most of the time), it should also
//! implement `AlphaColor` to expose its alpha channel.
//!
//! If a transparent color has a natural underlying color, it should implement `HasAlphaColor` on
//! its underlying color to make it easy to create a blendable version of the color.
//!
//! # Implementation example
//! ```
//! use embedded_graphics::pixelcolor::{Gray8, ColorBlend};
//! use embedded_graphics::image::{Image, ImageRaw};
//! use embedded_graphics::prelude::*;
//! use embedded_graphics::pixelcolor::raw::RawU16;
//!
//! // A transparent color where alpha is handled as a u8
//! #[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
//! pub struct TransparentGray {
//!     alpha: u8,
//!     gray: Gray8,
//! }
//!
//! // PixelColor is needed for TransparentColor
//! impl PixelColor for TransparentGray {
//!     // this implementation doesn't expose raw data
//!     type Raw = RawU16;
//! }
//!
//! // From<RawU16> is needed for PixelColor
//! impl From<RawU16> for TransparentGray {
//!     fn from(value: RawU16) -> Self {
//!         let inner = value.into_inner();
//!         let alpha = ((inner & 0xFF00) >> 8) as u8;
//!         TransparentGray { alpha, gray: Gray8::new((inner & 0xFF) as u8)}
//!     }
//! }
//!
//! // From<TransparentGray> is needed for PixelColor
//! impl From<TransparentGray> for RawU16 {
//!     fn from(value: TransparentGray) -> Self {
//!         let TransparentGray { alpha, gray } = value;
//!         RawU16::new((alpha as u16) << 8 + gray.luma())
//!     }
//! }
//!
//! // Implement required methods
//! impl ColorBlend<Gray8> for TransparentGray {
//!     fn blend_over(self, other: Gray8) -> Gray8 {
//!         let luma = (self.alpha as u16 * self.gray.luma() as u16) +
//!                    (255 - self.alpha as u16) * other.luma() as u16;
//!         Gray8::new((luma / 255) as u8)
//!     }
//! }
//!
//! // Make conversion available for `DrawTargetExt::color_converted`
//! impl From<Gray8> for TransparentGray {
//!     fn from(value: Gray8) -> Self {
//!         TransparentGray { alpha: 255, gray: value }
//!     }
//! }
//!
//! // Mock transparent handling display
//! pub struct Display {}
//! impl DrawTarget for Display {
//!     type Color = TransparentGray;
//!     type Error = core::convert::Infallible;
//!
//!     fn draw_iter<I>(&mut self, pixels: I) -> Result<(), core::convert::Infallible>
//!     where
//!         I: IntoIterator<Item = Pixel<Self::Color>>,
//!     {
//!         for Pixel(point, color) in pixels.into_iter() {
//!             // mock existing pixel as middle gray
//!             let current_color = Gray8::new(127);
//!             // blend the color with existing color
//!             let final_color = color.blend_over(current_color);
//!             // ... draw final color on display
//!         }
//!         Ok(())
//!     }
//! }
//! impl OriginDimensions for Display {
//!     fn size(&self) -> Size {
//!         Size::new(300, 300)
//!     }
//! }
//!
//! // The image data.
//! const DATA: &[u8] = &[
//!     0x80, 0xFF, 0xFF, 0x80, //
//!     0xFF, 0x00, 0x00, 0xFF, //
//!     0xFF, 0x00, 0x00, 0xFF, //
//!     0x80, 0xFF, 0xFF, 0x80, //
//! ];
//!
//! // Create a 4x4 non-transparent image
//! let raw_image = ImageRaw::<Gray8>::new(DATA, Size::new(4,4)).unwrap();
//! let image = Image::new(&raw_image, Point::zero());
//!
//! // Create a transparent handling display
//! let mut display = Display{};
//!
//! // Draw a non-transparent object on a transparent handling display
//! // Conversion is automatically handled by the `From` trait above and the `DrawTargetExt` trait
//! image.draw(&mut display.color_converted()).unwrap();
//! ```

use crate::pixelcolor::PixelColor;

/// Transparent color trait.
///
/// This trait is used to represent color that handle transparency level.
/// They can blend with other colors, transparent or nor.
///
/// Transparent colors are a special kind of [`PixelColor`].
pub trait ColorBlend<C: PixelColor = Self>: PixelColor {
    /// Blend this color over another underlying opaque color
    fn blend_over(self, other: C) -> C;
}

/// Trait for transparent colors that expose their alpha channel
pub trait AlphaColor: PixelColor {
    ///Returns the alpha channel value.
    fn alpha(&self) -> u8;

    /// The maximum value of the alpha channel.
    const MAX_A: u8;
}

/// Trait used to create a transparent color that hold a pixelcolor with a compatible alpha channel
/// There can be only one matching AlphaColor for a given color.
///
/// Example:
/// ```ignore
/// # TODO we need some Rgba implementation for this test to pass
/// use embedded_graphics_core::pixelcolor::{Rgb888,HasAlphaColor,AlphaColor};
/// let color = Rgb888::new(0x0,0x80,0xFF);
/// let transparent = color.with_alpha(0);
/// ```
pub trait HasAlphaColor: PixelColor {
    /// Associated AlphaColor
    type AlphaColor: AlphaColor + ColorBlend<Self>;
}
