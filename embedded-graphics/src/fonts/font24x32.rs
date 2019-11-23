use crate::fonts::Font;

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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Font24x32 {}

const FONT24X32_OBJECT: Font24x32 = Font24x32 {};

/// Font 24x32
pub const FONT24X32: &Font24x32 = &FONT24X32_OBJECT;

impl Font for Font24x32 {
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

    const WIDTH: usize = Font24x32::CHAR_WIDTH as usize;
    const HEIGHT: usize = Font24x32::CHAR_HEIGHT as usize;
    const HELLO_WORLD: &'static str = "Hello World!";

    #[test]
    fn text_dimensions() {
        let style = TextStyle::with_text_color(FONT24X32, BinaryColor::On);
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
        let style = TextStyle::with_text_color(FONT24X32, BinaryColor::On);
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
            .into_styled(TextStyle::with_text_color(FONT24X32, BinaryColor::On))
            .draw(&mut display);

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
        Text::new(" ~", Point::zero())
            .into_styled(TextStyle::with_text_color(FONT24X32, BinaryColor::On))
            .draw(&mut display);

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
        Text::new("$y", Point::zero())
            .into_styled(TextStyle::with_text_color(FONT24X32, BinaryColor::On))
            .draw(&mut display);

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

        let style = TextStyle::with_text_color(FONT24X32, BinaryColor::On);

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
        Text::new("¡ÿ", Point::zero())
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
