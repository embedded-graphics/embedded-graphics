//! Image drawable.

use crate::{
    draw_target::DrawTarget, geometry::OriginDimensions, pixelcolor::PixelColor,
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
/// # Implementing `ImageDrawable`
///
/// All image drawables are positioned at the origin and need to implement [`OriginDimensions`], in
/// addition to this trait, to define their dimensions.
///
/// [`Image`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/image/struct.Image.html
/// [`OriginDimensions`]: ../geometry/trait.OriginDimensions.html
pub trait ImageDrawable: OriginDimensions {
    /// The color type.
    type Color: PixelColor;

    /// Draws the entire image to the target.
    ///
    /// This method shouldn't be called directly by user code. Use an [`Image`] object instead.
    ///
    /// # Implementation notes
    ///
    /// The implementation of this method must draw the image inside the bounding box defined by
    /// the [`OriginDimensions`] trait implementation. This means that the top left corner is at the
    /// origin and no drawing operations outside the bounding box are allowed.
    ///
    /// [`Image`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/image/struct.Image.html
    /// [`OriginDimensions`]: ../geometry/trait.OriginDimensions.html
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
    /// [`Rectangle`]: ../primitives/rectangle/struct.Rectangle.html
    fn draw_sub_image<D>(&self, target: &mut D, area: &Rectangle) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>;
}
