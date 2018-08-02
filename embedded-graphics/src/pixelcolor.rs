//! Pixel color trait

/// Pixel color
pub trait PixelColor: Clone + Copy + From<u8> {}

impl PixelColor for u8 {}

/// Pixel wrapper around `u8` type
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct PixelColorU8(pub u8);

impl PixelColor for PixelColorU8 {}

impl From<u8> for PixelColorU8 {
    fn from(other: u8) -> Self {
        PixelColorU8(other)
    }
}
