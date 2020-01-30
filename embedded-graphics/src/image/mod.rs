//! Image drawables.
//!
//! Image drawables can be created for raw bitmap data and images in BMP and TGA
//! format.

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

/// TODO: Docs
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

/// TODO: Docs
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
    /// TODO: Docs
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
    /// TODO: Docs and example
    fn translate(&self, by: Point) -> Self {
        Self {
            offset: self.offset + by,
            ..self.clone()
        }
    }

    /// TODO: Docs and example
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
