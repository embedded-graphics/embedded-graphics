//! A small BMP parser designed for embedded, no-std environments but usable anywhere. Beyond
//! parsing the image header, no other allocations are made.
//!
//! To use `tinybmp` without [`embedded-graphics`] the raw data for individual pixels in an image
//! can be accessed using the methods provided by the [`RawBmp`] struct.
//!
//! # Examples
//!
//! ## Using `Bmp` to draw a BMP image
//!
//! If the color format inside the BMP file is known at compile time the [`Bmp`] type can be used
//! to draw an image to an [`embedded-graphics`] draw target. The BMP file used in this example
//! uses 16 bits per pixel with a RGB565 format.
//!
//! ```rust
//! # fn main() -> Result<(), core::convert::Infallible> {
//! use embedded_graphics::{image::Image, prelude::*};
//! use tinybmp::Bmp;
//! # use embedded_graphics::mock_display::MockDisplay;
//! # use embedded_graphics::pixelcolor::Rgb565;
//! # let mut display: MockDisplay<Rgb565> = MockDisplay::default();
//!
//! let bmp_data = include_bytes!("../tests/chessboard-8px-color-16bit.bmp");
//!
//! // Load 16 BPP 8x8px image.
//! // Note: The color type is specified explicitly to match the format used by the BMP image.
//! let bmp = Bmp::<Rgb565>::from_slice(bmp_data).unwrap();
//!
//! // Draw the image with the top left corner at (10, 20) by wrapping it in
//! // an embedded-graphics `Image`.
//! Image::new(&bmp, Point::new(10, 20)).draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(()) }
//! ```
//!
//! ## Using `DynamicBmp` to draw a BMP image
//!
//! If the exact color format used in the BMP file isn't known at compile time, for example to read
//! user supplied images, the [`DynamicBmp`] can be used. Because automatic color conversion will
//! be used the drawing performance might be degraded in comparison to [`Bmp`].
//!
//! ```rust
//! # fn main() -> Result<(), core::convert::Infallible> {
//! use embedded_graphics::{image::Image, prelude::*};
//! use tinybmp::DynamicBmp;
//! # use embedded_graphics::mock_display::MockDisplay;
//! # use embedded_graphics::pixelcolor::Rgb565;
//! # let mut display: MockDisplay<Rgb565> = MockDisplay::default();
//!
//! let bmp_data = include_bytes!("../tests/chessboard-8px-color-16bit.bmp");
//!
//! // Load BMP image with unknown color format.
//! // Note: There is no need to explicitly specify the color type.
//! let bmp = DynamicBmp::from_slice(bmp_data).unwrap();
//!
//! // Draw the image with the top left corner at (10, 20) by wrapping it in
//! // an embedded-graphics `Image`.
//! Image::new(&bmp, Point::new(10, 20)).draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(()) }
//! ```
//!
//! ## Accessing the raw image data
//!
//! The [`RawBmp`] struct provides methods to access lower level information about a BMP file,
//! like the BMP header or the raw image data. An instance of this type can be created by using
//! [`from_slice`] or by accessing the underlying raw object of a [`Bmp`] or [`DynamicBmp`] object
//! by using [`as_raw`].
//!
//! ```rust
//! use embedded_graphics::prelude::*;
//! use tinybmp::{RawBmp, Bpp, Header, RawPixel};
//!
//! let bmp = RawBmp::from_slice(include_bytes!("../tests/chessboard-8px-24bit.bmp"))
//!     .expect("Failed to parse BMP image");
//!
//! // Read the BMP header
//! assert_eq!(
//!     bmp.header(),
//!     &Header {
//!         file_size: 314,
//!         image_data_start: 122,
//!         bpp: Bpp::Bits24,
//!         image_size: Size::new(8, 8),
//!         image_data_len: 192,
//!         channel_masks: None,
//!     }
//! );
//!
//! // Check that raw image data slice is the correct length (according to parsed header)
//! assert_eq!(bmp.image_data().len(), bmp.header().image_data_len as usize);
//!
//! // Get an iterator over the pixel coordinates and values in this image and load into a vec
//! let pixels: Vec<RawPixel> = bmp.pixels().collect();
//!
//! // Loaded example image is 8x8px
//! assert_eq!(pixels.len(), 8 * 8);
//! ```
//!
//! [`embedded-graphics`]: https://crates.io/crates/embedded-graphics
//! [`Header`]: ./header/struct.Header.html
//! [`Bmp`]: ./struct.Bmp.html
//! [`as_raw`]: ./struct.Bmp.html#method.as_raw
//! [`DynamicBmp`]: ./struct.DynamicBmp.html
//! [`RawBmp`]: ./struct.RawBmp.html
//! [`from_slice`]: ./struct.RawBmp.html#method.from_slice
//! [`pixels`]: ./struct.RawBmp.html#method.pixels
//! [`image_data`]: ./struct.RawBmp.html#method.image_data

