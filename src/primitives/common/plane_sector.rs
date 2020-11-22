use crate::{
    geometry::{angle_consts::*, Angle, Dimensions, Point},
    primitives::{
        common::{LineSide, LinearEquation},
        rectangle, Primitive, Rectangle,
    },
};

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
enum Operation {
    /// Return the intersection of both half planes.
    Intersection,
    /// Return the union of both half planes.
    Union,
    /// Return the entire plane.
    EntirePlane,
}

/// Sector shaped part of a plane.
///
/// The shape is described by two half-planes that divide the XY plane along the two
/// lines from the center point to the arc's end points. For sweep angles < 180° the
/// intersection of both half-planes is used and for angles >= 180° the union of both
/// half-planes.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct PlaneSector {
    /// Half plane on the left side of a line.
    half_plane_left: LinearEquation,
    /// Half plane on the right side of a line.
    half_plane_right: LinearEquation,
    /// The operation used to combine the two half planes.
    operation: Operation,
}

impl PlaneSector {
    pub fn new(center_2x: Point, mut angle_start: Angle, angle_sweep: Angle) -> Self {
        let mut angle_end = angle_start + angle_sweep;

        // Swap angles for negative sweeps to use the correct sides of the half planes.
        if angle_sweep < Angle::zero() {
            core::mem::swap(&mut angle_start, &mut angle_end)
        }

        let angle_sweep_abs = angle_sweep.abs();
        let operation = if angle_sweep_abs < ANGLE_180DEG {
            Operation::Intersection
        } else if angle_sweep_abs < ANGLE_360DEG {
            Operation::Union
        } else {
            Operation::EntirePlane
        };

        Self {
            half_plane_left: LinearEquation::from_point_angle(center_2x, angle_start),
            half_plane_right: LinearEquation::from_point_angle(center_2x, angle_end),
            operation,
        }
    }

    /// TODO: This method doesn't really return an empty plane sector. Does this matter?
    fn empty() -> Self {
        Self {
            half_plane_left: LinearEquation::new_horizontal(),
            half_plane_right: LinearEquation::new_horizontal(),
            operation: Operation::Union,
        }
    }

    pub fn contains(&self, point: Point) -> bool {
        // `PlaneSector` uses scaled coordinates for an increased resolution.
        let point = point * 2;

        let correct_side_1 = self.half_plane_left.check_side(point, LineSide::Left);
        let correct_side_2 = self.half_plane_right.check_side(point, LineSide::Right);

        match self.operation {
            Operation::Intersection => correct_side_1 && correct_side_2,
            Operation::Union => correct_side_1 || correct_side_2,
            Operation::EntirePlane => true,
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
