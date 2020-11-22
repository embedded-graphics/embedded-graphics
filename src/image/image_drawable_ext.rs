use crate::image::SubImage;
use embedded_graphics_core::{image_drawable::ImageDrawable, Rectangle};

/// Extension trait for image drawables.
pub trait ImageDrawableExt: Sized {
    /// Returns a sub image of this image drawable.
    ///
    /// If any of the given `area` lies outside the bounding box of the parent image, the
    /// intersection of `area` and the bounding box will be used.
    ///
    /// # Examples
    ///
    /// This example loads a raw image containing multiple 32x32px sprites and draws two of them to
    /// a display, with their top-left corners positioned at `(100, 100)` and `(100, 140)`.
    ///
    /// ```rust
    /// use embedded_graphics::{
    ///     image::{Image, ImageRaw, ImageRawBE},
    ///     pixelcolor::Rgb565,
    ///     prelude::*,
    ///     primitives::Rectangle,
    /// };
    /// # use embedded_graphics::mock_display::MockDisplay as Display;
    /// let mut display: Display<Rgb565> = Display::default();
    ///
    /// let data = [ 0xF8, 0x00, 0x07, 0xE0, 0xFF, 0xE0, /* ... */ ];
    /// // or: let data = include_bytes!("sprite_atlas.raw");
    ///
    /// # let data = [0u8; 64 * 32 * 2];
    /// let sprite_atlas: ImageRawBE<Rgb565> = ImageRaw::new(&data, 64, 32);
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
