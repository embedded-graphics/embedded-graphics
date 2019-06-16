//! 2D unsigned coordinate

use crate::coord::Coord;

type UnsignedCoordPart = u32;

#[cfg(not(feature = "nalgebra_support"))]
mod internal_unsigned_coord {
    use super::UnsignedCoordPart;
    use crate::coord::Coord;
    use core::ops::{Add, AddAssign, Index, Neg, Sub, SubAssign};

    /// 2D unsigned coordinate in screen space
    ///
    /// As opposed to [`Coord`](../coord/index.html), this coordinate is unsigned. It is intended for
    /// use with [`Drawable`](../drawable/trait.Drawable.html) iterators to output valid _display pixel_
    /// coordinates, i.e. coordinates that are always positive.
    ///
    /// ```rust
    /// use embedded_graphics::{unsignedcoord::UnsignedCoord, ucoord};
    ///
    /// // Create a coord using the `new` constructor method
    /// let c1 = UnsignedCoord::new(10, 20);
    ///
    /// // Create a coord using the handy `ucoord` macro
    /// let c2 = ucoord!(10, 20);
    ///
    /// assert_eq!(c1, c2);
    /// ```
    ///
    /// Note that enabling the `nalgebra` feature will alias Nalgebra's [`Vector2<u32>`] type to
    /// `UnsignedCoord` instead of this builtin implementation.
    ///
    /// [`Vector2<u32>`]: https://docs.rs/nalgebra/0.18.0/nalgebra/base/type.Vector2.html
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
            UnsignedCoord(self.0 + other.0, self.1 + other.1)
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
            UnsignedCoord(self.0 - other.0, self.1 - other.1)
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

    impl Neg for UnsignedCoord {
        type Output = Coord;

        fn neg(self) -> Self::Output {
            Coord::new(-(self.0 as i32), -(self.1 as i32))
        }
    }

    impl From<(u32, u32)> for UnsignedCoord {
        fn from(other: (u32, u32)) -> Self {
            Self(other.0, other.1)
        }
    }

    impl From<[u32; 2]> for UnsignedCoord {
        fn from(other: [u32; 2]) -> Self {
            Self(other[0], other[1])
        }
    }

    impl From<&[u32; 2]> for UnsignedCoord {
        fn from(other: &[u32; 2]) -> Self {
            Self(other[0], other[1])
        }
    }

    impl From<UnsignedCoord> for (u32, u32) {
        fn from(other: UnsignedCoord) -> (u32, u32) {
            (other.0, other.1)
        }
    }

    impl From<&UnsignedCoord> for (u32, u32) {
        fn from(other: &UnsignedCoord) -> (u32, u32) {
            (other.0, other.1)
        }
    }
}

/// Create an `UnsignedCoord` from a pair of integer values
///
/// Input values must be `u32`s
///
/// ```rust
/// use embedded_graphics::{unsignedcoord::UnsignedCoord, ucoord};
///
/// let c: UnsignedCoord = ucoord!(20, 30);
/// ```
#[macro_export]
macro_rules! ucoord {
    ($x:expr, $y:expr) => {
        $crate::unsignedcoord::UnsignedCoord::new($x, $y)
    };
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
    #[cfg(not(feature = "nalgebra_support"))]
    fn from_tuple() {
        assert_eq!(UnsignedCoord::from((20, 30)), UnsignedCoord::new(20, 30));
    }

    #[test]
    fn from_array() {
        assert_eq!(UnsignedCoord::from([20, 30]), UnsignedCoord::new(20, 30));
    }

    #[test]
    #[cfg(not(feature = "nalgebra_support"))]
    fn from_array_ref() {
        assert_eq!(UnsignedCoord::from(&[20, 30]), UnsignedCoord::new(20, 30));
    }

    #[test]
    #[cfg(feature = "nalgebra_support")]
    fn nalgebra_support() {
        let left = nalgebra::Vector2::new(30, 40);
        let right = nalgebra::Vector2::new(10, 20);

        assert_eq!(left - right, UnsignedCoord::new(20, 20));
    }

    #[test]
    #[cfg(not(feature = "nalgebra_support"))]
    fn neg() {
        assert_eq!(-UnsignedCoord::new(10, 20), Coord::new(-10, -20));
    }
}
