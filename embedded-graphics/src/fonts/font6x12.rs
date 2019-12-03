use crate::fonts::Font;
use crate::geometry::Size;

/// 6x12 pixel monospace font.
///
/// [![6x12 font spritemap screenshot](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/embedded-graphics/data/font6x12.png)](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/embedded-graphics/data/font6x12.png)
///
/// # Examples
///
/// See the [module-level documentation](./index.html) for examples.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Font6x12;

impl Font for Font6x12 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../data/font6x12_1bpp.raw");
    const FONT_IMAGE_WIDTH: u32 = 96;

    const CHARACTER_SIZE: Size = Size::new(6, 12);

    fn char_offset(c: char) -> u32 {
        let fallback = '?' as u32 - ' ' as u32;
        if c < ' ' {
            return fallback;
        }
        if c <= '~' {
            return c as u32 - ' ' as u32;
        }
        fallback
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::drawable::Drawable;
    use crate::fonts::{Font, Text};
    use crate::geometry::{Dimensions, Point, Size};
    use crate::mock_display::MockDisplay;
    use crate::pixelcolor::BinaryColor;
    use crate::style::TextStyle;
    use crate::transform::Transform;

    const WIDTH: usize = Font6x12::CHARACTER_SIZE.width as usize;
    const HEIGHT: usize = Font6x12::CHARACTER_SIZE.height as usize;
    const HELLO_WORLD: &'static str = "Hello World!";

    #[test]
    fn text_dimensions() {
        let style = TextStyle::new(Font6x12, BinaryColor::On);
        let hello = Text::new(HELLO_WORLD, Point::zero()).into_styled(style);
        let empty = Text::new("", Point::zero()).into_styled(style);

        assert_eq!(
            hello.size(),
            Size::new((HELLO_WORLD.len() * WIDTH) as u32, HEIGHT as u32)
        );
        assert_eq!(empty.size(), Size::new(0, 0));
    }

    #[test]
    fn text_corners() {
        let style = TextStyle::new(Font6x12, BinaryColor::On);
        let hello = Text::new(HELLO_WORLD, Point::zero())
            .into_styled(style)
            .translate(Point::new(5, -20));
        let empty = Text::new("", Point::zero())
            .into_styled(style)
            .translate(Point::new(10, 20));

        assert_eq!(hello.top_left(), Point::new(5, -20));
        assert_eq!(
            hello.bottom_right(),
            Point::new(
                ((HELLO_WORLD.len() * WIDTH) as i32) + 5,
                (HEIGHT as i32) - 20
            )
        );
        assert_eq!(empty.top_left(), Point::new(10, 20));
        assert_eq!(empty.bottom_right(), Point::new(10, 20));
    }

    #[test]
    fn correct_m() {
        let mut display = MockDisplay::new();
        Text::new("Mm", Point::zero())
            .into_styled(TextStyle::new(Font6x12, BinaryColor::On))
            .draw(&mut display);

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "            ",
                "#   #       ",
                "## ##       ",
                "## ##       ",
                "# # # ####  ",
                "# # # # # # ",
                "#   # # # # ",
                "#   # # # # ",
                "#   # # # # ",
                "#   # # # # ",
                "            ",
                "            ",
            ])
        );
    }

    #[test]
    fn correct_ascii_borders() {
        let mut display = MockDisplay::new();
        Text::new(" ~", Point::zero())
            .into_styled(TextStyle::new(Font6x12, BinaryColor::On))
            .draw(&mut display);

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "        # # ",
                "       #### ",
                "       # #  ",
                "            ",
                "            ",
                "            ",
                "            ",
                "            ",
                "            ",
                "            ",
                "            ",
                "            ",
            ])
        );
    }

    #[test]
    fn correct_dollar_y() {
        let mut display = MockDisplay::new();
        Text::new("$y", Point::zero())
            .into_styled(TextStyle::new(Font6x12, BinaryColor::On))
            .draw(&mut display);

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "            ",
                "  #         ",
                " ###        ",
                "# # #       ",
                "# #    #  # ",
                " ###   #  # ",
                "  # #  #  # ",
                "  # #  #  # ",
                "# # #  #  # ",
                " ###    ### ",
                "  #       # ",
                "        ##  ",
            ])
        );
    }

    #[test]
    fn dont_panic() {
        let two_question_marks = MockDisplay::from_pattern(&[
            "            ",
            "  ##    ##  ",
            " #  #  #  # ",
            " #  #  #  # ",
            "    #     # ",
            "   #     #  ",
            "  #     #   ",
            "  #     #   ",
            "            ",
            "  #     #   ",
            "            ",
            "            ",
        ]);

        let style = TextStyle::new(Font6x12, BinaryColor::On);

        let mut display = MockDisplay::new();
        Text::new("\0\n", Point::zero())
            .into_styled(style)
            .draw(&mut display);
        assert_eq!(display, two_question_marks);

        let mut display = MockDisplay::new();
        Text::new("\x7F\u{A0}", Point::zero())
            .into_styled(style)
            .draw(&mut display);
        assert_eq!(display, two_question_marks);

        let mut display = MockDisplay::new();
        Text::new("¡ÿ", Point::zero())
            .into_styled(style)
            .draw(&mut display);
        assert_eq!(display, two_question_marks);

        let mut display = MockDisplay::new();
        Text::new("Ā💣", Point::zero())
            .into_styled(style)
            .draw(&mut display);
        assert_eq!(display, two_question_marks);
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
