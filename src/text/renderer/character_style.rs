use crate::{pixelcolor::PixelColor, text::DecorationColor};

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
