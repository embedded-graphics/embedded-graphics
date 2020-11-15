use crate::geometry::{Angle, Point, Real, Trigonometry};

/// Define one side of a line
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum LineSide {
    Above,
    Below,
}

/// Linear equation representation
///
/// The equation is stored as the a, b and c coefficients of the ax + by + c = 0 equation
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct LinearEquation {
    a: Real,
    b: Real,
    c: Real,
}

impl LinearEquation {
    /// Create a new linear equation based on one point and one angle
    pub fn from_point_angle(point: Point, angle: Angle) -> Self {
        // FIXME: angle.tan() for 180.0 degrees isn't exactly 0 which causes problems when drawing
        //        a single quadrant. Is there a better solution to fix this?
        let (a, b) = if angle == Angle::from_degrees(180.0) {
            (Real::from(0.0), Real::from(-1.0))
        } else {
            match angle.tan() {
                None => (Real::from(1.0), Real::from(0.0)),
                Some(a) => (-a, Real::from(-1.0)),
            }
        };

        let c = -(a * point.x.into() + b * point.y.into());
        LinearEquation { a, b, c }
    }

    /// Create a horizontal line equation
    pub fn new_horizontal() -> Self {
        LinearEquation {
            a: Real::from(0.0),
            b: Real::from(1.0),
            c: Real::from(0.0),
        }
    }

    /// Checks if a point is on the given side of the line.
    ///
    /// Always returns `true` if the point is on the line.
    pub fn check_side(&self, point: Point, side: LineSide) -> bool {
        let t = self.a * point.x.into() + self.b * point.y.into() + self.c;

        match side {
            LineSide::Below => t <= Real::from(0.0),
            LineSide::Above => t >= Real::from(0.0),
        }
    }
}
