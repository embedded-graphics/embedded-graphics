use crate::pixelcolor::PixelColor;

/// Style properties for texts.
///
/// `TextStyle` can be applied to a [font] to define how the font is drawn.
///
/// [font]: ../fonts/index.html
#[derive(Debug, Copy, Clone)]
pub struct TextStyle<C>
where
    C: PixelColor,
{
    /// Text color.
    pub text_color: Option<C>,

    /// Background color.
    pub background_color: Option<C>,
}

impl<C> TextStyle<C>
where
    C: PixelColor,
{
    /// Creates a font style with transparent text and background.
    pub fn new() -> Self {
        Self {
            text_color: None,
            background_color: None,
        }
    }

    /// Creates a font style with transparent background.
    pub fn with_text_color(text_color: C) -> Self {
        Self {
            text_color: Some(text_color),
            background_color: None,
        }
    }
}

impl<C> Default for TextStyle<C>
where
    C: PixelColor,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelcolor::{Rgb888, RgbColor};

    #[test]
    fn constructor() {
        let style = TextStyle::with_text_color(Rgb888::MAGENTA);
        assert_eq!(style.text_color, Some(Rgb888::MAGENTA));
        assert_eq!(style.background_color, None);
    }
}
