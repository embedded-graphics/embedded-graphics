use crate::{
    drawable::{Drawable, Pixel},
    fonts::Font,
    geometry::{Dimensions, Point, Size},
    pixelcolor::PixelColor,
    style::{Styled, TextStyle},
    transform::Transform,
    DrawTarget,
};

/// A text object.
///
/// The `Text` struct represents a string that can be drawn onto a display.
///
/// The text object only contains the string and position and no additional styling information,
/// like the font or color. To draw a text object it is necessary to attach a style to it by using
/// the [`into_styled`] method to create a [`Styled`] object.
///
/// See the [module-level documentation] for examples how to use text objects.
///
/// [`into_styled`]: #method.into_styled
/// [`Styled`]: ../style/struct.Styled.html
/// [module-level documentation]: index.html
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Text<'a> {
    /// The string.
    pub text: &'a str,

    /// The position.
    ///
    /// Defines the top-left starting pixel of the text object.
    pub position: Point,
}

impl<'a> Text<'a> {
    /// Creates a text.
    pub const fn new(text: &'a str, position: Point) -> Self {
        Self { text, position }
    }

    /// Attaches a text style to the text object.
    pub fn into_styled<C, F>(self, style: TextStyle<C, F>) -> Styled<Self, TextStyle<C, F>>
    where
        C: PixelColor,
        F: Font,
    {
        Styled::new(self, style)
    }
}

impl Transform for Text<'_> {
    fn translate(&self, by: Point) -> Self {
        Self {
            position: self.position + by,
            ..*self
        }
    }

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
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        display.draw_iter(self.into_iter())
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

    /// Returns the size of the bounding box of a styled text.
    ///
    /// Currently does not handle newlines (but neither does the rasteriser).
    /// It will return [`Size::zero()`] if the string to render is empty.
    ///
    /// [`Size::zero()`]: ../geometry/struct.Size.html#method.zero
    fn size(&self) -> Size {
        let width = if !self.primitive.text.is_empty() {
            (F::CHARACTER_SIZE.width + F::CHARACTER_SPACING) * self.primitive.text.len() as u32
                - F::CHARACTER_SPACING
        } else {
            0
        };

        // TODO: Handle height of text with newlines in it
        let height = if width > 0 {
            F::CHARACTER_SIZE.height
        } else {
            0
        };

        Size::new(width, height)
    }
}

/// Pixel iterator for styled text.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
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
        loop {
            if let Some(current_char) = self.current_char {
                let color = if F::character_pixel(current_char, self.char_walk_x, self.char_walk_y)
                {
                    self.style.text_color.or(self.style.background_color)
                } else {
                    self.style.background_color
                };

                let x = self.pos.x
                    + ((F::CHARACTER_SIZE.width + F::CHARACTER_SPACING) * self.idx as u32) as i32
                    + self.char_walk_x as i32;
                let y = self.pos.y + self.char_walk_y as i32;

                self.char_walk_x += 1;

                if self.char_walk_x >= F::CHARACTER_SIZE.width {
                    self.char_walk_x = 0;
                    self.char_walk_y += 1;

                    // Done with this char, move on to the next one
                    if self.char_walk_y >= F::CHARACTER_SIZE.height {
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
    use crate::{mock_display::MockDisplay, pixelcolor::BinaryColor};

    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
    struct SpacedFont;

    impl Font for SpacedFont {
        const FONT_IMAGE: &'static [u8] = &[0xF0, 0xA0, 0x50, 0x10];
        const FONT_IMAGE_WIDTH: u32 = 8;
        const CHARACTER_SIZE: Size = Size::new(4, 4);
        const CHARACTER_SPACING: u32 = 5;

        fn char_offset(_c: char) -> u32 {
            0
        }
    }

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

    #[test]
    fn character_spacing() -> Result<(), core::convert::Infallible> {
        let mut display = MockDisplay::new();

        Text::new("##", Point::zero())
            .into_styled(TextStyle::new(SpacedFont, BinaryColor::On))
            .draw(&mut display)?;

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "####     ####",
                "# #      # # ",
                " # #      # #",
                "   #        #",
            ])
        );

        assert_eq!(
            Text::new("#", Point::zero())
                .into_styled(TextStyle::new(SpacedFont, BinaryColor::On))
                .size(),
            Size::new(4, 4)
        );
        assert_eq!(
            Text::new("##", Point::zero())
                .into_styled(TextStyle::new(SpacedFont, BinaryColor::On))
                .size(),
            Size::new(4 * 2 + 5, 4)
        );
        assert_eq!(
            Text::new("###", Point::zero())
                .into_styled(TextStyle::new(SpacedFont, BinaryColor::On))
                .size(),
            Size::new(4 * 3 + 5 * 2, 4)
        );

        Ok(())
    }
}
