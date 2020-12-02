use crate::{
    draw_target::DrawTarget, geometry::Point, pixelcolor::PixelColor, primitives::Rectangle,
};

/// Text style.
///
/// The `TextStyle` trait is used to integrate text renderers into embedded-graphics. Users should
/// not call it directly and instead use the functions provided by the [`Text`] type.
///
/// [`Text`]: ../fonts/struct.Text.html
pub trait TextStyle {
    /// Color type.
    type Color: PixelColor;

    /// Renders a single line of text using this style.
    ///
    /// Returns the offset from the current position to the start of the next line.
    ///
    /// TODO: document how `position` should be used
    fn render_line<D>(
        &self,
        text: &str,
        position: Point,
        target: &mut D,
    ) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = Self::Color>;

    /// Returns the bounding box of a single line of text rendered using this style.
    ///
    /// TODO: document how `position` should be used and the second return value
    fn line_bounding_box(&self, text: &str, position: Point) -> (Rectangle, Point);
}
