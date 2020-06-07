use crate::{
    geometry::Point,
    primitives::circle::{diameter_to_threshold, distance_iterator::DistanceIterator, Circle},
};

/// Iterator over all points inside the circle.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Points {
    iter: DistanceIterator,
    threshold: u32,
}

impl Points {
    pub(crate) fn new(circle: &Circle) -> Self {
        let threshold = diameter_to_threshold(circle.diameter);

        Self {
            iter: DistanceIterator::new(&circle),
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
        drawable::Pixel, geometry::Point, pixelcolor::BinaryColor, primitives::Primitive,
        style::PrimitiveStyle,
    };

    #[test]
    fn points_iter() {
        let circle = Circle::with_center(Point::new(10, 10), 5);

        let styled_points = circle
            .clone()
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .into_iter()
            .map(|Pixel(p, _)| p);

        assert!(circle.points().eq(styled_points));
    }
}
