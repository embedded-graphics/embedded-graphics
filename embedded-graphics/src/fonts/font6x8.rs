use crate::fonts::Font;
use crate::geometry::Size;

/// 6x8 pixel monospace font
///
/// There is also the [`text_6x8`] macro to provide an easier to use interface.
///
/// [![6x8 font spritemap screenshot](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/embedded-graphics/data/font6x8.png)](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/embedded-graphics/data/font6x8.png)
///
/// # Examples
///
/// See the [module-level documentation](./index.html) for examples.
///
/// [`text_6x8`]: ../macro.text_6x8.html
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Font6x8;

impl Font for Font6x8 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../data/font6x8_1bpp.raw");
    const FONT_IMAGE_WIDTH: u32 = 240;

    const CHARACTER_SIZE: Size = Size::new(6, 8);

    fn char_offset(c: char) -> u32 {
        let fallback = '?' as u32 - ' ' as u32;
        if c < ' ' {
            return fallback;
        }
        if c <= '~' {
            return c as u32 - ' ' as u32;
        }
        if c < '¡' || c > 'ÿ' {
            return fallback;
        }
        c as u32 - ' ' as u32 - 34
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

    const WIDTH: usize = Font6x8::CHARACTER_SIZE.width as usize;
    const HEIGHT: usize = Font6x8::CHARACTER_SIZE.height as usize;
    const HELLO_WORLD: &'static str = "Hello World!";

    #[test]
    fn text_dimensions() {
        let style = TextStyle::with_text_color(Font6x8, BinaryColor::On);
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
        let style = TextStyle::with_text_color(Font6x8, BinaryColor::On);
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
            .into_styled(TextStyle::with_text_color(Font6x8, BinaryColor::On))
            .draw(&mut display);

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "#   #       ",
                "## ##       ",
                "# # # ## #  ",
                "# # # # # # ",
                "#   # #   # ",
                "#   # #   # ",
                "#   # #   # ",
                "            ",
            ])
        );
    }

    #[test]
    fn correct_inverse_coloured_m() {
        let mut display = MockDisplay::new();
        let style = TextStyle {
            font: Font6x8,
            text_color: Some(BinaryColor::Off),
            background_color: Some(BinaryColor::On),
        };
        Text::new("Mm", Point::zero())
            .into_styled(style)
            .draw(&mut display);

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                ".###.#######",
                "..#..#######",
                ".#.#.#..#.##",
                ".#.#.#.#.#.#",
                ".###.#.###.#",
                ".###.#.###.#",
                ".###.#.###.#",
                "############",
            ])
        );
    }

    // tests if black on white has really the same behavior as white on black
    #[test]
    fn compare_inverse_coloured_m() {
        let mut display_inverse = MockDisplay::new();
        let style_inverse = TextStyle {
            font: Font6x8,
            text_color: Some(BinaryColor::Off),
            background_color: Some(BinaryColor::On),
        };
        Text::new("Mm", Point::zero())
            .into_styled(style_inverse)
            .draw(&mut display_inverse);

        let mut display_normal = MockDisplay::new();
        let style_normal = TextStyle {
            font: Font6x8,
            text_color: Some(BinaryColor::On),
            background_color: Some(BinaryColor::Off),
        };
        Text::new("Mm", Point::zero())
            .into_styled(style_normal)
            .draw(&mut display_normal);

        for y in 0..display_inverse.height() {
            for x in 0..display_inverse.width() {
                let p = Point::new(x as i32, y as i32);

                let inverse_color = display_inverse.get_pixel(p);
                let normal_color = display_normal.get_pixel(p);

                assert_eq!(inverse_color, normal_color.map(|c| c.invert()));
            }
        }
    }

    #[test]
    fn correct_ascii_borders() {
        let mut display = MockDisplay::new();
        Text::new(" ~", Point::zero())
            .into_styled(TextStyle::with_text_color(Font6x8, BinaryColor::On))
            .draw(&mut display);

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "       ## # ",
                "      #  #  ",
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
    fn no_fill_doesnt_hang() {
        let mut display = MockDisplay::new();
        Text::new(" ", Point::zero())
            .into_styled(TextStyle::with_text_color(Font6x8, BinaryColor::On))
            .draw(&mut display);

        assert_eq!(display, MockDisplay::new());
    }

    #[test]
    fn correct_dollar_y() {
        let mut display = MockDisplay::new();
        Text::new("$y", Point::zero())
            .into_styled(TextStyle::with_text_color(Font6x8, BinaryColor::On))
            .draw(&mut display);

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "  #         ",
                " ####       ",
                "# #   #   # ",
                " ###  #   # ",
                "  # # #   # ",
                "####   #### ",
                "  #       # ",
                "       ###  ",
            ])
        );
    }

    #[test]
    fn correct_latin1() {
        let mut display = MockDisplay::new();
        Text::new("¡ÿ", Point::zero())
            .into_styled(TextStyle::with_text_color(Font6x8, BinaryColor::On))
            .draw(&mut display);

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "  #    # #  ",
                "            ",
                "  #   #   # ",
                "  #   #   # ",
                "  #   #   # ",
                "  #    #### ",
                "  #       # ",
                "       ###  ",
                "            ",
            ])
        );
    }

    #[test]
    fn dont_panic() {
        let two_question_marks = MockDisplay::from_pattern(&[
            " ###   ### ",
            "#   # #   #",
            "    #     #",
            "   #     # ",
            "  #     #  ",
            "           ",
            "  #     #  ",
        ]);

        let style = TextStyle::with_text_color(Font6x8, BinaryColor::On);

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
        Text::new("Ā💣", Point::zero())
            .into_styled(style)
            .draw(&mut display);
        assert_eq!(display, two_question_marks);
    }
}