#![no_std]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

mod dynamic_bmp;
mod header;
mod pixels;
mod raw_bmp;
mod raw_pixels;

use core::marker::PhantomData;
use embedded_graphics::prelude::*;

pub use crate::{
    dynamic_bmp::DynamicBmp,
    header::{Bpp, ChannelMasks, Header},
    pixels::Pixels,
    raw_bmp::RawBmp,
    raw_pixels::{RawPixel, RawPixels},
};

/// A BMP-format bitmap
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Bmp<'a, C> {
    raw_bmp: RawBmp<'a>,
    color_type: PhantomData<C>,
}

impl<'a, C> Bmp<'a, C>
where
    C: PixelColor,
{
    /// Creates a bitmap object from a byte slice.
    ///
    /// The created object keeps a shared reference to the input and does not dynamically allocate
    /// memory.
    ///
    /// The color type must be explicitly specified when this method is called, for example by
    /// using the turbofish syntax. An error is returned if the bit depth of the specified color
    /// type doesn't match the bit depth of the BMP file.
    pub fn from_slice(bytes: &'a [u8]) -> Result<Self, ParseError> {
        let raw_bmp = RawBmp::from_slice(bytes)?;

        if C::Raw::BITS_PER_PIXEL != usize::from(raw_bmp.color_bpp().bits()) {
            if raw_bmp.color_bpp() == Bpp::Bits32 && C::Raw::BITS_PER_PIXEL == 24 {
                // Allow 24BPP color types for 32BPP images to support RGB888 BMP files with
                // 4 bytes per pixel.
                // This check could be improved by using the bit masks available in BMP headers
                // with version >= 4, but we don't currently parse this information.
            } else {
                return Err(ParseError::MismatchedBpp(raw_bmp.color_bpp().bits()));
            }
        }

        Ok(Self {
            raw_bmp,
            color_type: PhantomData,
        })
    }

    /// Returns an iterator over the pixels in this image.
    ///
    /// The iterator automatically converts the pixel colors into an `embedded-graphics` color type,
    /// that is when the [`from_slice`] constructor was called. This method isn't available when
    /// the [`from_slice_raw`] constructor was used and the pixel can only be accessed by using the
    /// [`raw_pixels`] method.
    ///
    /// [`from_slice`]: #method.from_slice
    /// [`from_slice_raw`]: #method.from_slice_raw
    /// [`raw_pixels`]: #method.raw_pixels
    pub fn pixels<'b>(&'b self) -> Pixels<'b, 'a, C> {
        Pixels::new(self.raw_bmp.pixels())
    }

    /// Returns a reference to the raw BMP image.
    ///
    /// The [`RawBmp`] instance can be used to access lower level information about the BMP file.
    ///
    /// [`RawBmp`]: struct.RawBmp.html
    pub fn as_raw(&self) -> &RawBmp<'a> {
        &self.raw_bmp
    }
}

impl<C> ImageDrawable for Bmp<'_, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    type Color = C;

    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        self.as_raw().draw(target)
    }
}

impl<C> OriginDimensions for Bmp<'_, C>
where
    C: PixelColor,
{
    fn size(&self) -> Size {
        self.raw_bmp.size()
    }
}

/// Parse error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ParseError {
    /// An error occurred while parsing the header.
    Header,

    /// The image uses a bit depth that isn't supported by tinybmp.
    UnsupportedBpp(u16),

    /// The image bit depth doesn't match the specified color type.
    MismatchedBpp(u16),

    /// The image format isn't supported by `DynamicBmp`.
    UnsupportedDynamicBmpFormat,
}
