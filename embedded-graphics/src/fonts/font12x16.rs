//! 12x16 pixel font. Image data taken from the [Uzebox Wiki page](http://uzebox.org/wiki/Font_Bitmaps)

use super::super::drawable::*;
use super::super::transform::*;
use super::Font;

const FONT_IMAGE: &[u8] = include_bytes!("../../data/font12x16_1bpp.raw");
const CHAR_HEIGHT: u32 = 16;
const CHAR_WIDTH: u32 = 12;
const FIRST_CHARCODE: u32 = 32; // A space
const FONT_IMAGE_WIDTH: u32 = 480;
const CHARS_PER_ROW: u32 = FONT_IMAGE_WIDTH / CHAR_WIDTH;

/// Container struct to hold a positioned piece of text
#[derive(Debug, Clone, Copy)]
pub struct Font12x16<'a> {
    /// Top left corner of the text
    pub pos: Coord,

    /// Text to draw
    text: &'a str,
}

impl<'a> Font<'a> for Font12x16<'a> {
    fn render_str(text: &'a str) -> Font12x16<'a> {
        Self { pos: (0, 0), text }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Font12x16Iterator<'a> {
    char_walk_x: u32,
    char_walk_y: u32,
    current_char: Option<char>,
    idx: usize,
    pos: Coord,
    text: &'a str,
}

impl<'a> IntoIterator for &'a Font12x16<'a> {
    type IntoIter = Font12x16Iterator<'a>;
    type Item = Pixel;

    fn into_iter(self) -> Self::IntoIter {
        Font12x16Iterator {
            current_char: self.text.chars().next(),
            idx: 0,
            text: self.text,
            char_walk_x: 0,
            char_walk_y: 0,
            pos: self.pos,
        }
    }
}

impl<'a> Iterator for Font12x16Iterator<'a> {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current_char) = self.current_char {
            // Char _code_ offset from first char, most often a space
            // E.g. first char = ' ' (32), target char = '!' (33), offset = 33 - 32 = 1
            let char_offset = current_char as u32 - FIRST_CHARCODE;
            let row = char_offset / CHARS_PER_ROW;

            // Top left corner of character, in pixels
            let char_x = (char_offset - (row * CHARS_PER_ROW)) * CHAR_WIDTH;
            let char_y = row * CHAR_HEIGHT;

            // Bit index
            // = X pixel offset for char
            // + Character row offset (row 0 = 0, row 1 = (192 * 8) = 1536)
            // + X offset for the pixel block that comprises this char
            // + Y offset for pixel block
            let bitmap_bit_index = char_x + (FONT_IMAGE_WIDTH * char_y) + self.char_walk_x
                + (self.char_walk_y * FONT_IMAGE_WIDTH);

            let bitmap_byte = bitmap_bit_index / 8;
            let bitmap_bit = 7 - (bitmap_bit_index % 8);

            let bit_value = (FONT_IMAGE[bitmap_byte as usize] >> bitmap_bit) & 1;

            self.char_walk_x += 1;

            if self.char_walk_x >= CHAR_WIDTH {
                self.char_walk_x = 0;
                self.char_walk_y += 1;

                // Done with this char, move on to the next one
                if self.char_walk_y >= CHAR_HEIGHT {
                    self.char_walk_y = 0;
                    self.idx += 1;
                    self.current_char = self.text.chars().skip(self.idx).next();
                }
            }

            let x = self.pos.0 + (CHAR_WIDTH * self.idx as u32) + self.char_walk_x;
            let y = self.pos.1 + self.char_walk_y;

            Some(((x, y), bit_value))
        } else {
            None
        }
    }
}

impl<'a> Drawable for Font12x16<'a> {}

impl<'a> Transform for Font12x16<'a> {
    /// Translate the image from its current position to a new position by (x, y) pixels, returning
    /// a new `Font12x16`.
    ///
    /// ```
    /// # use embedded_graphics::fonts::{ Font, Font12x16 };
    /// # use embedded_graphics::transform::Transform;
    ///
    /// // 8px x 1px test image
    /// let text = Font12x16::render_str("Hello world");
    /// let moved = text.translate((25, 30));
    ///
    /// assert_eq!(text.pos, (0, 0));
    /// assert_eq!(moved.pos, (25, 30));
    /// ```
    fn translate(&self, by: Coord) -> Self {
        Self {
            pos: (self.pos.0 + by.0, self.pos.1 + by.1),
            ..*self
        }
    }
}
