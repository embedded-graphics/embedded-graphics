use crate::{
    draw_target::DrawTarget, fonts::Text, pixelcolor::PixelColor, primitives::Rectangle, Pixel,
};

/// TODO: docs
pub trait TextStyle {
    /// TODO: docs
    type Color: PixelColor;

    /// TODO: docs
    fn render_text<D>(&self, text: &Text<'_>, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>;

    /// TODO: docs
    fn bounding_box(&self, text: &Text<'_>) -> Rectangle;
}

/// TODO: docs
pub trait TextStylePixels<'a>: TextStyle {
    /// TODO: docs
    type Iter: Iterator<Item = Pixel<Self::Color>> + 'a;

    /// TODO: docs
    fn pixels(&self, text: &Text<'a>) -> Self::Iter;
}
