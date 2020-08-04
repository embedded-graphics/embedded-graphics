//! A small BMP parser designed for embedded, no-std environments but usable anywhere. Beyond
//! parsing the image header, no other allocations are made.
//!
//! To access the individual pixels in an image, the [`Bmp`] struct implements `IntoIterator`. It is
//! also possible to access the raw image data by reading the [`pixel_data`] field.
//!
//! # Features
//!
//! * `graphics` - enables [embedded-graphics] integration.
//!
//! # Examples
//!
//! ## Load a BMP image and check its [`Header`] and returned pixels.
//!
//! ```rust
//! use tinybmp::{Bmp, FileType, Header, Pixel};
//!
//! let bmp = Bmp::from_slice(include_bytes!("../tests/chessboard-8px-24bit.bmp"))
//!     .expect("Failed to parse BMP image");
//!
//! // Read the BMP header
//! assert_eq!(
//!     bmp.header,
//!     Header {
//!         file_type: FileType::BM,
//!         file_size: 314,
//!         reserved_1: 0,
//!         reserved_2: 0,
//!         image_data_start: 122,
//!         bpp: 24,
//!         image_width: 8,
//!         image_height: 8,
//!         image_data_len: 192
//!     }
//! );
//!
//! // Check that raw image data slice is the correct length (according to parsed header)
//! assert_eq!(bmp.image_data().len(), bmp.header.image_data_len as usize);
//!
//! // Get an iterator over the pixel coordinates and values in this image and load into a vec
//! let pixels: Vec<Pixel> = bmp.into_iter().collect();
//!
//! // Loaded example image is 8x8px
//! assert_eq!(pixels.len(), 8 * 8);
//! ```
//!
//! ## Integrate with `embedded-graphics`
//!
//! This example loads a 16BPP image and draws it to an [embedded-graphics] compatible display.
//!
//! The `graphics` feature must be enabled for embedded-graphics support.
//!
//! ```rust
//! # #[cfg(feature = "graphics")] { fn main() -> Result<(), core::convert::Infallible> {
//! use embedded_graphics::{image::Image, prelude::*};
//! use tinybmp::EgBmp;
//! # use embedded_graphics::mock_display::MockDisplay;
//! # use embedded_graphics::pixelcolor::Rgb565;
//! # let mut display: MockDisplay<Rgb565> = MockDisplay::default();
//!
//! // Load 16BPP 8x8px image
//! let bmp: EgBmp<Rgb565> = EgBmp::from_slice(include_bytes!("../tests/chessboard-8px-color-16bit.bmp")).unwrap();
//!
//! let image = Image::new(&bmp, Point::zero());
//!
//! image.draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(()) } }
//! ```
//!
//! [embedded-graphics]: https://crates.io/crates/embedded-graphics
//! [`Header`]: ./header/struct.Header.html
//! [`Bmp`]: ./struct.Bmp.html
//! [`pixel_data`]: ./struct.Bmp.html#structfield.pixel_data

#![no_std]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

mod header;
mod pixel;

use crate::header::parse_header;
pub use crate::{
    header::{FileType, Header},
    pixel::Pixel,
};

/// A BMP-format bitmap
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Bmp<'a> {
    /// Image header
    pub header: Header,

    image_data: &'a [u8],
}

impl<'a> Bmp<'a> {
    /// Create a bitmap object from a byte array
    ///
    /// This method keeps a slice of the original input and does not dynamically allocate memory.
    /// The input data must live for as long as this BMP instance does.
    pub fn from_slice(bytes: &'a [u8]) -> Result<Self, ()> {
        let (_remaining, header) = parse_header(bytes).map_err(|_| ())?;

        let image_data = &bytes[header.image_data_start..];

        Ok(Bmp { header, image_data })
    }

