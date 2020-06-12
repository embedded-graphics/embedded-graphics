use crate::{
    geometry::{angle_consts::*, Angle, Dimensions, Point, Size},
    primitives::{
        arc::{linear_equation::LineSide, LinearEquation},
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
    line_a: LinearEquation,
    line_b: LinearEquation,
    draw_above_a: bool,
    draw_above_b: bool,
    sweep: Angle,
}

impl PlaneSector {
    pub fn new(center: Point, angle_start: Angle, angle_sweep: Angle) -> Self {
        let angle_end = angle_start + angle_sweep;

        let angle_start_norm = angle_start.normalize_from(-ANGLE_90DEG);
        let angle_end_norm = angle_end.normalize_from(-ANGLE_90DEG);
        let negative_sweep = angle_sweep < Angle::zero();

        Self {
            line_a: LinearEquation::from_point_angle(center, angle_start),
            line_b: LinearEquation::from_point_angle(center, angle_end),
            draw_above_a: (angle_start_norm < ANGLE_90DEG) ^ negative_sweep,
            draw_above_b: (angle_end_norm >= ANGLE_90DEG) ^ negative_sweep,
            sweep: angle_sweep.abs(),
        }
    }

    fn empty() -> Self {
        Self {
            line_a: LinearEquation::new_horizontal(),
            line_b: LinearEquation::new_horizontal(),
            draw_above_a: true,
            draw_above_b: true,
            sweep: Angle::zero(),
        }
    }

    pub fn contains(&self, point: Point) -> bool {
        let side_a = self.line_a.side(point);
        let side_b = self.line_b.side(point);

        let correct_a_side = self.draw_above_a ^ (side_a == LineSide::Below);
        let correct_b_side = self.draw_above_b ^ (side_b == LineSide::Below);

        if self.sweep < ANGLE_180DEG {
            correct_a_side && correct_b_side
        } else if self.sweep < ANGLE_360DEG {
            correct_a_side || correct_b_side
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
            points: Rectangle::new(Point::zero(), Size::zero()).points(),
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
    use crate::{
        geometry::AngleUnit,
        primitives::{circle::DistanceIterator, Arc},
    };

    #[test]
    fn plane_sector_iter() {
        let arc = Arc::new(Point::zero(), 3, 0.0.deg(), 90.0.deg());

        let mut iter =
            PlaneSectorIterator::new(&arc, arc.center(), arc.angle_start, arc.angle_sweep);
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

    #[test]
    fn distance_iter() {
        let arc = Arc::new(Point::zero(), 3, 0.0.deg(), 90.0.deg());

        let mut iter = DistanceIterator::new(
            arc.center_2x(),
            PlaneSectorIterator::new(&arc, arc.center(), arc.angle_start, arc.angle_sweep),
        );
        assert_eq!(iter.next(), Some((Point::new(1, 0), 4)));
        assert_eq!(iter.next(), Some((Point::new(2, 0), 8)));
        assert_eq!(iter.next(), Some((Point::new(1, 1), 0)));
        assert_eq!(iter.next(), Some((Point::new(2, 1), 4)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn distance_iter_empty() {
        let mut iter = DistanceIterator::new(Point::zero(), PlaneSectorIterator::empty());
        assert_eq!(iter.next(), None);
    }
}
