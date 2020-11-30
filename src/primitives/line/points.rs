use crate::{
    geometry::Point,
    primitives::line::{
        bresenham::{self, Bresenham, BresenhamParameters},
        Line,
    },
};

/// Iterator over all points on the line.
///
/// See the [`points`] method documentation for more details.
///
/// [`points`]: struct.Line.html#method.points
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Points {
    parameters: BresenhamParameters,
    bresenham: Bresenham,
    points_remaining: u32,
}

impl Points {
    /// Creates an iterator over all points on the given line.
    pub(in crate::primitives) fn new(line: &Line) -> Self {
        let length = bresenham::major_length(line);
        let parameters = BresenhamParameters::new(line);
        let bresenham = Bresenham::new(line.start);

        Self {
            parameters,
            bresenham,
            points_remaining: length,
        }
    }

    /// Creates an empty iterator.
    pub(in crate::primitives) fn empty() -> Self {
        let dummy = Line::new(Point::zero(), Point::zero());

        let mut self_ = Self::new(&dummy);
        self_.points_remaining = 0;

        self_
    }
}

impl Iterator for Points {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.points_remaining > 0 {
            self.points_remaining -= 1;

            Some(self.bresenham.next(&self.parameters))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{
        iterator::PixelIteratorExt, mock_display::MockDisplay, pixelcolor::BinaryColor,
        primitives::PointsIter, Pixel,
    };

    fn test_points(start: Point, end: Point, expected: &[(i32, i32)]) {
        let expected = expected.iter().copied().map(Point::from);

        let points = Line::new(start, end).points();
        assert!(points.eq(expected));
    }

    fn draw_lines(delta: Point) -> MockDisplay<BinaryColor> {
        let mut display = MockDisplay::new();

        for &quadrant in &[
            Point::new(-1, -1),
            Point::new(1, -1),
            Point::new(-1, 1),
            Point::new(1, 1),
        ] {
            let center = delta + Point::new_equal(1);
            let start = center + quadrant;
            let end = start + Point::new(delta.x * quadrant.x, delta.y * quadrant.y);

            let line = Line::new(start, end);

            line.points()
                .map(|point| Pixel(point, BinaryColor::On))
                .draw(&mut display)
                .unwrap();
        }

        display
    }

    #[test]
    fn lines_1() {
        let delta = Point::new(6, 3);

        let expected = MockDisplay::from_pattern(&[
            "#             #",
            " ##         ## ",
            "   ##     ##   ",
            "     ## ##     ",
            "               ",
            "     ## ##     ",
            "   ##     ##   ",
            " ##         ## ",
            "#             #",
        ]);

        draw_lines(delta).assert_eq(&expected);

        let expected = expected.swap_xy();
        let delta = Point::new(delta.y, delta.x);
        draw_lines(delta).assert_eq(&expected);
    }

    #[test]
    fn lines_2() {
        let delta = Point::new(9, 3);

        let expected = MockDisplay::from_pattern(&[
            "##                 ##",
            "  ###           ###  ",
            "     ###     ###     ",
            "        ## ##        ",
            "                     ",
            "        ## ##        ",
            "     ###     ###     ",
            "  ###           ###  ",
            "##                 ##",
        ]);
        draw_lines(delta).assert_eq(&expected);

        let expected = expected.swap_xy();
        let delta = Point::new(delta.y, delta.x);
        draw_lines(delta).assert_eq(&expected);
    }

    #[test]
    fn single_pixel() {
        let start = Point::new(10, 10);
        let end = Point::new(10, 10);

        let expected = [(10, 10)];
        test_points(start, end, &expected);
    }

    #[test]
    fn short_correctly() {
        let start = Point::new(2, 3);
        let end = Point::new(3, 2);

        let expected = [(2, 3), (3, 2)];
        test_points(start, end, &expected);
    }

    #[test]
    fn octant_1_correctly() {
        let start = Point::new(10, 10);
        let end = Point::new(15, 13);

        let expected = [(10, 10), (11, 11), (12, 11), (13, 12), (14, 12), (15, 13)];
        test_points(start, end, &expected);
    }

    #[test]
    fn octant_2_correctly() {
        let start = Point::new(10, 10);
        let end = Point::new(13, 15);

        let expected = [(10, 10), (11, 11), (11, 12), (12, 13), (12, 14), (13, 15)];
        test_points(start, end, &expected);
    }

    #[test]
    fn octant_3_correctly() {
        let start = Point::new(10, 10);
        let end = Point::new(7, 15);

        let expected = [(10, 10), (9, 11), (9, 12), (8, 13), (8, 14), (7, 15)];
        test_points(start, end, &expected);
    }

    #[test]
    fn octant_4_correctly() {
        let start = Point::new(10, 10);
        let end = Point::new(5, 13);

        let expected = [(10, 10), (9, 11), (8, 11), (7, 12), (6, 12), (5, 13)];
        test_points(start, end, &expected);
    }

    #[test]
    fn octant_5_correctly() {
        let start = Point::new(10, 10);
        let end = Point::new(5, 7);

        let expected = [(10, 10), (9, 9), (8, 9), (7, 8), (6, 8), (5, 7)];
        test_points(start, end, &expected);
    }

    #[test]
    fn octant_6_correctly() {
        let start = Point::new(10, 10);
        let end = Point::new(7, 5);

        let expected = [(10, 10), (9, 9), (9, 8), (8, 7), (8, 6), (7, 5)];
        test_points(start, end, &expected);
    }

    #[test]
    fn octant_7_correctly() {
        let start = Point::new(10, 10);
        let end = Point::new(13, 5);

        let expected = [(10, 10), (11, 9), (11, 8), (12, 7), (12, 6), (13, 5)];
        test_points(start, end, &expected);
    }

    #[test]
    fn octant_8_correctly() {
        let start = Point::new(10, 10);
        let end = Point::new(15, 7);

        let expected = [(10, 10), (11, 9), (12, 9), (13, 8), (14, 8), (15, 7)];
        test_points(start, end, &expected);
    }

    #[test]
    fn one_pixel_line() {
        let p = Point::new(5, 6);
        assert!(Line::new(p, p).points().eq(core::iter::once(p)));
    }

    #[test]
    fn empty() {
        assert!(Points::empty().eq(core::iter::empty()));
    }
}
