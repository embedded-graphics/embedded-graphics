use crate::geometry::Size;
use core::ops::{Add, AddAssign, Index, Neg, Sub, SubAssign};

/// 2D point.
///
/// A point can be used to define the position of a graphics object. For example, a [`Rectangle`]
/// may be defined that has its top left corner at `(-1, -2)`. To specify the size of an object
/// [`Size`] should be used instead.
///
/// [Nalgebra] support can be enabled with the `nalgebra_support` feature. This implements
/// `From<Vector2<N>>` and `From<&Vector2<N>>` where `N` is `Scalar + Into<i32>`. This allows use
/// of Nalgebra's [`Vector2`] with embedded-graphics where `i8`, `i16`, `i32`, `u16` or `u8` is used
/// for value storage.
///
/// # Examples
///
/// ## Create a `Point` from two integers
///
/// ```rust
/// use embedded_graphics::geometry::Point;
///
/// // Create a coord using the `new` constructor method
/// let p = Point::new(10, 20);
/// ```
///
/// ## Create a `Point` from a Nalgebra `Vector2`
///
/// _Be sure to enable the `nalgebra_support` feature to get [Nalgebra] integration._
///
/// ```rust
/// # #[cfg(feature = "nalgebra_support")] {
/// use embedded_graphics::geometry::Point;
/// use nalgebra::Vector2;
///
/// let n_coord = Vector2::new(10i32, 20);
///
/// assert_eq!(Point::from(n_coord), Point::new(10, 20));
/// # }
/// ```
///
/// ## Convert a `Vector2<u8>` into a `Point`
///
/// _Be sure to enable the `nalgebra_support` feature to get [Nalgebra] integration._
///
/// Smaller unsigned types that can be converted to `i32` are also supported in conversions.
///
/// ```rust
/// # #[cfg(feature = "nalgebra_support")] {
/// use embedded_graphics::geometry::Point;
/// use nalgebra::Vector2;

/// let n_coord = Vector2::new(10u8, 20);
///
/// assert_eq!(Point::from(n_coord), Point::new(10, 20));
/// # }
/// ```
///
/// [`Size`]: struct.Size.html
/// [`Rectangle`]: ../primitives/rectangle/struct.Rectangle.html
/// [`Vector2<N>`]: https://docs.rs/nalgebra/0.18.0/nalgebra/base/type.Vector2.html
/// [`Vector2`]: https://docs.rs/nalgebra/0.18.0/nalgebra/base/type.Vector2.html
/// [Nalgebra]: https://docs.rs/nalgebra
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Point {
    /// The x coordinate.
    pub x: i32,

    /// The y coordinate.
    pub y: i32,
}

impl Point {
    /// Creates a point from  X and Y coordinates.
    pub const fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    /// Creates a point with X and Y equal to zero.
    pub const fn zero() -> Self {
        Point { x: 0, y: 0 }
    }

