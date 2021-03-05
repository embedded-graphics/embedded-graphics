use crate::pixelcolor::PixelColor;

/// Character style.
///
/// This trait is used to modify character styles programmatically, for example, to implement
/// rendering of text with multiple colors. Applications shouldn't use this trait and instead use
/// the character style types that are provided by the text renderer, like `MonoTextStyle` and
/// `MonoTextStyleBuilder` for the integrated font support.
///
///  # Implementation notes
///
/// Text renderers don't need to support all settings in this trait. All calls to unsupported
/// setters should be ignored by the implementation. The trait provided empty default
/// implementations for all setters.
pub trait CharacterStyle: Clone {
    /// The color type.
    type Color: PixelColor;

    /// Sets the text color.
    fn set_text_color(&mut self, _text_color: Option<Self::Color>) {}

    /// Sets the background color.
    fn set_background_color(&mut self, _background_color: Option<Self::Color>) {}

    /// Sets the underline color.
    fn set_underline_color(&mut self, _underline_color: DecorationColor<Self::Color>) {}

    /// Sets the strikethrough color.
    fn set_strikethrough_color(&mut self, _strikethrough_color: DecorationColor<Self::Color>) {}
}

/// Text decoration color.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum DecorationColor<C> {
    /// No text decoration.
    None,
    /// Text decoration with the same color as the text.
    TextColor,
    /// Text decoration with a custom color.
    Custom(C),
}

impl<C> DecorationColor<C> {
    /// Returns `true` if the decoration_color is `None`.
    pub fn is_none(&self) -> bool {
        // MSRV: replace with matches! for rust >= 1.42.0
        match self {
            Self::None => true,
            _ => false,
        }
    }

    /// Returns `true` if the decoration_color is `TextColor`.
    pub fn is_text_color(&self) -> bool {
        // MSRV: replace with matches! for rust >= 1.42.0
        match self {
            Self::TextColor => true,
            _ => false,
        }
    }

    /// Returns `true` if the decoration_color is `Custom`.
    pub fn is_custom(&self) -> bool {
        // MSRV: replace with matches! for rust >= 1.42.0
        match self {
            Self::Custom(_) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelcolor::BinaryColor;

    #[test]
    fn decoration_color_is_methods() {
        let none = DecorationColor::<BinaryColor>::None;
        assert!(none.is_none());
        assert!(!none.is_text_color());
        assert!(!none.is_custom());

        let text_color = DecorationColor::<BinaryColor>::TextColor;
        assert!(!text_color.is_none());
        assert!(text_color.is_text_color());
        assert!(!text_color.is_custom());

        let custom = DecorationColor::Custom(BinaryColor::On);
        assert!(!custom.is_none());
        assert!(!custom.is_text_color());
        assert!(custom.is_custom());
    }
}
