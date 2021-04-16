use crate::{
    geometry::Point,
    primitives::{
        arc::Arc,
        common::{DistanceIterator, PlaneSector},
        OffsetOutline,
    },
};

/// Iterator over all points on the arc line.
#[derive(Clone, PartialEq, Debug)]
pub struct Points {
    iter: DistanceIterator,

    plane_sector: PlaneSector,

    outer_threshold: u32,
    inner_threshold: u32,
}

impl Points {
    pub(in crate::primitives) fn new(arc: &Arc) -> Self {
        let outer_circle = arc.to_circle();
        let inner_circle = outer_circle.offset(-1);

        let plane_sector = PlaneSector::new(arc.angle_start, arc.angle_sweep);

        Self {
            // PERF: The distance iterator should use the smaller arc bounding box
            iter: outer_circle.distances(),
            plane_sector,
            outer_threshold: outer_circle.threshold(),
            inner_threshold: inner_circle.threshold(),
        }
    }
}

impl Iterator for Points {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let outer_threshold = self.outer_threshold;
        let inner_threshold = self.inner_threshold;
        let plane_sector = self.plane_sector;

        self.iter
            .find(|(_, delta, distance)| {
                *distance < outer_threshold
                    && *distance >= inner_threshold
                    && plane_sector.contains(*delta)
            })
            .map(|(point, ..)| point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::AngleUnit,
        pixelcolor::BinaryColor,
        primitives::{PointsIter, Primitive, PrimitiveStyle},
        Pixel,
    };

    #[test]
    fn points_equals_filled() {
        let arc = Arc::with_center(Point::new(10, 10), 5, 0.0.deg(), 90.0.deg());

        let styled_points = arc
            .clone()
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .pixels()
            .map(|Pixel(p, _)| p);

        assert!(arc.points().eq(styled_points));
    }
}
