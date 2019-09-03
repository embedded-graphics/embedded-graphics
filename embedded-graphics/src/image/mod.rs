//! Image drawables.
//!
//! Image drawables can be created for raw bitmap data and images in BMP and TGA
//! format.

#[cfg(feature = "bmp")]
mod image_bmp;
mod image_raw;
#[cfg(feature = "tga")]
mod image_tga;

pub use self::image_raw::{Image, ImageBE, ImageLE};

#[cfg(feature = "bmp")]
pub use self::image_bmp::ImageBmp;
#[cfg(feature = "tga")]
pub use self::image_tga::ImageTga;

/// Image file trait.
pub trait ImageFile<'a>: crate::Dimensions + Sized {
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
