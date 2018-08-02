//! Pixel color trait

/// Pixel color
pub trait PixelColor: Clone + Copy + From<u8> {}

impl PixelColor for u8 {}
