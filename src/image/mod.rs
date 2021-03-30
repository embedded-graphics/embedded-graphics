//! Image support for embedded-graphics
//!
//! The two main types used to draw images are [`ImageDrawable`] and [`Image`].
//!
//! [`ImageDrawable`] is implemented to add support for different image formats. This crate includes
//! an implementation for [raw pixel data]. Additional implementations for other image formats are
//! provided by external crates like [tinybmp] and [tinytga].
//!
//! The [`Image`] object is used to specify the location at which an [`ImageDrawable`] is drawn.
//! Images are drawn relative to their top-left corner.
//!
//! # Examples
//!
//! ## Display an RGB565 raw data image
//!
//! This example displays a small image created from a raw data array. The image is RGB565 encoded,
//! so it uses the `Rgb565` color type.
//!
//! ```rust
//! use embedded_graphics::{
//!     image::{Image, ImageRaw, ImageRawBE},
//!     pixelcolor::Rgb565,
//!     prelude::*,
//! };
//! # use embedded_graphics::mock_display::MockDisplay as Display;
//!
//! let mut display: Display<Rgb565> = Display::default();
//!
//! // Raw big endian image data for demonstration purposes. A real image would likely be much
//! // larger.
//! let data = [
//!     0x00, 0x00, 0xF8, 0x00, 0x07, 0xE0, 0xFF, 0xE0, //
//!     0x00, 0x1F, 0x07, 0xFF, 0xF8, 0x1F, 0xFF, 0xFF, //
//! ];
//!
//! // Create a raw image instance. Other image formats will require different code to load them.
//! // All code after loading is the same for any image format.
//! let raw: ImageRawBE<Rgb565> = ImageRaw::new(&data, 4);
//!
//! // Create an `Image` object to position the image at `Point::zero()`.
//! let image = Image::new(&raw, Point::zero());
//!
//! // Draw the image to the display.
//! image.draw(&mut display)?;
//!
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//!
//! ## Sub images
//!
//! [`SubImage`]s are used to split a larger image drawables into multiple parts, e.g. to draw a
//! single sprite from a sprite atlas as in this example. Use the [`sub_image`] method provided by
//! [`ImageDrawableExt`] to get a sub image from an image drawable. [`ImageDrawableExt`] is included
//! in the [`prelude`], which this example takes advantage of.
//!
//! ```rust
//! use embedded_graphics::{
//!     image::{Image, ImageRaw, ImageRawBE},
//!     pixelcolor::Rgb565,
//!     prelude::*,
//!     primitives::Rectangle,
//! };
//! # use embedded_graphics::mock_display::MockDisplay as Display;
//!
//! let mut display: Display<Rgb565> = Display::default();
//!
//! let data = [ 0xF8, 0x00, 0x07, 0xE0, 0xFF, 0xE0, /* ... */ ];
//! // or: let data = include_bytes!("sprite_atlas.raw");
//!
//! # let data = [0u8; 32 * 16 * 2];
//! let sprite_atlas: ImageRawBE<Rgb565> = ImageRaw::new(&data, 32);
//!
//! // Create individual sub images for each sprite in the sprite atlas.
//! // The position and size of the sub images is defined by a `Rectangle`.
//! let sprite_1 = sprite_atlas.sub_image(&Rectangle::new(Point::new(0, 0), Size::new(16, 16)));
//! let sprite_2 = sprite_atlas.sub_image(&Rectangle::new(Point::new(16, 0), Size::new(16, 16)));
//!
//! // Create `Image` objects to draw the sprites at different positions on the display.
//! Image::new(&sprite_1, Point::new(10, 10)).draw(&mut display)?;
//! Image::new(&sprite_2, Point::new(40, 30)).draw(&mut display)?;
//!
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//!
//! # Implementing new image formats
//!
//! To add embedded-graphics support for an new image format the [`ImageDrawable`] and
//! [`OriginDimensions`] traits must be implemented. See the [`ImageDrawable`] documentation
//! for more information.
//!
//! [tinytga]: https://crates.io/crates/tinytga
//! [tinybmp]: https://crates.io/crates/tinybmp
//! [raw pixel data]: struct.ImageRaw.html
//! [`ImageDrawable`]: trait.ImageDrawable.html
//! [`ImageDrawableExt`]: trait.ImageDrawableExt.html
//! [`sub_image`]: trait.ImageDrawableExt.html#tymethod.sub_image
//! [`OriginDimensions`]: ../geometry/trait.OriginDimensions.html
//! [`Image`]: ./struct.Image.html
//! [`SubImage`]: struct.SubImage.html
//! [`prelude`]: ../prelude/index.html

mod image_drawable_ext;
mod image_raw;
mod sub_image;

pub use embedded_graphics_core::image::ImageDrawable;
pub use image_drawable_ext::ImageDrawableExt;
pub use image_raw::{ImageRaw, ImageRawBE, ImageRawLE};
pub use sub_image::SubImage;

use crate::{
    draw_target::{DrawTarget, DrawTargetExt},
    geometry::{Dimensions, OriginDimensions, Point},
    primitives::Rectangle,
    transform::Transform,
    Drawable,
};
use core::fmt::Debug;

