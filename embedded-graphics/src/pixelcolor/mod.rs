//! Pixel color

mod binary_color;
mod luma_color;
mod rgb_color;

pub use binary_color::*;
pub use luma_color::*;
pub use rgb_color::*;

use core::fmt;

/// Pixel color trait
pub trait PixelColor: Clone + Copy + PartialEq + fmt::Debug {
    /// Default background color
    const DEFAULT_BG: Self;

    /// Default foreground color
    const DEFAULT_FG: Self;
}

/// Convert raw data to color structs.
pub trait FromSlice {
    /// Convert big endian data to color.
    fn from_be_slice(data: &[u8]) -> Self;

    /// Convert little endian data to color.
    fn from_le_slice(data: &[u8]) -> Self;
}
