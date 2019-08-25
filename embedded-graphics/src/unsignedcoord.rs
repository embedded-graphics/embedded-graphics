//! 2D unsigned coordinate

use crate::coord::Coord;

use core::ops::{Add, AddAssign, Index, Neg, Sub, SubAssign};

/// 2D unsigned coordinate in screen space
///
/// Unlike [`Coord`](../coord/index.html), this coordinate is unsigned. It is intended for
/// use with [`Drawable`](../drawable/trait.Drawable.html) iterators to output valid _display pixel_
/// coordinates, i.e. coordinates that are always positive.
///
/// [Nalgebra] support can be enabled with the `nalgebra_support` feature. This implements
/// `From<Vector2<N>>` and `From<&Vector2<N>>` where `N` is `Scalar + Into<u32>`. This allows use
/// of Nalgebra's [`Vector2`] with embedded-graphics where `u32`, `u16` or `u8` is used for value
/// storage.
///
/// # Examples
///
/// ## Create an `UnsignedCoord` from two integers
///
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
/// ## Create an `UnsignedCoord` from a Nalgebra `Vector2`
///
/// _Be sure to enable the `nalgebra_support` feature to get [Nalgebra] integration._
///
/// Any `Vector2<N>` can be used where `N: Into<u32> + nalgebra::Scalar`. This includes the primitive types `u32`, `u16` and `u8`.
///
/// ```rust
/// # #[cfg(feature = "nalgebra_support")] {
/// use nalgebra::Vector2;
/// use embedded_graphics::unsignedcoord::UnsignedCoord;
///
/// assert_eq!(UnsignedCoord::from(Vector2::new(10u32, 20)), UnsignedCoord::new(10u32, 20));
/// assert_eq!(UnsignedCoord::from(Vector2::new(10u16, 20)), UnsignedCoord::new(10u32, 20));
/// assert_eq!(UnsignedCoord::from(Vector2::new(10u8, 20)), UnsignedCoord::new(10u32, 20));
/// # }
/// ```
///
/// `.into()` can also be used, but may require more type annotations:
///
/// ```rust
/// # #[cfg(feature = "nalgebra_support")] {
/// use nalgebra::Vector2;
/// use embedded_graphics::unsignedcoord::UnsignedCoord;
///
/// let c: UnsignedCoord = Vector2::new(10u32, 20).into();
///
/// assert_eq!(c, UnsignedCoord::new(10u32, 20));
/// # }
/// ```
///
/// [`Vector2<N>`]: https://docs.rs/nalgebra/0.18.0/nalgebra/base/type.Vector2.html
/// [`Vector2`]: https://docs.rs/nalgebra/0.18.0/nalgebra/base/type.Vector2.html
/// [Nalgebra]: https://docs.rs/nalgebra
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct UnsignedCoord(pub u32, pub u32);

impl UnsignedCoord {
    /// Create a new coordinate with X and Y values
    pub const fn new(x: u32, y: u32) -> Self {
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
    type Output = u32;

    fn index(&self, idx: usize) -> &u32 {
        match idx {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("index out of bounds: the len is 2 but the index is {}", idx),
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

#[cfg(feature = "nalgebra_support")]
use nalgebra::{base::Scalar, Vector2};

#[cfg(feature = "nalgebra_support")]
impl<N> From<Vector2<N>> for UnsignedCoord
where
    N: Into<u32> + Scalar,
{
    fn from(other: Vector2<N>) -> Self {
        Self::new(other[0].into(), other[1].into())
    }
}

#[cfg(feature = "nalgebra_support")]
impl<N> From<&Vector2<N>> for UnsignedCoord
where
    N: Into<u32> + Scalar,
{
    fn from(other: &Vector2<N>) -> Self {
        Self::new(other[0].into(), other[1].into())
    }
}

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
    fn from_tuple() {
        assert_eq!(UnsignedCoord::from((20, 30)), UnsignedCoord::new(20, 30));
    }

    #[test]
    fn from_array() {
        assert_eq!(UnsignedCoord::from([20, 30]), UnsignedCoord::new(20, 30));
    }

    #[test]
    fn from_array_ref() {
        assert_eq!(UnsignedCoord::from(&[20, 30]), UnsignedCoord::new(20, 30));
    }

    #[test]
    #[cfg(feature = "nalgebra_support")]
    fn nalgebra_support() {
        let left = nalgebra::Vector2::new(30u32, 40);
        let right = nalgebra::Vector2::new(10, 20);

        assert_eq!(
            UnsignedCoord::from(left - right),
            UnsignedCoord::new(20, 20)
        );
    }

    #[test]
    fn neg() {
        assert_eq!(-UnsignedCoord::new(10, 20), Coord::new(-10, -20));
    }
}
