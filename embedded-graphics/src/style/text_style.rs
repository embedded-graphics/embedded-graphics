use crate::fonts::Font;
use crate::pixelcolor::PixelColor;

/// Style properties for texts.
///
/// `TextStyle` can be applied to a [font] to define how the font is drawn.
///
/// [font]: ../fonts/index.html
#[derive(Debug, Copy, Clone)]
pub struct TextStyle<C, F>
where
    C: PixelColor,
    F: Font,
{
    /// Text color.
    pub text_color: Option<C>,

    /// Background color.
    pub background_color: Option<C>,

    /// Font,
    pub font: F,
}

impl<C, F> TextStyle<C, F>
where
    C: PixelColor,
    F: Font,
{
    // TODO: Set default value for `font`
    // /// Creates a font style with transparent text and background.
    // pub fn new() -> Self {
    //     Self {
    //         text_color: None,
    //         background_color: None,
    //     }
    // }

    /// Creates a font style with transparent background.
    pub fn with_text_color(font: F, text_color: C) -> Self {
        Self {
            font,
            text_color: Some(text_color),
            background_color: None,
        }
    }
}

// TODO: uncomment when `TextStyle::new` is implemented
//
// impl<C> Default for TextStyle<C>
// where
//     C: PixelColor,
// {
//     fn default() -> Self {
//         Self::new()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fonts::Font6x8;
    use crate::pixelcolor::{Rgb888, RgbColor};

    #[test]
    fn constructor() {
        let style = TextStyle::with_text_color(Font6x8, Rgb888::MAGENTA);
        assert_eq!(style.font, Font6x8);
        assert_eq!(style.text_color, Some(Rgb888::MAGENTA));
        assert_eq!(style.background_color, None);
    }
}
