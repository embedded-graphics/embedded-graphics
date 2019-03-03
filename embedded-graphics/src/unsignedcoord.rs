//! 2D unsigned coordinate

use crate::coord::Coord;

type UnsignedCoordPart = u32;

#[cfg(not(feature = "nalgebra_support"))]
mod internal_unsigned_coord {
    use super::UnsignedCoordPart;
    use core::ops::{Add, AddAssign, Index, Sub, SubAssign};

    /// 2D unsigned coordinate in screen space
    ///
    /// As opposed to [`Coord`](../coord/index.html), this coordinate is unsigned. It is intended for
    /// use with [`Drawable`](../drawable/trait.Drawable.html) iterators to output valid _display pixel_
    /// coordinates, i.e. coordinates that are always positive.
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct UnsignedCoord(pub UnsignedCoordPart, pub UnsignedCoordPart);

    impl UnsignedCoord {
        /// Create a new coordinate with X and Y values
        pub fn new(x: UnsignedCoordPart, y: UnsignedCoordPart) -> Self {
            UnsignedCoord(x, y)
        }
    }

    impl Add for UnsignedCoord {
        type Output = UnsignedCoord;

        fn add(self, other: UnsignedCoord) -> UnsignedCoord {
            UnsignedCoord::new(self.0 + other.0, self.1 + other.1)
        }
    }

    impl AddAssign for UnsignedCoord {
        fn add_assign(&mut self, other: UnsignedCoord) {
            self.0 += other.0;
            self.1 += other.1;
        }
    }

    impl Sub for UnsignedCoord {
        type Output = UnsignedCoord;

        fn sub(self, other: UnsignedCoord) -> UnsignedCoord {
            UnsignedCoord::new(self.0 - other.0, self.1 - other.1)
        }
    }

    impl SubAssign for UnsignedCoord {
        fn sub_assign(&mut self, other: UnsignedCoord) {
            self.0 -= other.0;
            self.1 -= other.1;
        }
    }

    impl Index<usize> for UnsignedCoord {
        type Output = UnsignedCoordPart;

        fn index(&self, idx: usize) -> &UnsignedCoordPart {
            match idx {
                0 => &self.0,
                1 => &self.1,
                _ => panic!("Unreachable index {}", idx),
            }
        }
    }
}

#[cfg(not(feature = "nalgebra_support"))]
pub use self::internal_unsigned_coord::UnsignedCoord;

#[cfg(feature = "nalgebra_support")]
use nalgebra;

#[cfg(feature = "nalgebra_support")]
/// 2D coordinate type with Nalgebra support
pub type UnsignedCoord = nalgebra::Vector2<UnsignedCoordPart>;

/// Convert an unsigned coordinate to a signed representation
///
/// The coordinate is guaranteed to be positive. This trait is provided as a convenience method for
/// converting between signed/unsigned coords.
pub trait ToSigned {
    /// Convert self into a signed `Coord`
    fn to_signed(self) -> Coord;
}

impl ToSigned for UnsignedCoord {
    fn to_signed(self) -> Coord {
        Coord::new(self[0] as i32, self[1] as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coords_can_be_added() {
        let left = UnsignedCoord::new(10, 20);
        let right = UnsignedCoord::new(30, 40);

        assert_eq!(left + right, UnsignedCoord::new(40, 60));
    }

    #[test]
    fn coords_can_be_subtracted() {
        let left = UnsignedCoord::new(30, 40);
        let right = UnsignedCoord::new(10, 20);

        assert_eq!(left - right, UnsignedCoord::new(20, 20));
    }

    #[test]
    #[cfg(feature = "nalgebra_support")]
    fn nalgebra_support() {
        let left = nalgebra::Vector2::new(30, 40);
        let right = nalgebra::Vector2::new(10, 20);

        assert_eq!(left - right, UnsignedCoord::new(20, 20));
    }
}
