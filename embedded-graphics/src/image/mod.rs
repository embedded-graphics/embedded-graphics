//! Image object

use crate::coord::Coord;
use crate::drawable::Dimensions;
use crate::drawable::Drawable;
use crate::pixelcolor::PixelColor;
use crate::transform::Transform;
use crate::unsignedcoord::{ToSigned, UnsignedCoord};
use core::marker::PhantomData;

mod image16bpp;
mod image1bpp;
mod image8bpp;
mod image_bmp;

/// Raw image type
pub trait ImageType {}

/// Raw image
#[derive(Debug)]
pub struct Image<'a, C, T>
where
    C: PixelColor,
    T: ImageType,
{
    /// Image width in pixels
    width: u32,

    /// Image height in pixels
    height: u32,

    /// Image data, 1 bit per byte, 1 byte per 8 horizontal pixels
    imagedata: &'a [u8],

    /// Image offset in pixels from screen origin (0,0)
    pub offset: Coord,

    pixel_type: PhantomData<C>,
    image_type: PhantomData<T>,
}

impl<'a, C, T> Image<'a, C, T>
where
    C: PixelColor,
    T: ImageType,
{
    /// Create a new image with given pixel data, width and height
    pub fn new(imagedata: &'a [u8], width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            imagedata,
            offset: Coord::new(0, 0),
            pixel_type: PhantomData,
            image_type: PhantomData,
        }
    }
}

impl<'a, C, T> Dimensions for Image<'a, C, T>
where
    C: PixelColor,
    T: ImageType,
{
    fn top_left(&self) -> Coord {
        self.offset
    }

    fn bottom_right(&self) -> Coord {
        self.top_left() + self.size().to_signed()
    }

    fn size(&self) -> UnsignedCoord {
        let height = self.height;
        let width = self.width;

        UnsignedCoord::new(width, height)
    }
}

impl<'a, C, T> Drawable for Image<'a, C, T>
where
    C: PixelColor,
    T: ImageType,
{
}

impl<'a, C, T> Transform for Image<'a, C, T>
where
    C: PixelColor,
    T: ImageType,
{
    /// Translate the image from its current position to a new position by (x, y) pixels, returning
    /// a new `Image`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::image::{ Image, Image1BPP };
    /// # use embedded_graphics::transform::Transform;
    /// # use embedded_graphics::coord::Coord;
    /// # use embedded_graphics::pixelcolor::PixelColorU8;
    /// #
    /// // 8px x 1px test image
    /// let image: Image1BPP<PixelColorU8> = Image1BPP::new(&[ 0xff ], 8, 1);
    /// let moved = image.translate(Coord::new(25, 30));
    ///
    /// assert_eq!(image.offset, Coord::new(0, 0));
    /// assert_eq!(moved.offset, Coord::new(25, 30));
    /// ```
    fn translate(&self, by: Coord) -> Self {
        Self {
            offset: self.offset + by,
            ..*self.clone()
        }
    }

    /// Translate the image from its current position to a new position by (x, y) pixels.
    ///
    /// ```
    /// # use embedded_graphics::image::{ Image, Image1BPP };
    /// # use embedded_graphics::transform::Transform;
    /// # use embedded_graphics::coord::Coord;
    /// # use embedded_graphics::pixelcolor::PixelColorU8;
    /// #
    /// let mut image: Image1BPP<PixelColorU8> = Image1BPP::new(&[ 0xff ], 8, 1);
    /// image.translate_mut(Coord::new(25, 30));
    ///
    /// assert_eq!(image.offset, Coord::new(25, 30));
    /// ```
    fn translate_mut(&mut self, by: Coord) -> &mut Self {
        self.offset += by;

        self
    }
}

/// Image trait
pub trait ImageFile<'a>: Dimensions + Sized {
    /// Create a new image with given input file
    ///
    /// The input file is expected to be of a particular format (BMP, TGA, etc) and contain file
    /// metadata like width/height and pixel data. Because parsing may fail, this returns a
    /// `Result<Self, ()>`.
    fn new(filedata: &'a [u8]) -> Result<Self, ()>;

    /// Get the width in pixels of an image
    fn width(&self) -> u32;

    /// Get the height in pixels of an image
    fn height(&self) -> u32;
}

pub use self::image16bpp::{Image16BPP, ImageType16BPP};
pub use self::image1bpp::{Image1BPP, ImageType1BPP};
pub use self::image8bpp::{Image8BPP, ImageType8BPP};
pub use self::image_bmp::ImageBmp;
