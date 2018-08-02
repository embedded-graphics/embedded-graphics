//! Pixel color trait
pub trait PixelColor: Clone + Copy {}

impl PixelColor for u8 {}
