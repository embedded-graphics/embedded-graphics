use crate::{
    drawable::{Drawable, Pixel},
    fonts::Font,
    geometry::{Dimensions, Point, Size},
    pixelcolor::PixelColor,
    primitives::Rectangle,
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

impl<C, F> Drawable<C> for Styled<Text<'_>, TextStyle<C, F>>
where
    C: PixelColor,
    F: Font + Copy,
{
    fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
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
            idx: 0,
            text: self.primitive.text,
            top_left: self.primitive.position,
            pos: self.primitive.position,
            style: self.style,
            state: IteratorState::NextChar,
        }
    }
}

impl<C, F> Dimensions for Styled<Text<'_>, TextStyle<C, F>>
where
    C: PixelColor,
    F: Font,
{
    fn bounding_box(&self) -> Rectangle {
        let width = if !self.primitive.text.is_empty() {
            self.primitive
                .text
                .lines()
                .map(|l| {
                    l.chars()
                        .map(|c| F::char_width(c) + F::CHARACTER_SPACING)
                        .sum::<u32>()
                        - F::CHARACTER_SPACING
                })
                .max()
                .unwrap_or(0)
        } else {
            0
        };

        let height = if width > 0 {
            F::CHARACTER_SIZE.height * self.primitive.text.lines().count() as u32
        } else {
            0
        };

        let size = Size::new(width, height);

        Rectangle::new(self.primitive.position, size)
    }
}

/// Pixel iterator to render a styled character
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct StyledCharacterIterator<C, F>
where
    C: PixelColor,
    F: Font,
{
    character: char,
    style: TextStyle<C, F>,
    pos: Point,
    char_walk: Point,
    max_x: i32,
}

impl<C, F> StyledCharacterIterator<C, F>
where
    C: PixelColor,
    F: Font,
{
    pub fn new(character: char, pos: Point, style: TextStyle<C, F>) -> Self {
        Self {
            character,
            style,
            pos,
            char_walk: Point::zero(),
            max_x: F::char_width(character) as i32 - 1,
        }
    }
}

impl<C, F> Iterator for StyledCharacterIterator<C, F>
where
    C: PixelColor,
    F: Font,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.char_walk.y >= F::CHARACTER_SIZE.height as i32 {
                // Done with this char, move on to the next one
                break None;
            } else {
                let color = if F::character_pixel(
                    self.character,
                    self.char_walk.x as u32,
                    self.char_walk.y as u32,
                ) {
                    self.style.text_color.or(self.style.background_color)
                } else {
                    self.style.background_color
                };

                let p = self.pos + self.char_walk;

                if self.char_walk.x < self.max_x {
                    self.char_walk.x += 1;
                } else {
                    self.char_walk.x = 0;
                    self.char_walk.y += 1;
                }

                // Skip to next point if pixel is transparent
                if let Some(color) = color {
                    break Some(Pixel(p, color));
                }
            }
        }
    }
}

/// Pixel iterator to render font spacing
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct EmptySpaceIterator<C, F>
where
    C: PixelColor,
    F: Font,
{
    style: TextStyle<C, F>,
    pos: Point,
    char_walk: Point,
}

impl<C, F> EmptySpaceIterator<C, F>
where
    C: PixelColor,
    F: Font,
{
    pub fn new(pos: Point, style: TextStyle<C, F>) -> Self {
        Self {
            style,
            pos,
            char_walk: Point::zero(),
        }
    }
}

