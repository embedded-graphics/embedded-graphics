//! Image drawables.
//!
//! Image drawables can be created for raw bitmap data and images in BMP and TGA
//! format.

// #[cfg(feature = "bmp")]
// mod image_bmp;
// mod image_raw;
// #[cfg(feature = "tga")]
// mod image_tga;

// pub use self::image_raw::{ImageBE, ImageLE, ImageRaw};

use crate::draw_target::DrawTarget;
use crate::drawable::Drawable;
use crate::drawable::Pixel;
use crate::geometry::Point;
use crate::pixelcolor::PixelColor;
use crate::transform::Transform;
use core::marker::PhantomData;

// #[cfg(feature = "bmp")]
// pub use self::image_bmp::ImageBmp;
// #[cfg(feature = "tga")]
// pub use self::image_tga::ImageTga;

// /// Image file trait.
// pub trait ImageFile<'a, C>: Dimensions + Sized + Drawable<C> + Transform
// where
//     C: PixelColor,
// {
//     /// Error type to return when loading of the image data failed
//     type LoadError;

//     /// Create a new image with given input file
//     ///
//     /// The input file is expected to be of a particular format (BMP, TGA, etc) and contain file
//     /// metadata like width/height and pixel data. Because parsing may fail, this returns a
//     /// `Result<Self, ()>`.
//     fn new(filedata: &'a [u8]) -> Result<Self, Self::LoadError>;

//     /// Get the width in pixels of an image
//     fn width(&self) -> u32;

//     /// Get the height in pixels of an image
//     fn height(&self) -> u32;
// }

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
        display.draw_iter(&mut self.image.pixel_iter())
    }
}
