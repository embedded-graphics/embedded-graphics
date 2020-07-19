use crate::{
    geometry::Point,
    primitives::{
        arc::PlaneSectorIterator,
        circle::{self, DistanceIterator},
        sector::Sector,
    },
};

/// Iterator over all points inside the sector.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Points {
    iter: DistanceIterator<PlaneSectorIterator>,
    threshold: u32,
}

impl Points {
    pub(in crate::primitives) fn new(sector: &Sector) -> Self {
        let threshold = circle::diameter_to_threshold(sector.diameter);

        Self {
            iter: DistanceIterator::new(
                sector.center_2x(),
                PlaneSectorIterator::new(
                    sector,
                    sector.center(),
                    sector.angle_start,
                    sector.angle_sweep,
                ),
            ),
            threshold,
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
        drawable::Pixel, geometry::AngleUnit, pixel_iterator::IntoPixels, pixelcolor::BinaryColor,
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