    /// Get a reference to the range of bytes that represents the pixel data in the image
    pub fn image_data(&'a self) -> &'a [u8] {
        self.image_data
    }

    /// Get the image width in pixels
    pub fn width(&self) -> u32 {
        self.header.image_width
    }

    /// Get the image height in pixels
    pub fn height(&self) -> u32 {
        self.header.image_height
    }

    /// Get image dimensions as `(width, height)` in pixels
    pub fn dimensions(&self) -> (u32, u32) {
        (self.header.image_width, self.header.image_height)
    }

    /// Get the BPP (bits per pixel) for this image
    pub fn bpp(&self) -> u32 {
        u32::from(self.header.bpp)
    }

    /// Returns the row length in bytes.
    ///
    /// Each row in a BMP file is a multiple of 4 bytes long.
    fn bytes_per_row(&self) -> usize {
        let bits_per_row = self.width() as usize * self.bpp() as usize;

        (bits_per_row + 31) / 32 * (32 / 8)
    }
}

impl<'a> IntoIterator for &'a Bmp<'a> {
    type Item = Pixel;
    type IntoIter = BmpIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let pixel_stride = match self.bpp() {
            1 | 8 | 16 | 24 | 32 => self.bpp() as usize,
            depth => panic!("Bit depth {} not supported", depth),
        };

        BmpIterator {
            bmp: self,
            pixel_data: self.image_data(),
            pixel_stride,
            x: 0,
            y: 0,
            bit_idx: 0,
        }
    }
}

/// Iterator over individual BMP pixels
///
/// Each pixel is returned as a `u32` regardless of the bit depth of the source image.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct BmpIterator<'a> {
    /// Reference to original BMP image
    bmp: &'a Bmp<'a>,

    /// Image pixel data as a byte slice, little endian ordering
    pixel_data: &'a [u8],

    /// Number of bits per pixel
    pixel_stride: usize,

    /// Current X position
    x: u32,

    /// Current Y position
    y: u32,

    /// Start bit index for the current pixel.
    ///
    /// This is incremented by `pixel_stride` bits every iteration.
    bit_idx: usize,
}

impl<'a> Iterator for BmpIterator<'a> {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        let px = self.pixel_data;

        if self.y < self.bmp.height() {
            let x = self.x;
            let y = self.y;

            if self.x == 0 {
                let row_index = (self.bmp.height() - 1) - self.y;
                let row_start = self.bmp.bytes_per_row() * row_index as usize;

                self.bit_idx = row_start * 8;
            }

            self.x += 1;
            if self.x >= self.bmp.width() {
                self.y += 1;
                self.x = 0;
            }

            let byte_idx = self.bit_idx / 8;

            let pixel_value = match self.pixel_stride {
                1 => {
                    let mask = 0b_1000_0000 >> self.bit_idx % 8;
                    (px[byte_idx] & mask != 0) as u32
                }
                8 => u32::from(px[byte_idx]),
                16 => u32::from_le_bytes([px[byte_idx], px[byte_idx + 1], 0, 0]),
                24 => u32::from_le_bytes([px[byte_idx], px[byte_idx + 1], px[byte_idx + 2], 0]),
                32 => u32::from_le_bytes([
                    px[byte_idx],
                    px[byte_idx + 1],
                    px[byte_idx + 2],
                    px[byte_idx + 3],
                ]),
                _ => unreachable!(),
            };

            self.bit_idx += self.pixel_stride;

            Some(Pixel {
                x,
                y,
                color: pixel_value,
            })
        } else {
            None
        }
    }
}

#[cfg(feature = "graphics")]
mod e_g {
    use super::*;
    use core::marker::PhantomData;
    use embedded_graphics::{
        pixelcolor::{raw::RawData, PixelColor},
        prelude::*,
    };

    /// TODO: docs
    #[derive(Debug)]
    pub struct EgBmp<'a, C> {
        bmp: Bmp<'a>,
        color_type: PhantomData<C>,
    }

    impl<'a, C> EgBmp<'a, C>
    where
        C: PixelColor,
    {
        /// TODO: docs
        pub fn from_slice(data: &'a [u8]) -> Result<Self, ()> {
            Ok(Self {
                bmp: Bmp::from_slice(data)?,
                color_type: PhantomData,
            })
        }
    }

    impl<C> ImageDrawable<C> for EgBmp<'_, C>
    where
        C: PixelColor + From<<C as PixelColor>::Raw>,
    {
        fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
        where
            D: DrawTarget<Color = C>,
        {
            target.fill_contiguous(
                &self.bounding_box(),
                self.bmp
                    .into_iter()
                    .map(|p| C::Raw::from_u32(p.color).into()),
            )
        }
    }

    impl<C> OriginDimensions for EgBmp<'_, C>
    where
        C: PixelColor,
    {
        fn size(&self) -> Size {
            Size::new(self.bmp.width(), self.bmp.height())
        }
    }
}

#[cfg(feature = "graphics")]
pub use e_g::*;
