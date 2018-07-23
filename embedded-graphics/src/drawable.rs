//! `Drawable` trait and helpers

use core::ops::{Add, AddAssign, Sub, SubAssign};

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

impl Add for Coord {
    type Output = Coord;

    fn add(self, other: Coord) -> Coord {
        Coord::new(self.0 + other.0, self.1 + other.1)
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, other: Coord) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

impl Sub for Coord {
    type Output = Coord;

    fn sub(self, other: Coord) -> Coord {
        Coord::new(self.0 - other.0, self.1 - other.1)
    }
}

impl SubAssign for Coord {
    fn sub_assign(&mut self, other: Coord) {
        self.0 -= other.0;
        self.1 -= other.1;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coords_can_be_added() {
        let left = Coord::new(10, 20);
        let right = Coord::new(30, 40);

        assert_eq!(left + right, Coord::new(40, 60));
    }

    #[test]
    fn coords_can_be_subtracted() {
        let left = Coord::new(30, 40);
        let right = Coord::new(10, 20);

        assert_eq!(left - right, Coord::new(20, 20));
    }
}
