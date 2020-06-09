use crate::geometry::Point;

/// Iterator that returns the squared distance to the center for all points in the bounding box.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct DistanceIterator<I> {
    center: Point,
    points: I,
}

impl<I> DistanceIterator<I>
where
    I: Iterator<Item = Point>,
{
    pub(in crate::primitives) fn new(center: Point, points: I) -> Self {
        Self { center, points }
    }
}

impl<I> Iterator for DistanceIterator<I>
where
    I: Iterator<Item = Point>,
{
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
    use super::super::*;

    #[test]
    fn distance_iter() {
        let circle = Circle::new(Point::zero(), 3);

        let mut iter = DistanceIterator::new(circle.center_2x(), circle.bounding_box().points());
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
        let mut iter = DistanceIterator::new(
            Point::zero(),
            Rectangle::new(Point::zero(), Size::zero()).points(),
        );
        assert_eq!(iter.next(), None);
    }
}
