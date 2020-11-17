use crate::{
    geometry::{angle_consts::*, Angle, Dimensions, Point, Real},
    primitives::{
        common::{LineSide, LinearEquation},
        rectangle, Primitive, Rectangle,
    },
};

/// Sector shaped part of a plane.
///
/// The shape is described by two half-planes that divide the XY plane along the two
/// lines from the center point to the arc's end points. For sweep angles < 180° the
/// intersection of both half-planes is used and for angles >= 180° the union of both
/// half-planes.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct PlaneSector {
    line_a: LinearEquation<Real>,
    line_b: LinearEquation<Real>,
    side_a: LineSide,
    side_b: LineSide,
    sweep: Angle,
}

impl PlaneSector {
    pub fn new(center_2x: Point, angle_start: Angle, angle_sweep: Angle) -> Self {
        let angle_end = angle_start + angle_sweep;

        let angle_start_norm = angle_start.normalize_from(-ANGLE_90DEG);
        let angle_end_norm = angle_end.normalize_from(-ANGLE_90DEG);
        let negative_sweep = angle_sweep < Angle::zero();

        let side_a = if (angle_start_norm < ANGLE_90DEG) ^ negative_sweep {
            LineSide::Left
        } else {
            LineSide::Right
        };

        let side_b = if (angle_end_norm >= ANGLE_90DEG) ^ negative_sweep {
            LineSide::Left
        } else {
            LineSide::Right
        };

        Self {
            line_a: LinearEquation::from_point_angle(center_2x, angle_start),
            line_b: LinearEquation::from_point_angle(center_2x, angle_end),
            side_a,
            side_b,
            sweep: angle_sweep.abs(),
        }
    }

    fn empty() -> Self {
        Self {
            line_a: LinearEquation::new_horizontal(),
            line_b: LinearEquation::new_horizontal(),
            side_a: LineSide::Left,
            side_b: LineSide::Left,
            sweep: Angle::zero(),
        }
    }

    pub fn contains(&self, point: Point) -> bool {
        // `PlaneSector` uses scaled coordinates for an increased resolution.
        let point = point * 2;

        let correct_side_a = self.line_a.check_side(point, self.side_a);
        let correct_side_b = self.line_b.check_side(point, self.side_b);

        if self.sweep < ANGLE_180DEG {
            correct_side_a && correct_side_b
        } else if self.sweep < ANGLE_360DEG {
            correct_side_a || correct_side_b
        } else {
            true
        }
    }
}

/// Iterator over the points in the intersection of a plane sector and the bounding box of a primitive.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct PlaneSectorIterator {
    plane_sector: PlaneSector,
    points: rectangle::Points,
}

impl PlaneSectorIterator {
    pub fn new<D: Dimensions>(
        primitive: &D,
        center: Point,
        angle_start: Angle,
        angle_sweep: Angle,
    ) -> Self {
        Self {
            plane_sector: PlaneSector::new(center, angle_start, angle_sweep),
            points: primitive.bounding_box().points(),
        }
    }

    pub fn empty() -> Self {
        Self {
            plane_sector: PlaneSector::empty(),
            points: Rectangle::zero().points(),
        }
    }
}

impl Iterator for PlaneSectorIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let plane_sector = self.plane_sector;
        self.points.find(|p| plane_sector.contains(*p))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{geometry::AngleUnit, primitives::Arc};

    #[test]
    fn plane_sector_iter() {
        let arc = Arc::new(Point::zero(), 3, 0.0.deg(), 90.0.deg());

        let mut iter =
            PlaneSectorIterator::new(&arc, arc.center_2x(), arc.angle_start, arc.angle_sweep);
        assert_eq!(iter.next(), Some(Point::new(1, 0)));
        assert_eq!(iter.next(), Some(Point::new(2, 0)));
        assert_eq!(iter.next(), Some(Point::new(1, 1)));
        assert_eq!(iter.next(), Some(Point::new(2, 1)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn plane_sector_iter_empty() {
        let mut iter = PlaneSectorIterator::empty();
        assert_eq!(iter.next(), None);
    }

    /// Checks if the plane sector contains 8 different points.
    ///
    /// Four of the points lie on the boundary between two adjacent quadrants and should be
    /// contained in both quadrants. The remaining points are inside a single quadrant.
    fn contains(plane_sector: &PlaneSector) -> [bool; 8] {
        [
            plane_sector.contains(Point::new(10, 0)),
            plane_sector.contains(Point::new(10, -10)),
            plane_sector.contains(Point::new(0, -10)),
            plane_sector.contains(Point::new(-10, -10)),
            plane_sector.contains(Point::new(-10, 0)),
            plane_sector.contains(Point::new(-10, 10)),
            plane_sector.contains(Point::new(0, 10)),
            plane_sector.contains(Point::new(10, 10)),
        ]
    }

    #[test]
    fn plane_sector_quadrants_positive_sweep() {
        let plane_sector = PlaneSector::new(Point::zero(), 0.0.deg(), 90.0.deg());
        assert_eq!(
            contains(&plane_sector),
            [true, true, true, false, false, false, false, false]
        );

        let plane_sector = PlaneSector::new(Point::zero(), 90.0.deg(), 90.0.deg());
        assert_eq!(
            contains(&plane_sector),
            [false, false, true, true, true, false, false, false]
        );

        let plane_sector = PlaneSector::new(Point::zero(), 180.0.deg(), 90.0.deg());
        assert_eq!(
            contains(&plane_sector),
            [false, false, false, false, true, true, true, false]
        );

        let plane_sector = PlaneSector::new(Point::zero(), 270.0.deg(), 90.0.deg());
        assert_eq!(
            contains(&plane_sector),
            [true, false, false, false, false, false, true, true]
        );
    }

    #[test]
    fn plane_sector_quadrants_negative_sweep() {
        let plane_sector = PlaneSector::new(Point::zero(), 0.0.deg(), -90.0.deg());
        assert_eq!(
            contains(&plane_sector),
            [true, false, false, false, false, false, true, true]
        );

        let plane_sector = PlaneSector::new(Point::zero(), 90.0.deg(), -90.0.deg());
        assert_eq!(
            contains(&plane_sector),
            [true, true, true, false, false, false, false, false]
        );

        let plane_sector = PlaneSector::new(Point::zero(), 180.0.deg(), -90.0.deg());
        assert_eq!(
            contains(&plane_sector),
            [false, false, true, true, true, false, false, false]
        );

        let plane_sector = PlaneSector::new(Point::zero(), 270.0.deg(), -90.0.deg());
        assert_eq!(
            contains(&plane_sector),
            [false, false, false, false, true, true, true, false]
        );
    }
}
