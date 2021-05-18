//! Text renderer.
//!
//! For more complex font rendering cases that are not covered by the [`mono_font`] module, the
//! `TextRenderer` trait can be implemented for a font.
//!
//! An implementation can be found in the [embedded-bdf] repository and may be useful as a reference
//! for other implementations.
//!
//! [`mono_font`]: ../../mono_font/index.html [embedded-bdf]:
//! https://github.com/embedded-graphics/bdf/tree/a73a34cf45a5ef90cc7441afc12ec611cfe15563/eg-bdf

use crate::{
    draw_target::DrawTarget, geometry::Point, pixelcolor::PixelColor, primitives::Rectangle,
    text::Baseline,
};

mod character_style;

pub use character_style::CharacterStyle;

/// Text renderer.
///
/// The `TextRenderer` trait is used to integrate text renderers into embedded-graphics. Users should
/// not call it directly and instead use the functions provided by the `Text` type.
pub trait TextRenderer {
    /// Color type.
    type Color: PixelColor;

    /// Draws a string.
    ///
    /// The method returns the start position of the next character to allow chaining of multiple
    /// draw calls.
    ///
    /// # Implementation notes
    ///
    /// This method must not interpret any control characters and only render a single line of text.
    /// Any control character in the `text` should be handled the same way as any other character
    /// that isn't included in the font.
    fn draw_string<D>(
        &self,
        text: &str,
        position: Point,
        baseline: Baseline,
        target: &mut D,
    ) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = Self::Color>;

    /// Draws whitespace of the given width.
    ///
    /// The method returns the start position of the next character to allow chaining of multiple
    /// draw calls.
    fn draw_whitespace<D>(
        &self,
        width: u32,
        position: Point,
        baseline: Baseline,
        target: &mut D,
    ) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = Self::Color>;

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
