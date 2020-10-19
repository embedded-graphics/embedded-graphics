use crate::{Bmp, Bpp};
use embedded_graphics::prelude::*;

/// Iterator over individual BMP pixels
///
/// Each pixel is returned as a `u32` regardless of the bit depth of the source image.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct RawPixels<'a, 'b, C> {
    /// Reference to original BMP image
    bmp: &'a Bmp<'b, C>,

    /// Image pixel data as a byte slice, little endian ordering
    pixel_data: &'b [u8],

    /// Current position
    position: Point,

    /// Start bit index for the current pixel.
    ///
    /// This is incremented by `pixel_stride` bits every iteration.
    bit_idx: usize,
}

impl<'a, 'b, C> RawPixels<'a, 'b, C> {
    pub(crate) fn new(bmp: &'a Bmp<'b, C>) -> Self {
        Self {
            bmp,
            pixel_data: bmp.image_data,
            position: Point::zero(),
            bit_idx: 0,
        }
    }
}

impl<C> Iterator for RawPixels<'_, '_, C> {
    type Item = RawPixel;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position.y >= self.bmp.header.image_height as i32 {
            return None;
        }

        let p = self.position;

        if self.position.x == 0 {
            let row_index = (self.bmp.header.image_height as i32 - 1) - self.position.y;
            let row_start = self.bmp.bytes_per_row() * row_index as usize;

            self.bit_idx = row_start * 8;
        }

        self.position.x += 1;
        if self.position.x >= self.bmp.header.image_width as i32 {
            self.position.y += 1;
            self.position.x = 0;
        }

        let byte_idx = self.bit_idx / 8;

        let px = self.pixel_data;
        let pixel_value = match self.bmp.color_bpp() {
            Bpp::Bits1 => {
                let mask = 0b_1000_0000 >> self.bit_idx % 8;
                (px[byte_idx] & mask != 0) as u32
            }
            Bpp::Bits8 => u32::from(px[byte_idx]),
            Bpp::Bits16 => u32::from_le_bytes([px[byte_idx], px[byte_idx + 1], 0, 0]),
            Bpp::Bits24 => {
                u32::from_le_bytes([px[byte_idx], px[byte_idx + 1], px[byte_idx + 2], 0])
            }
            Bpp::Bits32 => u32::from_le_bytes([
                px[byte_idx],
                px[byte_idx + 1],
                px[byte_idx + 2],
                px[byte_idx + 3],
            ]),
        };

        self.bit_idx += usize::from(self.bmp.color_bpp().bits());

        Some(RawPixel::new(p, pixel_value))
    }
}

/// Pixel with raw pixel color.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct RawPixel {
    /// The position relative to the top left corner of the image.
    pub position: Point,

    /// The raw pixel color.
    pub color: u32,
}

impl RawPixel {
    /// Creates a new raw pixel.
    pub fn new(position: Point, color: u32) -> Self {
        Self { position, color }
    }
}
