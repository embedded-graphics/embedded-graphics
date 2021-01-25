//! Line intersection parameters.

use crate::{
    geometry::{Point, PointExt},
    primitives::{
        common::{LineSide, LinearEquation},
        Line,
    },
};

/// Intersection test result.
#[derive(Copy, Clone, Debug)]
pub enum Intersection {
    /// Intersection at point
    Point {
        /// Intersection point.
        point: Point,

        /// The "outer" side of the intersection, i.e. the side that has the joint's reflex angle.
        ///
        /// For example:
        ///
        /// ```text
        /// # Left outer side:
        ///
        ///  ⎯
        /// ╱
        ///
        /// # Right outer side:
        ///  │
        /// ╱
        /// ```
        ///
        /// This is used to find the outside edge of a corner.
        outer_side: LineSide,
    },

    /// No intersection: lines are colinear or parallel.
    Colinear,
}
/// Line intersection parameters.
#[derive(Debug, Copy, Clone)]
pub struct IntersectionParams<'a> {
    line1: &'a Line,
    line2: &'a Line,
    le1: LinearEquation,
    le2: LinearEquation,

    /// Determinant, used to solve linear equations using Cramer's rule.
    denominator: i32,
}

impl<'a> IntersectionParams<'a> {
    pub fn from_lines(line1: &'a Line, line2: &'a Line) -> Self {
        let le1 = LinearEquation::from_line(line1);
        let le2 = LinearEquation::from_line(line2);
        let denominator = le1.normal_vector.determinant(le2.normal_vector);

        Self {
            line1,
            line2,
            le1,
            le2,
            denominator,
        }
    }

    /// Check whether two almost-colinear lines are intersecting in the wrong place due to numerical
    /// innacuracies.
    pub fn nearly_colinear_has_error(&self) -> bool {
        self.denominator.pow(2) < self.line1.delta().dot_product(self.line2.delta())
    }

    /// Compute the intersection point.
    pub fn intersection(&self) -> Intersection {
        let Self {
            denominator,
            le1: line1,
            le2: line2,
            ..
        } = *self;

        // The system of linear equations has no solutions if the determinant is zero. In this case,
        // the lines must be colinear.
        if denominator == 0 {
            return Intersection::Colinear;
        }

        let outer_side = if denominator > 0 {
            LineSide::Right
        } else {
            LineSide::Left
        };

        // If we got here, line segments intersect. Compute intersection point using method similar
        // to that described here: http://paulbourke.net/geometry/pointlineplane/#i2l

        // The denominator/2 is to get rounding instead of truncating.
        let offset = denominator.abs() / 2;

        let origin_distances = Point::new(line1.origin_distance, line2.origin_distance);

        let numerator =
            origin_distances.determinant(Point::new(line1.normal_vector.y, line2.normal_vector.y));
        let x_numerator = if numerator < 0 {
            numerator - offset
        } else {
            numerator + offset
        };

        let numerator =
            Point::new(line1.normal_vector.x, line2.normal_vector.x).determinant(origin_distances);
        let y_numerator = if numerator < 0 {
            numerator - offset
        } else {
            numerator + offset
        };

        Intersection::Point {
            point: Point::new(x_numerator, y_numerator) / denominator,
            outer_side,
        }
    }
}
