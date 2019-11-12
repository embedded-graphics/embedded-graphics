use crate::fonts::font_builder::{FontBuilder, FontBuilderConf};

#[derive(Debug, Copy, Clone)]
/// Config for 24x32 font
pub enum Font24x32Conf {}
impl FontBuilderConf for Font24x32Conf {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../data/font24x32_1bpp.raw");
    const CHAR_HEIGHT: u32 = 32;
    const CHAR_WIDTH: u32 = 24;
    const FONT_IMAGE_WIDTH: u32 = 960;
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

/// 24x32 pixel monospace font
///
/// There is also the [`text_24x32`] macro to provide an easier to use interface.
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
/// [`text_24x32`]: ../macro.text_24x32.html
pub type Font24x32<'a, C> = FontBuilder<'a, C, Font24x32Conf>;

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

    const WIDTH: usize = Font24x32Conf::CHAR_WIDTH as usize;
    const HEIGHT: usize = Font24x32Conf::CHAR_HEIGHT as usize;
    const HELLO_WORLD: &'static str = "Hello World!";

    #[test]
    fn text_dimensions() {
        let style = TextStyle::new(BinaryColor::On);
        let hello = Font24x32::render_str(HELLO_WORLD, style);
        let empty = Font24x32::render_str("", style);

        assert_eq!(
            hello.size(),
            Size::new((HELLO_WORLD.len() * WIDTH) as u32, HEIGHT as u32)
        );
        assert_eq!(empty.size(), Size::new(0, 0));
    }

    #[test]
    fn text_corners() {
        let style = TextStyle::new(BinaryColor::On);
        let hello = Font24x32::render_str(HELLO_WORLD, style).translate(Point::new(5, -20));
        let empty = Font24x32::render_str("", style).translate(Point::new(10, 20));

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
        Font24x32::render_str("Mm", TextStyle::new(BinaryColor::On)).draw(&mut display);

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
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
            ])
        );
    }

    #[test]
    fn correct_ascii_borders() {
        let mut display = MockDisplay::new();
        Font24x32::render_str(" ~", TextStyle::new(BinaryColor::On)).draw(&mut display);

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "                            ########    #### ",
                "                            ########    #### ",
                "                            ########    #### ",
                "                            ########    #### ",
                "                        ####        ####     ",
                "                        ####        ####     ",
                "                        ####        ####     ",
                "                        ####        ####     ",
            ])
        );
    }

    #[test]
    fn correct_dollar_y() {
        let mut display = MockDisplay::new();
        Font24x32::render_str("$y", TextStyle::new(BinaryColor::On)).draw(&mut display);

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
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
            ])
        );
    }

    #[test]
    fn dont_panic() {
        let two_question_marks = MockDisplay::from_pattern(&[
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
        ]);

        let style = TextStyle::new(BinaryColor::On);

        let mut display = MockDisplay::new();
        Font24x32::render_str("\0\n", style).draw(&mut display);
        assert_eq!(display, two_question_marks);

        let mut display = MockDisplay::new();
        Font24x32::render_str("\x7F\u{A0}", style).draw(&mut display);
        assert_eq!(display, two_question_marks);

        let mut display = MockDisplay::new();
        Font24x32::render_str("¡ÿ", style).draw(&mut display);
        assert_eq!(display, two_question_marks);

        let mut display = MockDisplay::new();
        Font24x32::render_str("Ā💣", style).draw(&mut display);
        assert_eq!(display, two_question_marks);
    }
}
