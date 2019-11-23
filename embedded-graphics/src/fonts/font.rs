use crate::drawable::Pixel;
use crate::geometry::Point;
use crate::pixelcolor::PixelColor;
use crate::style::TextStyle;

/// Font
pub trait Font {
    /// Raw image containing the font
    const FONT_IMAGE: &'static [u8];
    /// `char` height of the font
    const CHAR_HEIGHT: u32;

    /// `char` width of the font
    const CHAR_WIDTH: u32;
    /// Font image width, must be divisible by `8` and `CHAR_WIDTH`.
    const FONT_IMAGE_WIDTH: u32 = 240;
    /// Returns the index in the font of the correponding `char`
    fn char_offset(_: char) -> u32;
}

/// Pixel iterator for the `FontBuilder` object
#[derive(Debug, Clone, Copy)]
pub struct FontIterator<'a, 'b, C, F>
where
    C: PixelColor,
    F: Font,
{
    pub(crate) char_walk_x: u32,
    pub(crate) char_walk_y: u32,
    pub(crate) current_char: Option<char>,
    pub(crate) idx: usize,
    pub(crate) pos: Point,
    pub(crate) text: &'a str,
    pub(crate) style: TextStyle<'b, C, F>,
}

impl<C, F> Iterator for FontIterator<'_, '_, C, F>
where
    C: PixelColor,
    F: Font,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        let char_per_row = F::FONT_IMAGE_WIDTH / F::CHAR_WIDTH;

        loop {
            if let Some(current_char) = self.current_char {
                // Char _code_ offset from first char, most often a space
                // E.g. first char = ' ' (32), target char = '!' (33), offset = 33 - 32 = 1
                let char_offset = F::char_offset(current_char);
                let row = char_offset / char_per_row;

                // Top left corner of character, in pixels
                let char_x = (char_offset - (row * char_per_row)) * F::CHAR_WIDTH;
                let char_y = row * F::CHAR_HEIGHT;

                // Bit index
                // = X pixel offset for char
                // + Character row offset (row 0 = 0, row 1 = (192 * 8) = 1536)
                // + X offset for the pixel block that comprises this char
                // + Y offset for pixel block
                let bitmap_bit_index = char_x
                    + (F::FONT_IMAGE_WIDTH * char_y)
                    + self.char_walk_x
                    + (self.char_walk_y * F::FONT_IMAGE_WIDTH);

                let bitmap_byte = bitmap_bit_index / 8;
                let bitmap_bit = 7 - (bitmap_bit_index % 8);

                let color = if F::FONT_IMAGE[bitmap_byte as usize] & (1 << bitmap_bit) != 0 {
                    self.style.text_color.or(self.style.background_color)
                } else {
                    self.style.background_color
                };

                let x =
                    self.pos.x + (F::CHAR_WIDTH * self.idx as u32) as i32 + self.char_walk_x as i32;
                let y = self.pos.y + self.char_walk_y as i32;

                self.char_walk_x += 1;

                if self.char_walk_x >= F::CHAR_WIDTH {
                    self.char_walk_x = 0;
                    self.char_walk_y += 1;

                    // Done with this char, move on to the next one
                    if self.char_walk_y >= F::CHAR_HEIGHT {
                        self.char_walk_y = 0;
                        self.idx += 1;
                        self.current_char = self.text.chars().nth(self.idx);
                    }
                }

                // Skip to next point if pixel is transparent
                if let Some(color) = color {
                    break Some(Pixel(Point::new(x, y), color));
                }
            } else {
                break None;
            }
        }
    }
}
