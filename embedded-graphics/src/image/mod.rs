//! Image object

use drawable::Dimensions;

mod image16bpp;
mod image1bpp;
mod image8bpp;

/// Image trait
pub trait Image<'a>: Dimensions {
    /// Create a new image with given pixel data, width and height
    fn new(imagedata: &'a [u8], width: u32, height: u32) -> Self;
}

pub use self::image16bpp::Image16BPP;
pub use self::image1bpp::Image1BPP;
pub use self::image8bpp::Image8BPP;
