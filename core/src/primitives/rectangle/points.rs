use core::ops::Range;

use crate::{geometry::Point, primitives::Rectangle};

/// Iterator over all points inside the rectangle.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct Points {
    x: Range<i32>,
    y: Range<i32>,
    x_start: i32,
}

impl Points {
    pub(in crate::primitives::rectangle) fn new(rectangle: &Rectangle) -> Self {
        // Return `Self::empty` for all zero sized rectangles.
        // The iterator would behave correctly without this check, but would loop unnecessarily for
        // rectangles with zero width.
        if rectangle.is_zero_sized() {
            return Self::empty();
        }

        let x = rectangle.columns();
        let y = rectangle.rows();
        let x_start = x.start;

        Self { x, y, x_start }
    }

    /// Create a points iterator that returns no items.
    pub const fn empty() -> Self {
        Self {
            x: 0..0,
            y: 0..0,
            x_start: 0,
        }
    }
}

impl Iterator for Points {
    type Item = Point;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while !self.y.is_empty() {
            if let Some(x) = self.x.next() {
                return Some(Point::new(x, self.y.start));
            }

            self.y.next();
            self.x.start = self.x_start;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::{Point, Size},
        primitives::{PointsIter, Rectangle},
    };

    #[test]
    fn points_iter() {
        let rectangle = Rectangle::new(Point::new(10, 20), Size::new(2, 3));

        let mut points = rectangle.points();
        assert_eq!(points.next(), Some(Point::new(10, 20)));
        assert_eq!(points.next(), Some(Point::new(11, 20)));
        assert_eq!(points.next(), Some(Point::new(10, 21)));
        assert_eq!(points.next(), Some(Point::new(11, 21)));
        assert_eq!(points.next(), Some(Point::new(10, 22)));
        assert_eq!(points.next(), Some(Point::new(11, 22)));
        assert_eq!(points.next(), None);
    }

    #[test]
    fn points_iter_zero_size() {
        let rectangle = Rectangle::new(Point::new(1, 2), Size::zero());

        let mut points = rectangle.points();
        assert_eq!(points.next(), None);
    }

    #[test]
    fn points_iter_zero_size_x() {
        let rectangle = Rectangle::new(Point::new(1, 2), Size::new(0, 1));

        let mut points = rectangle.points();
        assert_eq!(points.next(), None);
    }

    #[test]
    fn points_iter_zero_size_y() {
        let rectangle = Rectangle::new(Point::new(1, 2), Size::new(1, 0));

        let mut points = rectangle.points();
        assert_eq!(points.next(), None);
    }

    #[test]
    fn points_iter_empty() {
        let mut points = Points::empty();
        assert_eq!(points.next(), None);
    }
}
