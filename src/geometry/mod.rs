//! Geometry module.

mod angle;
mod real;

pub(crate) use angle::angle_consts;
pub(crate) use angle::Trigonometry;
pub use angle::{Angle, AngleUnit};
pub use embedded_graphics_core::geometry::{
    AnchorPoint, Dimensions, OriginDimensions, Point, Size,
};
pub(crate) use real::Real;

pub(crate) trait PointExt {
    /// Returns a point that is rotated by 90° relative to the origin.
    fn rotate_90(self) -> Self;

    /// Calculates the dot product of two points.
    fn dot_product(self, other: Point) -> i32;

    /// Calculates the determinant of a 2x2 matrix formed by this and another point.
    ///
    /// ```text
    ///          | self.x  self.y  |
    /// result = |                 |
    ///          | other.x other.y |
    /// ```
    fn determinant(self, other: Point) -> i32;

    /// Returns the squared length.
    ///
    /// The returned value is the square of the length of a vector from `(0, 0)`
    /// to `(self.x, self.y)`.
    fn length_squared(self) -> i32;
}

impl PointExt for Point {
    fn rotate_90(self) -> Self {
        Self::new(self.y, -self.x)
    }

    fn dot_product(self, other: Point) -> i32 {
        self.x * other.x + self.y * other.y
    }

    fn determinant(self, other: Point) -> i32 {
        self.x * other.y - self.y * other.x
    }

    fn length_squared(self) -> i32 {
        self.x.pow(2) + self.y.pow(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_length_squared() {
        let p = Point::new(3, 4);

        assert_eq!(p.length_squared(), 25);
    }

    #[test]
    fn rotate_90() {
        assert_eq!(Point::new(1, 0).rotate_90(), Point::new(0, -1));
        assert_eq!(Point::new(0, -2).rotate_90(), Point::new(-2, 0));
        assert_eq!(Point::new(-3, 0).rotate_90(), Point::new(0, 3));
        assert_eq!(Point::new(0, 4).rotate_90(), Point::new(4, 0));
    }
}
