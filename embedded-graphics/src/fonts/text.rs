use crate::{
    drawable::{Drawable, Pixel},
    fonts::Font,
    geometry::{Dimensions, Point, Size},
    pixelcolor::PixelColor,
    style::{AlignH, Styled, TextStyle},
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

    /// The box size that the text should fit in.
    ///
    /// Defines the width and height of the text object.
    pub size: Option<Size>,
}

impl<'a> Text<'a> {
    /// Creates a text.
    pub const fn new(text: &'a str, position: Point) -> Self {
        Self {
            text,
            position,
            size: None,
        }
    }

    /// Specify a box size for the text.
    pub const fn sized(mut self, size: Size) -> Self {
        self.size = Some(size);
        self
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
            first: true,
            current_char: self.primitive.text.chars().next(),
            idx: 0,
            text: self.primitive.text,
            char_width: 0,
            char_walk_x: 0,
            char_walk_y: 0,
            top_left: self.primitive.position,
            pos: self.primitive.position,
            size: self.size(),
            style: self.style,
        }
    }
}

fn line_width<F>(line: &str) -> u32
where
    F: Font,
{
    line.chars()
        .map(|c| F::char_width(c) + F::CHARACTER_SPACING)
        .sum::<u32>()
        - F::CHARACTER_SPACING
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
        let width = if self.primitive.size.is_some() {
            self.primitive.size.unwrap().width
        } else if !self.primitive.text.is_empty() {
            self.primitive
                .text
                .lines()
                .map(|l| line_width::<F>(l))
                .max()
                .unwrap_or(0)
        } else {
            0
        };

        let height = if self.primitive.size.is_some() {
            self.primitive.size.unwrap().height
        } else if width > 0 {
            F::CHARACTER_SIZE.height * self.primitive.text.lines().count() as u32
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
    first: bool,
    char_width: u32,
    char_walk_x: i32,
    char_walk_y: i32,
    current_char: Option<char>,
    idx: usize,
    top_left: Point,
    pos: Point,
    size: Size,
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
            if self.current_char == Some('\n') || self.first {
                if self.first {
                    self.first = false;
                } else {
                    self.pos.y += F::CHARACTER_SIZE.height as i32;
                    self.idx += 1;
                    self.current_char = self.text.chars().nth(self.idx);
                }
                let len = self.text[self.idx..].lines().next().map_or(0, |l| l.len());
                let width = line_width::<F>(&self.text[self.idx..self.idx + len]);
                self.pos.x = match self.style.horizontal_alignment {
                    AlignH::LEFT => self.top_left.x,
                    AlignH::CENTER => self.top_left.x + (self.size.width as i32 - width as i32) / 2,
                    AlignH::RIGHT => self.top_left.x + self.size.width as i32 - width as i32,
                }
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
                if self.char_width == 0 {
                    self.char_width = F::char_width(current_char);
                }

                let color = if F::character_pixel(
                    current_char,
                    self.char_walk_x as u32,
                    self.char_walk_y as u32,
                ) {
                    self.style.text_color.or(self.style.background_color)
                } else {
                    self.style.background_color
                };

                let x = self.pos.x + self.char_walk_x;
                let y = self.pos.y + self.char_walk_y;

                self.char_walk_x += 1;

                if self.char_walk_x >= self.char_width as i32 {
                    self.char_walk_x = 0;
                    self.char_walk_y += 1;

                    // Done with this char, move on to the next one
                    if self.char_walk_y >= F::CHARACTER_SIZE.height as i32 {
                        self.pos.x += (self.char_width + F::CHARACTER_SPACING) as i32;
                        self.char_width = 0;
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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        fonts::Font6x8, mock_display::MockDisplay, pixelcolor::BinaryColor, style::TextStyleBuilder,
    };

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
                size: None
            }
        );

        let sized_text = text.sized(Size::new(42, 8));

        assert_eq!(
            sized_text,
            Text {
                text: "Hello e-g",
                position: Point::new(10, 11),
                size: Some(Size::new(42, 8))
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

    #[test]
    fn multiline() -> Result<(), core::convert::Infallible> {
        let mut display = MockDisplay::new();

        Text::new("AB\nC", Point::zero())
            .into_styled(TextStyle::new(Font6x8, BinaryColor::On))
            .draw(&mut display)?;

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
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
            ])
        );

        assert_eq!(
            Text::new("AB\nC", Point::zero())
                .into_styled(TextStyle::new(Font6x8, BinaryColor::On))
                .size(),
            Size::new(2 * 6, 2 * 8)
        );

        Ok(())
    }

    #[test]
    fn text_horizontal_alignment() -> Result<(), core::convert::Infallible> {
        let mut display = MockDisplay::new();

        Text::new("AB", Point::zero())
            .sized(Size::new(20, 8))
            .into_styled(
                TextStyleBuilder::new(Font6x8)
                    .text_color(BinaryColor::On)
                    .horizontal_alignment(AlignH::LEFT)
                    .build(),
            )
            .draw(&mut display)?;

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                " ###  ####          ",
                "#   # #   #         ",
                "#   # #   #         ",
                "##### ####          ",
                "#   # #   #         ",
                "#   # #   #         ",
                "#   # ####          ",
                "                    ",
            ])
        );

        let mut display = MockDisplay::new();

        Text::new("AB", Point::zero())
            .sized(Size::new(20, 8))
            .into_styled(
                TextStyleBuilder::new(Font6x8)
                    .text_color(BinaryColor::On)
                    .horizontal_alignment(AlignH::CENTER)
                    .build(),
            )
            .draw(&mut display)?;

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "     ###  ####      ",
                "    #   # #   #     ",
                "    #   # #   #     ",
                "    ##### ####      ",
                "    #   # #   #     ",
                "    #   # #   #     ",
                "    #   # ####      ",
                "                    ",
            ])
        );

        let mut display = MockDisplay::new();

        Text::new("AB", Point::zero())
            .sized(Size::new(20, 8))
            .into_styled(
                TextStyleBuilder::new(Font6x8)
                    .text_color(BinaryColor::On)
                    .horizontal_alignment(AlignH::RIGHT)
                    .build(),
            )
            .draw(&mut display)?;

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "         ###  ####  ",
                "        #   # #   # ",
                "        #   # #   # ",
                "        ##### ####  ",
                "        #   # #   # ",
                "        #   # #   # ",
                "        #   # ####  ",
                "                    ",
            ])
        );

        assert_eq!(
            Text::new("AB", Point::zero())
                .sized(Size::new(20, 4))
                .into_styled(TextStyle::new(SpacedFont, BinaryColor::On))
                .size(),
            Size::new(20, 4)
        );
        assert_eq!(
            Text::new("ABCD", Point::zero())
                .sized(Size::new(20, 4))
                .into_styled(TextStyle::new(SpacedFont, BinaryColor::On))
                .size(),
            Size::new(20, 4)
        );

        Ok(())
    }
}
