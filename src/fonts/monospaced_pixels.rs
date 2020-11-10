use crate::{
    fonts::{MonospacedFont, Text},
    geometry::Point,
    pixelcolor::PixelColor,
    style::MonospacedTextStyle,
    Pixel,
};

/// Pixel iterator for styled text with a monospaced font.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct MonospacedPixels<'a, C, F>
where
    C: PixelColor,
    F: MonospacedFont,
{
    char_walk_x: i32,
    char_walk_y: i32,
    current_char: Option<char>,
    idx: usize,
    top_left: Point,
    pos: Point,
    text: &'a str,
    style: MonospacedTextStyle<C, F>,
}

impl<'a, C, F> MonospacedPixels<'a, C, F>
where
    C: PixelColor,
    F: MonospacedFont,
{
    pub(crate) fn new(text: &Text<'a>, style: MonospacedTextStyle<C, F>) -> Self {
        Self {
            current_char: text.text.chars().next(),
            idx: 0,
            text: text.text,
            char_walk_x: 0,
            char_walk_y: 0,
            top_left: text.position,
            pos: text.position,
            style,
        }
    }
}

impl<C, F> Iterator for MonospacedPixels<'_, C, F>
where
    C: PixelColor,
    F: MonospacedFont,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current_char == Some('\n') {
                self.pos.x = self.top_left.x;
                self.pos.y += F::CHARACTER_SIZE.height as i32;
                self.idx += 1;
                self.current_char = self.text.chars().nth(self.idx);
            } else if self.char_walk_x < 0 {
                let x = self.pos.x + self.char_walk_x;
                let y = self.pos.y + self.char_walk_y;

                self.char_walk_y += 1;

                if self.char_walk_y >= F::CHARACTER_SIZE.height as i32 {
                    self.char_walk_y = 0;
                    self.char_walk_x += 1;
                }

                if let Some(color) = self.style.background_color {
                    break Some(Pixel(Point::new(x, y), color));
                }
            } else if let Some(current_char) = self.current_char {
                let color = if F::character_pixel(
                    current_char,
                    self.char_walk_x as u32,
                    self.char_walk_y as u32,
                ) {
                    self.style.text_color
                } else {
                    self.style.background_color
                };

                let x = self.pos.x + self.char_walk_x;
                let y = self.pos.y + self.char_walk_y;

                self.char_walk_x += 1;

                if self.char_walk_x >= F::CHARACTER_SIZE.width as i32 {
                    self.char_walk_x = 0;
                    self.char_walk_y += 1;

                    // Done with this char, move on to the next one
                    if self.char_walk_y >= F::CHARACTER_SIZE.height as i32 {
                        self.pos.x += (F::CHARACTER_SIZE.width + F::CHARACTER_SPACING) as i32;
                        self.char_walk_y = 0;
                        self.char_walk_x -= F::CHARACTER_SPACING as i32;
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
