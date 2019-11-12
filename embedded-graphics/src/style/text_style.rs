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
    /// Creates a font style with transparent background.
    pub fn new(text_color: C) -> Self {
        Self {
            text_color: Some(text_color),
            background_color: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelcolor::{Rgb888, RgbColor};

    #[test]
    fn constructor() {
        let style = TextStyle::new(Rgb888::MAGENTA);
        assert_eq!(style.text_color, Some(Rgb888::MAGENTA));
        assert_eq!(style.background_color, None);
    }
}
