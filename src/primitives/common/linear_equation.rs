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
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
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
            LineSide::Left => distance <= 0,
            LineSide::Right => distance >= 0,
        }
    }
}

/// Linear equation with zero distance to the origin.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct OriginLinearEquation {
    pub normal_vector: Point,
}

impl OriginLinearEquation {
    /// Creates a new linear equation with the given angle.
    pub fn with_angle(angle: Angle) -> Self {
        // FIXME: angle.tan() for 180.0 degrees isn't exactly 0 which causes problems when drawing
        //        a single quadrant. Is there a better solution to fix this?
        let normal_vector = if angle == Angle::from_degrees(180.0) {
            Point::new(0, -NORMAL_VECTOR_SCALE)
        } else {
            Point::new(
                i32::from(angle.cos() * Real::from(NORMAL_VECTOR_SCALE)),
                i32::from(angle.sin() * Real::from(NORMAL_VECTOR_SCALE)),
            )
            .rotate_90()
        };

        Self { normal_vector }
    }

    /// Creates a new horizontal linear equation.
    pub const fn new_horizontal() -> Self {
        Self {
            normal_vector: Point::new(0, NORMAL_VECTOR_SCALE),
        }
    }

    /// Returns the distance between the line and a point.
    ///
    /// The scaling of the returned value depends on the length of the normal vector.
    /// Positive values will be returned for points on the right side of the line and negative
    /// values for points on the left.
    pub fn distance(&self, point: Point) -> i32 {
        point.dot_product(self.normal_vector)
    }

    /// Checks if a point is on the given side of the line.
    ///
    /// Always returns `true` if the point is on the line.
    pub fn check_side(&self, point: Point, side: LineSide) -> bool {
        let distance = self.distance(point);

        match side {
            LineSide::Left => distance <= 0,
            LineSide::Right => distance >= 0,
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
                normal_vector: Point::new(0, 1),
                origin_distance: 0, // line goes through the origin
            }
        );

        assert_eq!(
            LinearEquation::from_line(&Line::new(Point::zero(), Point::new(0, 1))),
            LinearEquation {
                normal_vector: Point::new(-1, 0),
                origin_distance: 0, // line goes through the origin
            }
        );

        assert_eq!(
            LinearEquation::from_line(&Line::new(Point::new(2, 3), Point::new(-2, 3))),
            LinearEquation {
                normal_vector: Point::new(0, -4),
                // origin_distance = min. distance between line and origin * length of unit vector
                //                 = 3 * 4
                origin_distance: -12,
            }
        );
    }

    #[test]
    fn with_angle() {
        assert_eq!(
            OriginLinearEquation::with_angle(0.0.deg()),
            OriginLinearEquation {
                normal_vector: Point::new(0, NORMAL_VECTOR_SCALE),
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
    fn check_side_horizontal_0deg() {
        let eq1 = OriginLinearEquation::with_angle(0.0.deg());
        let eq2 = LinearEquation::from_line(&Line::with_delta(Point::zero(), Point::new(10, 0)));

        use LineSide::*;
        for (point, side, expected) in [
            ((0, 0), Left, true),
            ((1, 0), Right, true),
            ((-2, 1), Left, false),
            ((3, 1), Right, true),
            ((-4, -1), Left, true),
            ((5, -1), Right, false),
        ]
        .into_iter()
        {
            assert_eq!(
                eq1.check_side(point.into(), side),
                expected,
                "{:?}, {:?}",
                point,
                side
            );

            assert_eq!(
                eq2.check_side(point.into(), side),
                expected,
                "{:?}, {:?}",
                point,
                side
            );
        }
    }

    #[test]
    fn check_side_horizontal_180deg() {
        let eq1 = OriginLinearEquation::with_angle(180.0.deg());
        let eq2 = LinearEquation::from_line(&Line::with_delta(Point::zero(), Point::new(-10, 0)));

        use LineSide::*;
        for (point, side, expected) in [
            ((0, 0), Left, true),
            ((1, 0), Right, true),
            ((-2, 1), Left, true),
            ((3, 1), Right, false),
            ((-4, -1), Left, false),
            ((5, -1), Right, true),
        ]
        .into_iter()
        {
            assert_eq!(
                eq1.check_side(point.into(), side),
                expected,
                "{:?}, {:?}",
                point,
                side
            );

            assert_eq!(
                eq2.check_side(point.into(), side),
                expected,
                "{:?}, {:?}",
                point,
                side
            );
        }
    }

    #[test]
    fn check_side_vertical_90deg() {
        let eq1 = OriginLinearEquation::with_angle(90.0.deg());
        let eq2 = LinearEquation::from_line(&Line::with_delta(Point::zero(), Point::new(0, 10)));

        use LineSide::*;
        for (point, side, expected) in [
            ((0, 0), Left, true),
            ((0, 1), Right, true),
            ((-1, -2), Left, false),
            ((-1, 3), Right, true),
            ((1, -4), Left, true),
            ((1, 5), Right, false),
        ]
        .into_iter()
        {
            assert_eq!(
                eq1.check_side(point.into(), side),
                expected,
                "{:?}, {:?}",
                point,
                side
            );

            assert_eq!(
                eq2.check_side(point.into(), side),
                expected,
                "{:?}, {:?}",
                point,
                side
            );
        }
    }

    #[test]
    fn check_side_vertical_270deg() {
        let eq1 = OriginLinearEquation::with_angle(270.0.deg());
        let eq2 = LinearEquation::from_line(&Line::with_delta(Point::zero(), Point::new(0, -10)));

        use LineSide::*;
        for (point, side, expected) in [
            ((0, 0), Left, true),
            ((0, -1), Right, true),
            ((-1, 2), Left, true),
            ((-1, -3), Right, false),
            ((1, 4), Left, false),
            ((1, -5), Right, true),
        ]
        .into_iter()
        {
            assert_eq!(
                eq1.check_side(point.into(), side),
                expected,
                "{:?}, {:?}",
                point,
                side
            );

            assert_eq!(
                eq2.check_side(point.into(), side),
                expected,
                "{:?}, {:?}",
                point,
                side
            );
        }
    }
}
