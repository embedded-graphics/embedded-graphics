use crate::{
    geometry::{Point, PointExt},
    primitives::{
        rectangle::{self, Rectangle},
        PointsIter,
    },
};

/// Iterator that returns the squared distance to the center for all points in the bounding box.
///
/// The iterator returns a tuple of three values:
///
/// 1. The current position inside the bounding box.
/// 2. The difference between the current position and the center point of the bounding box.
///    Note that this value is scaled up by a factor of 2 to increase the resolution.
/// 3. The squared length of the second value.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct DistanceIterator {
    center_2x: Point,
    points: rectangle::Points,
}

impl DistanceIterator {
    /// Creates a distance iterator for the given bounding box.
    ///
    /// Note that `DistanceIterator` internally uses coordinates that are scaled up by a factor
    /// of two to increase the resolution. The given center point must be scaled up by the caller.
    pub fn new(center_2x: Point, bounding_box: &Rectangle) -> Self {
        Self {
            center_2x,
            points: bounding_box.points(),
        }
    }

    /// Creates an empty distance iterator.
    pub fn empty() -> Self {
        Self {
            center_2x: Point::zero(),
            points: rectangle::Points::empty(),
        }
    }
}

impl Iterator for DistanceIterator {
    type Item = (Point, Point, u32);

    fn next(&mut self) -> Option<Self::Item> {
        self.points.next().map(|point| {
            let delta = point * 2 - self.center_2x;
            let distance = delta.length_squared() as u32;

            (point, delta, distance)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{geometry::Dimensions, primitives::Circle};

    #[test]
    fn distance_iter() {
        let circle = Circle::new(Point::zero(), 3);

        let mut iter = DistanceIterator::new(circle.center_2x(), &circle.bounding_box());
        assert_eq!(iter.next(), Some((Point::new(0, 0), Point::new(-2, -2), 8)));
        assert_eq!(iter.next(), Some((Point::new(1, 0), Point::new(0, -2), 4)));
        assert_eq!(iter.next(), Some((Point::new(2, 0), Point::new(2, -2), 8)));
        assert_eq!(iter.next(), Some((Point::new(0, 1), Point::new(-2, 0), 4)));
        assert_eq!(iter.next(), Some((Point::new(1, 1), Point::new(0, 0), 0)));
        assert_eq!(iter.next(), Some((Point::new(2, 1), Point::new(2, 0), 4)));
        assert_eq!(iter.next(), Some((Point::new(0, 2), Point::new(-2, 2), 8)));
        assert_eq!(iter.next(), Some((Point::new(1, 2), Point::new(0, 2), 4)));
        assert_eq!(iter.next(), Some((Point::new(2, 2), Point::new(2, 2), 8)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn distance_iter_empty() {
        let mut iter = DistanceIterator::empty();
        assert_eq!(iter.next(), None);
    }
}
