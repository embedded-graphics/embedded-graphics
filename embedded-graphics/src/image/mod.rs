//! Image support for embedded-graphics

mod image_raw;

pub use self::image_raw::{ImageBE, ImageLE, ImageRaw};

use crate::draw_target::DrawTarget;
use crate::drawable::Drawable;
use crate::drawable::Pixel;
use crate::geometry::Dimensions;
use crate::geometry::Point;
use crate::geometry::Size;
use crate::pixelcolor::PixelColor;
use crate::transform::Transform;
use core::marker::PhantomData;

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

    /// Get the width in pixels of an image
    fn width(&self) -> u32;

    /// Get the height in pixels of an image
    fn height(&self) -> u32;

    /// Get an iterator over the pixels of the image
    fn pixel_iter(&self) -> Self::PixelIterator;
}

/// A wrapper for any image type
///
/// This is a wrapper around [`ImageData`] implementations to better integrate into
/// embedded-graphics.
///
/// [`ImageData`]: ./trait.ImageData.html
#[derive(Debug, Clone, Copy)]
pub struct Image<I: Clone, C> {
    image: I,
    offset: Point,
    c: PhantomData<C>,
}

impl<'a, I, C> Image<I, C>
where
    I: ImageData<C> + Clone,
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    /// Create a new `Image` with a given [`ImageData`]
    ///
    /// The passed [`ImageData`] provides a source of pixel data from the original image.
    ///
    /// [`ImageData`]: ./trait.ImageData.html
    pub fn new(image: I) -> Self {
        Self {
            image,
            offset: Point::zero(),
            c: PhantomData,
        }
    }
}

impl<'a, I, C> Transform for Image<I, C>
where
    I: ImageData<C> + Clone,
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
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
    ///    use embedded_graphics::{image::{Image,ImageRaw}, prelude::*, geometry::Point, pixelcolor::BinaryColor};
    ///
    ///    let image: ImageRaw<BinaryColor> = ImageRaw::new(&[0xff, 0x00, 0xff, 0x00], 4, 4);
    ///
    ///    let image = Image::new(&image).translate(Point::new(10, 20));
    ///
    ///    assert_eq!(image.top_left(), Point::new(10, 20));
    /// ```
    fn translate(&self, by: Point) -> Self {
        Self {
            offset: self.offset + by,
            ..self.clone()
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
    ///    use embedded_graphics::{image::{Image,ImageRaw}, prelude::*, geometry::Point, pixelcolor::BinaryColor};
    ///
    ///    let image: ImageRaw<BinaryColor> = ImageRaw::new(&[0xff, 0x00, 0xff, 0x00], 4, 4);
    ///
    ///    let mut image = Image::new(&image);
    ///
    /// image.translate_mut(Point::new(10, 20));
    ///
    ///    assert_eq!(image.top_left(), Point::new(10, 20));
    /// ```
    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.offset += by;

        self
    }
}

impl<'a, I, C> Drawable<C> for Image<I, C>
where
    I: ImageData<C> + Clone,
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        display.draw_iter(
            &mut self
                .image
                .pixel_iter()
                .map(|p| Pixel(p.0 + self.offset, p.1)),
        )
    }
}

impl<I, C> Dimensions for Image<I, C>
where
    I: ImageData<C> + Clone,
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    fn top_left(&self) -> Point {
        self.offset
    }

    fn bottom_right(&self) -> Point {
        self.top_left() + self.size()
    }

    fn size(&self) -> Size {
        Size::new(self.image.width(), self.image.height())
    }
}

impl<'a, I, C> IntoIterator for &'a Image<I, C>
where
    I: ImageData<C> + Clone,
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    type Item = Pixel<C>;
    type IntoIter = ImageIterator<'a, I, C>;

    fn into_iter(self) -> Self::IntoIter {
        ImageIterator {
            it: self.image.pixel_iter(),
            image: self,
        }
    }
}

/// Pixel iterator over `Image` objects
#[derive(Debug)]
pub struct ImageIterator<'a, I: ImageData<C> + Clone, C: PixelColor + From<<C as PixelColor>::Raw>>
{
    image: &'a Image<I, C>,
    it: I::PixelIterator,
}

impl<'a, I, C> Iterator for ImageIterator<'a, I, C>
where
    I: ImageData<C> + Clone,
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next().map(|p| Pixel(p.0 + self.image.offset, p.1))
    }
}
