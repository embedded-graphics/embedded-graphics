use embedded_graphics_core::{pixelcolor::BinaryColor, primitives::Rectangle};

use crate::{
    fonts::MonoFont,
    geometry::Point,
    primitives::{rectangle, PointsIter},
    Pixel,
};
use core::marker::PhantomData;

/// Pixel iterator for styled text with a monospaced font.
#[derive(Debug)]
pub struct MonoCharPixels<F>
where
    F: MonoFont,
{
    points: rectangle::Points,

    char_px_offset: u32,
    byte_index: usize,
    bit_mask: u8,

    font: PhantomData<F>,
}

impl<F> MonoCharPixels<F>
where
    F: MonoFont,
{
    pub(crate) fn new(c: char) -> Self {
        let char_per_row = F::FONT_IMAGE_WIDTH / F::CHARACTER_SIZE.width;

        // Char _code_ offset from first char, most often a space
        // E.g. first char = ' ' (32), target char = '!' (33), offset = 33 - 32 = 1
        let char_offset = F::char_offset(c);
        let row = char_offset / char_per_row;

        // Top left corner of character, in pixels
        let char_x = (char_offset - (row * char_per_row)) * F::CHARACTER_SIZE.width;
        let char_y = row * F::CHARACTER_SIZE.height;

        Self {
            points: Rectangle::new(Point::zero(), F::CHARACTER_SIZE).points(),
            char_px_offset: char_x + char_y * F::FONT_IMAGE_WIDTH,
            byte_index: 0,
            bit_mask: 0,
            font: PhantomData,
        }
    }

    fn start_row(&mut self, y: i32) {
        // Bit index
        // = X pixel offset for char
        // + Character row offset (row 0 = 0, row 1 = (192 * 8) = 1536)
        // + X offset for the pixel block that comprises this char
        // + Y offset for pixel block
        let index = self.char_px_offset + y as u32 * F::FONT_IMAGE_WIDTH;

        self.byte_index = (index / 8) as usize;
        self.bit_mask = 0x80 >> (index % 8);
    }
}

impl<F> Iterator for MonoCharPixels<F>
where
    F: MonoFont,
{
    type Item = Pixel<BinaryColor>;

    fn next(&mut self) -> Option<Self::Item> {
        self.points.next().map(|point| {
            if point.x == 0 {
                self.start_row(point.y);
            }

            let color = BinaryColor::from(F::FONT_IMAGE[self.byte_index] & self.bit_mask != 0);

            if self.bit_mask != 0x01 {
                self.bit_mask >>= 1;
            } else {
                self.bit_mask = 0x80;
                self.byte_index += 1;
            }

            Pixel(point, color)
        })
    }
}
