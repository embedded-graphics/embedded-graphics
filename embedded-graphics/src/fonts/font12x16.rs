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
        drawable::Drawable,
        fonts::{Font, Text},
        geometry::{Dimensions, Point, Size},
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        primitives::Rectangle,
        style::TextStyle,
        transform::Transform,
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
    fn text_corners() {
        let style = TextStyle::new(Font12x16, BinaryColor::On);
        let hello = Text::new(HELLO_WORLD, Point::zero())
            .into_styled(style)
            .translate(Point::new(5, -20));
        let empty = Text::new("", Point::zero())
            .into_styled(style)
            .translate(Point::new(10, 20));

        assert_eq!(
            hello.bounding_box(),
            Rectangle::new(
                Point::new(5, -20),
                Size::new((HELLO_WORLD.len() * WIDTH) as u32, HEIGHT as u32)
            )
        );
        assert_eq!(
            empty.bounding_box(),
            Rectangle::new(Point::new(10, 20), Size::zero())
        );
    }

    #[test]
    fn correct_m() -> Result<(), core::convert::Infallible> {
        let mut display = MockDisplay::new();
        Text::new("Mm", Point::zero())
            .into_styled(TextStyle::new(Font12x16, BinaryColor::On))
            .draw(&mut display)?;

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

        Ok(())
    }

    #[test]
    fn correct_ascii_borders() -> Result<(), core::convert::Infallible> {
        let mut display = MockDisplay::new();
        Text::new(" ~", Point::zero())
            .into_styled(TextStyle::new(Font12x16, BinaryColor::On))
            .draw(&mut display)?;

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

        Ok(())
    }

    #[test]
    fn correct_dollar_y() -> Result<(), core::convert::Infallible> {
        let mut display = MockDisplay::new();
        Text::new("$y", Point::zero())
            .into_styled(TextStyle::new(Font12x16, BinaryColor::On))
            .draw(&mut display)?;

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

        Ok(())
    }

    #[test]
    fn correct_latin1() -> Result<(), core::convert::Infallible> {
        let mut display = MockDisplay::new();
        Text::new("¡ÿ", Point::zero())
            .into_styled(TextStyle::new(Font12x16, BinaryColor::On))
            .draw(&mut display)?;

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
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
            ])
        );

        Ok(())
    }

    #[test]
    fn dont_panic() -> Result<(), core::convert::Infallible> {
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

        let style = TextStyle::new(Font12x16, BinaryColor::On);

        let mut display = MockDisplay::new();
        Text::new("\0\r", Point::zero())
            .into_styled(style)
            .draw(&mut display)?;
        assert_eq!(display, two_question_marks);

        let mut display = MockDisplay::new();
        Text::new("\x7F\u{A0}", Point::zero())
            .into_styled(style)
            .draw(&mut display)?;
        assert_eq!(display, two_question_marks);

        let mut display = MockDisplay::new();
        Text::new("Ā💣", Point::zero())
            .into_styled(style)
            .draw(&mut display)?;
        assert_eq!(display, two_question_marks);

        Ok(())
    }
}
