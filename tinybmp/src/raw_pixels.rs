use embedded_graphics::prelude::*;

use crate::{header::Bpp, raw_bmp::RawBmp};

/// Iterator over individual BMP pixels.
///
/// Each pixel is returned as a `u32` regardless of the bit depth of the source image.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct RawPixels<'a, 'b> {
    /// Reference to original BMP image.
    raw_bmp: &'a RawBmp<'b>,

    /// Image pixel data as a byte slice, little endian ordering.
    pixel_data: &'b [u8],

    /// Current position.
    position: Point,

    /// Start bit index for the current pixel.
    ///
    /// This is incremented by `pixel_stride` bits every iteration.
    bit_idx: usize,
}

impl<'a, 'b> RawPixels<'a, 'b> {
    pub(crate) fn new(raw_bmp: &'a RawBmp<'b>) -> Self {
        Self {
            raw_bmp,
            pixel_data: raw_bmp.image_data(),
            position: Point::zero(),
            bit_idx: 0,
        }
    }
}

impl Iterator for RawPixels<'_, '_> {
    type Item = RawPixel;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position.y >= self.raw_bmp.size().height as i32 {
            return None;
        }

        let p = self.position;

        if self.position.x == 0 {
            let row_index = (self.raw_bmp.size().height as i32 - 1) - self.position.y;
            let row_start = self.raw_bmp.bytes_per_row() * row_index as usize;

            self.bit_idx = row_start * 8;
        }

        self.position.x += 1;
        if self.position.x >= self.raw_bmp.size().width as i32 {
            self.position.y += 1;
            self.position.x = 0;
        }

        let byte_idx = self.bit_idx / 8;

        let mut pixel_value = [0u8; 4];

        match self.raw_bmp.color_bpp() {
            Bpp::Bits1 => self.pixel_data.get(byte_idx).map(|byte| {
                let mask = 0b_1000_0000 >> self.bit_idx % 8;
                pixel_value[0] = (byte & mask != 0) as u8;
            }),
            Bpp::Bits8 => self
                .pixel_data
                .get(byte_idx)
                .map(|byte| pixel_value[0] = *byte),
            Bpp::Bits16 => self.pixel_data.get(byte_idx..byte_idx + 2).map(|data| {
                pixel_value[0..2].copy_from_slice(data);
            }),
            Bpp::Bits24 => self.pixel_data.get(byte_idx..byte_idx + 3).map(|data| {
                pixel_value[0..3].copy_from_slice(data);
            }),
            Bpp::Bits32 => self.pixel_data.get(byte_idx..byte_idx + 4).map(|data| {
                pixel_value[0..4].copy_from_slice(data);
            }),
        };

        self.bit_idx += usize::from(self.raw_bmp.color_bpp().bits());

        Some(RawPixel::new(p, u32::from_le_bytes(pixel_value)))
    }
}

/// Pixel with raw pixel color stored as a `u32`.
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
