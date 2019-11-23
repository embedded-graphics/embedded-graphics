use crate::drawable::{Drawable, Pixel};
use crate::fonts::{Font, FontIterator};
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
    pub fn into_styled<'b, C, F>(
        self,
        style: TextStyle<'b, C, F>,
    ) -> Styled<Self, TextStyle<'b, C, F>>
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

impl<C, F> Drawable<C> for &Styled<Text<'_>, TextStyle<'_, C, F>>
where
    C: PixelColor,
    F: Font + Copy,
{
    fn draw<T: DrawTarget<C>>(self, display: &mut T) {
        display.draw_iter(self.into_iter());
    }
}

impl<'a, 'b, C, F> IntoIterator for &Styled<Text<'a>, TextStyle<'b, C, F>>
where
    C: PixelColor,
    F: Font + Copy,
{
    type Item = Pixel<C>;
    type IntoIter = FontIterator<'a, 'b, C, F>;

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

impl<C, F> Dimensions for Styled<Text<'_>, TextStyle<'_, C, F>>
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
