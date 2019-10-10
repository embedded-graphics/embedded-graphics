use crate::fonts::font_builder::{FontBuilder, FontBuilderConf};

#[derive(Debug, Copy, Clone)]
pub enum Font6x8Conf {}
impl FontBuilderConf for Font6x8Conf {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../data/font6x8_1bpp.raw");
    const CHAR_HEIGHT: u32 = 8;
    const CHAR_WIDTH: u32 = 6;
    const FONT_IMAGE_WIDTH: u32 = 240;
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

pub type Font6x8<'a, C> = FontBuilder<'a, C, Font6x8Conf>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use crate::fonts::Font;
    use crate::geometry::{Dimensions, Point, Size};
    use crate::mock_display::MockDisplay;
    use crate::pixelcolor::BinaryColor;
    use crate::style::WithStyle;
    use crate::transform::Transform;

    #[test]
    fn text_with_negative_y_does_not_infinite_loop() {
        let text: Font6x8<BinaryColor> = Font6x8::render_str("Hello World!")
            .stroke_color(Some(BinaryColor::On))
            .fill_color(Some(BinaryColor::Off))
            .translate(Point::new(5, -10));

        assert_eq!(text.into_iter().count(), 6 * 8 * "Hello World!".len());
    }

    #[test]
    fn text_dimensions() {
        let hello: Font6x8<BinaryColor> = Font6x8::render_str("Hello World!");
        let empty: Font6x8<BinaryColor> = Font6x8::render_str("");

        assert_eq!(hello.size(), Size::new(72, 8));
        assert_eq!(empty.size(), Size::new(0, 0));
    }

    #[test]
    fn text_corners() {
        let hello: Font6x8<BinaryColor> =
            Font6x8::render_str("Hello World!").translate(Point::new(5, -20));
        let empty: Font6x8<BinaryColor> = Font6x8::render_str("").translate(Point::new(10, 20));

        assert_eq!(hello.top_left(), Point::new(5, -20));
        assert_eq!(hello.bottom_right(), Point::new(72 + 5, 8 - 20));
        assert_eq!(empty.top_left(), Point::new(10, 20));
        assert_eq!(empty.bottom_right(), Point::new(10, 20));
    }

    #[test]
    fn correct_m() {
        let mut display = MockDisplay::new();
        Font6x8::render_str("Mm").stroke_color(Some(BinaryColor::On)).draw(&mut display);

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
        Font6x8::render_str("Mm")
            .stroke_color(Some(BinaryColor::Off))
            .fill_color(Some(BinaryColor::On))
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

    // tests if black on white has really the same behaviour as white on black
    #[test]
    fn compare_inverse_coloured_m() {
        let mut display_inverse = MockDisplay::new();
        Font6x8::render_str("Mm")
            .stroke_color(Some(BinaryColor::Off))
            .fill_color(Some(BinaryColor::On))
            .draw(&mut display_inverse);

        let mut display_normal = MockDisplay::new();
        Font6x8::render_str("Mm")
            .stroke_color(Some(BinaryColor::On))
            .fill_color(Some(BinaryColor::Off))
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
        Font6x8::render_str(" ~").stroke_color(Some(BinaryColor::On)).draw(&mut display);

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
        Font6x8::render_str(" ").stroke_color(Some(BinaryColor::On)).draw(&mut display);

        assert_eq!(display, MockDisplay::new());
    }

    #[test]
    fn correct_dollar_y() {
        let mut display = MockDisplay::new();
        Font6x8::render_str("$y").stroke_color(Some(BinaryColor::On)).draw(&mut display);

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
        Font6x8::render_str("¡ÿ").stroke_color(Some(BinaryColor::On)).draw(&mut display);

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

        let mut display = MockDisplay::new();
        Font6x8::render_str("\0\n").stroke_color(Some(BinaryColor::On)).draw(&mut display);
        assert_eq!(display, two_question_marks);

        let mut display = MockDisplay::new();
        Font6x8::render_str("\x7F\u{A0}").stroke_color(Some(BinaryColor::On)).draw(&mut display);
        assert_eq!(display, two_question_marks);

        let mut display = MockDisplay::new();
        Font6x8::render_str("Ā💣").stroke_color(Some(BinaryColor::On)).draw(&mut display);
        assert_eq!(display, two_question_marks);
    }
}