impl<C, F> Iterator for EmptySpaceIterator<C, F>
where
    C: PixelColor,
    F: Font,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        if F::CHARACTER_SPACING == 0 {
            None
        } else if let Some(color) = self.style.background_color {
            if self.char_walk.y >= F::CHARACTER_SIZE.height as i32 {
                // Done with filling this space
                None
            } else {
                let p = self.pos + self.char_walk;

                if self.char_walk.x < F::CHARACTER_SPACING as i32 - 1 {
                    self.char_walk.x += 1;
                } else {
                    self.char_walk.x = 0;
                    self.char_walk.y += 1;
                }

                // Skip to next point if pixel is transparent
                Some(Pixel(p, color))
            }
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum IteratorState<C, F>
where
    C: PixelColor,
    F: Font,
{
    NextChar,
    DrawCharacter(StyledCharacterIterator<C, F>),
    DrawSpace(EmptySpaceIterator<C, F>),
}

impl<C, F> Default for IteratorState<C, F>
where
    C: PixelColor,
    F: Font,
{
    fn default() -> Self {
        Self::NextChar
    }
}

/// Pixel iterator for styled text.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct StyledTextIterator<'a, C, F>
where
    C: PixelColor,
    F: Font,
{
    top_left: Point,
    pos: Point,
    idx: usize,
    text: &'a str,
    style: TextStyle<C, F>,
    state: IteratorState<C, F>,
}

impl<C, F> Iterator for StyledTextIterator<'_, C, F>
where
    C: PixelColor,
    F: Font,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.state {
                IteratorState::NextChar => {
                    if let Some(c) = self.text.chars().nth(self.idx) {
                        self.idx += 1;
                        match c {
                            '\n' => {
                                self.pos.x = self.top_left.x;
                                self.pos.y += F::CHARACTER_SIZE.height as i32;
                            }
                            c => {
                                let char_pos = self.pos;

                                self.pos.x += F::char_width(c) as i32;

                                self.state = IteratorState::DrawCharacter(
                                    StyledCharacterIterator::new(c, char_pos, self.style),
                                );
                            }
                        };
                    } else {
                        break None;
                    }
                }

                IteratorState::DrawSpace(ref mut iterator) => {
                    let pixel = iterator.next();
                    if pixel.is_some() {
                        break pixel;
                    }
                    self.state = IteratorState::NextChar;
                }

                IteratorState::DrawCharacter(ref mut iterator) => {
                    let pixel = iterator.next();
                    if pixel.is_some() {
                        break pixel;
                    }
                    let pos = self.pos;
                    self.pos.x += F::CHARACTER_SPACING as i32;
                    self.state = IteratorState::DrawSpace(EmptySpaceIterator::new(pos, self.style));
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        fonts::{tests::assert_text_from_pattern, Font6x12, Font6x8},
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
    };

    const HELLO_WORLD: &'static str = "Hello World!";

    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
    struct SpacedFont;

    impl Font for SpacedFont {
        const FONT_IMAGE: &'static [u8] = Font6x8::FONT_IMAGE;
        const FONT_IMAGE_WIDTH: u32 = Font6x8::FONT_IMAGE_WIDTH;
        const CHARACTER_SIZE: Size = Font6x8::CHARACTER_SIZE;
        const CHARACTER_SPACING: u32 = 5;

        fn char_offset(c: char) -> u32 {
            Font6x8::char_offset(c)
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
    fn character_spacing() {
        assert_text_from_pattern(
            "##",
            SpacedFont,
            &[
                " # #        # #  ",
                " # #        # #  ",
                "#####      ##### ",
                " # #        # #  ",
                "#####      ##### ",
                " # #        # #  ",
                " # #        # #  ",
                "                 ",
            ],
        );
    }

    #[test]
    fn character_spacing_dimensions() {
        let style = TextStyle::new(SpacedFont, BinaryColor::On);

        assert_eq!(
            Text::new("#", Point::zero())
                .into_styled(style)
                .bounding_box(),
            Rectangle::new(Point::zero(), Size::new(6, 8)),
        );

        assert_eq!(
            Text::new("##", Point::zero())
                .into_styled(style)
                .bounding_box(),
            Rectangle::new(Point::zero(), Size::new(6 * 2 + 5, 8)),
        );
        assert_eq!(
            Text::new("###", Point::zero())
                .into_styled(style)
                .bounding_box(),
            Rectangle::new(Point::zero(), Size::new(6 * 3 + 5 * 2, 8)),
        );
    }

    #[test]
    fn multiline() {
        assert_text_from_pattern(
            "AB\nC",
            Font6x8,
            &[
                " ###  ####  ",
                "#   # #   # ",
                "#   # #   # ",
                "##### ####  ",
                "#   # #   # ",
                "#   # #   # ",
                "#   # ####  ",
                "            ",
                " ###        ",
                "#   #       ",
                "#           ",
                "#           ",
                "#           ",
                "#   #       ",
                " ###        ",
                "            ",
            ],
        );
    }

    #[test]
    fn multiline_dimensions() {
        let style = TextStyle::new(Font6x8, BinaryColor::On);
        let text = Text::new("AB\nC", Point::zero()).into_styled(style);

        assert_eq!(
            text.bounding_box(),
            Rectangle::new(Point::zero(), Size::new(2 * 6, 2 * 8))
        );
    }

    #[test]
    fn position_and_translate() {
        let style = TextStyle::new(Font6x8, BinaryColor::On);

        let hello = Text::new(HELLO_WORLD, Point::zero()).into_styled(style);

        let hello_translated = hello.translate(Point::new(5, -20));
        assert_eq!(
            hello.bounding_box().size,
            hello_translated.bounding_box().size
        );

        let mut hello_with_point = Text::new(HELLO_WORLD, Point::new(5, -20)).into_styled(style);
        assert_eq!(hello_translated, hello_with_point);

        hello_with_point.translate_mut(Point::new(-5, 20));
        assert_eq!(hello, hello_with_point);
    }

    #[test]
    fn inverted_text() {
        let mut display_inverse = MockDisplay::new();
        let style_inverse = TextStyle {
            font: Font6x8,
            text_color: Some(BinaryColor::Off),
            background_color: Some(BinaryColor::On),
        };
        Text::new("Mm", Point::zero())
            .into_styled(style_inverse)
            .draw(&mut display_inverse)
            .unwrap();

        let mut display_normal = MockDisplay::new();
        let style_normal = TextStyle {
            font: Font6x8,
            text_color: Some(BinaryColor::On),
            background_color: Some(BinaryColor::Off),
        };
        Text::new("Mm", Point::zero())
            .into_styled(style_normal)
            .draw(&mut display_normal)
            .unwrap();

        assert_eq!(display_inverse, display_normal.map(|c| c.invert()));
    }

    #[test]
    fn no_fill_does_not_hang() {
        let mut display = MockDisplay::new();
        Text::new(" ", Point::zero())
            .into_styled(TextStyle::new(Font6x8, BinaryColor::On))
            .draw(&mut display)
            .unwrap();

        assert_eq!(display, MockDisplay::new());
    }

    #[test]
    fn negative_y_no_infinite_loop() {
        let style = TextStyle {
            font: Font6x12,
            text_color: Some(BinaryColor::On),
            background_color: Some(BinaryColor::Off),
        };

        let mut text = Text::new("Testing string", Point::zero()).into_styled(style);
        text.translate_mut(Point::new(0, -12));

        assert_eq!(text.into_iter().count(), 6 * 12 * "Testing string".len());
    }

    #[test]
    fn negative_x_no_infinite_loop() {
        let style = TextStyle {
            font: Font6x12,
            text_color: Some(BinaryColor::On),
            background_color: Some(BinaryColor::Off),
        };

        let mut text = Text::new("A", Point::zero()).into_styled(style);
        text.translate_mut(Point::new(-6, 0));

        assert_eq!(text.into_iter().count(), 6 * 12);
    }
}
