use crate::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point},
    iterator::IntoPixels,
    pixelcolor::PixelColor,
    primitives::Rectangle,
    style::{Styled, TextStyle, TextStylePixels},
    transform::Transform,
    Drawable,
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
    pub fn into_styled<C, S>(self, style: S) -> Styled<Self, S>
    where
        C: PixelColor,
        S: TextStyle<Color = C>,
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

impl<C, S> Drawable for Styled<Text<'_>, S>
where
    C: PixelColor,
    S: TextStyle<Color = C>,
{
    type Color = C;

    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        self.style.render_text(&self.primitive, target)
    }
}

impl<'a, C, S> IntoPixels for &Styled<Text<'a>, S>
where
    C: PixelColor,
    S: TextStyle<Color = C> + TextStylePixels<'a>,
{
    type Color = C;

    type Iter = S::Iter;

    fn into_pixels(self) -> Self::Iter {
        self.style.pixels(&self.primitive)
    }
}

impl<C, S> Dimensions for Styled<Text<'_>, S>
where
    C: PixelColor,
    S: TextStyle<Color = C>,
{
    fn bounding_box(&self) -> Rectangle {
        self.style.bounding_box(&self.primitive)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        fonts::{tests::assert_text_from_pattern, Font6x12, Font6x8, MonoFont},
        geometry::Size,
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        prelude::*,
        style::MonoTextStyle,
        style::PrimitiveStyle,
    };

    const HELLO_WORLD: &'static str = "Hello World!";

    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
    struct SpacedFont;

    impl MonoFont for SpacedFont {
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
        let style = MonoTextStyle::new(SpacedFont, BinaryColor::On);

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
        let style = MonoTextStyle::new(Font6x8, BinaryColor::On);
        let text = Text::new("AB\nC", Point::zero()).into_styled(style);

        assert_eq!(
            text.bounding_box(),
            Rectangle::new(Point::zero(), Size::new(2 * 6, 2 * 8))
        );
    }

    #[test]
    fn position_and_translate() {
        let style = MonoTextStyle::new(Font6x8, BinaryColor::On);

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
        let style_inverse = MonoTextStyle {
            font: Font6x8,
            text_color: Some(BinaryColor::Off),
            background_color: Some(BinaryColor::On),
        };
        Text::new("Mm", Point::zero())
            .into_styled(style_inverse)
            .draw(&mut display_inverse)
            .unwrap();

        let mut display_normal = MockDisplay::new();
        let style_normal = MonoTextStyle {
            font: Font6x8,
            text_color: Some(BinaryColor::On),
            background_color: Some(BinaryColor::Off),
        };
        Text::new("Mm", Point::zero())
            .into_styled(style_normal)
            .draw(&mut display_normal)
            .unwrap();

        display_inverse.assert_eq(&display_normal.map(|c| c.invert()));
    }

    #[test]
    fn no_fill_does_not_hang() {
        let mut display = MockDisplay::new();
        Text::new(" ", Point::zero())
            .into_styled(MonoTextStyle::new(Font6x8, BinaryColor::On))
            .draw(&mut display)
            .unwrap();

        display.assert_eq(&MockDisplay::new());
    }

    #[test]
    fn negative_y_no_infinite_loop() {
        let style = MonoTextStyle {
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
        let style = MonoTextStyle {
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
        let style = MonoTextStyle {
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

        display.assert_pattern(&[
            "#...###   ##",
            ".###.# ### #",
            ".###.# ### #",
            ".....#     #",
            ".###.# ### #",
            ".###.# ### #",
            ".###.# ### #",
            "############",
        ]);
    }

    #[test]
    fn transparent_text_has_zero_size_but_retains_position() {
        let style: MonoTextStyle<BinaryColor, _> = MonoTextStyle {
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
