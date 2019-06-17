use crate::fonts::font_builder::{FontBuilder, FontBuilderConf};

#[derive(Debug, Copy, Clone)]
pub enum Font6x12Conf {}
impl FontBuilderConf for Font6x12Conf {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../data/font6x12_1bpp.raw");
    const CHAR_HEIGHT: u32 = 12;
    const CHAR_WIDTH: u32 = 6;
    const FONT_IMAGE_WIDTH: u32 = 96;
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

/// 6x12 pixel monospace font
///
/// There is also the [`text_6x12`] macro to provide an easier to use interface.
///
/// # Examples
///
/// ## Write some text to the screen at the default `(0, 0)` position
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::fonts::Font6x12;
/// use embedded_graphics::text_6x12;
/// # use embedded_graphics::mock_display::MockDisplay;
/// # use embedded_graphics::pixelcolor::BinaryColor;
/// # let mut display: MockDisplay<BinaryColor> = MockDisplay::default();
///
/// // Use struct methods directly
/// display.draw(Font6x12::render_str("Hello Rust!"));
///
/// // Use a macro instead
/// display.draw(text_6x12!("Hello Rust!"));
/// ```
///
/// ## Translate text by (20px, 30px)
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::fonts::Font6x12;
/// # use embedded_graphics::mock_display::MockDisplay;
/// # use embedded_graphics::pixelcolor::BinaryColor;
/// # let mut display: MockDisplay<BinaryColor> = MockDisplay::default();
///
/// display.draw(
///     Font6x12::render_str("Hello Rust!").translate(Coord::new(20, 30))
/// );
/// ```
///
/// ## Add some styling to the text
///
/// Use [any method provided by the `WithStyle` trait](../style/trait.WithStyle.html#required-methods).
/// Properties like `fill` or `stroke` passed to the `text_6x12` macro are converted into method
/// calls verbatim.
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::text_6x12;
/// use embedded_graphics::fonts::Font6x12;
/// use embedded_graphics::pixelcolor::Rgb565;
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::default();
///
/// display.draw(text_6x12!(
///     "Hello Rust!",
///     fill = Some(Rgb565::BLUE),
///     stroke = Some(Rgb565::YELLOW)
/// ));
///
/// display.draw(
///     Font6x12::render_str("Hello Rust!")
///         .translate(Coord::new(20, 30))
///         .fill(Some(Rgb565::BLUE))
///         .stroke(Some(Rgb565::YELLOW)),
/// );
/// ```
///
/// [`text_6x12`]: ../macro.text_6x12.html
pub type Font6x12<'a, C> = FontBuilder<'a, C, Font6x12Conf>;

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
    use BinaryColor::Off as C0;
    use BinaryColor::On as C1;

    #[test]
    fn off_screen_text_does_not_infinite_loop() {
        let text: Font6x12<BinaryColor> = Font6x12::render_str("Hello World!")
            .style(Style::stroke(C1))
            .translate(Coord::new(5, -20));
        let mut it = text.into_iter();

        assert_eq!(it.next(), None);
    }

    #[test]
    fn text_dimensions() {
        let hello: Font6x12<BinaryColor> = Font6x12::render_str("Hello World!");
        let empty: Font6x12<BinaryColor> = Font6x12::render_str("");

        assert_eq!(hello.size(), UnsignedCoord::new(72, 12));
        assert_eq!(empty.size(), UnsignedCoord::new(0, 0));
    }

    #[test]
    fn text_corners() {
        let hello: Font6x12<BinaryColor> =
            Font6x12::render_str("Hello World!").translate(Coord::new(5, -20));
        let empty: Font6x12<BinaryColor> = Font6x12::render_str("").translate(Coord::new(10, 20));

        assert_eq!(hello.top_left(), Coord::new(5, -20));
        assert_eq!(hello.bottom_right(), Coord::new(72 + 5, 12 - 20));
        assert_eq!(empty.top_left(), Coord::new(10, 20));
        assert_eq!(empty.bottom_right(), Coord::new(10, 20));
    }

