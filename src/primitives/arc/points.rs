use crate::{
    geometry::Point,
    primitives::{arc::Arc, circle::DistanceIterator, common::PlaneSectorIterator, OffsetOutline},
};

/// Iterator over all points on the arc line.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Points {
    iter: DistanceIterator<PlaneSectorIterator>,

    outer_threshold: u32,
    inner_threshold: u32,
}

impl Points {
    pub(in crate::primitives) fn new(arc: &Arc) -> Self {
        let outer_circle = arc.to_circle();
        let inner_circle = outer_circle.offset(-1);

        let points =
            PlaneSectorIterator::new(arc, arc.center_2x(), arc.angle_start, arc.angle_sweep);

        Self {
            iter: outer_circle.distances(points),
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

        self.iter
            .find(|(_, distance)| *distance < outer_threshold && *distance >= inner_threshold)
            .map(|(point, _)| point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        drawable::Pixel, geometry::AngleUnit, iterator::IntoPixels, pixelcolor::BinaryColor,
        primitives::Primitive, style::PrimitiveStyle,
    };

    #[test]
    fn points_equals_filled() {
        let arc = Arc::with_center(Point::new(10, 10), 5, 0.0.deg(), 90.0.deg());

        let styled_points = arc
            .clone()
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .into_pixels()
            .map(|Pixel(p, _)| p);

        assert!(arc.points().eq(styled_points));
    }
}
