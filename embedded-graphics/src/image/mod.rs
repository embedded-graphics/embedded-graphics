//! Image support for embedded-graphics

mod image_raw;

pub use self::image_raw::{ImageRaw, ImageRawBE, ImageRawLE};

use crate::{
    draw_target::DrawTarget,
    drawable::{Drawable, Pixel},
    geometry::{Dimensions, Point, Size},
    pixelcolor::PixelColor,
    transform::Transform,
};
use core::{fmt, marker::PhantomData};

/// Image data trait
///
/// This trait is the main integration point for image loading crates. Images are made drawable
/// through use of the [`Image`] struct which accepts an `ImageData`. `Image` implements
/// [`Drawable`], allowing it to be drawn on any display that supports embedded-graphcis via the
/// [`DrawTarget`] trait.
///
/// [`DrawTarget`]: ../trait.DrawTarget.html
/// [`Drawable`]: ../drawable/trait.Drawable.html
/// [`Image`]: ./struct.Image.html
pub trait ImageData<C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    /// Iterator over pixels in the image
    type PixelIterator: Iterator<Item = Pixel<C>>;

    /// Get an iterator over the pixels of the image
    fn pixel_iter(self) -> Self::PixelIterator;
}

/// TODO: docs
pub trait ImageDimensions {
    /// Get the width in pixels of an image
    fn width(&self) -> u32;

    /// Get the height in pixels of an image
    fn height(&self) -> u32;
}

/// A wrapper for any image type
///
/// This is a wrapper around [`ImageData`] implementations to better integrate into
/// embedded-graphics.
///
/// [`ImageData`]: ./trait.ImageData.html
#[derive(Debug, Clone, Copy)]
pub struct Image<'a, I, C> {
    image_data: &'a I,
    offset: Point,
    c: PhantomData<C>,
}

impl<'a, I, C> Image<'a, I, C>
where
    &'a I: ImageData<C>,
    I: ImageDimensions,
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    /// Create a new `Image` with a given [`ImageData`]
    ///
    /// The passed [`ImageData`] provides a source of pixel data from the original image.
    ///
    /// [`ImageData`]: ./trait.ImageData.html
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
    /// let image = Image::new(&image, Point::zero());
    ///
    /// let image_moved = image.translate(Point::new(10, 20));
    ///
    /// assert_eq!(image.top_left(), Point::zero());
    /// assert_eq!(image_moved.top_left(), Point::new(10, 20));
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
    /// let mut image = Image::new(&image, Point::zero());
    ///
    /// image.translate_mut(Point::new(10, 20));
    ///
    /// assert_eq!(image.top_left(), Point::new(10, 20));
    /// ```
    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.offset += by;

        self
    }
}

impl<'a, 'b, I, C> Drawable<C> for &'a Image<'b, I, C>
where
    &'b I: ImageData<C>,
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        display.draw_iter(
            self.image_data
                .pixel_iter()
                .map(|p| Pixel(p.0 + self.offset, p.1)),
        )
    }
}

impl<'a, I, C> Dimensions for Image<'a, I, C>
where
    I: ImageDimensions,
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    fn top_left(&self) -> Point {
        self.offset
    }

    fn bottom_right(&self) -> Point {
        self.top_left() + self.size()
    }

    fn size(&self) -> Size {
        Size::new(self.image_data.width(), self.image_data.height())
    }
}

impl<'a, 'b, I, C> IntoIterator for &'a Image<'b, I, C>
where
    &'b I: ImageData<C>,
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
    &'b I: ImageData<C>,
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    image: &'a Image<'b, I, C>,
    it: <&'b I as ImageData<C>>::PixelIterator,
}

impl<'a, 'b, I, C> fmt::Debug for ImageIterator<'a, 'b, I, C>
where
    &'b I: ImageData<C>,
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // The Debug derive didn't work anymore without additional trait bounds
        // TODO: add fields
        f.debug_struct("ImageIterator").finish()
    }
}

impl<'a, 'b, I, C> Iterator for ImageIterator<'a, 'b, I, C>
where
    &'b I: ImageData<C>,
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next().map(|p| Pixel(p.0 + self.image.offset, p.1))
    }
}
