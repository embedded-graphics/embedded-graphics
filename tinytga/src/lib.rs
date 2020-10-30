//! A small TGA parser designed for use with [embedded-graphics] targeting no-std environments but
//! usable anywhere. Beyond parsing the image header, no other allocations are made.
//!
//! tinytga provides two methods of accessing the pixel data inside a TGA file. The most convenient
//! way is to use a color type provided by [embedded-graphics] to define the format stored inside
//! the TGA file. But it is also possible to directly access the raw pixel representation instead.
//!
//! # Examples
//!
//! ## Using `Tga` to draw an image
//!
//! This example demonstrates how a TGA image can be drawn to a [embedded-graphics] draw target.
//!
//! The code uses the [`Tga`] struct and only works if the color format inside the TGA file is known
//! at compile time. While this makes the code less flexible it offers the best performance by
//! making sure that no unnecessary color conversions are used.
//!
//! ```rust
//! # fn main() -> Result<(), core::convert::Infallible> {
//! # let mut display = embedded_graphics::mock_display::MockDisplay::default();
//! use embedded_graphics::{image::Image, pixelcolor::Rgb888, prelude::*};
//! use tinytga::Tga;
//!
//! // Include an image from a local path as bytes
//! let data = include_bytes!("../tests/chessboard_4px_rle.tga");
//!
//! let tga: Tga<Rgb888> = Tga::from_slice(data).unwrap();
//!
//! let image = Image::new(&tga, Point::zero());
//!
//! image.draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(()) }
//! ```
//!
//! ## Using `DynamicTga` to draw an image
//!
//! The previous example had the limitation that the color format needed to be known at compile
//! time. In some use cases this can be a problem, for example if user supplied images should
//! be displayed. To handle these cases [`DynamicTga`] can be used, which performs color conversion
//! if necessary.
//!
//! ```rust
//! # fn main() -> Result<(), core::convert::Infallible> {
//! # let mut display = embedded_graphics::mock_display::MockDisplay::<Rgb888>::default();
//! use embedded_graphics::{image::Image, pixelcolor::Rgb888, prelude::*};
//! use tinytga::DynamicTga;
//!
//! // Include an image from a local path as bytes
//! let data = include_bytes!("../tests/chessboard_4px_rle.tga");
//!
//! let tga = DynamicTga::from_slice(data).unwrap();
//!
//! let image = Image::new(&tga, Point::zero());
//!
//! image.draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(()) }
//! ```
//! ## Accessing pixels using an embedded-graphics color type
//!
//! If [embedded-graphics] is not used to draw the TGA image, the color types provided by
//! [embedded-graphics] can still be used to access the pixel data using the
//! [`pixels`](struct.Tga.html#method.pixels) method.
//!
//! ```rust
//! use embedded_graphics::{prelude::*, pixelcolor::Rgb888};
//! use tinytga::{Bpp, ImageOrigin, ImageType, RawPixel, Tga, TgaHeader};
//!
//! // Include an image from a local path as bytes
//! let data = include_bytes!("../tests/chessboard_4px_rle.tga");
//!
//! // Create a TGA instance from a byte slice.
//! // The color type is set by defining the type of the `img` variable.
//! let img: Tga<Rgb888> = Tga::from_slice(data).unwrap();
//!
//! // Check the size of the image.
//! assert_eq!(img.size(), Size::new(4, 4));
//!
//! // Collect pixels into a vector.
//! let pixels: Vec<_> = img.pixels().collect();
//! ```
//!
//! ## Accessing raw pixel data
//!
//! If [embedded-graphics] is not used in the target application, the raw image data can be
//! accessed with the [`pixels`](struct.RawTga.html#method.pixels) method on
//! [`RawTga`]. The returned iterator produces a `u32` for each pixel value.
//!
//! ```rust
//! use embedded_graphics::{prelude::*, pixelcolor::Rgb888};
//! use tinytga::{Bpp, ImageOrigin, ImageType, RawPixel, RawTga, TgaHeader};
//!
//! // Include an image from a local path as bytes.
//! let data = include_bytes!("../tests/chessboard_4px_rle.tga");
//!
//! // Create a TGA instance from a byte slice.
//! let img = RawTga::from_slice(data).unwrap();
//!
//! // Take a look at the raw image header.
//! assert_eq!(
//!     img.header(),
//!     TgaHeader {
//!         id_len: 0,
//!         has_color_map: false,
//!         image_type: ImageType::RleTruecolor,
//!         color_map_start: 0,
//!         color_map_len: 0,
//!         color_map_depth: None,
//!         x_origin: 0,
//!         y_origin: 4,
//!         width: 4,
//!         height: 4,
//!         pixel_depth: Bpp::Bits24,
//!         image_origin: ImageOrigin::TopLeft,
//!         alpha_channel_depth: 0,
//!     }
//! );
//!
//! // Collect raw pixels into a vector.
//! let pixels: Vec<_> = img.pixels().collect();
//! ```
//!
//! # Embedded-graphics drawing performance
//!
//! [`Tga`] should by used instead of [`DynamicTga`] when possible to reduce the risk of
//! accidentally adding unnecessary color conversions.
//!
//! `tinytga` uses different code paths to draw images with different [`ImageOrigin`]s.
//! The performance difference between the origins will depend on the display driver, but using
//! images with the origin at the top left corner will generally result in the best performance.
//!
//! [`ImageOrigin`]: enum.ImageOrigin.html
//! [embedded-graphics]: https://docs.rs/embedded-graphics
//! [`Tga`]: ./struct.Tga.html
//! [`RawTga`]: ./struct.RawTga.html
//! [`DynamicTga`]: ./struct.DynamicTga.html
//! [`image_type`]: ./struct.TgaHeader.html#structfield.image_type
//! [`pixel_data`]: ./struct.Tga.html#structfield.pixel_data

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

