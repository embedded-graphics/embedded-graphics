use crate::{fonts::MonoFont, geometry::Size};

/// 6x8 pixel monospace font.
///
/// [![6x8 font spritemap screenshot](https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/master/data/font6x8.png)](https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/master/data/font6x8.png)
///
/// # Examples
///
/// See the [module-level documentation](./index.html) for examples.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font6x8;

impl MonoFont for Font6x8 {
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
    use crate::{
        fonts::{tests::assert_text_from_pattern, MonoFont, Text},
        geometry::{Dimensions, Point, Size},
        pixelcolor::BinaryColor,
        style::MonoTextStyle,
    };

    const WIDTH: usize = Font6x8::CHARACTER_SIZE.width as usize;
    const HEIGHT: usize = Font6x8::CHARACTER_SIZE.height as usize;
    const HELLO_WORLD: &'static str = "Hello World!";

    #[test]
    fn text_dimensions() {
        let style = MonoTextStyle::new(Font6x8, BinaryColor::On);
        let hello = Text::new(HELLO_WORLD, Point::zero()).into_styled(style);
        let empty = Text::new("", Point::zero()).into_styled(style);

        assert_eq!(
            hello.bounding_box().size,
            Size::new((HELLO_WORLD.len() * WIDTH) as u32, HEIGHT as u32)
        );
        assert_eq!(empty.bounding_box().size, Size::zero());
    }

    #[test]
    fn correct_m() {
        assert_text_from_pattern(
            "Mm",
            Font6x8,
            &[
                "#   #       ",
                "## ##       ",
                "# # # ## #  ",
                "# # # # # # ",
                "#   # #   # ",
                "#   # #   # ",
                "#   # #   # ",
                "            ",
            ],
        );
    }

    #[test]
    fn correct_ascii_borders() {
        assert_text_from_pattern(
            " ~",
            Font6x8,
            &[
                "       ## # ",
                "      #  #  ",
                "            ",
                "            ",
                "            ",
                "            ",
                "            ",
                "            ",
            ],
        );
    }

    #[test]
    fn correct_dollar_y() {
        assert_text_from_pattern(
            "$y",
            Font6x8,
            &[
                "  #         ",
                " ####       ",
                "# #   #   # ",
                " ###  #   # ",
                "  # # #   # ",
                "####   #### ",
                "  #       # ",
                "       ###  ",
            ],
        );
    }

    #[test]
    fn correct_latin1() {
        assert_text_from_pattern(
            "¡ÿ",
            Font6x8,
            &[
                "  #    # #  ",
                "            ",
                "  #   #   # ",
                "  #   #   # ",
                "  #   #   # ",
                "  #    #### ",
                "  #       # ",
                "       ###  ",
                "            ",
            ],
        );
    }

    #[test]
    fn dont_panic() {
        let two_question_marks = &[
            " ###   ### ",
            "#   # #   #",
            "    #     #",
            "   #     # ",
            "  #     #  ",
            "           ",
            "  #     #  ",
        ];

        assert_text_from_pattern("\0\r", Font6x8, two_question_marks);
        assert_text_from_pattern("\x7F\u{A0}", Font6x8, two_question_marks);
        assert_text_from_pattern("Ā💣", Font6x8, two_question_marks);
    }
}
