use crate::{geometry::Point, primitives::rectangle::Rectangle};

/// Iterator over all points inside the rectangle.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Points {
    left: i32,
    bottom_right: Point,
    current_point: Point,
}

impl Points {
    pub(in crate::primitives) fn new(rectangle: &Rectangle) -> Self {
        // This doesn't use rectangle.bottom_right() to intentionally set bottom_right
        // to an coordinate outside the rectangle if the width or height is zero, which
        // stops the iterator.
        let bottom_right = rectangle.top_left + rectangle.size - Point::new(1, 1);

        Self {
            left: rectangle.top_left.x,
            bottom_right,
            current_point: rectangle.top_left,
        }
    }

    pub(in crate::primitives) const fn empty() -> Self {
        Self {
            left: 0,
            bottom_right: Point::new(-1, -1),
            current_point: Point::zero(),
        }
    }
}

impl Iterator for Points {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        // Finished, i.e. we're below the rect
        if self.current_point.y > self.bottom_right.y {
            return None;
        }

        let ret = self.current_point;

        self.current_point.x += 1;

        // Reached end of row? Jump down one line
        if self.current_point.x > self.bottom_right.x {
            self.current_point.x = self.left;
            self.current_point.y += 1;
        }

        Some(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::{Point, Size},
        primitives::{Primitive, Rectangle},
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
    fn points_iter_empty() {
        let mut points = Points::empty();
        assert_eq!(points.next(), None);
    }
}
