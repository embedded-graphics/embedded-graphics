use crate::{fonts::Font, geometry::Size};

/// 6x6 pixel variable width font.
///
/// [![6x6 font spritemap screenshot](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/embedded-graphics/data/font6x6.png)](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/embedded-graphics/data/font6x6.png)
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
        fonts::{Font, Text},
        geometry::{Dimensions, Point, Size},
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        style::TextStyle,
        transform::Transform,
    };

    const HEIGHT: usize = Font6x6::CHARACTER_SIZE.height as usize;
    const HELLO_WORLD: &'static str = "Hello World!";
    const HELLO_WORLD_WIDTH: u32 = 51;

    #[test]
    fn text_dimensions() {
        let style = TextStyle::new(Font6x6, BinaryColor::On);
        let hello = Text::new(HELLO_WORLD, Point::zero()).into_styled(style);
        let empty = Text::new("", Point::zero()).into_styled(style);

        assert_eq!(hello.size(), Size::new(HELLO_WORLD_WIDTH, HEIGHT as u32));
        assert_eq!(empty.size(), Size::new(0, 0));
    }

    #[test]
    fn text_corners() {
        let style = TextStyle::new(Font6x6, BinaryColor::On);
        let hello = Text::new(HELLO_WORLD, Point::zero())
            .into_styled(style)
            .translate(Point::new(5, -20));
        let empty = Text::new("", Point::zero())
            .into_styled(style)
            .translate(Point::new(10, 20));

        assert_eq!(hello.top_left(), Point::new(5, -20));
        assert_eq!(
            hello.bottom_right(),
            Point::new((HELLO_WORLD_WIDTH as i32) + 5, (HEIGHT as i32) - 20)
        );
        assert_eq!(empty.top_left(), Point::new(10, 20));
        assert_eq!(empty.bottom_right(), Point::new(10, 20));
    }

    #[test]
    fn correct_m() -> Result<(), core::convert::Infallible> {
        let mut display = MockDisplay::new();
        Text::new("Mm", Point::zero())
            .into_styled(TextStyle::new(Font6x6, BinaryColor::On))
            .draw(&mut display)?;

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "#   #       ",
                "## ##  # #  ",
                "# # # # # # ",
                "#   # #   # ",
                "#   # #   # ",
                "            ",
            ])
        );

        Ok(())
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

        for y in 0..display_inverse.height() {
            for x in 0..display_inverse.width() {
                let p = Point::new(x as i32, y as i32);

                let inverse_color = display_inverse.get_pixel(p);
                let normal_color = display_normal.get_pixel(p);

                assert_eq!(inverse_color, normal_color.map(|c| c.invert()));
            }
        }

        Ok(())
    }

    #[test]
    fn correct_i() -> Result<(), core::convert::Infallible> {
        let mut display = MockDisplay::new();
        Text::new("Ii", Point::zero())
            .into_styled(TextStyle::new(Font6x6, BinaryColor::On))
            .draw(&mut display)?;

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "# #         ",
                "#           ",
                "# #         ",
                "# #         ",
                "# #         ",
                "            ",
            ])
        );

        Ok(())
    }

    #[test]
    fn correct_ascii_borders() -> Result<(), core::convert::Infallible> {
        let mut display = MockDisplay::new();
        Text::new(" ~", Point::zero())
            .into_styled(TextStyle::new(Font6x6, BinaryColor::On))
            .draw(&mut display)?;

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "            ",
                "   ## #     ",
                "  #  #      ",
                "            ",
                "            ",
                "            ",
            ])
        );

        Ok(())
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
    fn correct_dollar_y() -> Result<(), core::convert::Infallible> {
        let mut display = MockDisplay::new();
        Text::new("$y", Point::zero())
            .into_styled(TextStyle::new(Font6x6, BinaryColor::On))
            .draw(&mut display)?;

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                " #### #  #  ",
                "# #   #  #  ",
                "##### ####  ",
                "  # #    #  ",
                "####  ###   ",
                "            ",
            ])
        );

        Ok(())
    }

    #[test]
    fn dont_panic() -> Result<(), core::convert::Infallible> {
        let two_question_marks = MockDisplay::from_pattern(&[
            " ###   ### ",
            "#   # #   #",
            "  ##    ## ",
            "           ",
            "  #     #  ",
            "           ",
        ]);

        let style = TextStyle::new(Font6x6, BinaryColor::On);

        let mut display = MockDisplay::new();
        Text::new("\0\n", Point::zero())
            .into_styled(style)
            .draw(&mut display)?;
        assert_eq!(display, two_question_marks);

        let mut display = MockDisplay::new();
        Text::new("\x7F\u{A0}", Point::zero())
            .into_styled(style)
            .draw(&mut display)?;
        assert_eq!(display, two_question_marks);

        let mut display = MockDisplay::new();
        Text::new("Ā💣", Point::zero())
            .into_styled(style)
            .draw(&mut display)?;
        assert_eq!(display, two_question_marks);

        Ok(())
    }
}
