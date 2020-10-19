use crate::Bmp;
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

    /// Number of bits per pixel
    pixel_stride: usize,

    /// Current position
    position: Point,

    /// Start bit index for the current pixel.
    ///
    /// This is incremented by `pixel_stride` bits every iteration.
    bit_idx: usize,
}

impl<'a, 'b, C> RawPixels<'a, 'b, C> {
    pub(crate) fn new(bmp: &'a Bmp<'b, C>) -> Self {
        let pixel_stride = match bmp.color_bpp() {
            1 | 8 | 16 | 24 | 32 => bmp.color_bpp() as usize,
            depth => panic!("Bit depth {} not supported", depth),
        };

        Self {
            bmp,
            pixel_data: bmp.image_data,
            pixel_stride,
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
