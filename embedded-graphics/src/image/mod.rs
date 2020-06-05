//! Image support for embedded-graphics
//!
//! Adding embedded-graphics support to an image format requires the implementation of the
//! [`ImageDimensions`] and [`IntoPixelIter`] traits. These provide a common interface to image metadata
//! and an iterator over individual pixel values respectively.
//!
//! The [`Image`] struct is a wrapper around items that implement both [`ImageDimensions`] and
//! [`IntoPixelIter`] and allows them to be drawn to a [`DrawTarget`], reading pixel values from the
//! implementation of [`IntoPixelIter`].
//!
//! # Examples
//!
//! ## Load a TGA image and draw it to a display
//!
//! This example loads a TGA-formatted image using the [tinytga] crate and draws it to the display
//! using the [`Image`] wrapper. The image is positioned at the top left corner of the display.
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
//! [`IntoPixelIter`]: ./trait.IntoPixelIter.html
//! [`ImageDimensions`]: ./trait.ImageDimensions.html
//! [`Image`]: ./struct.Image.html
//! [`DrawTarget`]: ../trait.DrawTarget.html

mod image_raw;

pub use self::image_raw::{ImageRaw, ImageRawBE, ImageRawLE};

use crate::{
    draw_target::DrawTarget,
    drawable::{Drawable, Pixel},
    geometry::{Dimensions, Point, Size},
    pixelcolor::PixelColor,
    primitives::Rectangle,
    transform::Transform,
};
use core::{fmt, fmt::Debug, marker::PhantomData};

/// Conversion into an iterator over the pixels of the image.
pub trait IntoPixelIter<C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    /// Iterator over pixels in the image
    type PixelIterator: Iterator<Item = Pixel<C>>;

    /// Get an iterator over the pixels of the image
    fn pixel_iter(self) -> Self::PixelIterator;
}

/// A trait to get the dimensions of an image.
///
/// This trait provides an interface to get the width and height of an image. It should be
/// implemented along with [`IntoPixelIter`] for full embedded-graphics integration.
pub trait ImageDimensions {
    /// Get the width in pixels of an image
    fn width(&self) -> u32;

    /// Get the height in pixels of an image
    fn height(&self) -> u32;
}

/// Image drawable.
///
/// The `Image` struct serves as a wrapper around other image types that provide pixel data decoded
/// from a given format (raw bytes, BMP, TGA, etc). It allows an image to be repositioned using
/// [`Transform::translate()`] or [`Transform::translate_mut()`] and drawn to a display that
/// implements the [`DrawTarget`] trait.
///
/// `Image` accepts any item that implements `ImageDimensions` and `&'_ IntoPixelIter`.
///
/// Refer to the [module documentation] for examples.
///
/// [module documentation]: ./index.html
/// [`Transform::translate()`]: ../transform/trait.Transform.html#tymethod.translate
/// [`Transform::translate_mut()`]: ../transform/trait.Transform.html#tymethod.translate_mut
/// [`DrawTarget`]: ../trait.DrawTarget.html
#[derive(Debug, Clone, Copy)]
pub struct Image<'a, I, C> {
    image_data: &'a I,
    offset: Point,
    c: PhantomData<C>,
}

impl<'a, I, C> Image<'a, I, C>
where
    &'a I: IntoPixelIter<C>,
    I: ImageDimensions,
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    /// Create a new `Image` with the given image pixel data.
    ///
    /// The passed [`IntoPixelIter`] provides a source of pixel data from the original image.
    ///
    /// [`IntoPixelIter`]: ./trait.IntoPixelIter.html
    pub fn new(image_data: &'a I, position: Point) -> Self {
        Self {
            image_data,
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
            image_data: self.image_data,
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

impl<'a, 'b, I, C> Drawable<C> for &'a Image<'b, I, C>
where
    &'b I: IntoPixelIter<C>,
    I: ImageDimensions,
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    fn draw<D: DrawTarget<Color = C>>(self, display: &mut D) -> Result<(), D::Error> {
        display.fill_contiguous(&self.bounding_box(), self.into_iter().map(|p| p.1))
    }
}

impl<'a, I, C> Dimensions for Image<'a, I, C>
where
    I: ImageDimensions,
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    fn bounding_box(&self) -> Rectangle {
        let size = Size::new(self.image_data.width(), self.image_data.height());

        Rectangle::new(self.offset, size)
    }
}

impl<'a, 'b, I, C> IntoIterator for &'a Image<'b, I, C>
where
    &'b I: IntoPixelIter<C>,
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    type Item = Pixel<C>;
    type IntoIter = ImageIterator<'a, 'b, I, C>;

    fn into_iter(self) -> Self::IntoIter {
        ImageIterator {
            it: self.image_data.pixel_iter(),
            image: self,
        }
    }
}

/// Pixel iterator over `Image` objects
pub struct ImageIterator<'a, 'b, I, C>
where
    &'b I: IntoPixelIter<C>,
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    image: &'a Image<'b, I, C>,
    it: <&'b I as IntoPixelIter<C>>::PixelIterator,
}

impl<'a, 'b, I, C> Debug for ImageIterator<'a, 'b, I, C>
where
    &'b I: IntoPixelIter<C> + Debug,
    <&'b I as IntoPixelIter<C>>::PixelIterator: Debug,
    I: Debug,
    C: PixelColor + From<<C as PixelColor>::Raw> + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ImageIterator")
            .field("image", &self.image)
            .field("it", &self.it)
            .finish()
    }
}

impl<'a, 'b, I, C> Iterator for ImageIterator<'a, 'b, I, C>
where
    &'b I: IntoPixelIter<C>,
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next().map(|p| Pixel(p.0 + self.image.offset, p.1))
    }
}
