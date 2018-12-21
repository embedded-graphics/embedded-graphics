//! 2D signed coordinate in screen space
//!
//! This struct is what you should use when positioning graphics elements as it can handle negative
//! coordinates. To output pixel coordinates, use [`UnsignedCoord`](../unsignedcoord/index.html).

use super::unsignedcoord::UnsignedCoord;

type CoordPart = i32;

#[cfg(not(feature = "nalgebra_support"))]
mod internal_coord {
    use super::*;
    use core::ops::{Add, AddAssign, Index, Sub, SubAssign};

    /// 2D signed integer coordinate type
    ///
    /// This coordinate should be used to define graphics object coordinates. For example, a
    /// [`Rect`] may be defined that has its top left at `(-1,-2)`. To specify positive-only screen
    /// coordinates and the like, see [`UnsignedCoord`]
    ///
    /// [`UnsignedCoord`]: ../unsignedcoord/struct.UnsignedCoord.html
    /// [`Rect`]: ../primitives/rect/struct.Rect.html
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct Coord(pub CoordPart, pub CoordPart);

    impl Coord {
        /// Create a new coordinate with X and Y values
        pub fn new(x: CoordPart, y: CoordPart) -> Self {
            Coord(x, y)
        }

        /// Clamp coordinate components to positive integer range
        pub fn clamp_positive(&self) -> Self {
            Coord::new(self.0.max(0), self.1.max(0))
        }

        /// Remove the sign from a coordinate
        ///
        ///
        /// ```
        /// # use embedded_graphics::coord::Coord;
        /// #
        /// let coord = Coord::new(-5, -10);
        ///
        /// assert_eq!(coord.abs(), Coord::new(5, 10));
        /// ```
        pub fn abs(&self) -> Self {
            Coord::new(self.0.abs(), self.1.abs())
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

    impl Index<usize> for Coord {
        type Output = CoordPart;

        fn index(&self, idx: usize) -> &CoordPart {
            match idx {
                0 => &self.0,
                1 => &self.1,
                _ => panic!("Unreachable index {}", idx),
            }
        }
    }
}

#[cfg(not(feature = "nalgebra_support"))]
pub use self::internal_coord::Coord;

#[cfg(feature = "nalgebra_support")]
use nalgebra;

/// 2D coordinate type with Nalgebra support
#[cfg(feature = "nalgebra_support")]
pub type Coord = nalgebra::Vector2<CoordPart>;

/// Convert a value to an unsigned coordinate
pub trait ToUnsigned {
    /// Convert the signed coordinate to an unsigned coordinate
    fn to_unsigned(self) -> UnsignedCoord;
}

impl ToUnsigned for Coord {
    /// Convert to a positive-only coordinate, clamping negative values to zero
    ///
    /// # use embedded_graphics::coord::Coord;
    /// # use embedded_graphics::unsignedcoord::UnsignedCoord;
    /// #
    /// let coord = Coord::new(-5, 10);
    ///
    /// assert_eq!(coord.to_unsigned(), UnsignedCoord::new(0, 10));
    fn to_unsigned(self) -> UnsignedCoord {
        UnsignedCoord::new(self[0].max(0) as u32, self[1].max(0) as u32)
    }
}

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

    #[test]
    fn coords_can_be_negative_subtracted() {
        let left = Coord::new(10, 20);
        let right = Coord::new(30, 40);

        assert_eq!(left - right, Coord::new(-20, -20));
    }

    #[test]
    #[cfg(feature = "nalgebra_support")]
    fn nalgebra_support() {
        let left = nalgebra::Vector2::new(30, 40);
        let right = nalgebra::Vector2::new(10, 20);

        assert_eq!(left - right, Coord::new(20, 20));
    }
}
