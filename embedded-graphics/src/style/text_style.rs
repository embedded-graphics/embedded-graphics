use crate::fonts::Font;
use crate::pixelcolor::PixelColor;

/// Style properties for texts.
///
/// `TextStyle` can be applied to a [`Text`] object to define how a text is drawn.
///
/// [`Text`]: ../fonts/struct.Text.html
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TextStyle<C, F>
where
    C: PixelColor,
    F: Font,
{
    /// Text color.
    pub text_color: Option<C>,

    /// Background color.
    pub background_color: Option<C>,

    /// Font.
    pub font: F,
}

impl<C, F> TextStyle<C, F>
where
    C: PixelColor,
    F: Font,
{
    /// Creates a text style with transparent background.
    pub fn new(font: F, text_color: C) -> Self {
        Self {
            font,
            text_color: Some(text_color),
            background_color: None,
        }
    }

    /// Create a text style with only the font defined
    pub fn with_font(font: F) -> Self {
        Self {
            font,
            text_color: None,
            background_color: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fonts::Font6x8;
    use crate::pixelcolor::{Rgb888, RgbColor};

    #[test]
    fn constructor() {
        let style = TextStyle::new(Font6x8, Rgb888::MAGENTA);
        assert_eq!(style.font, Font6x8);
        assert_eq!(style.text_color, Some(Rgb888::MAGENTA));
        assert_eq!(style.background_color, None);
    }
}
