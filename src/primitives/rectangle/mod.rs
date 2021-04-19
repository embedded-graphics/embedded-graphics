//! The rectangle primitive. Also good for drawing squares.

use crate::{
    geometry::{Point, Size},
    primitives::{ContainsPoint, OffsetOutline, Primitive},
    transform::Transform,
};

pub use embedded_graphics_core::primitives::{rectangle::Points, Rectangle};

mod styled;

pub use styled::StyledPixelsIterator;

impl Primitive for Rectangle {}

impl ContainsPoint for Rectangle {
    fn contains(&self, point: Point) -> bool {
        if point.x >= self.top_left.x && point.y >= self.top_left.y {
            self.bottom_right().map_or(false, |bottom_right| {
                point.x <= bottom_right.x && point.y <= bottom_right.y
            })
        } else {
            false
        }
    }
}

impl OffsetOutline for Rectangle {
    fn offset(&self, offset: i32) -> Self {
        let size = if offset >= 0 {
            self.size.saturating_add(Size::new_equal(offset as u32 * 2))
        } else {
            self.size
                .saturating_sub(Size::new_equal((-offset) as u32 * 2))
        };

        Self::with_center(self.center(), size)
    }
}

impl Transform for Rectangle {
    /// Translate the rect from its current position to a new position by (x, y) pixels, returning
    /// a new `Rectangle`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Rectangle;
    /// # use embedded_graphics::prelude::*;
    /// let rect = Rectangle::new(Point::new(5, 10), Size::new(10, 10));
    /// let moved = rect.translate(Point::new(10, 10));
    ///
    /// assert_eq!(moved.top_left, Point::new(15, 20));
    /// assert_eq!(moved.size, Size::new(10, 10));
    /// ```
    fn translate(&self, by: Point) -> Self {
        Self {
            top_left: self.top_left + by,
            ..*self
        }
    }

