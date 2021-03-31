//! Text renderer.
//!
//! TODO: Describe how this API can be implemented and add an example to the docs or link to an
//!       external example.

use crate::{draw_target::DrawTarget, geometry::Point, pixelcolor::PixelColor, text::Baseline};

mod character_style;

pub use character_style::{CharacterStyle, ModifyCharacterStyle, TextMetrics};

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
}

/// Target specific text renderer.
///
/// This trait is a target specific version of the [`TextRenderer`] trait.
/// 
/// [`TextRenderer`]: trait.TextRenderer.html
pub trait TargetSpecificTextRenderer<D: DrawTarget> {
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
    fn draw_string(
        &self,
        text: &str,
        position: Point,
        baseline: Baseline,
        target: &mut D,
    ) -> Result<Point, D::Error>;

    /// Draws whitespace of the given width.
    ///
    /// The method returns the start position of the next character to allow chaining of multiple
    /// draw calls.
    fn draw_whitespace(
        &self,
        width: u32,
        position: Point,
        baseline: Baseline,
        target: &mut D,
    ) -> Result<Point, D::Error>;
}
