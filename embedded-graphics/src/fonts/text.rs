use crate::drawable::{Drawable, Pixel};
use crate::fonts::Font;
use crate::geometry::{Point, Size};
use crate::pixelcolor::PixelColor;
use crate::style::{Styled, TextStyle};
use crate::transform::Transform;
use crate::{Dimensions, DrawTarget};

/// Text
#[derive(Debug, PartialEq, Eq)]
pub struct Text<'a> {
    /// Text
    pub text: &'a str,
    /// Position
    pub position: Point,
}

impl<'a> Text<'a> {
    /// Creates a text.
    pub const fn new(text: &'a str, position: Point) -> Self {
        Self { text, position }
    }

    /// Converts this text into a styled.
    pub fn into_styled<C, F>(self, style: TextStyle<C, F>) -> Styled<Self, TextStyle<C, F>>
    where
        C: PixelColor,
        F: Font,
    {
        Styled::new(self, style)
    }
}

impl Transform for Text<'_> {
    /// Translate.
    fn translate(&self, by: Point) -> Self {
        Self {
            position: self.position + by,
            ..*self
        }
    }

    /// Translate_mut.
    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.position += by;

        self
    }
}

impl<C, F> Drawable<C> for &Styled<Text<'_>, TextStyle<C, F>>
where
    C: PixelColor,
    F: Font + Copy,
{
    fn draw<T: DrawTarget<C>>(self, display: &mut T) {
        display.draw_iter(self.into_iter());
    }
}

impl<'a, C, F> IntoIterator for &Styled<Text<'a>, TextStyle<C, F>>
where
    C: PixelColor,
    F: Font + Copy,
{
    type Item = Pixel<C>;
    type IntoIter = StyledTextIterator<'a, C, F>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            current_char: self.primitive.text.chars().next(),
            idx: 0,
            text: self.primitive.text,
            char_walk_x: 0,
            char_walk_y: 0,
            pos: self.primitive.position,
            style: self.style,
        }
    }
}

impl<C, F> Dimensions for Styled<Text<'_>, TextStyle<C, F>>
where
    C: PixelColor,
    F: Font,
{
    fn top_left(&self) -> Point {
        self.primitive.position
    }

    fn bottom_right(&self) -> Point {
        self.top_left() + self.size()
    }

    /// Get the bounding box of a piece of text
    ///
    /// Currently does not handle newlines (but neither does the rasteriser). It will give `(0, 0)`
    /// if the string to render is empty.
    fn size(&self) -> Size {
        // TODO: Handle height of text with newlines in it
        let width = F::CHAR_WIDTH * self.primitive.text.len() as u32;
        let height = if width > 0 { F::CHAR_HEIGHT } else { 0 };

        Size::new(width, height)
    }
}

/// Pixel iterator for the `FontBuilder` object
#[derive(Debug, Clone, Copy)]
pub struct StyledTextIterator<'a, C, F>
where
    C: PixelColor,
    F: Font,
{
    char_walk_x: u32,
    char_walk_y: u32,
    current_char: Option<char>,
    idx: usize,
    pos: Point,
    text: &'a str,
    style: TextStyle<C, F>,
}

impl<C, F> Iterator for StyledTextIterator<'_, C, F>
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor() {
        let text = Text::new("Hello e-g", Point::new(10, 11));

        assert_eq!(
            text,
            Text {
                text: "Hello e-g",
                position: Point::new(10, 11),
            }
        );
    }
}