    #[test]
    fn correct_m() {
        let mut display = MockDisplay::default();
        display.draw(Font6x12::render_str("Mm").stroke(Some(C1)));

        #[cfg_attr(rustfmt, rustfmt_skip)]
        assert_eq!(
            display,
            MockDisplay::new([
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C1, C0, C0, C0, C1, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C1, C1, C0, C1, C1, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C1, C1, C0, C1, C1, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C1, C0, C1, C0, C1, C0, C1, C1, C1, C1, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C1, C0, C1, C0, C1, C0, C1, C0, C1, C0, C1, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C1, C0, C0, C0, C1, C0, C1, C0, C1, C0, C1, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C1, C0, C0, C0, C1, C0, C1, C0, C1, C0, C1, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C1, C0, C0, C0, C1, C0, C1, C0, C1, C0, C1, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C1, C0, C0, C0, C1, C0, C1, C0, C1, C0, C1, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                //
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
            ])
        );
    }

    #[test]
    fn correct_ascii_borders() {
        let mut display = MockDisplay::default();
        display.draw(Font6x12::render_str(" ~").stroke(Some(C1)));

        #[cfg_attr(rustfmt, rustfmt_skip)]
        assert_eq!(
            display,
            MockDisplay::new([
                [C0, C0, C0, C0, C0, C0, C0, C0, C1, C0, C1, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C0, C0, C0, C0, C0, C1, C1, C1, C1, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C0, C0, C0, C0, C0, C1, C0, C1, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                //
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
            ])
        );
    }

    #[test]
    fn correct_dollar_y() {
        let mut display = MockDisplay::default();
        display.draw(Font6x12::render_str("$y").stroke(Some(C1)));

        #[cfg_attr(rustfmt, rustfmt_skip)]
        assert_eq!(
            display,
            MockDisplay::new([
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C1, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C1, C1, C1, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C1, C0, C1, C0, C1, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C1, C0, C1, C0, C0, C0, C0, C1, C0, C0, C1, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C1, C1, C1, C0, C0, C0, C1, C0, C0, C1, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C1, C0, C1, C0, C0, C1, C0, C0, C1, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C1, C0, C1, C0, C0, C1, C0, C0, C1, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C1, C0, C1, C0, C1, C0, C0, C1, C0, C0, C1, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C1, C1, C1, C0, C0, C0, C0, C1, C1, C1, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C1, C0, C0, C0, C0, C0, C0, C0, C1, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C0, C0, C0, C0, C0, C0, C1, C1, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                //
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
            ])
        );
    }

    #[test]
    fn dont_panic() {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let two_question_marks = MockDisplay::new(
            [
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C1, C1, C0, C0, C0, C0, C1, C1, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C1, C0, C0, C1, C0, C0, C1, C0, C0, C1, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C1, C0, C0, C1, C0, C0, C1, C0, C0, C1, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C0, C0, C1, C0, C0, C0, C0, C0, C1, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C0, C1, C0, C0, C0, C0, C0, C1, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C1, C0, C0, C0, C0, C0, C1, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C1, C0, C0, C0, C0, C0, C1, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C1, C0, C0, C0, C0, C0, C1, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                //
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
                [C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0, C0],
            ]
        );

        let mut display = MockDisplay::default();
        display.draw(Font6x12::render_str("\0\n").stroke(Some(C1)));
        assert_eq!(display, two_question_marks);

        let mut display = MockDisplay::default();
        display.draw(Font6x12::render_str("\x7F\u{A0}").stroke(Some(C1)));
        assert_eq!(display, two_question_marks);

        let mut display = MockDisplay::default();
        display.draw(Font6x12::render_str("¡ÿ").stroke(Some(C1)));
        assert_eq!(display, two_question_marks);

        let mut display = MockDisplay::default();
        display.draw(Font6x12::render_str("Ā💣").stroke(Some(C1)));
        assert_eq!(display, two_question_marks);
    }

    #[test]
    fn negative_y_no_infinite_loop() {
        let text: Font6x12<BinaryColor> = Font6x12::render_str("Testing string")
            .stroke(Some(C1))
            .translate(Coord::new(0, -12));

        let mut it = text.into_iter();

        // Font is completely off the top edge of the screen; no pixels should be rendered
        assert_eq!(it.next(), None);
    }

    #[test]
    fn negative_x_no_infinite_loop() {
        let text: Font6x12<BinaryColor> = Font6x12::render_str("A")
            .stroke(Some(C1))
            .translate(Coord::new(-6, 0));

        let mut it = text.into_iter();

        // Font is completely off the left edge of the screen; no pixels should be rendered
        assert_eq!(it.next(), None);
    }
}
