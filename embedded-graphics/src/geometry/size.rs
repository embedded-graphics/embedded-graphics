use crate::geometry::Point;
use core::ops::{Add, AddAssign, Index, Sub, SubAssign};

/// 2D size.
///
/// `Size` is used to define the width and height of an object.
///
/// [Nalgebra] support can be enabled with the `nalgebra_support` feature. This implements
/// `From<Vector2<N>>` and `From<&Vector2<N>>` where `N` is `Scalar + Into<u32>`. This allows use
/// of Nalgebra's [`Vector2`] with embedded-graphics where `u32`, `u16` or `u8` is used for value
/// storage.
///
/// # Examples
///
/// ## Create a `Size` from two integers
///
///
/// ```rust
/// use embedded_graphics::geometry::Size;
///
/// // Create a size using the `new` constructor method
/// let s = Size::new(10, 20);
/// ```
///
/// ## Create a `Size` from a Nalgebra `Vector2`
///
/// _Be sure to enable the `nalgebra_support` feature to get [Nalgebra] integration._
///
/// Any `Vector2<N>` can be used where `N: Into<u32> + nalgebra::Scalar`. This includes the primitive types `u32`, `u16` and `u8`.
///
/// ```rust
/// # #[cfg(feature = "nalgebra_support")] {
/// use embedded_graphics::geometry::Size;
/// use nalgebra::Vector2;
///
/// assert_eq!(Size::from(Vector2::new(10u32, 20)), Size::new(10u32, 20));
/// assert_eq!(Size::from(Vector2::new(10u16, 20)), Size::new(10u32, 20));
/// assert_eq!(Size::from(Vector2::new(10u8, 20)), Size::new(10u32, 20));
/// # }
/// ```
///
/// `.into()` can also be used, but may require more type annotations:
///
/// ```rust
/// # #[cfg(feature = "nalgebra_support")] {
/// use embedded_graphics::geometry::Size;
/// use nalgebra::Vector2;
///
/// let c: Size = Vector2::new(10u32, 20).into();
///
/// assert_eq!(c, Size::new(10u32, 20));
/// # }
/// ```
///
/// [`Drawable`]: ../drawable/trait.Drawable.html
/// [`Point`]: struct.Point.html
/// [`Vector2<N>`]: https://docs.rs/nalgebra/0.18.0/nalgebra/base/type.Vector2.html
/// [`Vector2`]: https://docs.rs/nalgebra/0.18.0/nalgebra/base/type.Vector2.html
/// [Nalgebra]: https://docs.rs/nalgebra
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Size {
    /// The width.
    pub width: u32,

    /// The height.
    pub height: u32,
}

impl Size {
    /// Creates a size from a width and a height.
    pub const fn new(width: u32, height: u32) -> Self {
        Size { width, height }
    }

    /// Creates a size with width and height equal to zero.
    pub const fn zero() -> Self {
        Size {
            width: 0,
            height: 0,
        }
    }

    /// Creates a size from two corner points of a bounding box.
    pub(crate) fn from_bounding_box(corner_1: Point, corner_2: Point) -> Self {
        let width = (corner_1.x - corner_2.x).abs() as u32;
        let height = (corner_1.y - corner_2.y).abs() as u32;

        Self { width, height }
    }
}

impl Add for Size {
    type Output = Size;

    fn add(self, other: Size) -> Size {
        Size::new(self.width + other.width, self.height + other.height)
    }
}

impl AddAssign for Size {
    fn add_assign(&mut self, other: Size) {
        self.width += other.width;
        self.height += other.height;
    }
}

impl Sub for Size {
    type Output = Size;

    fn sub(self, other: Size) -> Size {
        Size::new(self.width - other.width, self.height - other.height)
    }
}

impl SubAssign for Size {
    fn sub_assign(&mut self, other: Size) {
        self.width -= other.width;
        self.height -= other.height;
    }
}

impl Index<usize> for Size {
    type Output = u32;

    fn index(&self, idx: usize) -> &u32 {
        match idx {
            0 => &self.width,
            1 => &self.height,
            _ => panic!("index out of bounds: the len is 2 but the index is {}", idx),
        }
    }
}

impl From<(u32, u32)> for Size {
    fn from(other: (u32, u32)) -> Self {
        Size::new(other.0, other.1)
    }
}

impl From<[u32; 2]> for Size {
    fn from(other: [u32; 2]) -> Self {
        Size::new(other[0], other[1])
    }
}

impl From<&[u32; 2]> for Size {
    fn from(other: &[u32; 2]) -> Self {
        Size::new(other[0], other[1])
    }
}

impl From<Size> for (u32, u32) {
    fn from(other: Size) -> (u32, u32) {
        (other.width, other.height)
    }
}

impl From<&Size> for (u32, u32) {
    fn from(other: &Size) -> (u32, u32) {
        (other.width, other.height)
    }
}

#[cfg(feature = "nalgebra_support")]
use nalgebra::{base::Scalar, Vector2};

#[cfg(feature = "nalgebra_support")]
impl<N> From<Vector2<N>> for Size
where
    N: Into<u32> + Scalar,
{
    fn from(other: Vector2<N>) -> Self {
        Self::new(other[0].into(), other[1].into())
    }
}

#[cfg(feature = "nalgebra_support")]
impl<N> From<&Vector2<N>> for Size
where
    N: Into<u32> + Scalar,
{
    fn from(other: &Vector2<N>) -> Self {
        Self::new(other[0].into(), other[1].into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizes_can_be_added() {
        let left = Size::new(10, 20);
        let right = Size::new(30, 40);

        assert_eq!(left + right, Size::new(40, 60));
    }

    #[test]
    fn sizes_can_be_subtracted() {
        let left = Size::new(30, 40);
        let right = Size::new(10, 20);

        assert_eq!(left - right, Size::new(20, 20));
    }

    #[test]
    fn from_tuple() {
        assert_eq!(Size::from((20, 30)), Size::new(20, 30));
    }

    #[test]
    fn from_array() {
        assert_eq!(Size::from([20, 30]), Size::new(20, 30));
    }

    #[test]
    fn from_array_ref() {
        assert_eq!(Size::from(&[20, 30]), Size::new(20, 30));
    }

    #[test]
    fn index() {
        let size = Size::new(1, 2);

        assert_eq!(size.width, size[0]);
        assert_eq!(size.height, size[1]);
    }

    #[test]
    #[should_panic]
    fn index_out_of_bounds() {
        let size = Size::new(1, 2);
        let _ = size[2];
    }

    #[test]
    #[cfg(feature = "nalgebra_support")]
    fn nalgebra_support() {
        let left = nalgebra::Vector2::new(30u32, 40);
        let right = nalgebra::Vector2::new(10, 20);

        assert_eq!(Size::from(left - right), Size::new(20, 20));
    }
}
