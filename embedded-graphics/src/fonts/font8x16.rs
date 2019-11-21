use crate::fonts::font_builder::{FontBuilder, FontBuilderConf};

#[derive(Debug, Copy, Clone)]
/// Config for 8x16 font
pub enum Font8x16Conf {}
impl FontBuilderConf for Font8x16Conf {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../data/font8x16_1bpp.raw");
    const CHAR_HEIGHT: u32 = 16;
    const CHAR_WIDTH: u32 = 8;
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

/// 8x16 pixel monospace font
///
/// There is also the [`text_8x16`] macro to provide an easier to use interface.
///
/// [![8x16 font spritemap screenshot](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/embedded-graphics/data/font8x16.png)](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/embedded-graphics/data/font8x16.png)
///
/// # Examples
///
/// See the [module-level documentation](./index.html) for examples.
///
/// [`text_8x16`]: ../macro.text_8x16.html
pub type Font8x16<'a, C> = FontBuilder<'a, C, Font8x16Conf>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::drawable::Drawable;
    use crate::fonts::Font;
    use crate::geometry::{Dimensions, Point, Size};
    use crate::mock_display::MockDisplay;
    use crate::pixelcolor::BinaryColor;
    use crate::style::TextStyle;
    use crate::transform::Transform;

    const WIDTH: usize = Font8x16Conf::CHAR_WIDTH as usize;
    const HEIGHT: usize = Font8x16Conf::CHAR_HEIGHT as usize;
    const HELLO_WORLD: &'static str = "Hello World!";

    #[test]
    fn text_dimensions() {
        let style = TextStyle::with_text_color(BinaryColor::On);
        let hello = Font8x16::render_str(HELLO_WORLD, style);
        let empty = Font8x16::render_str("", style);

        assert_eq!(
            hello.size(),
            Size::new((HELLO_WORLD.len() * WIDTH) as u32, HEIGHT as u32)
        );
        assert_eq!(empty.size(), Size::new(0, 0));
    }

    #[test]
    fn text_corners() {
        let style = TextStyle::with_text_color(BinaryColor::On);
        let hello = Font8x16::render_str(HELLO_WORLD, style).translate(Point::new(5, -20));
        let empty = Font8x16::render_str("", style).translate(Point::new(10, 20));

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
        Font8x16::render_str("Mm", TextStyle::with_text_color(BinaryColor::On)).draw(&mut display);

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "                ",
                "                ",
                "##   ##         ",
                "### ###         ",
                "#######         ",
                "####### ### ##  ",
                "## # ## ####### ",
                "##   ## ## # ## ",
                "##   ## ## # ## ",
                "##   ## ## # ## ",
                "##   ## ## # ## ",
                "##   ## ##   ## ",
                "                ",
                "                ",
                "                ",
                "                ",
            ])
        );
    }

    #[test]
    fn correct_ascii_borders() {
        let mut display = MockDisplay::new();
        Font8x16::render_str(" ~", TextStyle::with_text_color(BinaryColor::On)).draw(&mut display);

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "                ",
                "         ### ## ",
                "        ## ###  ",
                "                ",
                "                ",
                "                ",
                "                ",
                "                ",
                "                ",
                "                ",
                "                ",
                "                ",
                "                ",
                "                ",
                "                ",
                "                ",
            ])
        );
    }

    #[test]
    fn correct_dollar_y() {
        let mut display = MockDisplay::new();
        Font8x16::render_str("$y", TextStyle::with_text_color(BinaryColor::On)).draw(&mut display);

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "   ##                   ",
                "   ##                   ",
                " #####                  ",
                "##   ##                 ",
                "##    #                 ",
                "##      ##   ##         ",
                " #####  ##   ##         ",
                "     ## ##   ##         ",
                "     ## ##   ##         ",
                "#    ## ##   ##         ",
                "##   ## ##   ##         ",
                " #####   ######         ",
                "   ##        ##         ",
                "   ##       ##          ",
                "        #####           ",
                "                        ",
            ])
        );
    }

    #[test]
    fn correct_latin1() {
        let mut display = MockDisplay::new();
        Font8x16::render_str("¡ÿ", TextStyle::with_text_color(BinaryColor::On)).draw(&mut display);

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "                        ",
                "        ##   ##         ",
                "   ##   ##   ##         ",
                "   ##                   ",
                "                        ",
                "   ##   ##   ##         ",
                "   ##   ##   ##         ",
                "   ##   ##   ##         ",
                "  ####  ##   ##         ",
                "  ####  ##   ##         ",
                "  ####  ##   ##         ",
                "   ##    ######         ",
                "             ##         ",
                "            ##          ",
                "        #####           ",
                "                        ",
            ])
        );
    }

    #[test]
    fn dont_panic() {
        let two_question_marks = MockDisplay::from_pattern(&[
            "                        ",
            "                        ",
            " #####   #####          ",
            "##   ## ##   ##         ",
            "##   ## ##   ##         ",
            "    ##      ##          ",
            "   ##      ##           ",
            "   ##      ##           ",
            "   ##      ##           ",
            "                        ",
            "   ##      ##           ",
            "   ##      ##           ",
            "                        ",
            "                        ",
            "                        ",
            "                        ",
        ]);

        let style = TextStyle::with_text_color(BinaryColor::On);

        let mut display = MockDisplay::new();
        Font8x16::render_str("\0\n", style).draw(&mut display);
        assert_eq!(display, two_question_marks);

        let mut display = MockDisplay::new();
        Font8x16::render_str("\x7F\u{A0}", style).draw(&mut display);
        assert_eq!(display, two_question_marks);

        let mut display = MockDisplay::new();
        Font8x16::render_str("Ā💣", style).draw(&mut display);
        assert_eq!(display, two_question_marks);
    }
}
