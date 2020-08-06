use super::SubImage;
use crate::{
    draw_target::DrawTarget, draw_target::DrawTargetExt, geometry::OriginDimensions,
    pixelcolor::PixelColor, primitives::Rectangle,
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
/// [`Image`]: struct.Image.html
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
    /// [`Image`]: struct.Image.html
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
    /// [`SubImage`]: struct.SubImage.html
    /// [`Rectangle`]: ../primitives/rectangle/struct.Rectangle.html
    fn draw_sub_image<D>(&self, target: &mut D, area: &Rectangle) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        self.draw(&mut target.translated(-area.top_left).clipped(area))
    }
}

/// Extension trait for image drawables.
pub trait ImageDrawableExt: Sized {
    /// Returns a sub image of this image drawable.
    ///
    /// If any of the given `area` lies outside the bounding box of the parent image, the
    /// intersection of `area` and the bounding box will be used.
    ///
    /// # Examples
    ///
    /// This example loads an image containing multiple 32x32px sprites and draws two of them to a
    /// display, with their top-left corners positioned at `(100, 100)` and `(100, 140)`.
    ///
    /// ```rust
    /// use embedded_graphics::{image::Image, pixelcolor::Rgb565, prelude::*, primitives::Rectangle};
    /// # use embedded_graphics::mock_display::MockDisplay as Display;
    /// use tinytga::Tga;
    ///
    /// let mut display: Display<Rgb565> = Display::default();
    ///
    /// let sprite_atlas: Tga<Rgb565> = Tga::from_slice(include_bytes!(
    ///     "../../../assets/tiles.tga"
    /// ))
    /// .unwrap();
    ///
    /// let sprite_1 = sprite_atlas.sub_image(&Rectangle::new(Point::new(0, 0), Size::new(32, 32)));
    /// let sprite_2 = sprite_atlas.sub_image(&Rectangle::new(Point::new(32, 0), Size::new(32, 32)));
    ///
    /// Image::new(&sprite_1, Point::new(100, 100)).draw(&mut display)?;
    /// Image::new(&sprite_2, Point::new(100, 140)).draw(&mut display)?;
    ///
    /// # Ok::<(), core::convert::Infallible>(())
    /// ```
    fn sub_image(&self, area: &Rectangle) -> SubImage<Self>;
}

impl<T> ImageDrawableExt for T
where
    T: ImageDrawable,
{
    fn sub_image(&self, area: &Rectangle) -> SubImage<T> {
        SubImage::new(self, area)
    }
}