    /// Remove the sign from a coordinate
    ///
    /// ```
    /// # use embedded_graphics::geometry::Point;
    /// #
    /// let point = Point::new(-5, -10);
    ///
    /// assert_eq!(point.abs(), Point::new(5, 10));
    /// ```
    pub fn abs(self) -> Self {
        Point::new(self.x.abs(), self.y.abs())
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl Add<Size> for Point {
    type Output = Point;

    /// Offsets a point by adding a size.
    ///
    /// # Panics
    ///
    /// This function will panic if `width` or `height` are too large to be represented as an `i32`
    /// and debug assertions are enabled.
    fn add(self, other: Size) -> Point {
        let width = other.width as i32;
        let height = other.height as i32;

        debug_assert!(width >= 0, "width is too large");
        debug_assert!(height >= 0, "height is too large");

        Point::new(self.x + width, self.y + height)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl AddAssign<Size> for Point {
    /// Offsets a point by adding a size.
    ///
    /// # Panics
    ///
    /// This function will panic if `width` or `height` are too large to be represented as an `i32`
    /// and debug assertions are enabled.
    fn add_assign(&mut self, other: Size) {
        let width = other.width as i32;
        let height = other.height as i32;

        debug_assert!(width >= 0, "width is too large");
        debug_assert!(height >= 0, "height is too large");

        self.x += width;
        self.y += height;
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl Sub<Size> for Point {
    type Output = Point;

    /// Offsets a point by subtracting a size.
    ///
    /// # Panics
    ///
    /// This function will panic if `width` or `height` are too large to be represented as an `i32`
    /// and debug assertions are enabled.
    fn sub(self, other: Size) -> Point {
        let width = other.width as i32;
        let height = other.height as i32;

        debug_assert!(width >= 0, "width is too large");
        debug_assert!(height >= 0, "height is too large");

        Point::new(self.x - width, self.y - height)
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Point) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl SubAssign<Size> for Point {
    /// Offsets a point by subtracting a size.
    ///
    /// # Panics
    ///
    /// This function will panic if `width` or `height` are too large to be represented as an `i32`
    /// and debug assertions are enabled.
    fn sub_assign(&mut self, other: Size) {
        let width = other.width as i32;
        let height = other.height as i32;

        debug_assert!(width >= 0, "width is too large");
        debug_assert!(height >= 0, "height is too large");

        self.x -= width;
        self.y -= height;
    }
}

impl Index<usize> for Point {
    type Output = i32;

    fn index(&self, idx: usize) -> &i32 {
        match idx {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("index out of bounds: the len is 2 but the index is {}", idx),
        }
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point::new(-self.x, -self.y)
    }
}

impl From<(i32, i32)> for Point {
    fn from(other: (i32, i32)) -> Self {
        Point::new(other.0, other.1)
    }
}

impl From<[i32; 2]> for Point {
    fn from(other: [i32; 2]) -> Self {
        Point::new(other[0], other[1])
    }
}

impl From<&[i32; 2]> for Point {
    fn from(other: &[i32; 2]) -> Self {
        Point::new(other[0], other[1])
    }
}

impl From<Point> for (i32, i32) {
    fn from(other: Point) -> (i32, i32) {
        (other.x, other.y)
    }
}

impl From<&Point> for (i32, i32) {
    fn from(other: &Point) -> (i32, i32) {
        (other.x, other.y)
    }
}

#[cfg(feature = "nalgebra_support")]
use nalgebra::{base::Scalar, Vector2};

#[cfg(feature = "nalgebra_support")]
impl<N> From<Vector2<N>> for Point
where
    N: Into<i32> + Scalar,
{
    fn from(other: Vector2<N>) -> Self {
        Self::new(other[0].into(), other[1].into())
    }
}

#[cfg(feature = "nalgebra_support")]
impl<N> From<&Vector2<N>> for Point
where
    N: Into<i32> + Scalar,
{
    fn from(other: &Vector2<N>) -> Self {
        Self::new(other[0].into(), other[1].into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points_can_be_added() {
        let mut left = Point::new(10, 20);
        let right = Point::new(30, 40);

        assert_eq!(left + right, Point::new(40, 60));

        left += right;
        assert_eq!(left, Point::new(40, 60));
    }

    #[test]
    fn point_and_size_can_be_added() {
        let mut left = Point::new(11, 21);
        let right = Size::new(30, 40);

        assert_eq!(left + right, Point::new(41, 61));

        left += right;
        assert_eq!(left, Point::new(41, 61));
    }

    #[test]
    fn points_can_be_subtracted() {
        let mut left = Point::new(30, 50);
        let right = Point::new(10, 20);

        assert_eq!(left - right, Point::new(20, 30));

        left -= right;
        assert_eq!(left, Point::new(20, 30));
    }

    #[test]
    fn point_and_size_can_be_subtracted() {
        let mut left = Point::new(30, 40);
        let right = Size::new(11, 22);

        assert_eq!(left - right, Point::new(19, 18));

        left -= right;
        assert_eq!(left, Point::new(19, 18));
    }

    #[test]
    fn points_can_be_negative_after_subtraction() {
        let left = Point::new(10, 20);
        let right = Point::new(30, 50);

        assert_eq!(left - right, Point::new(-20, -30));

        let left = Point::new(10, 20);
        let right = Size::new(31, 42);

        assert_eq!(left - right, Point::new(-21, -22));
    }

    #[test]
    #[should_panic(expected = "width is too large")]
    #[cfg(debug_assertions)]
    fn too_large_width_can_not_be_added() {
        let p = Point::zero();
        let _ = p + Size::new(u32::max_value(), 0);
    }

    #[test]
    #[should_panic(expected = "width is too large")]
    #[cfg(debug_assertions)]
    fn too_large_width_can_not_be_add_assigned() {
        let mut p = Point::zero();
        p += Size::new(u32::max_value(), 0);
    }

    #[test]
    #[should_panic(expected = "height is too large")]
    #[cfg(debug_assertions)]
    fn too_large_height_can_not_be_added() {
        let p = Point::zero();
        let _ = p + Size::new(0, 0x80000000);
    }

    #[test]
    #[should_panic(expected = "height is too large")]
    #[cfg(debug_assertions)]
    fn too_large_height_can_not_be_add_assigned() {
        let mut p = Point::zero();
        p += Size::new(0, 0x80000000);
    }

    #[test]
    #[should_panic(expected = "width is too large")]
    #[cfg(debug_assertions)]
    fn too_large_width_can_not_be_subtracted() {
        let p = Point::zero();
        let _ = p - Size::new(u32::max_value(), 0);
    }

    #[test]
    #[should_panic(expected = "width is too large")]
    #[cfg(debug_assertions)]
    fn too_large_width_can_not_be_sub_assigned() {
        let mut p = Point::zero();
        p -= Size::new(u32::max_value(), 0);
    }

    #[test]
    #[should_panic(expected = "height is too large")]
    #[cfg(debug_assertions)]
    fn too_large_height_can_not_be_subtracted() {
        let p = Point::zero();
        let _ = p - Size::new(0, 0x80000000);
    }

    #[test]
    #[should_panic(expected = "height is too large")]
    #[cfg(debug_assertions)]
    fn too_large_height_can_not_be_sub_assigned() {
        let mut p = Point::zero();
        p -= Size::new(0, 0x80000000);
    }

    #[test]
    fn from_tuple() {
        assert_eq!(Point::from((20i32, 30i32)), Point::new(20, 30));
        assert_eq!(Point::from((20i32, 30i32)), Point::new(20, 30));
    }

    #[test]
    fn from_array() {
        assert_eq!(Point::from([20i32, 30i32]), Point::new(20, 30));
        assert_eq!(Point::from([20i32, 30i32]), Point::new(20, 30));
    }

    #[test]
    fn from_array_ref() {
        assert_eq!(Point::from(&[20i32, 30i32]), Point::new(20, 30));
        assert_eq!(Point::from(&[20i32, 30i32]), Point::new(20, 30));
    }

    #[test]
    fn neg() {
        assert_eq!(-Point::new(10, 20), Point::new(-10, -20));
        assert_eq!(-Point::new(-40, -50), Point::new(40, 50));
    }

    #[test]
    fn index() {
        let point = Point::new(12, -34);

        assert_eq!(point.x, point[0]);
        assert_eq!(point.y, point[1]);
    }

    #[test]
    #[should_panic]
    fn index_out_of_bounds() {
        let point = Point::new(1, 2);
        let _ = point[2];
    }

    #[test]
    #[cfg(feature = "nalgebra_support")]
    fn nalgebra_support() {
        let left = nalgebra::Vector2::new(30, 40);
        let right = nalgebra::Vector2::new(10, 20);

        assert_eq!(Point::from(left - right), Point::new(20, 20));
    }

    #[test]
    #[cfg(feature = "nalgebra_support")]
    fn convert_ref() {
        let left = nalgebra::Vector2::new(30, 40);
        let right = nalgebra::Vector2::new(10, 20);

        let c = left - right;

        assert_eq!(Point::from(&c), Point::new(20, 20));
    }
}
