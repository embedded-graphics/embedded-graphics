//! Small BMP format image parser supporting no-std environments. Specifically designed to work with
//! [embedded-graphics]
//!
//! # Examples
//!
//! ## Load a BMP image and check its [`Header`] and returned pixels.
//!
//! ```rust
//! use tinybmp::{Bmp, FileType, Header};
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
//! let pixels: Vec<((u32, u32), u32)> = bmp.into_iter().collect();
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
//! # #[cfg(feature = "graphics")] fn main() -> Result<(), core::convert::Infallible> {
//! use tinybmp::Bmp;
//! use embedded_graphics::prelude::*;
//! # use embedded_graphics::mock_display::MockDisplay;
//! # use embedded_graphics::pixelcolor::Rgb565;
//! # let mut display: MockDisplay<Rgb565> = MockDisplay::default();
//!
//! // Load 16BPP 8x8px image
//! let mut image = Bmp::from_slice(include_bytes!("../tests/chessboard-8px-color-16bit.bmp")).unwrap();
//!
//! image.draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(())
//! # }
//! ```
//!
//! [embedded-graphics]: https://crates.io/crates/embedded-graphics
//! [`Header`]: ./header/struct.Header.html

#![no_std]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

mod check_readme;
mod header;

use crate::header::parse_header;
pub use crate::header::{FileType, Header};

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
    // TODO: Should this return an enum?
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
    type Item = ((u32, u32), u32);
    type IntoIter = BmpIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let pixel_stride = match self.bpp() {
            8 => 1,
            16 => 2,
            24 => 3,
            32 => 4,
            depth => panic!("Bit depth {} not supported", depth),
        };

        BmpIterator {
            bmp: self,
            pixel_data: self.image_data(),
            pixel_stride,
            x: 0,
            y: 0,
            start_idx: 0,
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

    /// Number of bytes per pixel
    pixel_stride: usize,

    /// Current X position
    x: u32,

    /// Current Y position
    y: u32,

    /// Start byte index for the current pixel.
    ///
    /// This is incremented by `pixel_stride` bytes every iteration.
    start_idx: usize,
}

impl<'a> Iterator for BmpIterator<'a> {
    type Item = ((u32, u32), u32);

    fn next(&mut self) -> Option<Self::Item> {
        let px = self.pixel_data;

        if self.y < self.bmp.height() {
            if self.x == 0 {
                let row_index = (self.bmp.height() - 1) - self.y;
                let row_start = self.bmp.bytes_per_row() * row_index as usize;

                self.start_idx = row_start;
            }

            self.x += 1;
            if self.x >= self.bmp.width() {
                self.y += 1;
                self.x = 0;
            }

            // Shorter alias for easier to read code
            let start = self.start_idx;

            let pixel_value = match self.pixel_stride {
                1 => u32::from(px[start]),
                2 => u32::from_le_bytes([px[start], px[start + 1], 0, 0]),
                3 => u32::from_le_bytes([px[start], px[start + 1], px[start + 2], 0]),
                4 => u32::from_le_bytes([px[start], px[start + 1], px[start + 2], px[start + 3]]),
                _ => unreachable!(),
            };

            self.start_idx += self.pixel_stride;

            Some(((self.x, self.y), pixel_value))
        } else {
            None
        }
    }
}

#[cfg(feature = "graphics")]
use embedded_graphics::{
    drawable::{Drawable, Pixel},
    pixelcolor::{raw::RawData, PixelColor},
    DrawTarget,
};

#[cfg(feature = "graphics")]
impl<'a, C> Drawable<C> for Bmp<'a>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        display.draw_iter(self.into_iter().map(|((x, y), color)| {
            let raw = C::Raw::from_u32(color);

            Pixel((x as i32, y as i32).into(), raw.into())
        }))
    }
}
