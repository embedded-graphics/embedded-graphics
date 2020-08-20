//! A scanline iterator that returns every point in a triangle once.
use crate::{
    geometry::Point,
    primitives::{
        line::{self, Line},
        triangle::{sort_two_x, sort_yx, Triangle},
        Primitive,
    },
};

/// A bit more memory-friendly way to chain two lines together.
/// Horizontal lines are optimized for the scanline iterator.
///
/// TODO: maybe generalize this?
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ChainedLines {
    line: line::Points,
    next_point: Option<Point>,
}

impl ChainedLines {
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        if a.y == b.y {
            // A -> B walk is unnecessary, the horizontal iterator will return those points
            Self {
                line: Line::new(b, c).points(),
                next_point: None
            }
        } else if b.y == c.y {
            // B -> C walk is unnecessary, the horizontal iterator will return those points
            Self {
                line: Line::new(a, b).points(),
                next_point: None
            }
        } else {
            Self {
                line: Line::new(a, b).points(),
                next_point: Some(c)
            }
        }
    }
}

impl Iterator for ChainedLines {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(p) = self.line.next() {
            if self.line.is_empty() {
                if let Some(p2) = self.next_point.take() {
                    self.line = Line::new(p, p2).points();
                    self.line.next();
                }
            }
            Some(p)
        } else {
            None
        }
    }
}

/// Iterator over all points inside the triangle.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct FillScanlineIterator {
    /// Edges on the one side of the triangle
    line_ab: ChainedLines,

    /// Edges on the other side of the triangle
    line_c: line::Points,

    /// Horizontal line
    scan_points: line::Points,

    /// The first point of the ab edge in the next line
    next_a: Option<Point>,

    /// The first point of the c edge in the next line
    next_c: Option<Point>,
}

impl FillScanlineIterator {
    pub(in crate::primitives) fn new(triangle: &Triangle) -> Self {
        let (v1, v2, v3) = sort_yx(triangle.p1, triangle.p2, triangle.p3);

        Self {
            line_ab: ChainedLines::new(v1, v2, v3),
            line_c: Line::new(v1, v3).points(),
            next_a: None,
            next_c: None,
            scan_points: line::Points::empty(),
        }
    }

    fn update_ab(&mut self) -> Option<(Point, Point)> {
        let mut next_a = self
            .next_a
            .take()
            .or_else(|| self.line_ab.next())?;
        let first = next_a;
        while let Some(a) = self.line_ab.next() {
            if a.y == next_a.y {
                next_a = a;
            } else {
                self.next_a = Some(a);
                return Some((first, next_a));
            }
        }
        Some((first, next_a))
    }

    fn update_c(&mut self) -> Option<(Point, Point)> {
        let mut next_c = self.next_c.take().or_else(|| self.line_c.next())?;
        let first = next_c;
        while let Some(c) = self.line_c.next() {
            if c.y == next_c.y {
                next_c = c;
            } else {
                self.next_c = Some(c);
                return Some((first, next_c));
            }
        }
        Some((first, next_c))
    }

    fn next_scanline(&mut self) -> bool {
        let a = self.update_ab();
        let c = self.update_c();

        match (a, c) {
            (Some((fa, la)), Some((fc, lc))) => {
                let mut arr = [fa, la, fc, lc];
                arr.sort_by(|&a, &b| a.x.cmp(&b.x));
                let [left, _, _, right] = arr;

                self.scan_points = Line::new(left, right).points();

                true
            }
            (Some((l, r)), None) | (None, Some((l, r))) => {
                let (l, r) = sort_two_x(l, r);
                self.scan_points = Line::new(l, r).points();

                true
            }
            (None, None) => false,
        }
    }
}

impl Iterator for FillScanlineIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(point) = self.scan_points.next() {
                return Some(point);
            } else if !self.next_scanline() {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        drawable::{Drawable, Pixel},
        geometry::Dimensions,
        mock_display::MockDisplay,
        pixel_iterator::IntoPixels,
        pixelcolor::BinaryColor,
        primitives::ContainsPoint,
        style::PrimitiveStyle,
        transform::Transform,
    };

    #[test]
    fn points_are_part_of_triangle() {
        fn check(triangle: Triangle) {
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

        check(Triangle::new(
            Point::new(30, 0),
            Point::new(40, 10),
            Point::new(42, 10),
        ));
        check(Triangle::new(
            Point::new(5, 5),
            Point::new(15, 0),
            Point::new(10, 10),
        ));
        check(Triangle::new(
            Point::new(0, 0),
            Point::new(0, 10),
            Point::new(40, 10),
        ));
        check(Triangle::new(
            Point::new(0, 0),
            Point::new(0, 10),
            Point::new(40, 0),
        ));
        check(Triangle::new(
            Point::new(0, 0),
            Point::new(40, 10),
            Point::new(40, 0),
        ));
        check(Triangle::new(
            Point::new(0, 0),
            Point::new(60, 10),
            Point::new(60, 15),
        ));
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
