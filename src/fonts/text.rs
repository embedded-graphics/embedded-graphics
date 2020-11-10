use crate::{
    draw_target::DrawTarget,
    drawable::{Drawable, Pixel},
    fonts::MonospacedFont,
    geometry::{Dimensions, Point, Size},
    iterator::IntoPixels,
    pixelcolor::PixelColor,
    primitives::Rectangle,
    style::{MonospacedTextStyle, Styled},
    transform::Transform,
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
    pub fn into_styled<C, F>(
        self,
        style: MonospacedTextStyle<C, F>,
    ) -> Styled<Self, MonospacedTextStyle<C, F>>
    where
        C: PixelColor,
        F: MonospacedFont,
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

impl<C, F> Drawable for Styled<Text<'_>, MonospacedTextStyle<C, F>>
where
    C: PixelColor,
    F: MonospacedFont + Copy,
{
    type Color = C;

    fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        display.draw_iter(self.into_pixels())
    }
}

impl<'a, C, F> IntoPixels for &Styled<Text<'a>, MonospacedTextStyle<C, F>>
where
    C: PixelColor,
    F: MonospacedFont + Copy,
{
    type Color = C;

    type Iter = StyledTextIterator<'a, C, F>;

    fn into_pixels(self) -> Self::Iter {
        Self::Iter {
            current_char: self.primitive.text.chars().next(),
            idx: 0,
            text: self.primitive.text,
            char_walk_x: 0,
            char_walk_y: 0,
            top_left: self.primitive.position,
            pos: self.primitive.position,
            style: self.style,
        }
    }
}

impl<C, F> Dimensions for Styled<Text<'_>, MonospacedTextStyle<C, F>>
where
    C: PixelColor,
    F: MonospacedFont,
{
    fn bounding_box(&self) -> Rectangle {
        // If a piece of text is completely transparent, return an empty bounding box
        if self.style.text_color.is_none() && self.style.background_color.is_none() {
            return Rectangle::new(self.primitive.position, Size::new(0, 0));
        }

        let width = self
            .primitive
            .text
            .lines()
            .map(|line| {
                (line.len() as u32 * (F::CHARACTER_SPACING + F::CHARACTER_SIZE.width))
                    .saturating_sub(F::CHARACTER_SPACING)
            })
            .max()
            .unwrap_or(0);

        let height = if width > 0 {
            F::CHARACTER_SIZE.height * self.primitive.text.lines().count() as u32
        } else {
            0
        };

        let size = Size::new(width, height);

        Rectangle::new(self.primitive.position, size)
    }
}

/// Pixel iterator for styled text.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct StyledTextIterator<'a, C, F>
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

impl<C, F> Iterator for StyledTextIterator<'_, C, F>
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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        fonts::{tests::assert_text_from_pattern, Font6x12, Font6x8},
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        prelude::*,
        style::PrimitiveStyle,
    };

    const HELLO_WORLD: &'static str = "Hello World!";

    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
    struct SpacedFont;

    impl MonospacedFont for SpacedFont {
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
        let style = MonospacedTextStyle::new(SpacedFont, BinaryColor::On);

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
        let style = MonospacedTextStyle::new(Font6x8, BinaryColor::On);
        let text = Text::new("AB\nC", Point::zero()).into_styled(style);

        assert_eq!(
            text.bounding_box(),
            Rectangle::new(Point::zero(), Size::new(2 * 6, 2 * 8))
        );
    }

    #[test]
    fn position_and_translate() {
        let style = MonospacedTextStyle::new(Font6x8, BinaryColor::On);

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
        let style_inverse = MonospacedTextStyle {
            font: Font6x8,
            text_color: Some(BinaryColor::Off),
            background_color: Some(BinaryColor::On),
        };
        Text::new("Mm", Point::zero())
            .into_styled(style_inverse)
            .draw(&mut display_inverse)
            .unwrap();

        let mut display_normal = MockDisplay::new();
        let style_normal = MonospacedTextStyle {
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
            .into_styled(MonospacedTextStyle::new(Font6x8, BinaryColor::On))
            .draw(&mut display)
            .unwrap();

        assert_eq!(display, MockDisplay::new());
    }

    #[test]
    fn negative_y_no_infinite_loop() {
        let style = MonospacedTextStyle {
            font: Font6x12,
            text_color: Some(BinaryColor::On),
            background_color: Some(BinaryColor::Off),
        };

        let mut text = Text::new("Testing string", Point::zero()).into_styled(style);
        text.translate_mut(Point::new(0, -12));

        assert_eq!(text.into_pixels().count(), 6 * 12 * "Testing string".len());
    }

    #[test]
    fn negative_x_no_infinite_loop() {
        let style = MonospacedTextStyle {
            font: Font6x12,
            text_color: Some(BinaryColor::On),
            background_color: Some(BinaryColor::Off),
        };

        let mut text = Text::new("A", Point::zero()).into_styled(style);
        text.translate_mut(Point::new(-6, 0));

        assert_eq!(text.into_pixels().count(), 6 * 12);
    }

    #[test]
    fn transparent_text_color_does_not_overwrite_background() {
        let style = TextStyle {
            font: Font6x8,
            text_color: None,
            background_color: Some(BinaryColor::On),
        };

        let mut display = MockDisplay::new();
        display.set_allow_overdraw(true);

        // Draw a background for the first character
        Rectangle::new(Point::zero(), Size::new(6, 8))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
            .draw(&mut display)
            .unwrap();

        Text::new("AA", Point::zero())
            .into_styled(style)
            .draw(&mut display)
            .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "#...###   ##",
                ".###.# ### #",
                ".###.# ### #",
                ".....#     #",
                ".###.# ### #",
                ".###.# ### #",
                ".###.# ### #",
                "############",
            ])
        );
    }

    #[test]
    fn transparent_text_has_zero_size_but_retains_position() {
        let style: TextStyle<BinaryColor, _> = TextStyle {
            font: Font6x8,
            text_color: None,
            background_color: None,
        };

        let styled = Text::new(" A", Point::new(7, 11)).into_styled(style);

        assert_eq!(
            styled.bounding_box(),
            Rectangle::new(Point::new(7, 11), Size::zero()),
            "Transparent text is expected to have a zero sized bounding box with the top left corner at the text position",
        );
    }
}
