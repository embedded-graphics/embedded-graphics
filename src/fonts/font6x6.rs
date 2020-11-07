use crate::{fonts::Font, geometry::Size};

/// 6x6 pixel variable width font.
///
/// [![6x6 font spritemap screenshot](https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/master/data/font6x6.png)](https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/master/data/font6x6.png)
///
/// # Examples
///
/// See the [module-level documentation](./index.html) for examples.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font6x6;

impl Font for Font6x6 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../data/font6x6_1bpp.raw");
    const FONT_IMAGE_WIDTH: u32 = 192;

    const CHARACTER_SIZE: Size = Size::new(6, 6);
    const VARIABLE_WIDTH: bool = true;
    const CHARACTER_SPACING: u32 = 1;

    fn char_offset(c: char) -> u32 {
        let fallback = '?' as u32 - ' ' as u32;
        if c < ' ' {
            return fallback;
        }
        if c <= '~' {
            return c as u32 - ' ' as u32;
        }
        if c == '€' {
            return 100;
        }
        if c < '¡' || c > '¿' {
            return fallback;
        }
        c as u32 - ' ' as u32 - 0x20
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        drawable::Drawable,
        fonts::{tests::assert_text_from_pattern, Font, Text},
        geometry::{Dimensions, Point, Size},
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        style::TextStyle,
    };

    const HEIGHT: usize = Font6x6::CHARACTER_SIZE.height as usize;
    const HELLO_WORLD: &'static str = "Hello World!";
    const HELLO_WORLD_WIDTH: u32 = 51;

    #[test]
    fn text_dimensions() {
        let style = TextStyle::new(Font6x6, BinaryColor::On);
        let hello = Text::new(HELLO_WORLD, Point::zero()).into_styled(style);
        let empty = Text::new("", Point::zero()).into_styled(style);

        assert_eq!(
            hello.bounding_box().size,
            Size::new(HELLO_WORLD_WIDTH, HEIGHT as u32)
        );
        assert_eq!(empty.bounding_box().size, Size::zero());
    }

    #[test]
    fn correct_m() {
        assert_text_from_pattern(
            "Mm",
            Font6x6,
            &[
                "#   #       ",
                "## ##  # #  ",
                "# # # # # # ",
                "#   # #   # ",
                "#   # #   # ",
                "            ",
            ],
        );
    }

    #[test]
    fn correct_inverse_colored_m() -> Result<(), core::convert::Infallible> {
        let mut display = MockDisplay::new();
        let style = TextStyle {
            font: Font6x6,
            text_color: Some(BinaryColor::Off),
            background_color: Some(BinaryColor::On),
        };
        Text::new("Mm", Point::zero())
            .into_styled(style)
            .draw(&mut display)?;

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                ".###.#######",
                "..#..##.#.##",
                ".#.#.#.#.#.#",
                ".###.#.###.#",
                ".###.#.###.#",
                "############",
            ])
        );

        Ok(())
    }

    // tests if black on white has really the same behavior as white on black
    #[test]
    fn compare_inverse_colored_m() -> Result<(), core::convert::Infallible> {
        let mut display_inverse = MockDisplay::new();
        let style_inverse = TextStyle {
            font: Font6x6,
            text_color: Some(BinaryColor::Off),
            background_color: Some(BinaryColor::On),
        };
        Text::new("Mm", Point::zero())
            .into_styled(style_inverse)
            .draw(&mut display_inverse)?;

        let mut display_normal = MockDisplay::new();
        let style_normal = TextStyle {
            font: Font6x6,
            text_color: Some(BinaryColor::On),
            background_color: Some(BinaryColor::Off),
        };
        Text::new("Mm", Point::zero())
            .into_styled(style_normal)
            .draw(&mut display_normal)?;

        assert_eq!(display_inverse, display_normal.map(|c| c.invert()));

        Ok(())
    }

    #[test]
    fn correct_i() {
        assert_text_from_pattern(
            "Ii",
            Font6x6,
            &[
                "# #         ",
                "#           ",
                "# #         ",
                "# #         ",
                "# #         ",
                "            ",
            ],
        );
    }

    #[test]
    fn correct_ascii_borders() {
        assert_text_from_pattern(
            " ~",
            Font6x6,
            &[
                "            ",
                "   ## #     ",
                "  #  #      ",
                "            ",
                "            ",
                "            ",
            ],
        );
    }

    #[test]
    fn no_fill_doesnt_hang() -> Result<(), core::convert::Infallible> {
        let mut display = MockDisplay::new();
        Text::new(" ", Point::zero())
            .into_styled(TextStyle::new(Font6x6, BinaryColor::On))
            .draw(&mut display)?;

        assert_eq!(display, MockDisplay::new());

        Ok(())
    }

    #[test]
    fn correct_dollar_y() {
        assert_text_from_pattern(
            "$y",
            Font6x6,
            &[
                " #### #  #  ",
                "# #   #  #  ",
                "##### ####  ",
                "  # #    #  ",
                "####  ###   ",
                "            ",
            ],
        );
    }

    #[test]
    fn dont_panic() {
        let two_question_marks = &[
            " ###   ### ",
            "#   # #   #",
            "  ##    ## ",
            "           ",
            "  #     #  ",
            "           ",
        ];

        assert_text_from_pattern("\0\r", Font6x6, two_question_marks);
        assert_text_from_pattern("\x7F\u{A0}", Font6x6, two_question_marks);
        assert_text_from_pattern("Ā💣", Font6x6, two_question_marks);
    }
}
