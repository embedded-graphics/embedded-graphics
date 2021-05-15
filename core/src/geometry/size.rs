use core::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Sub, SubAssign},
};

use crate::geometry::Point;

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
/// [`Drawable`]: super::drawable::Drawable
/// [`Vector2<N>`]: https://docs.rs/nalgebra/0.18.0/nalgebra/base/type.Vector2.html
/// [`Vector2`]: https://docs.rs/nalgebra/0.18.0/nalgebra/base/type.Vector2.html
/// [Nalgebra]: https://docs.rs/nalgebra
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
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

    /// Creates a size with width and height set to an equal value.
    ///
    /// ```rust
    /// use embedded_graphics::geometry::Size;
    ///
    /// let size = Size::new_equal(11);
    ///
    /// assert_eq!(
    ///     size,
    ///     Size {
    ///         width: 11,
    ///         height: 11
    ///     }
    /// );
    /// ```
    pub const fn new_equal(value: u32) -> Self {
        Size {
            width: value,
            height: value,
        }
    }

    /// Creates a size with width and height equal to zero.
    pub const fn zero() -> Self {
        Size {
            width: 0,
            height: 0,
        }
    }

    /// Returns a size with equal `width` value and `height` set to `0`.
    ///
    /// # Examples
    ///
    /// ## Move a `Point` along the X axis.
    ///
    /// ```rust
    /// use embedded_graphics::geometry::{Point, Size};
    ///
    /// let size = Size::new(20, 30);
    ///
    /// let point = Point::new(10, 15);
    ///
    /// let moved_x = point + size.x_axis();
    ///
    /// assert_eq!(moved_x, Point::new(30, 15));
    /// ```
    pub const fn x_axis(self) -> Self {
        Self {
            width: self.width,
            height: 0,
        }
    }

    /// Returns a size with equal `height` value and `width` set to `0`.
    ///
    /// # Examples
    ///
    /// ## Move a `Point` along the Y axis.
    ///
    /// ```rust
    /// use embedded_graphics::geometry::{Point, Size};
    ///
    /// let size = Size::new(20, 30);
    ///
    /// let point = Point::new(10, 15);
    ///
    /// let moved_y = point + size.y_axis();
    ///
    /// assert_eq!(moved_y, Point::new(10, 45));
    /// ```
    pub const fn y_axis(self) -> Self {
        Self {
            width: 0,
            height: self.height,
        }
    }

    /// Saturating addition.
    ///
    /// Returns `u32::max_value()` for `width` and/or `height` instead of overflowing.
    pub const fn saturating_add(self, other: Self) -> Self {
        Self {
            width: self.width.saturating_add(other.width),
            height: self.height.saturating_add(other.height),
        }
    }

    /// Saturating subtraction.
    ///
    /// Returns `0` for `width` and/or `height` instead of overflowing, if the
    /// value in `other` is larger then in `self`.
    pub const fn saturating_sub(self, other: Self) -> Self {
        Self {
            width: self.width.saturating_sub(other.width),
            height: self.height.saturating_sub(other.height),
        }
    }

    /// Division.
    ///
    /// This method provides a workaround for the `Div` trait not being usable in `const` contexts.
    pub(crate) const fn div_u32(self, rhs: u32) -> Size {
        Size::new(self.width / rhs, self.height / rhs)
    }

    /// Creates a size from two corner points of a bounding box.
    pub(crate) const fn from_bounding_box(corner_1: Point, corner_2: Point) -> Self {
        let width = (corner_1.x - corner_2.x).abs() as u32 + 1;
        let height = (corner_1.y - corner_2.y).abs() as u32 + 1;

        Self { width, height }
    }

    /// Returns the componentwise minimum of two `Size`s.
    ///
    /// ```rust
    /// use embedded_graphics::geometry::Size;
    ///
    /// let min = Size::new(20, 30).component_min(Size::new(15, 50));
    ///
    /// assert_eq!(min, Size::new(15, 30));
    /// ```
    pub fn component_min(self, other: Self) -> Self {
        Self::new(self.width.min(other.width), self.height.min(other.height))
    }

    /// Returns the componentwise maximum of two `Size`s.
    ///
    /// ```rust
    /// use embedded_graphics::geometry::Size;
    ///
    /// let min = Size::new(20, 30).component_max(Size::new(15, 50));
    ///
    /// assert_eq!(min, Size::new(20, 50));
    /// ```
    pub fn component_max(self, other: Self) -> Self {
        Self::new(self.width.max(other.width), self.height.max(other.height))
    }

    /// Returns the componentwise multiplication of two `Size`s.
    ///
    /// ```rust
    /// use embedded_graphics::geometry::Size;
    ///
    /// let result = Size::new(20, 30).component_mul(Size::new(2, 3));
    ///
    /// assert_eq!(result, Size::new(40, 90));
    /// ```
    pub const fn component_mul(self, other: Self) -> Self {
        Self::new(self.width * other.width, self.height * other.height)
    }

    /// Returns the componentwise division of two `Size`s.
    ///
    /// # Panics
    ///
    /// Panics if one of the components of `other` equals zero.
    ///
    /// ```rust
    /// use embedded_graphics::geometry::Size;
    ///
    /// let result = Size::new(20, 30).component_div(Size::new(5, 10));
    ///
    /// assert_eq!(result, Size::new(4, 3));
    /// ```
    pub const fn component_div(self, other: Self) -> Self {
        Self::new(self.width / other.width, self.height / other.height)
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

impl Mul<u32> for Size {
    type Output = Size;

    fn mul(self, rhs: u32) -> Size {
        Size::new(self.width * rhs, self.height * rhs)
    }
}

impl MulAssign<u32> for Size {
    fn mul_assign(&mut self, rhs: u32) {
        self.width *= rhs;
        self.height *= rhs;
    }
}

impl Div<u32> for Size {
    type Output = Size;

    fn div(self, rhs: u32) -> Size {
        self.div_u32(rhs)
    }
}

impl DivAssign<u32> for Size {
    fn div_assign(&mut self, rhs: u32) {
        self.width /= rhs;
        self.height /= rhs;
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

impl From<Size> for [u32; 2] {
    fn from(other: Size) -> [u32; 2] {
        [other.width, other.height]
    }
}

impl From<&Size> for (u32, u32) {
    fn from(other: &Size) -> (u32, u32) {
        (other.width, other.height)
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} x {}", self.width, self.height)
    }
}

#[cfg(feature = "nalgebra_support")]
use nalgebra::{base::Scalar, Vector2};

#[cfg(feature = "nalgebra_support")]
impl<N> From<Vector2<N>> for Size
where
    N: Into<u32> + Scalar + Copy,
{
    fn from(other: Vector2<N>) -> Self {
        Self::new(other[0].into(), other[1].into())
    }
}

#[cfg(feature = "nalgebra_support")]
impl<N> From<&Vector2<N>> for Size
where
    N: Into<u32> + Scalar + Copy,
{
    fn from(other: &Vector2<N>) -> Self {
        Self::new(other[0].into(), other[1].into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use core::fmt::Write;

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
    fn saturating_sub() {
        let p = Size::new(10, 20);

        assert_eq!(p.saturating_sub(Size::new(9, 18)), Size::new(1, 2));
        assert_eq!(p.saturating_sub(Size::new(11, 18)), Size::new(0, 2));
        assert_eq!(p.saturating_sub(Size::new(9, 21)), Size::new(1, 0));
        assert_eq!(p.saturating_sub(Size::new(11, 21)), Size::new(0, 0));
    }

    #[test]
    fn sizes_can_be_multiplied_by_scalar() {
        let s = Size::new(1, 2);
        assert_eq!(s * 3, Size::new(3, 6));

        let mut s = Size::new(2, 3);
        s *= 4;
        assert_eq!(s, Size::new(8, 12));
    }

    #[test]
    fn sizes_can_be_divided_by_scalar() {
        let s = Size::new(10, 20);
        assert_eq!(s / 2, Size::new(5, 10));

        let mut s = Size::new(20, 30);
        s /= 5;
        assert_eq!(s, Size::new(4, 6));
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
    fn to_array() {
        let array: [u32; 2] = Size::new(20, 30).into();

        assert_eq!(array, [20, 30]);
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

    #[test]
    fn component_min_max() {
        let a = Size::new(20, 30);
        let b = Size::new(15, 50);

        assert_eq!(a.component_min(b), Size::new(15, 30));
        assert_eq!(a.component_max(b), Size::new(20, 50));
    }

    #[test]
    fn display() {
        let mut buffer = arrayvec::ArrayString::<32>::new();
        write!(buffer, "{}", Size::new(123, 456)).unwrap();

        assert_eq!(&buffer, "123 x 456");
    }
}
