//! `Drawable` trait and helpers

#[cfg(feature = "nalgebra_support")]
use nalgebra;

/// 2D coordinate type
#[cfg(not(feature = "nalgebra_support"))]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Coord(pub u32, pub u32);

impl Coord {
    /// Create a new coordinate with X and Y values
    pub fn new(x: u32, y: u32) -> Self {
        Coord(x, y)
    }
}

/// 2D coordinate type
#[cfg(feature = "nalgebra_support")]
pub type Coord = nalgebra::Vector2<u32>;

// TODO: Refactor to use both with monochrome and multicolour displays
/// Monochrome colour type
pub type Color = u8;

/// A single pixel
pub type Pixel = (Coord, Color);

/// Marks an object as "drawable". Must be implemented for all graphics objects
pub trait Drawable {}
