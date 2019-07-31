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
/// # Examples
///
/// ## Write some text to the screen at the default `(0, 0)` position
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::fonts::Font6x8;
/// use embedded_graphics::text_6x8;
/// # use embedded_graphics::mock_display::MockDisplay;
/// # use embedded_graphics::pixelcolor::BinaryColor;
/// # let mut display: MockDisplay<BinaryColor> = MockDisplay::default();
///
/// // Use struct methods directly
/// display.draw(Font6x8::render_str("Hello Rust!"));
///
/// // Use a macro instead
/// display.draw(text_6x8!("Hello Rust!"));
/// ```
///
/// ## Translate text by (20px, 30px)
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::fonts::Font6x8;
/// # use embedded_graphics::mock_display::MockDisplay;
/// # use embedded_graphics::pixelcolor::BinaryColor;
/// # let mut display: MockDisplay<BinaryColor> = MockDisplay::default();
///
/// display.draw(
///     Font6x8::render_str("Hello Rust!").translate(Coord::new(20, 30))
/// );
/// ```
///
/// ## Add some styling to the text
///
/// Use [any method provided by the `WithStyle` trait](../style/trait.WithStyle.html#required-methods).
/// Properties like `fill` or `stroke` passed to the `text_6x8` macro are converted into method
/// calls verbatim.
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::text_6x8;
/// use embedded_graphics::fonts::Font6x8;
/// use embedded_graphics::pixelcolor::Rgb565;
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::default();
///
/// display.draw(text_6x8!(
///     "Hello Rust!",
///     fill = Some(Rgb565::BLUE),
///     stroke = Some(Rgb565::YELLOW)
/// ));
///
/// display.draw(
///     Font6x8::render_str("Hello Rust!")
///         .translate(Coord::new(20, 30))
///         .fill(Some(Rgb565::BLUE))
///         .stroke(Some(Rgb565::YELLOW)),
/// );
/// ```
///
/// [`text_6x8`]: ../macro.text_6x8.html
pub type Font6x8<'a, C> = FontBuilder<'a, C, Font6x8Conf>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coord::Coord;
    use crate::drawable::Dimensions;
    use crate::fonts::Font;
    use crate::mock_display::MockDisplay;
    use crate::pixelcolor::BinaryColor;
    use crate::style::Style;
    use crate::style::WithStyle;
    use crate::transform::Transform;
    use crate::unsignedcoord::UnsignedCoord;
    use crate::Drawing;

    #[test]
    fn off_screen_text_does_not_infinite_loop() {
        let text: Font6x8<BinaryColor> = Font6x8::render_str("Hello World!")
            .style(Style::stroke(BinaryColor::On))
            .translate(Coord::new(5, -10));
        let mut it = text.into_iter();

        assert_eq!(it.next(), None);
    }

    #[test]
    fn unstroked_text_does_not_infinite_loop() {
        let text: Font6x8<BinaryColor> = Font6x8::render_str("Hello World!")
            .style(Style::stroke(BinaryColor::On))
            .translate(Coord::new(5, -10));
        let mut it = text.into_iter();

        assert_eq!(it.next(), None);
    }

    #[test]
    fn text_dimensions() {
        let hello: Font6x8<BinaryColor> = Font6x8::render_str("Hello World!");
        let empty: Font6x8<BinaryColor> = Font6x8::render_str("");

        assert_eq!(hello.size(), UnsignedCoord::new(72, 8));
        assert_eq!(empty.size(), UnsignedCoord::new(0, 0));
    }

    #[test]
    fn text_corners() {
        let hello: Font6x8<BinaryColor> =
            Font6x8::render_str("Hello World!").translate(Coord::new(5, -20));
        let empty: Font6x8<BinaryColor> = Font6x8::render_str("").translate(Coord::new(10, 20));

        assert_eq!(hello.top_left(), Coord::new(5, -20));
        assert_eq!(hello.bottom_right(), Coord::new(72 + 5, 8 - 20));
        assert_eq!(empty.top_left(), Coord::new(10, 20));
        assert_eq!(empty.bottom_right(), Coord::new(10, 20));
    }

    #[test]
    fn correct_m() {
        let mut display = MockDisplay::new();
        display.draw(Font6x8::render_str("Mm").stroke(Some(BinaryColor::On)));

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
        display.draw(
            Font6x8::render_str("Mm")
                .stroke(Some(BinaryColor::Off))
                .fill(Some(BinaryColor::On)),
        );

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
        display_inverse.draw(
            Font6x8::render_str("Mm")
                .stroke(Some(BinaryColor::Off))
                .fill(Some(BinaryColor::On)),
        );

        let mut display_normal = MockDisplay::new();
        display_normal.draw(
            Font6x8::render_str("Mm")
                .stroke(Some(BinaryColor::On))
                .fill(Some(BinaryColor::Off)),
        );

        let inverted_display_normal = display_normal.pixels().map(|p| p.map(|c| c.invert()));

        assert!(inverted_display_normal.eq(display_inverse.pixels()));
    }

    #[test]
    fn correct_ascii_borders() {
        let mut display = MockDisplay::new();
        display.draw(Font6x8::render_str(" ~").stroke(Some(BinaryColor::On)));

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
        display.draw(Font6x8::render_str(" ").stroke(Some(BinaryColor::On)));

        assert_eq!(display, MockDisplay::new());
    }

    #[test]
    fn correct_dollar_y() {
        let mut display = MockDisplay::new();
        display.draw(Font6x8::render_str("$y").stroke(Some(BinaryColor::On)));

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
        display.draw(Font6x8::render_str("¡ÿ").stroke(Some(BinaryColor::On)));

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
        display.draw(Font6x8::render_str("\0\n").stroke(Some(BinaryColor::On)));
        assert_eq!(display, two_question_marks);

        let mut display = MockDisplay::new();
        display.draw(Font6x8::render_str("\x7F\u{A0}").stroke(Some(BinaryColor::On)));
        assert_eq!(display, two_question_marks);

        let mut display = MockDisplay::new();
        display.draw(Font6x8::render_str("Ā💣").stroke(Some(BinaryColor::On)));
        assert_eq!(display, two_question_marks);
    }
}
