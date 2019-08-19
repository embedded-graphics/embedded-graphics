//! 2D signed coordinate in screen space
//!
//! To output (non-negative) pixel coordinates, use [`UnsignedCoord`](../unsignedcoord/index.html).

use crate::unsignedcoord::UnsignedCoord;
use core::ops::{Add, AddAssign, Index, Neg, Sub, SubAssign};

/// 2D signed integer coordinate type
///
/// This coordinate should be used to define graphics object coordinates. For example, a
/// [`Rect`] may be defined that has its top left at `(-1,-2)`. To specify positive-only screen
/// coordinates and the like, see [`UnsignedCoord`].
///
/// [Nalgebra] support can be enabled with the `nalgebra_support` feature. This implements
/// `From<Vector2<N>>` and `From<&Vector2<N>>` where `N` is `Scalar + Into<i32>`. This allows use
/// of Nalgebra's [`Vector2`] with embedded-graphics where `i8`, `i16`, `i32`, `u16` or `u8` is used
/// for value storage.
///
/// # Examples
///
/// ## Create a `Coord` from two integers
///
/// ```rust
/// use embedded_graphics::{coord::Coord, icoord};
///
/// // Create a coord using the `new` constructor method
/// let c1 = Coord::new(10, 20);
///
/// // Create a coord using the handy `icoord` macro
/// let c2 = icoord!(10, 20);
///
/// assert_eq!(c1, c2);
/// ```
///
/// ## Create a `Coord` from a Nalgebra `Vector2`
///
/// _Be sure to enable the `nalgebra_support` feature to get [Nalgebra] integration._
///
/// ```rust
/// # #[cfg(feature = "nalgebra_support")] {
/// use embedded_graphics::{coord::Coord, icoord};
/// use nalgebra::Vector2;
///
/// let n_coord = Vector2::new(10i32, 20);
///
/// assert_eq!(Coord::from(n_coord), Coord::new(10, 20));
/// # }
/// ```
///
/// ## Convert a `Vector2<u8>` into a `Coord`
///
/// _Be sure to enable the `nalgebra_support` feature to get [Nalgebra] integration._
///
/// Smaller unsigned types that can be converted to `i32` are also supported in conversions.
///
/// ```rust
/// # #[cfg(feature = "nalgebra_support")] {
/// use embedded_graphics::{coord::Coord, icoord};
/// use nalgebra::Vector2;

/// let n_coord = Vector2::new(10u8, 20);
///
/// assert_eq!(Coord::from(n_coord), Coord::new(10, 20));
/// # }
/// ```
///
/// [`UnsignedCoord`]: ../unsignedcoord/struct.UnsignedCoord.html
/// [`Rect`]: ../primitives/rectangle/struct.Rectangle.html
/// [`Vector2<N>`]: https://docs.rs/nalgebra/0.18.0/nalgebra/base/type.Vector2.html
/// [`Vector2`]: https://docs.rs/nalgebra/0.18.0/nalgebra/base/type.Vector2.html
/// [Nalgebra]: https://docs.rs/nalgebra
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Coord(pub i32, pub i32);

impl Coord {
    /// Create a new coordinate with X and Y values
    pub fn new(x: i32, y: i32) -> Self {
        Coord(x, y)
    }

    /// Clamp coordinate components to positive integer range
    ///
    /// ```
    /// # use embedded_graphics::coord::Coord;
    /// #
    /// let coord = Coord::new(-5, 10);
    ///
    /// assert_eq!(coord.clamp_positive(), Coord::new(0, 10));
    /// ```
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
    type Output = i32;

    fn index(&self, idx: usize) -> &i32 {
        match idx {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("Unreachable index {}", idx),
        }
    }
}

impl Neg for Coord {
    type Output = Coord;

    fn neg(self) -> Self::Output {
        Coord::new(-self.0, -self.1)
    }
}

impl From<(u32, u32)> for Coord {
    fn from(other: (u32, u32)) -> Self {
        Self(other.0 as i32, other.1 as i32)
    }
}

impl From<[u32; 2]> for Coord {
    fn from(other: [u32; 2]) -> Self {
        Self(other[0] as i32, other[1] as i32)
    }
}

