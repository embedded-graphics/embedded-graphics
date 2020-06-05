use crate::{
    geometry::{Dimensions, Point, Size},
    primitives::{
        circle::Circle,
        rectangle::{self, Rectangle},
        Primitive,
    },
};

/// Iterator that returns the squared distance to the center for all points in the bounding box.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct DistanceIterator {
    center: Point,
    points: rectangle::Points,
}

impl DistanceIterator {
    pub(crate) fn new(circle: &Circle) -> Self {
        Self {
            center: circle.center_2x(),
            points: circle.bounding_box().points(),
        }
    }

    pub(crate) fn empty() -> Self {
        Self {
            center: Point::zero(),
            points: Rectangle::new(Point::zero(), Size::zero()).points(),
        }
    }
}

impl Iterator for DistanceIterator {
    type Item = (Point, u32);

    fn next(&mut self) -> Option<Self::Item> {
        self.points.next().map(|p| {
            let delta = self.center - p * 2;
            let distance = delta.length_squared() as u32;

            (p, distance)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_iter() {
        let circle = Circle::new(Point::zero(), 3);

        let mut iter = DistanceIterator::new(&circle);
        assert_eq!(iter.next(), Some((Point::new(0, 0), 8)));
        assert_eq!(iter.next(), Some((Point::new(1, 0), 4)));
        assert_eq!(iter.next(), Some((Point::new(2, 0), 8)));
        assert_eq!(iter.next(), Some((Point::new(0, 1), 4)));
        assert_eq!(iter.next(), Some((Point::new(1, 1), 0)));
        assert_eq!(iter.next(), Some((Point::new(2, 1), 4)));
        assert_eq!(iter.next(), Some((Point::new(0, 2), 8)));
        assert_eq!(iter.next(), Some((Point::new(1, 2), 4)));
        assert_eq!(iter.next(), Some((Point::new(2, 2), 8)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn distance_iter_empty() {
        let mut iter = DistanceIterator::empty();
        assert_eq!(iter.next(), None);
    }
}
