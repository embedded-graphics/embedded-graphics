//! A small TGA parser designed for use with [embedded-graphics] targetting no-std environments but 
//! usable anywhere. Beyond parsing the image header, no other allocations are made.
//!
//! tinytga provides two methods of accessing the pixel data inside a TGA file. The most convenient
//! way is to use a color type provided by [embedded-graphics] to define the format stored inside
//! the TGA file. But it is also possible to directly access the raw pixel representation instead.
//!
//! # Examples
//!
//! ## Load a Run Length Encoded (RLE) TGA image
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
//! ## Drawing an image using `embedded-graphics`
//!
//! This example demonstrates how a TGA image can be drawn to a [embedded-graphics] draw target.
//!
//! ```rust
//! # fn main() -> Result<(), core::convert::Infallible> {
//! # let mut display = embedded_graphics::mock_display::MockDisplay::default();
//! use embedded_graphics::{image::Image, pixelcolor::Rgb888, prelude::*};
//! use tinytga::Tga;
//!
//! // Include an image from a local path as bytes
//! let data = include_bytes!("../tests/chessboard_4px_rle.tga");

//! let tga: Tga<Rgb888> = Tga::from_slice(data).unwrap();
//!
//! let image = Image::new(&tga, Point::zero());
//!
//! image.draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(()) }
//! ```
//!
//! ## Accessing raw pixel data
//!
//! If you do not want to use the color types provided by [embedded-graphics] you can also access
//! the raw image data.
//!
//! ```rust
//! use embedded_graphics::{prelude::*, pixelcolor::Rgb888};
//! use tinytga::{Bpp, ImageOrigin, ImageType, RawPixel, Tga, TgaHeader};
//!
//! // Include an image from a local path as bytes.
//! let data = include_bytes!("../tests/chessboard_4px_rle.tga");
//!
//! // Create a TGA instance from a byte slice.
//! let img = Tga::from_slice_raw(data).unwrap();
//!
//! // Take a look at the raw image header.
//! assert_eq!(
//!     img.raw_header(),
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
//! let pixels: Vec<_> = img.raw_pixels().collect();
//! ```
//!
//! # Embedded-graphics drawing performance
//!
//! `tinytga` uses different code paths to draw images with different [`ImageOrigin`]s.
//! The performance difference between the origins will depend on the display driver, but using
//! images with the origin at the top left corner will generally result in the best performance.
//!
//! [`ImageOrigin`]: enum.ImageOrigin.html
//! [embedded-graphics]: https://docs.rs/embedded-graphics
//! [`Tga`]: ./struct.Tga.html
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
mod footer;
mod header;
mod packet;
mod parse_error;
mod pixels;
mod raw_pixels;

use ::embedded_graphics::prelude::*;
use core::marker::PhantomData;
use nom::{bytes::complete::take, IResult};

use crate::footer::TgaFooter;
pub use crate::{
    color_map::ColorMap,
    header::{Bpp, ImageOrigin, ImageType, TgaHeader},
    parse_error::ParseError,
    pixels::Pixels,
    raw_pixels::{RawPixel, RawPixels},
};

/// TGA image
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Tga<'a, C> {
    /// Image data
    data: &'a [u8],

    /// Color map
    pub color_map: Option<ColorMap<'a>>,

    /// Image pixel data
    pixel_data: &'a [u8],

    /// Image size
    size: Size,

    /// Image type
    image_type: ImageType,

    /// Bits per pixel
    bpp: Bpp,

    /// Image origin
    image_origin: ImageOrigin,

    /// Color type
    color_type: PhantomData<C>,
}

impl<'a, C> Tga<'a, C> {
    /// Common part of `from_slice` and `from_slice_raw`.
    fn from_slice_common(data: &'a [u8]) -> Result<Self, ParseError> {
        let input = data;
        let (input, header) = TgaHeader::parse(input).map_err(|_| ParseError::Header)?;
        let (input, _image_id) = image_id(input, &header).map_err(|_| ParseError::Header)?;
        let (input, color_map) = ColorMap::parse(input, &header)?;

        let footer_length = TgaFooter::parse(data).map_or(0, |footer| footer.length(data));

        // Use saturating_sub to make sure this can't panic
        let pixel_data = &input[0..input.len().saturating_sub(footer_length)];

        let size = Size::new(u32::from(header.width), u32::from(header.height));

        Ok(Self {
            data,
            color_map,
            pixel_data,
            size,
            bpp: header.pixel_depth,
            image_origin: header.image_origin,
            image_type: header.image_type,
            color_type: PhantomData,
        })
    }

