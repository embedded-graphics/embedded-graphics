//! Image object

use crate::drawable::Dimensions;

mod image16bpp;
mod image1bpp;
mod image8bpp;
mod image_bmp;

/// Raw image trait
pub trait Image<'a>: Dimensions {
    /// Create a new image with given pixel data, width and height
    fn new(imagedata: &'a [u8], width: u32, height: u32) -> Self;
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

pub use self::image16bpp::Image16BPP;
pub use self::image1bpp::Image1BPP;
pub use self::image8bpp::Image8BPP;
pub use self::image_bmp::ImageBmp;
