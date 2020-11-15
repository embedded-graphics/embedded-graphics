use crate::{
    geometry::Point,
    primitives::{arc::PlaneSectorIterator, circle::DistanceIterator, sector::Sector},
};

/// Iterator over all points inside the sector.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Points {
    iter: DistanceIterator<PlaneSectorIterator>,
    threshold: u32,
}

impl Points {
    pub(in crate::primitives) fn new(sector: &Sector) -> Self {
        let circle = sector.to_circle();
        let points = PlaneSectorIterator::new(
            sector,
            sector.center_2x(),
            sector.angle_start,
            sector.angle_sweep,
        );

        Self {
            iter: circle.distances(points),
            threshold: circle.threshold(),
        }
    }
}

impl Iterator for Points {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let threshold = self.threshold;
        self.iter
            .find(|(_, distance)| *distance < threshold)
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
        let sector = Sector::with_center(Point::new(10, 10), 5, 0.0.deg(), 90.0.deg());

        let styled_points = sector
            .clone()
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .into_pixels()
            .map(|Pixel(p, _)| p);

        assert!(sector.points().eq(styled_points));
    }
}