impl From<&[u32; 2]> for Coord {
    fn from(other: &[u32; 2]) -> Self {
        Self(other[0] as i32, other[1] as i32)
    }
}

impl From<(i32, i32)> for Coord {
    fn from(other: (i32, i32)) -> Self {
        Self(other.0, other.1)
    }
}

impl From<[i32; 2]> for Coord {
    fn from(other: [i32; 2]) -> Self {
        Self(other[0], other[1])
    }
}

impl From<&[i32; 2]> for Coord {
    fn from(other: &[i32; 2]) -> Self {
        Self(other[0], other[1])
    }
}

impl From<Coord> for (i32, i32) {
    fn from(other: Coord) -> (i32, i32) {
        (other.0, other.1)
    }
}

impl From<&Coord> for (i32, i32) {
    fn from(other: &Coord) -> (i32, i32) {
        (other.0, other.1)
    }
}

#[cfg(feature = "nalgebra_support")]
use nalgebra::{base::Scalar, Vector2};

#[cfg(feature = "nalgebra_support")]
impl<N> From<Vector2<N>> for Coord
where
    N: Into<i32> + Scalar,
{
    fn from(other: Vector2<N>) -> Self {
        Self::new(other[0].into(), other[1].into())
    }
}

#[cfg(feature = "nalgebra_support")]
impl<N> From<&Vector2<N>> for Coord
where
    N: Into<i32> + Scalar,
{
    fn from(other: &Vector2<N>) -> Self {
        Self::new(other[0].into(), other[1].into())
    }
}

/// Convert a value to an unsigned coordinate
pub trait ToUnsigned {
    /// Convert the signed coordinate to an unsigned coordinate
    fn to_unsigned(self) -> UnsignedCoord;
}

impl ToUnsigned for Coord {
    /// Convert to a positive-only coordinate, clamping negative values to zero
    ///
    /// ```rust
    /// # use embedded_graphics::coord::{ToUnsigned, Coord};
    /// # use embedded_graphics::unsignedcoord::UnsignedCoord;
    /// #
    /// let coord = Coord::new(-5, 10);
    ///
    /// assert_eq!(coord.to_unsigned(), UnsignedCoord::new(0, 10));
    /// ```
    fn to_unsigned(self) -> UnsignedCoord {
        UnsignedCoord::new(self[0].max(0) as u32, self[1].max(0) as u32)
    }
}

/// Create a `Coord` from a pair of integer values
///
/// Input values must be `i32`s
///
/// ```rust
/// use embedded_graphics::{coord::Coord, icoord};
///
/// let c: Coord = icoord!(20, 30);
/// ```
#[macro_export]
macro_rules! icoord {
    ($x:expr, $y:expr) => {
        $crate::coord::Coord::new($x, $y)
    };
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
    fn from_tuple() {
        assert_eq!(Coord::from((20u32, 30u32)), Coord::new(20, 30));
        assert_eq!(Coord::from((20i32, 30i32)), Coord::new(20, 30));
    }

    #[test]
    fn from_array() {
        assert_eq!(Coord::from([20u32, 30u32]), Coord::new(20, 30));
        assert_eq!(Coord::from([20i32, 30i32]), Coord::new(20, 30));
    }

    #[test]
    fn from_array_ref() {
        assert_eq!(Coord::from(&[20u32, 30u32]), Coord::new(20, 30));
        assert_eq!(Coord::from(&[20i32, 30i32]), Coord::new(20, 30));
    }

    #[test]
    #[cfg(feature = "nalgebra_support")]
    fn nalgebra_support() {
        let left = nalgebra::Vector2::new(30, 40);
        let right = nalgebra::Vector2::new(10, 20);

        assert_eq!(Coord::from(left - right), Coord::new(20, 20));
    }

    #[test]
    #[cfg(feature = "nalgebra_support")]
    fn convert_ref() {
        let left = nalgebra::Vector2::new(30, 40);
        let right = nalgebra::Vector2::new(10, 20);

        let c = left - right;

        assert_eq!(Coord::from(&c), Coord::new(20, 20));
    }

    #[test]
    fn neg() {
        assert_eq!(-Coord::new(10, 20), Coord::new(-10, -20));
        assert_eq!(-Coord::new(-40, -50), Coord::new(40, 50));
    }
}
