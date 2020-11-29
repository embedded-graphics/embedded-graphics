use crate::{
    geometry::{Angle, Point, PointExt, Real, Trigonometry},
    primitives::{common::LineSide, Line},
};

/// Scaling factor for unit length normal vectors.
pub const NORMAL_VECTOR_SCALE: i32 = 1 << 10;

/// Linear equation.
///
/// The equation is stored as a normal vector and the distance to the origin.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct LinearEquation {
    /// Normal vector, perpendicular to the line.
    ///
    /// The unit vector is scaled up to increase the resolution.
    pub normal_vector: Point,

    /// Distance from the origin.
    ///
    /// The distance doesn't directly correlate to the distance in pixels, but is
    /// scaled up by the length of the normal vector.
    pub origin_distance: i32,
}

impl LinearEquation {
    /// Creates a new linear equation with the given angle and distance to the origin.
    pub fn with_angle_and_distance(angle: Angle, origin_distance: i32) -> Self {
        Self {
            normal_vector: OriginLinearEquation::with_angle(angle).normal_vector,
            origin_distance,
        }
    }

    /// Creates a new linear equation from a line.
    pub fn from_line(line: &Line) -> Self {
        let normal_vector = line.delta().rotate_90();
        let origin_distance = line.start.dot_product(normal_vector);

        Self {
            normal_vector,
            origin_distance,
        }
    }

    /// Returns the distance between the line and a point.
    ///
    /// The scaling of the returned value depends on the length of the normal vector.
    /// Positive values will be returned for points on the left side of the line and negative
    /// values for points on the right.
    pub fn distance(&self, point: Point) -> i32 {
        point.dot_product(self.normal_vector) - self.origin_distance
    }

    /// Checks if a point is on the given side of the line.
    ///
    /// Always returns `true` if the point is on the line.
    pub fn check_side(&self, point: Point, side: LineSide) -> bool {
        let distance = self.distance(point);

        match side {
            LineSide::Right => distance <= 0,
            LineSide::Left => distance >= 0,
        }
    }
}

/// Linear equation with zero distance to the origin.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct OriginLinearEquation {
    pub normal_vector: Point,
}

impl OriginLinearEquation {
    /// Creates a new linear equation with the given angle.
    pub fn with_angle(angle: Angle) -> Self {
        // FIXME: angle.tan() for 180.0 degrees isn't exactly 0 which causes problems when drawing
        //        a single quadrant. Is there a better solution to fix this?
        let normal_vector = if angle == Angle::from_degrees(180.0) {
            Point::new(0, NORMAL_VECTOR_SCALE)
        } else {
            -Point::new(
                i32::from(angle.sin() * Real::from(NORMAL_VECTOR_SCALE)),
                i32::from(angle.cos() * Real::from(NORMAL_VECTOR_SCALE)),
            )
        };

        Self { normal_vector }
    }

    /// Creates a new horizontal linear equation.
    pub fn new_horizontal() -> Self {
        Self {
            normal_vector: Point::new(0, -NORMAL_VECTOR_SCALE),
        }
    }

    /// Returns the distance between the line and a point.
    ///
    /// The scaling of the returned value depends on the length of the normal vector.
    /// Positive values will be returned for points on the left side of the line and negative
    /// values for points on the right.
    pub fn distance(&self, point: Point) -> i32 {
        point.dot_product(self.normal_vector)
    }

    /// Checks if a point is on the given side of the line.
    ///
    /// Always returns `true` if the point is on the line.
    pub fn check_side(&self, point: Point, side: LineSide) -> bool {
        let distance = self.distance(point);

        match side {
            LineSide::Right => distance <= 0,
            LineSide::Left => distance >= 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::AngleUnit;

    #[test]
    fn from_line() {
        assert_eq!(
            LinearEquation::from_line(&Line::new(Point::zero(), Point::new(1, 0))),
            LinearEquation {
                normal_vector: Point::new(0, -1),
                origin_distance: 0, // line goes through the origin
            }
        );

        assert_eq!(
            LinearEquation::from_line(&Line::new(Point::zero(), Point::new(0, 1))),
            LinearEquation {
                normal_vector: Point::new(1, 0),
                origin_distance: 0, // line goes through the origin
            }
        );

        assert_eq!(
            LinearEquation::from_line(&Line::new(Point::new(2, 3), Point::new(-2, 3))),
            LinearEquation {
                normal_vector: Point::new(0, 4),
                // origin_distance = min. distance between line and origin * length of unit vector
                //                 = 3 * 4
                origin_distance: 12,
            }
        );
    }

    #[test]
    fn with_angle() {
        assert_eq!(
            OriginLinearEquation::with_angle(0.0.deg()),
            OriginLinearEquation {
                normal_vector: Point::new(0, -NORMAL_VECTOR_SCALE),
            }
        );

        assert_eq!(
            OriginLinearEquation::with_angle(90.0.deg()),
            OriginLinearEquation {
                normal_vector: Point::new(-NORMAL_VECTOR_SCALE, 0),
            }
        );
    }

    #[test]
    fn distance() {
        let line = OriginLinearEquation::with_angle(90.0.deg());
        assert_eq!(line.distance(Point::new(-1, 0)), NORMAL_VECTOR_SCALE);
        assert_eq!(line.distance(Point::new(1, 0)), -NORMAL_VECTOR_SCALE);
    }

    #[test]
    fn check_side_horizontal() {
        let line = OriginLinearEquation::with_angle(0.0.deg());
        assert!(line.check_side(Point::new(0, 0), LineSide::Left));
        assert!(line.check_side(Point::new(1, 0), LineSide::Right));
        assert!(!line.check_side(Point::new(-2, 1), LineSide::Left));
        assert!(line.check_side(Point::new(3, 1), LineSide::Right));
        assert!(line.check_side(Point::new(-4, -1), LineSide::Left));
        assert!(!line.check_side(Point::new(5, -1), LineSide::Right));

        let line = OriginLinearEquation::with_angle(180.0.deg());
        assert!(line.check_side(Point::new(0, 0), LineSide::Left));
        assert!(line.check_side(Point::new(1, 0), LineSide::Right));
        assert!(line.check_side(Point::new(-2, 1), LineSide::Left));
        assert!(!line.check_side(Point::new(3, 1), LineSide::Right));
        assert!(!line.check_side(Point::new(-4, -1), LineSide::Left));
        assert!(line.check_side(Point::new(5, -1), LineSide::Right));
    }

    #[test]
    fn check_side_vertical() {
        let line = OriginLinearEquation::with_angle(90.0.deg());
        assert!(line.check_side(Point::new(0, 0), LineSide::Left));
        assert!(line.check_side(Point::new(0, -1), LineSide::Right));
        assert!(line.check_side(Point::new(-1, 2), LineSide::Left));
        assert!(!line.check_side(Point::new(-1, -3), LineSide::Right));
        assert!(!line.check_side(Point::new(1, 4), LineSide::Left));
        assert!(line.check_side(Point::new(1, -5), LineSide::Right));

        let line = OriginLinearEquation::with_angle(270.0.deg());
        assert!(line.check_side(Point::new(0, 0), LineSide::Left));
        assert!(line.check_side(Point::new(0, 1), LineSide::Right));
        assert!(!line.check_side(Point::new(-1, -2), LineSide::Left));
        assert!(line.check_side(Point::new(-1, 3), LineSide::Right));
        assert!(line.check_side(Point::new(1, -4), LineSide::Left));
        assert!(!line.check_side(Point::new(1, 5), LineSide::Right));
    }
}
