use core::{
    convert::{TryFrom, TryInto},
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::geometry::Size;

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
///
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
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
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

    /// Creates a point with X and Y values set to an equal value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use embedded_graphics::geometry::Point;
    ///
    /// let point = Point::new_equal(11);
    ///
    /// assert_eq!(point, Point { x: 11, y: 11 });
    /// ```
    pub const fn new_equal(value: i32) -> Self {
        Point { x: value, y: value }
    }

    /// Creates a point with X and Y equal to zero.
    pub const fn zero() -> Self {
        Point { x: 0, y: 0 }
    }

    /// Returns a point with equal `x` value and `y` set to `0`.
    ///
    /// # Examples
    ///
    /// ## Move a `Point` along the X axis.
    ///
    /// ```rust
    /// use embedded_graphics::geometry::Point;
    ///
    /// let translate = Point::new(20, 30);
    ///
    /// let point = Point::new(10, 15);
    ///
    /// let moved_x = point + translate.x_axis();
    ///
    /// assert_eq!(moved_x, Point::new(30, 15));
    /// ```
    pub const fn x_axis(self) -> Self {
        Self { x: self.x, y: 0 }
    }

    /// Returns a point with equal `y` value and `x` set to `0`.
    ///
    /// # Examples
    ///
    /// ## Move a `Point` along the Y axis.
    ///
    /// ```rust
    /// use embedded_graphics::geometry::Point;
    ///
    /// let translate = Point::new(20, 30);
    ///
    /// let point = Point::new(10, 15);
    ///
    /// let moved_y = point + translate.y_axis();
    ///
    /// assert_eq!(moved_y, Point::new(10, 45));
    /// ```
    pub const fn y_axis(self) -> Self {
        Self { x: 0, y: self.y }
    }

    /// Remove the sign from a coordinate
    ///
    /// # Examples
    ///
    /// ```
    /// # use embedded_graphics::geometry::Point;
    /// #
    /// let point = Point::new(-5, -10);
    ///
    /// assert_eq!(point.abs(), Point::new(5, 10));
    /// ```
    pub const fn abs(self) -> Self {
        Point::new(self.x.abs(), self.y.abs())
    }

    /// Returns the componentwise minimum of two `Point`s
    ///
    /// # Examples
    ///
    /// ```rust
    /// use embedded_graphics::geometry::Point;
    ///
    /// let min = Point::new(20, 30).component_min(Point::new(15, 50));
    ///
    /// assert_eq!(min, Point::new(15, 30));
    /// ```
    pub fn component_min(self, other: Self) -> Self {
        Self::new(self.x.min(other.x), self.y.min(other.y))
    }

    /// Returns the componentwise maximum of two `Point`s
    ///
    /// # Examples
    ///
    /// ```rust
    /// use embedded_graphics::geometry::Point;
    ///
    /// let min = Point::new(20, 30).component_max(Point::new(15, 50));
    ///
    /// assert_eq!(min, Point::new(20, 50));
    /// ```
    pub fn component_max(self, other: Self) -> Self {
        Self::new(self.x.max(other.x), self.y.max(other.y))
    }

    /// Returns the componentwise multiplication of two `Point`s.
    ///
    /// ```rust
    /// use embedded_graphics::geometry::Point;
    ///
    /// let result = Point::new(20, 30).component_mul(Point::new(-2, 3));
    ///
    /// assert_eq!(result, Point::new(-40, 90));
    /// ```
    pub fn component_mul(self, other: Self) -> Self {
        Self::new(self.x * other.x, self.y * other.y)
    }

    /// Returns the componentwise division of two `Points`s.
    ///
    /// # Panics
    ///
    /// Panics if one of the components of `other` equals zero.
    ///
    /// ```rust
    /// use embedded_graphics::geometry::Point;
    ///
    /// let result = Point::new(20, 30).component_div(Point::new(10, -3));
    ///
    /// assert_eq!(result, Point::new(2, -10));
    /// ```
    pub fn component_div(self, other: Self) -> Self {
        Self::new(self.x / other.x, self.y / other.y)
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

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Point {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<i32> for Point {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<i32> for Point {
    type Output = Point;

    fn div(self, rhs: i32) -> Point {
        Point::new(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<i32> for Point {
    fn div_assign(&mut self, rhs: i32) {
        self.x /= rhs;
        self.y /= rhs;
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

impl From<Point> for [i32; 2] {
    fn from(other: Point) -> [i32; 2] {
        [other.x, other.y]
    }
}

impl From<&Point> for (i32, i32) {
    fn from(other: &Point) -> (i32, i32) {
        (other.x, other.y)
    }
}

impl TryFrom<Point> for (u32, u32) {
    type Error = core::num::TryFromIntError;

    fn try_from(point: Point) -> Result<Self, Self::Error> {
        Ok((point.x.try_into()?, point.y.try_into()?))
    }
}

impl TryFrom<(u32, u32)> for Point {
    type Error = core::num::TryFromIntError;

    fn try_from(point: (u32, u32)) -> Result<Self, Self::Error> {
        let x = point.0.try_into()?;
        let y = point.1.try_into()?;

        Ok(Point::new(x, y))
    }
}

impl TryFrom<Point> for [u32; 2] {
    type Error = core::num::TryFromIntError;

    fn try_from(point: Point) -> Result<Self, Self::Error> {
        Ok([point.x.try_into()?, point.y.try_into()?])
    }
}

impl TryFrom<[u32; 2]> for Point {
    type Error = core::num::TryFromIntError;

    fn try_from(point: [u32; 2]) -> Result<Self, Self::Error> {
        let x = point[0].try_into()?;
        let y = point[1].try_into()?;

        Ok(Point::new(x, y))
    }
}

impl TryFrom<&[u32; 2]> for Point {
    type Error = core::num::TryFromIntError;

    fn try_from(point: &[u32; 2]) -> Result<Self, Self::Error> {
        let x = point[0].try_into()?;
        let y = point[1].try_into()?;

        Ok(Point::new(x, y))
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

#[cfg(feature = "nalgebra_support")]
use nalgebra::{base::Scalar, Vector2};

#[cfg(feature = "nalgebra_support")]
impl<N> From<Vector2<N>> for Point
where
    N: Into<i32> + Scalar + Copy,
{
    fn from(other: Vector2<N>) -> Self {
        Self::new(other[0].into(), other[1].into())
    }
}

#[cfg(feature = "nalgebra_support")]
impl<N> From<&Vector2<N>> for Point
where
    N: Into<i32> + Scalar + Copy,
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
    fn convert_positive_to_u32_tuple() {
        let p = Point::new(10, 20);

        let tuple: (u32, u32) = p.try_into().unwrap();
        let array: [u32; 2] = p.try_into().unwrap();

        assert_eq!(tuple, (10, 20));
        assert_eq!(array, [10, 20]);
    }

    #[test]
    fn convert_i32_max_to_u32_tuple() {
        let p = Point::new(i32::max_value(), i32::max_value());

        let tuple: (u32, u32) = p.try_into().unwrap();
        let array: [u32; 2] = p.try_into().unwrap();

        // Literal value taken from https://doc.rust-lang.org/std/primitive.i32.html#method.max_value
        assert_eq!(tuple, (2147483647, 2147483647));
        assert_eq!(array, [2147483647, 2147483647]);
    }

    #[test]
    fn convert_negative_to_u32_tuple() {
        let p = Point::new(-50, -10);

        let tuple: Result<(u32, u32), _> = p.try_into();
        let array: Result<[u32; 2], _> = p.try_into();

        assert!(tuple.is_err());
        assert!(array.is_err());
    }

    #[test]
    fn convert_i32_min_to_u32_tuple() {
        let p = Point::new(i32::min_value(), i32::min_value());

        let tuple: Result<(u32, u32), _> = p.try_into();
        let array: Result<[u32; 2], _> = p.try_into();

        assert!(tuple.is_err());
        assert!(array.is_err());
    }

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
    fn points_can_be_multiplied_by_scalar() {
        let p = Point::new(1, 2);
        assert_eq!(p * 3, Point::new(3, 6));

        let mut p = Point::new(3, 4);
        p *= -5;
        assert_eq!(p, Point::new(-15, -20));
    }

    #[test]
    fn points_can_be_divided_by_scalar() {
        let p = Point::new(10, 20);
        assert_eq!(p / 2, Point::new(5, 10));

        let mut p = Point::new(-10, 10);
        p /= -5;
        assert_eq!(p, Point::new(2, -2));
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

    #[test]
    fn component_min_max() {
        let a = Point::new(20, 30);
        let b = Point::new(15, 50);

        assert_eq!(a.component_min(b), Point::new(15, 30));
        assert_eq!(a.component_max(b), Point::new(20, 50));
    }

    #[test]
    fn display() {
        let mut buffer = arrayvec::ArrayString::<[u8; 32]>::new();
        write!(buffer, "{}", Point::new(123, -456)).unwrap();

        assert_eq!(&buffer, "123, -456");
    }
}
