//! A scanline iterator that returns every point in a triangle once.
use crate::primitives::triangle::sort_two_yx;
use crate::{
    geometry::Point,
    primitives::{
        line::{self, Line},
        triangle::{sort_two_x, sort_yx, Triangle},
        Primitive,
    },
};

/// Iterator that returns points on a horizontal line.
///
/// Bresenham's algo is good, but not this good.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct HorizontalLine {
    p: Point,
    x_max: i32,
}

impl HorizontalLine {
    pub fn new(start: Point, end: Point) -> Self {
        Self {
            p: start,
            x_max: end.x,
        }
    }

    pub fn empty() -> Self {
        Self {
            p: Point::zero(),
            x_max: -1,
        }
    }
}

impl Iterator for HorizontalLine {
    type Item = Point;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.p.x <= self.x_max {
            let p = Some(self.p);
            self.p.x += 1;
            p
        } else {
            None
        }
    }
}

/// Which edge should not be drawn.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Edge {
    // "a" edge
    AB,

    // "b" edge
    AC,

    // "c" edge
    BC,
}

/// Iterator over all points inside the triangle.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct FillScanlineIterator {
    /// Edges on the one side of the triangle
    line_a: line::Points,
    line_b: line::Points,

    /// Edges on the other side of the triangle
    line_c: line::Points,

    /// Points in the current horizontal line
    scan_points: HorizontalLine,

    /// The first point of the ab edge in the next line
    next_a: Option<Point>,
    next_b: Option<Point>,

    /// The first point of the c edge in the next line
    next_c: Option<Point>,

    /// The ignored edge, or None if everything should be drawn
    ignore: Option<Edge>,
}

impl FillScanlineIterator {
    pub(in crate::primitives) fn new(triangle: &Triangle, ignore: Option<(Point, Point)>) -> Self {
        let (v1, v2, v3) = sort_yx(triangle.p1, triangle.p2, triangle.p3);

        let ignore = ignore.map(|(p1, p2)| {
            let (p1, p2) = sort_two_yx(p1, p2);

            if p1 == v1 {
                if p2 == v2 {
                    Edge::AB
                } else {
                    Edge::AC
                }
            } else {
                Edge::BC
            }
        });

        let mut line_a = Line::new(v1, v2).points();
        let mut line_b = Line::new(v2, v3).points();
        let mut line_c = Line::new(v1, v3).points();

        Self {
            line_a,
            line_b,
            line_c,
            next_a: line_a.next(),
            next_b: line_b.next(),
            next_c: line_c.next(),
            scan_points: HorizontalLine::empty(),

            ignore,
        }
    }

    /// Walk along the given line and return the left and right point of the current horizontal
    /// segment.
    fn next_edge_segment(
        line: &mut line::Points,
        last_point: &mut Option<Point>,
    ) -> Option<(Point, Point)> {
        let first = last_point.take()?;
        let mut next = first;
        while let Some(a) = line.next() {
            if a.y == first.y {
                next = a;
            } else {
                last_point.replace(a);
                break;
            }
        }
        Some(sort_two_x(first, next))
    }

    /// If the `cond` condition is true, modify the edge segment so that it will be skipped.
    fn skip_edge_if(edge: (Point, Point), cond: bool) -> (Point, Point) {
        if cond {
            (edge.1 + Point::new(1, 0), edge.0 - Point::new(1, 0))
        } else {
            edge
        }
    }

    /// Steps to the new scan line. Returns None if there are no points left to generate.
    fn next_scanline(&mut self) -> Option<()> {
        // Walk the edges, get the next horizontal segments.
        // These segments are used to figure out where the scanline has to start and end.
        // Each segment contains a left and right point, which are points drawn as a single
        // horizontal line on the edge of the triangle.
        let a = Self::next_edge_segment(&mut self.line_a, &mut self.next_a);
        let b = self.next_a.map_or_else(
            // only walk BC edge if AB has finished (even if AB just returned it's last segment)
            || Self::next_edge_segment(&mut self.line_b, &mut self.next_b),
            |_| None,
        );
        let c = Self::next_edge_segment(&mut self.line_c, &mut self.next_c)?;

        // Figure out which segment we need if we are at the point where AB and BC overlap.
        let ab = match (a, b) {
            // It's possible that the two sections that the update functions return, overlap.
            // In this case, use the ignore parameter to decide, since it's not possible to
            // ignore both AB and BC edges at the same time.
            (Some(_), Some(b)) if self.ignore == Some(Edge::BC) => b,

            // Prefer "a" edge, but use whichever is available or return if both are None
            _ => a.or(b)?,
        };

        // Decide what to draw and what to ignore
        let ignore_ab = match self.ignore {
            Some(Edge::AB) => Some(ab) == a,
            Some(Edge::BC) => Some(ab) == b,
            _ => false,
        };

        let (left_a, right_a) = Self::skip_edge_if(ab, ignore_ab);
        let (left_c, right_c) = Self::skip_edge_if(c, self.ignore == Some(Edge::AC));

        // In general, we want the scan line between the outermost points.
        // This check sorts the points to (outer, outer) and (inner, inner) pairs by checking the
        // signed lengths of the segments. This works even if the segments overlap.
        let length_1 = right_a.x - left_c.x;
        let length_2 = right_c.x - left_a.x;

        let (left, right) = if length_1 > length_2 {
            (left_c, right_a)
        } else {
            (left_a, right_c)
        };

        self.scan_points = HorizontalLine::new(left, right);
        Some(())
    }
}

