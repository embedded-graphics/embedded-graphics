use crate::{fonts::Font, geometry::Size};

/// 12x16 pixel monospace font.
///
/// [![12x16 font spritemap screenshot](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/embedded-graphics/data/font12x16.png)](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/embedded-graphics/data/font12x16.png)
///
/// # Examples
///
/// See the [module-level documentation](./index.html) for examples.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font12x16;

/// Config for 12x16 font
impl Font for Font12x16 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../data/font12x16_1bpp.raw");
    const FONT_IMAGE_WIDTH: u32 = 480;

    const CHARACTER_SIZE: Size = Size::new(12, 16);

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
        fonts::{tests::assert_text_from_pattern, Font, Text},
        geometry::{Dimensions, Point, Size},
        pixelcolor::BinaryColor,
        style::TextStyle,
    };

    const WIDTH: usize = Font12x16::CHARACTER_SIZE.width as usize;
    const HEIGHT: usize = Font12x16::CHARACTER_SIZE.height as usize;
    const HELLO_WORLD: &'static str = "Hello World!";

    #[test]
    fn text_dimensions() {
        let style = TextStyle::new(Font12x16, BinaryColor::On);
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
            Font12x16,
            &[
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
            ],
        );
    }

    #[test]
    fn correct_ascii_borders() {
        assert_text_from_pattern(
            " ~",
            Font12x16,
            &[
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
            ],
        );
    }

    #[test]
    fn correct_dollar_y() {
        assert_text_from_pattern(
            "$y",
            Font12x16,
            &[
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
            ],
        );
    }

    #[test]
    fn correct_latin1() {
        assert_text_from_pattern(
            "¡ÿ",
            Font12x16,
            &[
                "    ##        ##  ##    ",
                "    ##        ##  ##    ",
                "                        ",
                "                        ",
                "    ##      ##      ##  ",
                "    ##      ##      ##  ",
                "    ##      ##      ##  ",
                "    ##      ##      ##  ",
                "    ##      ##      ##  ",
                "    ##      ##      ##  ",
                "    ##        ########  ",
                "    ##        ########  ",
                "    ##              ##  ",
                "    ##              ##  ",
                "              ######    ",
                "              ######    ",
            ],
        );
    }

    #[test]
    fn dont_panic() {
        let two_question_marks = &[
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
        ];

        assert_text_from_pattern("\0\r", Font12x16, two_question_marks);
        assert_text_from_pattern("\x7F\u{A0}", Font12x16, two_question_marks);
        assert_text_from_pattern("Ā💣", Font12x16, two_question_marks);
    }
}
