//! Image support for embedded-graphics
//!
//! Adding embedded-graphics support to an image format requires the implementation of the
//! [`ImageDrawable`] and [`OriginDimensions`] traits. These provide a common interface to image
//! metadata and an iterator over individual pixel values respectively.
//!
//! The [`Image`] struct is a wrapper around items that implement both [`OriginDimensions`] and
//! [`ImageDrawable`] and allows them to be drawn to a [`DrawTarget`], reading pixel values from the
//! implementation of [`ImageDrawable`].
//!
//! # Examples
//!
//! ## Load a TGA image and draw it to a display
//!
//! This example loads a TGA-formatted image using the [tinytga] crate and draws it to the display
//! using the [`Image`] wrapper. The image is positioned at the top left corner of the display.
//!
//! The `graphics` feature of `tinytga` needs to be enabled in `Cargo.toml` to use the `Tga` object
//! with embedded-graphics.
//!
//! ```rust
//! use embedded_graphics::{image::Image, pixelcolor::Rgb565, prelude::*};
//! # use embedded_graphics::mock_display::MockDisplay as Display;
//! use tinytga::Tga;
//!
//! let mut display: Display<Rgb565> = Display::default();
//!
//! let tga = Tga::from_slice(include_bytes!(
//!     "../../../simulator/examples/assets/rust-pride.tga"
//! ))
//! .unwrap();
//!
//! let image: Image<Tga, Rgb565> = Image::new(&tga, Point::zero());
//!
//! image.draw(&mut display);
//!
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//!
//! [tinytga]: https://crates.io/crates/tinytga
//! [`ImageDrawable`]: trait.ImageDrawable.html
//! [`OriginDimensions`]: ../geometry/trait.OriginDimensions.html
//! [`Image`]: ./struct.Image.html
//! [`DrawTarget`]: ../draw_target/trait.DrawTarget.html

mod image_drawable;
mod image_file;
mod image_raw;
mod sub_image;

pub use image_drawable::{ImageDrawable, ImageDrawableExt};
pub use image_file::{ImageData, ImageFile};
pub use image_raw::{ImageRaw, ImageRawBE, ImageRawLE};
pub use sub_image::SubImage;

use crate::{
    draw_target::{DrawTarget, DrawTargetExt},
    drawable::Drawable,
    geometry::{Dimensions, OriginDimensions, Point},
    pixelcolor::PixelColor,
    primitives::Rectangle,
    transform::Transform,
};
use core::{fmt::Debug, marker::PhantomData};

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
#[derive(Debug, Clone, Copy)]
pub struct Image<'a, I, C> {
    image_drawable: &'a I,
    offset: Point,
    c: PhantomData<C>,
}

impl<'a, I, C> Image<'a, I, C>
where
    I: ImageDrawable<C>,
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    /// Creates a new `Image`.
    pub fn new(image_drawable: &'a I, position: Point) -> Self {
        Self {
            image_drawable,
            offset: position,
            c: PhantomData,
        }
    }
}

impl<I, C> Transform for Image<'_, I, C> {
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
    /// let image: ImageRaw<BinaryColor> = ImageRaw::new(&[0xff, 0x00, 0xff, 0x00], 4, 4);
    ///
    /// let image: Image<_, BinaryColor> = Image::new(&image, Point::zero());
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
            c: PhantomData,
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
    /// let image: ImageRaw<BinaryColor> = ImageRaw::new(&[0xff, 0x00, 0xff, 0x00], 4, 4);
    ///
    /// let mut image: Image<_, BinaryColor> = Image::new(&image, Point::zero());
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

impl<'a, I, C> Drawable for Image<'a, I, C>
where
    I: ImageDrawable<C>,
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    type Color = C;

    fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        self.image_drawable
            .draw(&mut display.translated(self.offset))
    }
}

impl<'a, I, C> Dimensions for Image<'a, I, C>
where
    I: OriginDimensions,
    C: PixelColor + From<<C as PixelColor>::Raw>,
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
        let image: ImageRaw<BinaryColor> = ImageRaw::new(&[0xff, 0x00, 0xff, 0x00], 4, 4);

        let image = Image::new(&image, Point::zero()).translate(Point::new(-1, -1));

        assert_eq!(
            image.bounding_box(),
            Rectangle::new(Point::new(-1, -1), Size::new(4, 4))
        );
    }

    #[test]
    fn dimensions() {
        let image: ImageRaw<BinaryColor> = ImageRaw::new(&[0xff, 0x00, 0xFF, 0x00], 4, 4);

        let image = Image::new(&image, Point::zero()).translate(Point::new(100, 200));

        assert_eq!(
            image.bounding_box(),
            Rectangle::new(Point::new(100, 200), Size::new(4, 4))
        );
    }

    #[test]
    fn position() {
        let image_raw: ImageRaw<BinaryColor> = ImageRaw::new(&[0xAA, 0x55, 0xAA, 0x55], 4, 4);

        let mut display = MockDisplay::new();
        Image::new(&image_raw, Point::new(1, 2))
            .draw(&mut display)
            .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "     ", //
                "     ", //
                " #.#.", //
                " .#.#", //
                " #.#.", //
                " .#.#", //
            ])
        )
    }
}