mod color_map;
mod dynamic_tga;
mod footer;
mod header;
mod packet;
mod parse_error;
mod pixels;
mod raw_pixels;
mod raw_tga;

use core::marker::PhantomData;
use embedded_graphics::prelude::*;

pub use crate::{
    color_map::ColorMap,
    dynamic_tga::DynamicTga,
    header::{Bpp, ImageOrigin, ImageType, TgaHeader},
    parse_error::ParseError,
    pixels::Pixels,
    raw_pixels::{RawPixel, RawPixels},
    raw_tga::RawTga,
};

/// TGA image.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Tga<'a, C> {
    /// Raw TGA file.
    raw: RawTga<'a>,

    /// Color type.
    color_type: PhantomData<C>,
}

impl<'a, C> Tga<'a, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    /// Parses a TGA image from a byte slice.
    ///
    /// # Errors
    ///
    /// If the bit depth of the source image does not match the bit depth of the output color type
    /// `C`, this method will return a [`ParseError::MismatchedBpp`] error.
    ///
    /// [`ParseError::MismatchedBpp`]: enum.ParseError.html#variant.MismatchedBpp
    pub fn from_slice(data: &'a [u8]) -> Result<Self, ParseError> {
        let raw = RawTga::from_slice(data)?;

        Self::from_raw(raw)
    }

    /// Converts a raw TGA object into a embedded-graphics TGA object.
    ///
    /// # Errors
    ///
    /// If the bit depth of the source image does not match the bit depth of the output color type
    /// `C`, this method will return a [`ParseError::MismatchedBpp`] error.
    ///
    /// [`ParseError::MismatchedBpp`]: enum.ParseError.html#variant.MismatchedBpp
    pub fn from_raw(raw: RawTga<'a>) -> Result<Self, ParseError> {
        if C::Raw::BITS_PER_PIXEL != usize::from(raw.color_bpp().bits()) {
            return Err(ParseError::MismatchedBpp(raw.color_bpp().bits()));
        }

        Ok(Tga {
            raw,
            color_type: PhantomData,
        })
    }

    /// Returns a reference to the raw TGA image.
    ///
    /// The [`RawTga`] object can be used to access lower level details about the TGA file.
    ///
    /// [`RawTga`]: struct.RawTga.html
    pub fn as_raw(&self) -> &RawTga<'a> {
        &self.raw
    }

    /// Returns an iterator over the pixels in this image.
    pub fn pixels<'b>(&'b self) -> Pixels<'b, 'a, C> {
        Pixels::new(self.raw.pixels())
    }
}

impl<C> OriginDimensions for Tga<'_, C> {
    fn size(&self) -> Size {
        self.raw.size()
    }
}

impl<C> ImageDrawable for Tga<'_, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    type Color = C;

    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        self.raw.draw(target)
    }
}