/// Image object.
///
/// The `Image` struct serves as a wrapper around an [`ImageDrawable`] that provides support for
/// an image format (raw bytes, BMP, TGA, etc). It allows an image to be repositioned using
/// [`Transform::translate`] or [`Transform::translate_mut`] and drawn to a display that
/// implements the [`DrawTarget`] trait.
///
/// Refer to the [module documentation] for examples.
///
/// [module documentation]: ./index.html
/// [`Transform::translate`]: ../transform/trait.Transform.html#tymethod.translate
/// [`Transform::translate_mut`]: ../transform/trait.Transform.html#tymethod.translate_mut
/// [`DrawTarget`]: ../draw_target/trait.DrawTarget.html
/// [`ImageDrawable`]: trait.ImageDrawable.html
#[derive(Debug, Clone, Copy)]
pub struct Image<'a, T> {
    image_drawable: &'a T,
    offset: Point,
}

impl<'a, T> Image<'a, T>
where
    T: ImageDrawable,
{
    /// Creates a new `Image`.
    pub fn new(image_drawable: &'a T, position: Point) -> Self {
        Self {
            image_drawable,
            offset: position,
        }
    }
}

impl<T> Transform for Image<'_, T> {
    /// Translate the image by a given delta, returning a new image
    ///
    /// # Examples
    ///
    /// ## Move an image around
    ///
    /// This examples moves a 4x4 black and white image by `(10, 20)` pixels without mutating the
    /// original image
    ///
    /// ```rust
    /// use embedded_graphics::{
    ///     geometry::Point,
    ///     image::{Image, ImageRaw},
    ///     pixelcolor::BinaryColor,
    ///     prelude::*,
    /// };
    ///
    /// let image: ImageRaw<BinaryColor> = ImageRaw::new(&[0xff, 0x00, 0xff, 0x00], 4);
    ///
    /// let image = Image::new(&image, Point::zero());
    ///
    /// let image_moved = image.translate(Point::new(10, 20));
    ///
    /// assert_eq!(image.bounding_box().top_left, Point::zero());
    /// assert_eq!(image_moved.bounding_box().top_left, Point::new(10, 20));
    /// ```
    fn translate(&self, by: Point) -> Self {
        Self {
            image_drawable: self.image_drawable,
            offset: self.offset + by,
        }
    }

    /// Translate the image by a given delta, modifying the original object
    ///
    /// # Examples
    ///
    /// ## Move an image around
    ///
    /// This examples moves a 4x4 black and white image by `(10, 20)` pixels by mutating the
    /// original image
    ///
    /// ```rust
    /// use embedded_graphics::{
    ///     geometry::Point,
    ///     image::{Image, ImageRaw},
    ///     pixelcolor::BinaryColor,
    ///     prelude::*,
    /// };
    ///
    /// let image: ImageRaw<BinaryColor> = ImageRaw::new(&[0xff, 0x00, 0xff, 0x00], 4);
    ///
    /// let mut image = Image::new(&image, Point::zero());
    ///
    /// image.translate_mut(Point::new(10, 20));
    ///
    /// assert_eq!(image.bounding_box().top_left, Point::new(10, 20));
    /// ```
    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.offset += by;

        self
    }
}

impl<'a, T> Drawable for Image<'a, T>
where
    T: ImageDrawable,
{
    type Color = T::Color;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        self.image_drawable
            .draw(&mut display.translated(self.offset))
    }
}

impl<'a, T> Dimensions for Image<'a, T>
where
    T: OriginDimensions,
{
    fn bounding_box(&self) -> Rectangle {
        self.image_drawable.bounding_box().translate(self.offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{geometry::Size, mock_display::MockDisplay, pixelcolor::BinaryColor};

    #[test]
    fn negative_top_left() {
        let image: ImageRaw<BinaryColor> = ImageRaw::new(&[0xff, 0x00, 0xff, 0x00], 4);

        let image = Image::new(&image, Point::zero()).translate(Point::new(-1, -1));

        assert_eq!(
            image.bounding_box(),
            Rectangle::new(Point::new(-1, -1), Size::new(4, 4))
        );
    }

    #[test]
    fn dimensions() {
        let image: ImageRaw<BinaryColor> = ImageRaw::new(&[0xff, 0x00, 0xFF, 0x00], 4);

        let image = Image::new(&image, Point::zero()).translate(Point::new(100, 200));

        assert_eq!(
            image.bounding_box(),
            Rectangle::new(Point::new(100, 200), Size::new(4, 4))
        );
    }

    #[test]
    fn position() {
        let image_raw: ImageRaw<BinaryColor> = ImageRaw::new(&[0xAA, 0x55, 0xAA, 0x55], 4);

        let mut display = MockDisplay::new();
        Image::new(&image_raw, Point::new(1, 2))
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "     ", //
            "     ", //
            " #.#.", //
            " .#.#", //
            " #.#.", //
            " .#.#", //
        ]);
    }
}
