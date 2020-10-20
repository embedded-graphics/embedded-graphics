//! A small BMP parser designed for embedded, no-std environments but usable anywhere. Beyond
//! parsing the image header, no other allocations are made.
//!
//! To use `tinybmp` without [`embedded-graphics`] the raw data for individual pixels in an image
//! can be accessed using the [`raw_pixels`] and [`raw_image_data`] methods provided by the [`Bmp`]
//! struct.
//!
//! # Examples
//!
//! ## Draw a BMP image to an `embedded-graphics` draw target
//!
//! This example loads a 16BPP image and draws it to an [`embedded-graphics`] compatible display.
//!
//! ```rust
//! # fn main() -> Result<(), core::convert::Infallible> {
//! use embedded_graphics::{image::Image, prelude::*};
//! use tinybmp::Bmp;
//! # use embedded_graphics::mock_display::MockDisplay;
//! # use embedded_graphics::pixelcolor::Rgb565;
//! # let mut display: MockDisplay<Rgb565> = MockDisplay::default();
//!
//! // Load 16BPP 8x8px image
//! let bmp: Bmp<Rgb565> = Bmp::from_slice(include_bytes!("../tests/chessboard-8px-color-16bit.bmp")).unwrap();
//!
//! let image = Image::new(&bmp, Point::zero());
//!
//! image.draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(()) }
//! ```
//!
//! ## Accessing the raw image data
//!
//! This example demonstrates how the image header and raw image data can be accessed to use
//! `tinybmp` without [`embedded-graphics`].
//!
//! ```rust
//! use tinybmp::{Bmp, Bpp, Header, RawPixel};
//!
//! let bmp = Bmp::from_slice_raw(include_bytes!("../tests/chessboard-8px-24bit.bmp"))
//!     .expect("Failed to parse BMP image");
//!
//! // Read the BMP header
//! assert_eq!(
//!     bmp.header,
//!     Header {
//!         file_size: 314,
//!         image_data_start: 122,
//!         bpp: Bpp::Bits24,
//!         image_width: 8,
//!         image_height: 8,
//!         image_data_len: 192
//!     }
//! );
//!
//! // Check that raw image data slice is the correct length (according to parsed header)
//! assert_eq!(bmp.raw_image_data().len(), bmp.header.image_data_len as usize);
//!
//! // Get an iterator over the pixel coordinates and values in this image and load into a vec
//! let pixels: Vec<RawPixel> = bmp.raw_pixels().collect();
//!
//! // Loaded example image is 8x8px
//! assert_eq!(pixels.len(), 8 * 8);
//! ```
//!
//! [`embedded-graphics`]: https://crates.io/crates/embedded-graphics
//! [`Header`]: ./header/struct.Header.html
//! [`Bmp`]: ./struct.Bmp.html
//! [`raw_pixels`]: ./struct.Bmp.html#method.raw_pixels
//! [`raw_image_data`]: ./struct.Bmp.html#method.raw_image_data

#![no_std]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

mod header;
mod pixels;
mod raw_pixels;

use core::marker::PhantomData;
use embedded_graphics::prelude::*;

use crate::header::parse_header;
pub use crate::{
    header::{Bpp, Header},
    pixels::Pixels,
    raw_pixels::{RawPixel, RawPixels},
};

/// A BMP-format bitmap
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Bmp<'a, C> {
    /// Image header
    pub header: Header,

    /// Image data
    image_data: &'a [u8],

    /// Color type
    color_type: PhantomData<C>,
}

impl<'a, C> Bmp<'a, C> {
    fn from_slice_common(bytes: &'a [u8]) -> Result<Self, ParseError> {
        let (_remaining, header) = parse_header(bytes).map_err(|_| ParseError::Header)?;

        let image_data = &bytes[header.image_data_start..];

        Ok(Self {
            header,
            image_data,
            color_type: PhantomData,
        })
    }

    /// Returns the BPP (bits per pixel) for this image.
    pub fn color_bpp(&self) -> Bpp {
        self.header.bpp
    }

    /// Returns a slice containing the raw image data.
    pub fn raw_image_data(&self) -> &[u8] {
        self.image_data
    }

    /// Returns an iterator over the raw pixels in the image.
    ///
    /// The iterator returns the raw pixel colors as `u32` values. To automatically convert the raw
    /// values into the color specified by `C` use [`pixels`] instead.
    ///
    /// [`pixels`]: #method.pixels
    pub fn raw_pixels<'b>(&'b self) -> RawPixels<'b, 'a, C> {
        RawPixels::new(self)
    }

    /// Returns the row length in bytes.
    ///
    /// Each row in a BMP file is a multiple of 4 bytes long.
    fn bytes_per_row(&self) -> usize {
        let bits_per_row = self.header.image_width as usize * usize::from(self.header.bpp.bits());

        (bits_per_row + 31) / 32 * (32 / 8)
    }
}

impl<'a> Bmp<'a, ()> {
    /// Create a bitmap object from a byte slice.
    ///
    /// The created object keeps a shared reference to the input and does not dynamically allocate
    /// memory.
    ///
    /// In contrast to the [`from_slice`] constructor no color type needs to be specified when
    /// calling this method. This will disable all functions that requires a specified color type,
    /// like the [`pixels`] method.
    ///
    /// [`from_slice`]: #method.from_slice
    /// [`pixels`]: #method.pixels
    pub fn from_slice_raw(bytes: &'a [u8]) -> Result<Self, ParseError> {
        Self::from_slice_common(bytes)
    }
}

impl<'a, C> Bmp<'a, C>
where
    C: PixelColor,
{
    /// Create a bitmap object from a byte slice.
    ///
    /// The created object keeps a shared reference to the input and does not dynamically allocate
    /// memory.
    ///
    /// The color type must be explicitly specified when this method is called, for example by
    /// using the turbofish syntax. An error is returned if the bit depth of the specified color
    /// type doesn't match the bit depth of the BMP file.
    pub fn from_slice(bytes: &'a [u8]) -> Result<Self, ParseError> {
        let bmp = Self::from_slice_common(bytes)?;

        if C::Raw::BITS_PER_PIXEL != usize::from(bmp.color_bpp().bits()) {
            if bmp.color_bpp() == Bpp::Bits32 && C::Raw::BITS_PER_PIXEL == 24 {
                // Allow 24BPP color types for 32BPP images to support RGB888 BMP files with
                // 4 bytes per pixel.
                // This check could be improved by using the bit masks available in BMP headers
                // with version >= 4, but we don't currently parse this information.
            } else {
                return Err(ParseError::MismatchedBpp(bmp.color_bpp().bits()));
            }
        }

        Ok(bmp)
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
        Pixels::new(self.raw_pixels())
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
        target.fill_contiguous(
            &self.bounding_box(),
            self.raw_pixels().map(|p| C::Raw::from_u32(p.color).into()),
        )
    }
}

impl<C> OriginDimensions for Bmp<'_, C>
where
    C: PixelColor,
{
    fn size(&self) -> Size {
        Size::new(self.header.image_width, self.header.image_height)
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
}