    /// Translate the rect from its current position to a new position by (x, y) pixels.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Rectangle;
    /// # use embedded_graphics::prelude::*;
    /// let mut rect = Rectangle::new(Point::new(5, 10), Size::new(10, 10));
    /// rect.translate_mut(Point::new(10, 10));
    ///
    /// assert_eq!(rect.top_left, Point::new(15, 20));
    /// assert_eq!(rect.size, Size::new(10, 10));
    /// ```
    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.top_left += by;

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::{AnchorPoint, Dimensions, Point, Size},
        primitives::PointsIter,
    };

    #[test]
    fn dimensions() {
        let rect = Rectangle::new(Point::new(5, 10), Size::new(10, 20));
        let moved = rect.translate(Point::new(-10, -20));

        assert_eq!(
            rect.bounding_box(),
            Rectangle::new(Point::new(5, 10), Size::new(10, 20))
        );

        assert_eq!(
            moved.bounding_box(),
            Rectangle::new(Point::new(-5, -10), Size::new(10, 20))
        );
    }

    #[test]
    fn it_can_be_translated() {
        let rect = Rectangle::new(Point::new(5, 10), Size::new(10, 20));
        let moved = rect.translate(Point::new(10, 15));

        let bounding_box = moved.bounding_box();
        assert_eq!(bounding_box.top_left, Point::new(15, 25));
        assert_eq!(bounding_box.size, Size::new(10, 20));
    }

    #[test]
    fn it_can_be_negative() {
        let negative = Rectangle::new(Point::new(-2, -2), Size::new(4, 4)).points();

        let positive = Rectangle::new(Point::new(2, 2), Size::new(4, 4)).points();

        assert!(negative.eq(positive.map(|p| p - Point::new(4, 4))));
    }

    #[test]
    fn contains() {
        let outer = Rectangle::new(Point::zero(), Size::new(10, 10));
        let inner = Rectangle::new(Point::new(2, 4), Size::new(3, 5));

        for p in outer.points() {
            let expected = p.x >= 2 && p.x < 2 + 3 && p.y >= 4 && p.y < 4 + 5;

            assert_eq!(inner.contains(p), expected, "{:?}", p);
        }
    }

    #[test]
    fn center() {
        let odd = Rectangle::new(Point::new(10, 20), Size::new(5, 7));
        assert_eq!(odd.center(), Point::new(12, 23));

        let even = Rectangle::new(Point::new(20, 30), Size::new(4, 8));
        assert_eq!(even.center(), Point::new(21, 33));
    }

    #[test]
    fn bottom_right() {
        let zero = Rectangle::new(Point::new(10, 20), Size::zero());
        assert_eq!(zero.bottom_right(), None);

        let odd = Rectangle::new(Point::new(10, 20), Size::new(5, 7));
        assert_eq!(odd.bottom_right(), Some(Point::new(14, 26)));

        let even = Rectangle::new(Point::new(20, 30), Size::new(4, 8));
        assert_eq!(even.bottom_right(), Some(Point::new(23, 37)));
    }

    #[test]
    fn rectangle_intersection() {
        let rect1 = Rectangle::new(Point::new_equal(10), Size::new(20, 30));
        let rect2 = Rectangle::new(Point::new_equal(25), Size::new(30, 40));

        assert_eq!(
            rect1.intersection(&rect2),
            Rectangle::new(Point::new_equal(25), Size::new(5, 15))
        );
    }

    #[test]
    fn rectangle_no_intersection() {
        let rect1 = Rectangle::new(Point::new_equal(10), Size::new(20, 30));
        let rect2 = Rectangle::new(Point::new_equal(35), Size::new(30, 40));

        assert!(rect1.intersection(&rect2).is_zero_sized());
    }

    #[test]
    fn rectangle_complete_intersection() {
        let rect1 = Rectangle::new(Point::new_equal(10), Size::new(20, 30));
        let rect2 = rect1;

        assert_eq!(rect1.intersection(&rect2), rect1);
    }

    #[test]
    fn rectangle_contained_intersection() {
        let rect1 = Rectangle::with_corners(Point::new_equal(10), Point::new(20, 30));
        let rect2 = Rectangle::with_corners(Point::new_equal(5), Point::new(30, 40));

        assert_eq!(rect1.intersection(&rect2), rect1);
    }

    #[test]
    fn zero_sized_intersection() {
        let rect1 = Rectangle::new(Point::new(1, 2), Size::new(0, 0));
        let rect2 = Rectangle::new(Point::new(-10, -10), Size::new(20, 20));

        assert_eq!(rect1.intersection(&rect2), rect1);

        let rect1 = Rectangle::new(Point::new(-10, -10), Size::new(20, 20));
        let rect2 = Rectangle::new(Point::new(2, 3), Size::new(0, 0));

        assert_eq!(rect1.intersection(&rect2), rect2);
    }

    /// Test for issue #452
    ///
    /// Rectangles can intersect even if no corner of any rectangle is contained inside the other
    /// rectangle.
    ///
    /// Example:
    ///
    ///     ****
    ///     *  *
    /// ############
    /// #   *  *   #
    /// #   *  *   #
    /// ############
    ///     *  *
    ///     ****
    #[test]
    fn issue_452_broken_intersection_check() {
        let rect1 = Rectangle::new(Point::new(50, 0), Size::new(75, 200));
        let rect2 = Rectangle::new(Point::new(0, 75), Size::new(200, 50));

        let expected = Rectangle::new(Point::new(50, 75), Size::new(75, 50));

        assert_eq!(rect1.intersection(&rect2), expected);
        assert_eq!(rect2.intersection(&rect1), expected);
    }

    #[test]
    fn offset() {
        let center = Point::new(10, 20);
        let rect = Rectangle::with_center(center, Size::new(3, 4));

        assert_eq!(rect.offset(0), rect);

        assert_eq!(
            rect.offset(1),
            Rectangle::with_center(center, Size::new(5, 6))
        );
        assert_eq!(
            rect.offset(2),
            Rectangle::with_center(center, Size::new(7, 8))
        );

        assert_eq!(
            rect.offset(-1),
            Rectangle::with_center(center, Size::new(1, 2))
        );
        assert_eq!(
            rect.offset(-2),
            Rectangle::with_center(center, Size::new(0, 0))
        );
        assert_eq!(
            rect.offset(-3),
            Rectangle::with_center(center, Size::new(0, 0))
        );
    }

    #[test]
    fn resized_smaller() {
        let rect = Rectangle::new(Point::new(10, 20), Size::new(30, 40));

        for &(anchor_point, expected_top_left) in &[
            (AnchorPoint::TopLeft, Point::new(10, 20)),
            (AnchorPoint::TopCenter, Point::new(20, 20)),
            (AnchorPoint::TopRight, Point::new(30, 20)),
            (AnchorPoint::CenterLeft, Point::new(10, 30)),
            (AnchorPoint::Center, Point::new(20, 30)),
            (AnchorPoint::CenterRight, Point::new(30, 30)),
            (AnchorPoint::BottomLeft, Point::new(10, 40)),
            (AnchorPoint::BottomCenter, Point::new(20, 40)),
            (AnchorPoint::BottomRight, Point::new(30, 40)),
        ] {
            let resized = rect.resized(Size::new(10, 20), anchor_point);

            assert_eq!(
                resized,
                Rectangle::new(expected_top_left, Size::new(10, 20)),
                "{:?}",
                anchor_point,
            );
        }
    }

    #[test]
    fn resized_larger() {
        let rect = Rectangle::new(Point::new(10, 20), Size::new(30, 40));

        for &(anchor_point, expected_top_left) in &[
            (AnchorPoint::TopLeft, Point::new(10, 20)),
            (AnchorPoint::TopCenter, Point::new(5, 20)),
            (AnchorPoint::TopRight, Point::new(0, 20)),
            (AnchorPoint::CenterLeft, Point::new(10, 15)),
            (AnchorPoint::Center, Point::new(5, 15)),
            (AnchorPoint::CenterRight, Point::new(0, 15)),
            (AnchorPoint::BottomLeft, Point::new(10, 10)),
            (AnchorPoint::BottomCenter, Point::new(5, 10)),
            (AnchorPoint::BottomRight, Point::new(0, 10)),
        ] {
            let resized = rect.resized(Size::new(40, 50), anchor_point);

            assert_eq!(
                resized,
                Rectangle::new(expected_top_left, Size::new(40, 50)),
                "{:?}",
                anchor_point,
            );
        }
    }

    #[test]
    fn resized_zero_sized() {
        let rect = Rectangle::new(Point::new(10, 20), Size::zero());

        for &(anchor_point, expected_top_left) in &[
            (AnchorPoint::TopLeft, Point::new(10, 20)),
            (AnchorPoint::TopCenter, Point::new(8, 20)),
            (AnchorPoint::TopRight, Point::new(6, 20)),
            (AnchorPoint::CenterLeft, Point::new(10, 17)),
            (AnchorPoint::Center, Point::new(8, 17)),
            (AnchorPoint::CenterRight, Point::new(6, 17)),
            (AnchorPoint::BottomLeft, Point::new(10, 14)),
            (AnchorPoint::BottomCenter, Point::new(8, 14)),
            (AnchorPoint::BottomRight, Point::new(6, 14)),
        ] {
            let resized = rect.resized(Size::new(5, 7), anchor_point);

            assert_eq!(
                resized,
                Rectangle::new(expected_top_left, Size::new(5, 7)),
                "{:?}",
                anchor_point,
            );
        }
    }

    #[test]
    fn resized_to_zero_sized() {
        let rect = Rectangle::new(Point::new(10, 20), Size::new(21, 31));

        for &(anchor_point, expected_top_left) in &[
            (AnchorPoint::TopLeft, Point::new(10, 20)),
            (AnchorPoint::TopCenter, Point::new(20, 20)),
            (AnchorPoint::TopRight, Point::new(30, 20)),
            (AnchorPoint::CenterLeft, Point::new(10, 35)),
            (AnchorPoint::Center, Point::new(20, 35)),
            (AnchorPoint::CenterRight, Point::new(30, 35)),
            (AnchorPoint::BottomLeft, Point::new(10, 50)),
            (AnchorPoint::BottomCenter, Point::new(20, 50)),
            (AnchorPoint::BottomRight, Point::new(30, 50)),
        ] {
            let resized = rect.resized(Size::zero(), anchor_point);

            assert_eq!(
                resized,
                Rectangle::new(expected_top_left, Size::zero()),
                "{:?}",
                anchor_point,
            );
        }
    }

    #[test]
    fn anchor_point() {
        let rect = Rectangle::new(Point::new(10, 20), Size::new(21, 31));

        for &(anchor_point, expected) in &[
            (AnchorPoint::TopLeft, Point::new(10, 20)),
            (AnchorPoint::TopCenter, Point::new(20, 20)),
            (AnchorPoint::TopRight, Point::new(30, 20)),
            (AnchorPoint::CenterLeft, Point::new(10, 35)),
            (AnchorPoint::Center, Point::new(20, 35)),
            (AnchorPoint::CenterRight, Point::new(30, 35)),
            (AnchorPoint::BottomLeft, Point::new(10, 50)),
            (AnchorPoint::BottomCenter, Point::new(20, 50)),
            (AnchorPoint::BottomRight, Point::new(30, 50)),
        ] {
            assert_eq!(
                rect.anchor_point(anchor_point),
                expected,
                "{:?}",
                anchor_point,
            );
        }
    }

    #[test]
    fn rows_and_columns_zero_sized() {
        let rect = Rectangle::zero();

        assert_eq!(
            rect.rows().next(),
            None,
            "the rows iterator for a zero sized rectangle shouldn't return any items"
        );

        assert_eq!(
            rect.columns().next(),
            None,
            "the columns iterator for a zero sized rectangle shouldn't return any items"
        );
    }
}
