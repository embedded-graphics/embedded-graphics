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
        let (a, b) = match angle.tan() {
            None => (Real::from(1.0), Real::from(0.0)),
            Some(a) => (-a, Real::from(-1.0)),
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

    /// Check on which side of the line a point is
    pub fn side(&self, point: Point) -> LineSide {
        if self.a * point.x.into() + self.b * point.y.into() + self.c < Real::from(0.0) {
            LineSide::Below
        } else {
            LineSide::Above
        }
    }
}
