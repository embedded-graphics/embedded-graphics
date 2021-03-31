use crate::{
    geometry::Point,
    pixelcolor::PixelColor,
    primitives::Rectangle,
    text::{Baseline, DecorationColor},
};

/// TODO: docs
pub trait CharacterStyle: Clone {
    /// Returns the text metrics for a string.
    ///
    /// # Implementation notes
    ///
    /// The returned bounding box must be independent of the text color. This is different to the
    /// `Dimensions` trait, which should return a zero sized bounding box for completely transparent
    /// drawables. But this behavior would make it impossible to correctly layout text which
    /// contains a mixture of transparent and non transparent words.
    ///
    /// This method must not interpret any control characters and only render a single line of text.
    /// Any control character in the `text` should be handled the same way as any other character
    /// that isn't included in the font.
    fn measure_string(&self, text: &str, position: Point, baseline: Baseline) -> TextMetrics;

    /// Returns the default line height.
    ///
    /// The line height is defined as the vertical distance between the baseline of two adjacent
    /// lines in pixels.
    fn line_height(&self) -> u32;
}

/// Modify character style.
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
pub trait ModifyCharacterStyle: CharacterStyle {
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

/// Text metrics.
///
/// See [`TextRenderer::measure_string`] for more information.
///
/// [`TextRenderer::measure_string`]: trait.TextRenderer.html#tymethod.measure_string
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TextMetrics {
    /// Bounding box.
    pub bounding_box: Rectangle,

    /// The position of the next text.
    pub next_position: Point,
}
