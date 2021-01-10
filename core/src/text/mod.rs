//! Text.

use crate::{
    draw_target::DrawTarget, geometry::Point, pixelcolor::PixelColor, primitives::Rectangle,
};

/// Text renderer.
///
/// The `TextStyle` trait is used to integrate text renderers into embedded-graphics. Users should
/// not call it directly and instead use the functions provided by the `Text` type.
pub trait TextRenderer {
    /// Color type.
    type Color: PixelColor;

    /// Draws a string.
    ///
    /// The text should not contain any control characters. Implementations of this trait need to
    /// ignore all control characters. The method returns the point at which the next character
    /// in the same row starts.
    fn draw_string<D>(
        &self,
        text: &str,
        position: Point,
        vertical_alignment: VerticalAlignment,
        target: &mut D,
    ) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = Self::Color>;

    /// Draws whitespace of the given width.
    ///
    /// The method returns the point at which the next character in the same row starts.
    fn draw_whitespace<D>(
        &self,
        width: u32,
        position: Point,
        vertical_alignment: VerticalAlignment,
        target: &mut D,
    ) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = Self::Color>;

    /// Returns the width of the text in pixels.
    fn string_width(&self, text: &str) -> u32;

    /// Returns the bounding box of a string.
    ///
    /// The text should not contain any control characters. Implementations of this trait need to
    /// ignore all control characters. The method returns the bounding box and the point at which
    /// the next character in the same row starts.
    fn string_bounding_box(
        &self,
        text: &str,
        position: Point,
        vertical_alignment: VerticalAlignment,
    ) -> (Rectangle, Point);

    /// Returns the line height.
    fn line_height(&self) -> u32;
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
