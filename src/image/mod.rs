mod image1bpp;
mod image8bpp;

pub trait Image { }

pub use self::image1bpp::Image1BPP;
pub use self::image8bpp::Image8BPP;