    /// Returns the color bit depth (BPP) of this image.
    pub fn color_bpp(&self) -> Bpp {
        if let Some(color_map) = &self.color_map {
            color_map.entry_bpp()
        } else {
            self.bpp
        }
    }

    /// Returns the image origin.
    pub fn image_origin(&self) -> ImageOrigin {
        self.image_origin
    }

    /// Returns the raw image data contained in this image.
    pub fn raw_image_data(&self) -> &'a [u8] {
        self.pixel_data
    }

    /// Returns an iterator over the raw pixels in this image.
    pub fn raw_pixels<'b>(&'b self) -> RawPixels<'b, 'a, C> {
        RawPixels::new(self)
    }

    /// Returns the TGA header.
    ///
    /// The returned object is a direct representation of the header contained
    /// in the TGA file. Most of the information contained in the header is also
    /// available using other methods, which are the preferred way of accessing
    /// them.
    ///
    /// # Performance
    ///
    /// To save memory the header is parsed every time this method is called.
    pub fn raw_header(&self) -> TgaHeader {
        // unwrap can't fail because the header was checked when self was created
        TgaHeader::parse(self.data).unwrap().1
    }

    /// Returns the developer directory.
    ///
    /// # Performance
    ///
    /// To save memory the footer is parsed every time this method is called.
    pub fn raw_developer_directory(&self) -> Option<&[u8]> {
        TgaFooter::parse(self.data).and_then(|footer| footer.developer_directory(self.data))
    }

    /// Returns the extension area.
    ///
    /// # Performance
    ///
    /// To save memory the footer is parsed every time this method is called.
    pub fn raw_extension_area(&self) -> Option<&[u8]> {
        TgaFooter::parse(self.data).and_then(|footer| footer.extension_area(self.data))
    }

    /// Returns the content of the image ID.
    ///
    /// If the TGA file doesn't contain an image ID `None` is returned.
    ///
    /// # Performance
    ///
    /// To save memory the header is parsed every time this method is called.
    pub fn image_id(&self) -> Option<&[u8]> {
        let (input, header) = TgaHeader::parse(self.data).ok()?;

        image_id(input, &header)
            .ok()
            .map(|(_input, id)| id)
            .filter(|id| !id.is_empty())
    }
}

impl<'a, C> Tga<'a, C>
where
    C: PixelColor,
{
    /// Parse a TGA image from a byte slice
    pub fn from_slice(data: &'a [u8]) -> Result<Self, ParseError> {
        let tga = Tga::from_slice_common(data)?;

        if C::Raw::BITS_PER_PIXEL != usize::from(tga.color_bpp().bits()) {
            return Err(ParseError::MismatchedBpp(tga.color_bpp().bits()));
        }

        Ok(tga)
    }

    /// Returns an iterator over the raw pixels in this image.
    pub fn pixels<'b>(&'b self) -> Pixels<'b, 'a, C> {
        Pixels::new(self.raw_pixels())
    }
}

impl<'a> Tga<'a, ()> {
    /// Parse a TGA image from a byte slice
    pub fn from_slice_raw(bytes: &'a [u8]) -> Result<Self, ParseError> {
        Tga::from_slice_common(bytes)
    }
}

impl<C> OriginDimensions for Tga<'_, C> {
    fn size(&self) -> Size {
        self.size
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
        // TGA files with the origin in the top left corner can be drawn using `fill_contiguous`.
        // All other origins are drawn by falling back to `draw_iter`.
        if self.image_origin == ImageOrigin::TopLeft {
            target.fill_contiguous(
                &self.bounding_box(),
                self.raw_pixels().map(|p| C::Raw::from_u32(p.color).into()),
            )
        } else {
            target.draw_iter(
                self.raw_pixels()
                    .map(|p| Pixel(p.position, C::Raw::from_u32(p.color).into())),
            )
        }
    }
}

fn image_id<'a>(input: &'a [u8], header: &TgaHeader) -> IResult<&'a [u8], &'a [u8]> {
    take(header.id_len)(input)
}
