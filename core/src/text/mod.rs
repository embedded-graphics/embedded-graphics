//! Text.

use crate::{
    draw_target::DrawTarget, geometry::Point, pixelcolor::PixelColor, primitives::Rectangle,
};

/// Text renderer.
///
/// The `TextRenderer` trait is used to integrate text renderers into embedded-graphics. Users should
/// not call it directly and instead use the functions provided by the `Text` type.
pub trait TextRenderer {
    /// Color type.
    type Color: PixelColor;

    /// Draws a string.
    ///
    /// The interpretation of the y coordinate of `position` is dependent on the implementation and
    /// can, for example, be the top edge of the bounding box or a point on the baseline. The
    /// caller must ensure that the coordinate is first converted with the `vertical_offset` method.
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
        target: &mut D,
    ) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = Self::Color>;

    /// Draws whitespace of the given width.
    ///
    /// The interpretation of the y coordinate of `position` is dependent on the implementation and
    /// can, for example, be the top edge of the bounding box or a point on the baseline. The
    /// caller must ensure that the coordinate is first converted with the `vertical_offset` method.
    ///
    /// The method returns the start position of the next character to allow chaining of multiple
    /// draw calls.
    fn draw_whitespace<D>(
        &self,
        width: u32,
        position: Point,
        target: &mut D,
    ) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = Self::Color>;

    /// Returns the text metrics for a string.
    ///
    /// The interpretation of the y coordinate of `position` is dependent on the implementation and
    /// can, for example, be the top edge of the bounding box or a point on the baseline. The
    /// caller must ensure that the coordinate is first converted with the `vertical_offset` method.
    ///
    /// The returned bounding box is zero sized, if the text is completely transparent.
    ///
    /// # Implementation notes
    ///
    /// This method must not interpret any control characters and only render a single line of text.
    /// Any control character in the `text` should be handled the same way as any other character
    /// that isn't included in the font.
    fn measure_string(&self, text: &str, position: Point) -> TextMetrics;

    /// Offsets the point to apply the vertical alignment.
    fn vertical_offset(&self, position: Point, vertical_alignment: VerticalAlignment) -> Point;

    /// Returns the line height.
    ///
    /// The line height is defined as the vertical distance between the baseline of two adjacent
    /// lines in pixels.
    fn line_height(&self) -> u32;
}

/// Text metrics.
///
/// See [`TextRenderer::measure_text`] for more information.
///
/// [`TextRenderer::measure_text`]: trait.TextRenderer.html#tymethod.measure_text
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TextMetrics {
    /// Bounding box.
    pub bounding_box: Rectangle,

    /// The position of the next text.
    pub next_position: Point,
}

/// Vertical text alignment.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum VerticalAlignment {
    /// Top.
    Top,
    /// Bottom.
    Bottom,
    /// Center.
    Center,
    /// Baseline.
    Baseline,
}

/// Horizontal text alignment.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum HorizontalAlignment {
    /// Left.
    Left,
    /// Center.
    Center,
    /// Right.
    Right,
}
