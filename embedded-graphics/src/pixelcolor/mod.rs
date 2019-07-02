//! Pixel color

mod binary_color;
mod conversion;
mod gray_color;
mod rgb_color;

pub use binary_color::*;
pub use gray_color::*;
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
///
/// This trait is used to generically convert raw color data with different
/// bitdepth to specific color types. If the number of bits required to build
/// the color is less than 32 the upper bits are ignored.
pub trait FromRawData {
    /// Convert raw data to color.
    fn from_raw_data(data: u32) -> Self;
}
