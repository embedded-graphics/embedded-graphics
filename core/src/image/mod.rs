//! Image drawable.

use crate::{
    draw_target::DrawTarget,
    geometry::{Point, Size},
    pixelcolor::PixelColor,
    primitives::Rectangle,
};

/// Image drawable.
///
/// `ImageDrawable` is implemented for types that contains image information, which makes them
/// usable with the [`Image`] object.
///
/// The methods in this trait shouldn't be called directly by user code. Instead the object
/// that implements `ImageDrawable` should be wrapped in an [`Image`] object.
///
/// [`Image`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/image/struct.Image.html
pub trait ImageDrawable {
    /// The color type.
    type Color: PixelColor;

    /// Returns the size of the image.
    fn size(&self) -> Size;

    /// Draws the entire image to the target.
    ///
    /// This method shouldn't be called directly by user code. Use an [`Image`] object instead.
    ///
    /// # Implementation notes
    ///
    /// The implementation of this method must draw the image with the top left corner in the origin
    /// of the draw target. No drawing operations outside the rectangular area with the size returned
    /// by the [`size`] method are allowed.
    ///
    /// [`Image`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/image/struct.Image.html
    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>;

    /// Draws a part of the image to the target.
    ///
    /// This method shouldn't be called directly by user code. Use a [`SubImage`] object instead.
    ///
    /// # Implementation notes
    ///
    /// The implementation of this method must draw the image inside the given `area`.
    /// It must be ensured that no drawing operation outside this [`Rectangle`] occur.
    ///
    /// [`SubImage`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/image/struct.SubImage.html
    /// [`Rectangle`]: crate::primitives::rectangle::Rectangle
    fn draw_sub_image<D>(&self, target: &mut D, area: &Rectangle) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>;
}

/// Pixel getter.
pub trait GetPixel {
    /// The color type.
    type Color: PixelColor;

    /// Gets the color of a pixel.
    ///
    /// Returns `None` if `p` is outside the bounding box.
    fn pixel(&self, p: Point) -> Option<Self::Color>;
}
