use crate::{fonts::Font, geometry::Size};

/// 24x32 pixel monospace font.
///
/// The 24x32 font is a 2x scaling of the [12x16 font].
///
/// The 12x16 font sprite is shown here for reference:
///
/// [![12x16 font spritemap screenshot](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/embedded-graphics/data/font12x16.png)](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/embedded-graphics/data/font12x16.png)
///
/// # Examples
///
/// See the [module-level documentation](./index.html) for examples.
///
/// [12x16 font]: struct.Font12x16.html
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font24x32;

impl Font for Font24x32 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../data/font24x32_1bpp.raw");
    const FONT_IMAGE_WIDTH: u32 = 960;

    const CHARACTER_SIZE: Size = Size::new(24, 32);

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

    const WIDTH: usize = Font24x32::CHARACTER_SIZE.width as usize;
    const HEIGHT: usize = Font24x32::CHARACTER_SIZE.height as usize;
    const HELLO_WORLD: &'static str = "Hello World!";

    #[test]
    fn text_dimensions() {
        let style = TextStyle::new(Font24x32, BinaryColor::On);
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
            Font24x32,
            &[
                "####            ####                          ",
                "####            ####                          ",
                "####            ####                          ",
                "####            ####                          ",
                "########    ########                          ",
                "########    ########                          ",
                "########    ########                          ",
                "########    ########                          ",
                "####    ####    ####    ########    ####      ",
                "####    ####    ####    ########    ####      ",
                "####    ####    ####    ########    ####      ",
                "####    ####    ####    ########    ####      ",
                "####    ####    ####    ####    ####    ####  ",
                "####    ####    ####    ####    ####    ####  ",
                "####    ####    ####    ####    ####    ####  ",
                "####    ####    ####    ####    ####    ####  ",
                "####            ####    ####            ####  ",
                "####            ####    ####            ####  ",
                "####            ####    ####            ####  ",
                "####            ####    ####            ####  ",
                "####            ####    ####            ####  ",
                "####            ####    ####            ####  ",
                "####            ####    ####            ####  ",
                "####            ####    ####            ####  ",
                "####            ####    ####            ####  ",
                "####            ####    ####            ####  ",
                "####            ####    ####            ####  ",
                "####            ####    ####            ####  ",
            ],
        );
    }

    #[test]
    fn correct_ascii_borders() {
        assert_text_from_pattern(
            " ~",
            Font24x32,
            &[
                "                            ########    #### ",
                "                            ########    #### ",
                "                            ########    #### ",
                "                            ########    #### ",
                "                        ####        ####     ",
                "                        ####        ####     ",
                "                        ####        ####     ",
                "                        ####        ####     ",
            ],
        );
    }

    #[test]
    fn correct_dollar_y() {
        assert_text_from_pattern(
            "$y",
            Font24x32,
            &[
                "        ####                                 ",
                "        ####                                 ",
                "        ####                                 ",
                "        ####                                 ",
                "    ################                         ",
                "    ################                         ",
                "    ################                         ",
                "    ################                         ",
                "####    ####            ####            #### ",
                "####    ####            ####            #### ",
                "####    ####            ####            #### ",
                "####    ####            ####            #### ",
                "    ############        ####            #### ",
                "    ############        ####            #### ",
                "    ############        ####            #### ",
                "    ############        ####            #### ",
                "        ####    ####    ####            #### ",
                "        ####    ####    ####            #### ",
                "        ####    ####    ####            #### ",
                "        ####    ####    ####            #### ",
                "################            ################ ",
                "################            ################ ",
                "################            ################ ",
                "################            ################ ",
                "        ####                            #### ",
                "        ####                            #### ",
                "        ####                            #### ",
                "        ####                            #### ",
                "                            ############     ",
                "                            ############     ",
                "                            ############     ",
                "                            ############     ",
            ],
        );
    }

    #[test]
    fn correct_latin1() {
        assert_text_from_pattern(
            "¡ÿ",
            Font24x32,
            &[
                "        ####                ####    ####         ",
                "        ####                ####    ####         ",
                "        ####                ####    ####         ",
                "        ####                ####    ####         ",
                "                                                 ",
                "                                                 ",
                "                                                 ",
                "                                                 ",
                "        ####            ####            ####     ",
                "        ####            ####            ####     ",
                "        ####            ####            ####     ",
                "        ####            ####            ####     ",
                "        ####            ####            ####     ",
                "        ####            ####            ####     ",
                "        ####            ####            ####     ",
                "        ####            ####            ####     ",
                "        ####            ####            ####     ",
                "        ####            ####            ####     ",
                "        ####            ####            ####     ",
                "        ####            ####            ####     ",
                "        ####                ################     ",
                "        ####                ################     ",
                "        ####                ################     ",
                "        ####                ################     ",
                "        ####                            ####     ",
                "        ####                            ####     ",
                "        ####                            ####     ",
                "        ####                            ####     ",
                "                            ############         ",
                "                            ############         ",
                "                            ############         ",
                "                            ############         ",
            ],
        );
    }

    #[test]
    fn dont_panic() {
        let two_question_marks = &[
            "    ############            ############     ",
            "    ############            ############     ",
            "    ############            ############     ",
            "    ############            ############     ",
            "####            ####    ####            #### ",
            "####            ####    ####            #### ",
            "####            ####    ####            #### ",
            "####            ####    ####            #### ",
            "                ####                    #### ",
            "                ####                    #### ",
            "                ####                    #### ",
            "                ####                    #### ",
            "            ####                    ####     ",
            "            ####                    ####     ",
            "            ####                    ####     ",
            "            ####                    ####     ",
            "        ####                    ####         ",
            "        ####                    ####         ",
            "        ####                    ####         ",
            "        ####                    ####         ",
            "                                             ",
            "                                             ",
            "                                             ",
            "                                             ",
            "        ####                    ####         ",
            "        ####                    ####         ",
            "        ####                    ####         ",
            "        ####                    ####         ",
        ];

        assert_text_from_pattern("\0\r", Font24x32, two_question_marks);
        assert_text_from_pattern("\x7F\u{A0}", Font24x32, two_question_marks);
        assert_text_from_pattern("Ā💣", Font24x32, two_question_marks);
    }
}
