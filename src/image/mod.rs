use super::drawable::Coord;

mod image1bpp;
mod image8bpp;

// TODO: Add to crate prelude
pub trait Image<'a> {
    fn new(imagedata: &'a [u8], width: u32, height: u32, offset: Coord) -> Self;
}

pub use self::image1bpp::Image1BPP;
pub use self::image8bpp::Image8BPP;
