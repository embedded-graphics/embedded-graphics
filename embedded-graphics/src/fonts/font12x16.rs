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
/// # Examples
///
/// ## Write some text to the screen at the default `(0, 0)` position
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::fonts::Font12x16;
/// use embedded_graphics::text_12x16;
/// # use embedded_graphics::mock_display::MockDisplay;
/// # use embedded_graphics::pixelcolor::BinaryColor;
/// # let mut display: MockDisplay<BinaryColor> = MockDisplay::default();
///
/// // Use struct methods directly
/// display.draw(Font12x16::render_str("Hello Rust!"));
///
/// // Use a macro instead
/// display.draw(text_12x16!("Hello Rust!"));
/// ```
///
/// ## Translate text by (20px, 30px)
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::fonts::Font12x16;
/// # use embedded_graphics::mock_display::MockDisplay;
/// # use embedded_graphics::pixelcolor::BinaryColor;
/// # let mut display: MockDisplay<BinaryColor> = MockDisplay::default();
///
/// display.draw(
///     Font12x16::render_str("Hello Rust!").translate(Point::new(20, 30))
/// );
/// ```
///
/// ## Add some styling to the text
///
/// Use [any method provided by the `WithStyle` trait](../style/trait.WithStyle.html#required-methods).
/// Properties like `fill` or `stroke` passed to the `text_12x16` macro are converted into method
/// calls verbatim.
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::text_12x16;
/// use embedded_graphics::fonts::Font12x16;
/// use embedded_graphics::pixelcolor::Rgb565;
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::default();
///
/// display.draw(text_12x16!(
///     "Hello Rust!",
///     fill = Some(Rgb565::BLUE),
///     stroke = Some(Rgb565::YELLOW)
/// ));
///
/// display.draw(
///     Font12x16::render_str("Hello Rust!")
///         .translate(Point::new(20, 30))
///         .fill(Some(Rgb565::BLUE))
///         .stroke(Some(Rgb565::YELLOW)),
/// );
/// ```
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
            .stroke(Some(BinaryColor::On))
            .fill(Some(BinaryColor::Off))
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
        display.draw(Font12x16::render_str("Mm").stroke(Some(BinaryColor::On)));

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
        display.draw(Font12x16::render_str(" ~").stroke(Some(BinaryColor::On)));

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
        display.draw(Font12x16::render_str("$y").stroke(Some(BinaryColor::On)));

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
        display.draw(Font12x16::render_str("\0\n").stroke(Some(BinaryColor::On)));
        assert_eq!(display, two_question_marks);

        let mut display = MockDisplay::new();
        display.draw(Font12x16::render_str("\x7F\u{A0}").stroke(Some(BinaryColor::On)));
        assert_eq!(display, two_question_marks);

        let mut display = MockDisplay::new();
        display.draw(Font12x16::render_str("¡ÿ").stroke(Some(BinaryColor::On)));
        assert_eq!(display, two_question_marks);

        let mut display = MockDisplay::new();
        display.draw(Font12x16::render_str("Ā💣").stroke(Some(BinaryColor::On)));
        assert_eq!(display, two_question_marks);
    }
}
