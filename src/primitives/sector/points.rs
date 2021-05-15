use crate::{
    geometry::Point,
    primitives::{
        common::{DistanceIterator, PlaneSector},
        sector::Sector,
    },
};

/// Iterator over all points inside the sector.
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct Points {
    iter: DistanceIterator,

    plane_sector: PlaneSector,

    threshold: u32,
}

impl Points {
    pub(in crate::primitives) fn new(sector: &Sector) -> Self {
        let circle = sector.to_circle();

        let plane_sector = PlaneSector::new(sector.angle_start, sector.angle_sweep);

        Self {
            // PERF: The distance iterator should use the smaller sector bounding box
            iter: circle.distances(),
            plane_sector,
            threshold: circle.threshold(),
        }
    }
}

impl Iterator for Points {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let threshold = self.threshold;
        let plane_sector = self.plane_sector;

        self.iter
            .find(|(_, delta, distance)| *distance < threshold && plane_sector.contains(*delta))
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
        let sector = Sector::with_center(Point::new(10, 10), 5, 0.0.deg(), 90.0.deg());

        let styled_points = sector
            .clone()
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .pixels()
            .map(|Pixel(p, _)| p);

        assert!(sector.points().eq(styled_points));
    }
}
