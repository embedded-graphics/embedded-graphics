/// A single pixel of a TGA image
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Pixel {
    /// Pixel X coordinate from top left of image
    pub x: u32,

    /// Pixel Y coordinate from top left of image
    pub y: u32,

    /// Pixel color
    pub color: u32,
}