impl Iterator for FillScanlineIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(point) = self.scan_points.next() {
                return Some(point);
            } else if self.next_scanline().is_none() {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::PrimitiveStyle;
    use crate::{
        drawable::{Drawable, Pixel},
        geometry::Dimensions,
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        primitives::ContainsPoint,
        transform::Transform,
    };

    fn check_iterator_and_contains(triangle: Triangle) {
        let mut mock_display1 = MockDisplay::new();
        let mut mock_display2 = MockDisplay::new();

        triangle
            .bounding_box()
            .points()
            .filter(|&p| triangle.contains(p))
            .for_each(|p| Pixel(p, BinaryColor::On).draw(&mut mock_display1).unwrap());

        triangle
            .points()
            .for_each(|p| Pixel(p, BinaryColor::On).draw(&mut mock_display2).unwrap());

        assert_eq!(mock_display1, mock_display2, "{:?}", triangle);
    }

    #[test]
    fn points_by_scanline_match_triangle_contains() {
        check_iterator_and_contains(Triangle::new(
            Point::new(30, 0),
            Point::new(40, 10),
            Point::new(42, 10),
        ));
        check_iterator_and_contains(Triangle::new(
            Point::new(5, 5),
            Point::new(15, 0),
            Point::new(10, 10),
        ));
        check_iterator_and_contains(Triangle::new(
            Point::new(0, 0),
            Point::new(0, 10),
            Point::new(40, 10),
        ));
        check_iterator_and_contains(Triangle::new(
            Point::new(0, 0),
            Point::new(0, 10),
            Point::new(40, 0),
        ));
        check_iterator_and_contains(Triangle::new(
            Point::new(0, 0),
            Point::new(40, 10),
            Point::new(40, 0),
        ));
        check_iterator_and_contains(Triangle::new(
            Point::new(0, 0),
            Point::new(60, 10),
            Point::new(60, 15),
        ));

        // this triangle fails even with the original contains() implementation
        check_iterator_and_contains(Triangle::new(
            Point::new(19, 0),
            Point::new(29, 22),
            Point::new(0, 8),
        ));

        // this triangle passes with the original contains() implementation
        check_iterator_and_contains(Triangle::new(
            Point::new(37, 0),
            Point::new(36, 38),
            Point::new(29, 52),
        ));
    }

    #[test]
    fn ignore_side() {
        fn check_ignored_edge(triangle: Triangle) {
            let Triangle { p1, p2, p3 } = triangle;

            for &(a, b) in [(p1, p2), (p2, p1), (p1, p3), (p3, p1), (p2, p3), (p3, p2)].iter() {
                let mut mock_display = MockDisplay::new();
                let mut expectation = MockDisplay::new();

                triangle
                    .points()
                    .for_each(|p| Pixel(p, BinaryColor::On).draw(&mut expectation).unwrap());

                let (start, end) = sort_two_yx(a, b);
                Line::new(start, end)
                    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
                    .draw(&mut mock_display)
                    .unwrap();

                FillScanlineIterator::new(&triangle, Some((start, end)))
                    .for_each(|p| Pixel(p, BinaryColor::On).draw(&mut mock_display).unwrap());

                assert_eq!(mock_display, expectation, "{:?}", triangle);
            }
        }

        check_ignored_edge(Triangle::new(
            Point::new(0, 0),
            Point::new(0, 10),
            Point::new(40, 0),
        ));
        check_ignored_edge(Triangle::new(
            Point::new(37, 0),
            Point::new(36, 38),
            Point::new(29, 52),
        ));
    }

    #[test]
    fn bug_corner_points_must_be_generated() {
        assert!(
            Triangle::new(Point::new(19, 0), Point::new(29, 22), Point::new(0, 8))
                .points()
                .any(|p| p == Point::new(0, 8))
        );
        assert!(
            Triangle::new(Point::new(19, 0), Point::new(29, 22), Point::new(0, 8))
                .points()
                .any(|p| p == Point::new(19, 0))
        );
        assert!(
            Triangle::new(Point::new(19, 0), Point::new(29, 22), Point::new(0, 8))
                .points()
                .any(|p| p == Point::new(29, 22))
        );
    }

    #[test]
    fn off_screen_still_draws_points() {
        let off_screen = Triangle::new(Point::new(10, 10), Point::new(20, 20), Point::new(30, -30));
        let on_screen = off_screen.translate(Point::new(0, 35));

        assert!(off_screen
            .points()
            .eq(on_screen.points().map(|p| p - Point::new(0, 35))));
    }
}
