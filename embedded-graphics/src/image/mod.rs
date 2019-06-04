//! Image object
//!
//! If the source image data is in BMP format, use [`ImageBMP`](struct.ImageBmp.html).
//!
//! If the source image data is a slice of raw bytes that represents a 1, 8 or 16 bits-per-pixel
//! (BPP) image, use [`Image1BPP`](type.Image1BPP.html), [`Image8BPP`](type.Image8BPP.html),
//! or [`Image16BPP`](type.Image16BPP.html) respectively.

mod image;
mod image16bpp;
mod image1bpp;
mod image8bpp;
mod image_bmp;
mod image_tga;

pub use self::image::{Image, ImageFile};

pub use self::image16bpp::Image16BPP;
pub use self::image1bpp::Image1BPP;
pub use self::image8bpp::Image8BPP;
pub use self::image_bmp::ImageBmp;
pub use self::image_tga::ImageTga;
