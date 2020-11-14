use crate::{
    draw_target::DrawTarget, fonts::Text, pixelcolor::PixelColor, primitives::Rectangle, Pixel,
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

    /// Render a text object using this style.
    fn render_text<D>(&self, text: &Text<'_>, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>;

    /// Returns the bounding box of a text object rendered using this style.
    fn bounding_box(&self, text: &Text<'_>) -> Rectangle;
}

/// Pixels iterator.
pub trait TextStylePixels<'a>: TextStyle {
    /// Iterator type.
    type Iter: Iterator<Item = Pixel<Self::Color>> + 'a;

    /// Returns an iterator over the drawn pixels.
    fn pixels(&self, text: &Text<'a>) -> Self::Iter;
}
