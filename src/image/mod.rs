//! Image support for embedded-graphics
//!
//! The two main types used to draw images are [`ImageDrawable`] and [`Image`].
//!
//! [`ImageDrawable`] is implemented to add support for different image formats. This crate includes
//! an implementation for [raw pixel data]. Additional implementations for other image formats are
//! provided by external crates. See the [external crates list] for a list of available image formats.
//!
//! The [`Image`] object is used to specify the location at which an [`ImageDrawable`] is drawn.
//! Images can be placed relative to their top-left corner or center point by using the
//! [`Image::new`] or [`Image::with_center`] constructors.
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
//!     image::{Image, ImageRaw},
//!     pixelcolor::{Rgb565, raw::order::BigEndian},
//!     prelude::*,
//! };
//! # use embedded_graphics::mock_display::MockDisplay as Display;
//!
//! let mut display: Display<Rgb565> = Display::default();
//!
//! // Raw big endian image data for demonstration purposes. A real image would likely be much
//! // larger and would be included via the `include_bytes!` macro.
//! let data = [
//!     0x00, 0x00, 0xF8, 0x00, 0x07, 0xE0, 0xFF, 0xE0, //
//!     0x00, 0x1F, 0x07, 0xFF, 0xF8, 0x1F, 0xFF, 0xFF, //
//! ];
//!
//! // Create a raw image instance. Note that the format of the image data is specified by adding
//! // type annotations for the color type and the byte order.
//! let raw = ImageRaw::<Rgb565, BigEndian>::new(&data, Size::new(4, 2)).unwrap();
//!
//! // Create an `Image` object to position the top-left corner of the image at `Point::zero()`.
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
//!     image::{Image, ImageRaw},
//!     pixelcolor::{Rgb565, raw::order::BigEndian},
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
//! let sprite_atlas = ImageRaw::<Rgb565, BigEndian>::new(&data, Size::new(32, 16)).unwrap();
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
//! [raw pixel data]: ImageRaw
//! [`sub_image`]: ImageDrawableExt::sub_image
//! [`OriginDimensions`]: super::geometry::OriginDimensions
//! [`prelude`]: super::prelude

mod image_drawable_ext;
mod image_raw;
mod sub_image;

pub use embedded_graphics_core::image::ImageDrawable;
pub use image_drawable_ext::ImageDrawableExt;
pub use image_raw::ImageRaw;
pub use sub_image::SubImage;

use crate::{
    common::ColorType,
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
/// [module documentation]: self
/// [`Transform::translate`]: super::transform::Transform::translate
/// [`Transform::translate_mut`]: super::transform::Transform::translate_mut
/// [`DrawTarget`]: super::draw_target::DrawTarget
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct Image<'a, T> {
    image_drawable: &'a T,
    offset: Point,
}

impl<'a, T: ImageDrawable> Image<'a, T> {
    /// Creates a new `Image`.
    pub const fn new(image_drawable: &'a T, position: Point) -> Self {
        Self {
            image_drawable,
            offset: position,
        }
    }

    /// Creates a new `Image` centered around a given point.
    pub fn with_center(image_drawable: &'a T, center: Point) -> Self {
        let offset = Rectangle::with_center(center, image_drawable.size()).top_left;

        Self {
            image_drawable,
            offset,
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
    /// This example moves a 4x4 black and white image by `(10, 20)` pixels without mutating the
    /// original image
    ///
    /// ```rust
    /// use embedded_graphics::{
    ///     geometry::Point,
    ///     image::{Image, ImageRaw},
    ///     pixelcolor::{BinaryColor, raw::order::Msb0},
    ///     prelude::*,
    /// };
    ///
    /// let data = &[0xff, 0x00, 0xff, 0x00];
    /// let image = ImageRaw::<BinaryColor, Msb0>::new(data, Size::new(4, 4)).unwrap();
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
    /// This example moves a 4x4 black and white image by `(10, 20)` pixels by mutating the
    /// original image
    ///
    /// ```rust
    /// use embedded_graphics::{
    ///     geometry::Point,
    ///     image::{Image, ImageRaw},
    ///     pixelcolor::{BinaryColor, raw::order::Msb0},
    ///     prelude::*,
    /// };
    ///
    /// let data = &[0xff, 0x00, 0xff, 0x00];
    /// let image = ImageRaw::<BinaryColor, Msb0>::new(data, Size::new(4, 4)).unwrap();
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

impl<'a, T: ImageDrawable> ColorType for Image<'a, T> {
    type Color = T::Color;
}

impl<'a, T: ImageDrawable> Drawable for Image<'a, T> {
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        self.image_drawable
            .draw(&mut display.translated(self.offset))
    }
}

impl<'a, T: OriginDimensions> Dimensions for Image<'a, T> {
    fn bounding_box(&self) -> Rectangle {
        self.image_drawable.bounding_box().translate(self.offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        geometry::Size,
        mock_display::MockDisplay,
        pixelcolor::{raw::order::Msb0, BinaryColor},
    };

    #[test]
    fn negative_top_left() {
        let data = &[0xff, 0x00, 0xff, 0x00];
        let image = ImageRaw::<BinaryColor, Msb0>::new(data, Size::new(4, 4)).unwrap();

        let image = Image::new(&image, Point::zero()).translate(Point::new(-1, -1));

        assert_eq!(
            image.bounding_box(),
            Rectangle::new(Point::new(-1, -1), Size::new(4, 4))
        );
    }

    #[test]
    fn dimensions() {
        let data = &[0xff, 0x00, 0xFF, 0x00];
        let image = ImageRaw::<BinaryColor, Msb0>::new(data, Size::new(4, 4)).unwrap();

        let image = Image::new(&image, Point::zero()).translate(Point::new(100, 200));

        assert_eq!(
            image.bounding_box(),
            Rectangle::new(Point::new(100, 200), Size::new(4, 4))
        );
    }

    #[test]
    fn position() {
        let data = &[0xAA, 0x55, 0xAA, 0x55];
        let image_raw = ImageRaw::<BinaryColor, Msb0>::new(data, Size::new(4, 4)).unwrap();

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

    #[test]
    fn with_center() {
        let data = &[0xAA, 0x55, 0xAA, 0x55];
        let image_raw = ImageRaw::<BinaryColor, Msb0>::new(data, Size::new(4, 4)).unwrap();

        let mut display = MockDisplay::new();
        Image::with_center(&image_raw, Point::new(1, 2))
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "    ", //
            "#.#.", //
            ".#.#", //
            "#.#.", //
            ".#.#", //
        ]);
    }
}
