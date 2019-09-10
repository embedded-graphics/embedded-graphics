use crate::fonts::font_builder::{FontBuilder, FontBuilderConf};

#[derive(Debug, Copy, Clone)]
pub enum Font12x16Conf {}
impl FontBuilderConf for Font12x16Conf {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../data/font12x16_1bpp.raw");
    const CHAR_HEIGHT: u32 = 16;
    const CHAR_WIDTH: u32 = 12;
    const FONT_IMAGE_WIDTH: u32 = 480;
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

/// 12x16 pixel monospace font
///
/// There is also the [`text_12x16`] macro to provide an easier to use interface.
///
/// [![12x16 font spritemap screenshot](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/embedded-graphics/data/font12x16.png)](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/embedded-graphics/data/font12x16.png)
///
/// # Examples
///
/// See the [module-level documentation](./index.html) for examples.
///
/// [`text_12x16`]: ../macro.text_12x16.html
pub type Font12x16<'a, C> = FontBuilder<'a, C, Font12x16Conf>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fonts::Font;
    use crate::geometry::{Dimensions, Point, Size};
    use crate::mock_display::MockDisplay;
    use crate::pixelcolor::BinaryColor;
    use crate::style::WithStyle;
    use crate::transform::Transform;
    use crate::Drawing;

    #[test]
    fn off_screen_text_does_not_infinite_loop() {
        let text: Font12x16<BinaryColor> = Font12x16::render_str("Hello World!")
            .stroke_color(Some(BinaryColor::On))
            .fill_color(Some(BinaryColor::Off))
            .translate(Point::new(5, -20));

        assert_eq!(text.into_iter().count(), 12 * 16 * "Hello World!".len());
    }

    #[test]
    fn text_dimensions() {
        let hello: Font12x16<BinaryColor> = Font12x16::render_str("Hello World!");
        let empty: Font12x16<BinaryColor> = Font12x16::render_str("");

        assert_eq!(hello.size(), Size::new(144, 16));
        assert_eq!(empty.size(), Size::new(0, 0));
    }

    #[test]
    fn text_corners() {
        let hello: Font12x16<BinaryColor> =
            Font12x16::render_str("Hello World!").translate(Point::new(5, -20));
        let empty: Font12x16<BinaryColor> = Font12x16::render_str("").translate(Point::new(10, 20));

        assert_eq!(hello.top_left(), Point::new(5, -20));
        assert_eq!(hello.bottom_right(), Point::new(144 + 5, 16 - 20));
        assert_eq!(empty.top_left(), Point::new(10, 20));
        assert_eq!(empty.bottom_right(), Point::new(10, 20));
    }

    #[test]
    fn correct_m() {
        let mut display = MockDisplay::new();
        display.draw(Font12x16::render_str("Mm").stroke_color(Some(BinaryColor::On)));

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "##      ##              ",
                "##      ##              ",
                "####  ####              ",
                "####  ####              ",
                "##  ##  ##  ####  ##    ",
                "##  ##  ##  ####  ##    ",
                "##  ##  ##  ##  ##  ##  ",
                "##  ##  ##  ##  ##  ##  ",
                "##      ##  ##      ##  ",
                "##      ##  ##      ##  ",
                "##      ##  ##      ##  ",
                "##      ##  ##      ##  ",
                "##      ##  ##      ##  ",
                "##      ##  ##      ##  ",
                "                        ",
                "                        ",
            ])
        );
    }

    #[test]
    fn correct_ascii_borders() {
        let mut display = MockDisplay::new();
        display.draw(Font12x16::render_str(" ~").stroke_color(Some(BinaryColor::On)));

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "              ####  ##  ",
                "              ####  ##  ",
                "            ##    ##    ",
                "            ##    ##    ",
                "                        ",
                "                        ",
                "                        ",
                "                        ",
                "                        ",
                "                        ",
                "                        ",
                "                        ",
                "                        ",
                "                        ",
                "                        ",
                "                        ",
            ])
        );
    }

    #[test]
    fn correct_dollar_y() {
        let mut display = MockDisplay::new();
        display.draw(Font12x16::render_str("$y").stroke_color(Some(BinaryColor::On)));

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "    ##                  ",
                "    ##                  ",
                "  ########              ",
                "  ########              ",
                "##  ##      ##      ##  ",
                "##  ##      ##      ##  ",
                "  ######    ##      ##  ",
                "  ######    ##      ##  ",
                "    ##  ##  ##      ##  ",
                "    ##  ##  ##      ##  ",
                "########      ########  ",
                "########      ########  ",
                "    ##              ##  ",
                "    ##              ##  ",
                "              ######    ",
                "              ######    ",
            ])
        );
    }

    #[test]
    fn dont_panic() {
        let two_question_marks = MockDisplay::from_pattern(&[
            "  ######      ######    ",
            "  ######      ######    ",
            "##      ##  ##      ##  ",
            "##      ##  ##      ##  ",
            "        ##          ##  ",
            "        ##          ##  ",
            "      ##          ##    ",
            "      ##          ##    ",
            "    ##          ##      ",
            "    ##          ##      ",
            "                        ",
            "                        ",
            "    ##          ##      ",
            "    ##          ##      ",
            "                        ",
            "                        ",
        ]);

        let mut display = MockDisplay::new();
        display.draw(Font12x16::render_str("\0\n").stroke_color(Some(BinaryColor::On)));
        assert_eq!(display, two_question_marks);

        let mut display = MockDisplay::new();
        display.draw(Font12x16::render_str("\x7F\u{A0}").stroke_color(Some(BinaryColor::On)));
        assert_eq!(display, two_question_marks);

        let mut display = MockDisplay::new();
        display.draw(Font12x16::render_str("¡ÿ").stroke_color(Some(BinaryColor::On)));
        assert_eq!(display, two_question_marks);

        let mut display = MockDisplay::new();
        display.draw(Font12x16::render_str("Ā💣").stroke_color(Some(BinaryColor::On)));
        assert_eq!(display, two_question_marks);
    }
}
