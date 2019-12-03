use crate::fonts::Font;
use crate::geometry::Size;

/// 8x16 pixel monospace font.
///
/// [![8x16 font spritemap screenshot](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/embedded-graphics/data/font8x16.png)](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/embedded-graphics/data/font8x16.png)
///
/// # Examples
///
/// See the [module-level documentation](./index.html) for examples.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Font8x16;

impl Font for Font8x16 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../data/font8x16_1bpp.raw");
    const FONT_IMAGE_WIDTH: u32 = 240;

    const CHARACTER_SIZE: Size = Size::new(8, 16);

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

    const WIDTH: usize = Font8x16::CHARACTER_SIZE.width as usize;
    const HEIGHT: usize = Font8x16::CHARACTER_SIZE.height as usize;
    const HELLO_WORLD: &'static str = "Hello World!";

    #[test]
    fn text_dimensions() {
        let style = TextStyle::new(Font8x16, BinaryColor::On);
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
        let style = TextStyle::new(Font8x16, BinaryColor::On);
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
            .into_styled(TextStyle::new(Font8x16, BinaryColor::On))
            .draw(&mut display);

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
        Text::new(" ~", Point::zero())
            .into_styled(TextStyle::new(Font8x16, BinaryColor::On))
            .draw(&mut display);

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
        Text::new("$y", Point::zero())
            .into_styled(TextStyle::new(Font8x16, BinaryColor::On))
            .draw(&mut display);

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
        Text::new("¡ÿ", Point::zero())
            .into_styled(TextStyle::new(Font8x16, BinaryColor::On))
            .draw(&mut display);

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

        let style = TextStyle::new(Font8x16, BinaryColor::On);

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
