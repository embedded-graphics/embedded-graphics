//! Image object

mod image;
mod image16bpp;
mod image1bpp;
mod image8bpp;
mod image_bmp;

pub use self::image::{Image, ImageFile};

pub use self::image16bpp::Image16BPP;
pub use self::image1bpp::Image1BPP;
pub use self::image8bpp::Image8BPP;
pub use self::image_bmp::ImageBmp;
