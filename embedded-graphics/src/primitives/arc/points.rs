use crate::{
    geometry::Point,
    primitives::{
        arc::{Arc, PlaneSectorIterator},
        circle,
        circle::DistanceIterator,
    },
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
        let outer_diameter = arc.diameter;
        let inner_diameter = outer_diameter.saturating_sub(2);

        let inner_threshold = circle::diameter_to_threshold(inner_diameter);
        let outer_threshold = circle::diameter_to_threshold(outer_diameter);

        let iter = DistanceIterator::new(
            arc.center_2x(),
            PlaneSectorIterator::new(arc, arc.center(), arc.angle_start, arc.angle_sweep),
        );

        Self {
            iter,
            outer_threshold,
            inner_threshold,
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
        drawable::Pixel, geometry::AngleUnit, pixel_iterator::IntoPixels, pixelcolor::BinaryColor,
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
