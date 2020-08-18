//!
use crate::{
    geometry::Point,
    primitives::{
        line::{self, Line},
        triangle::{sort_yx, sort_two_x, Triangle},
        Primitive,
    },
};

/// Iterator over all points inside the triangle.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct FillScanlineIterator {
    /// Left-most edge of the triangle
    line_a: line::Points,

    /// Right-most edge of the triangle
    line_b: line::Points,

    /// Bottom edge of the triangle
    line_c: line::Points,

    /// Horizontal line
    scan_points: line::Points,

    /// y
    current_y: i32,
}

impl FillScanlineIterator {
    pub(in crate::primitives) fn new(triangle: &Triangle) -> Self {
        fn standing_triangle(v1: Point, v2: Point, v3: Point) -> (line::Points, line::Points, line::Points) {
            (
                line::Points::empty(),
                Line::new(v1, v3).points(),
                Line::new(v2, v3).points()
            )
        }

        let (v1, v2, v3) = sort_yx(triangle.p1, triangle.p2, triangle.p3);

        let (mut line_a, mut line_b, mut line_c) = if v1.y == v2.y {
            standing_triangle(v1, v2, v3)
        } else if v2.y == v3.y {
            standing_triangle(v2, v3, v1)
        } else if v1.y == v3.y {
            standing_triangle(v1, v3, v2)
        } else {
            let line_a = Line::new(v1, v2).points();
            let line_b = Line::new(v2, v3).points();
            let line_c = Line::new(v1, v3).points();

            (line_a, line_b, line_c)
        };

        let scan_points = if let (Some(a), Some(b)) = (
            line_a.next().or_else(|| line_b.next()),
            line_c.next()
        ) {
            Line::new(a, b).points()
        } else {
            line::Points::empty()
        };

        Self {
            line_a,
            line_b,
            line_c,
            scan_points,
            current_y: v1.y,
        }
    }

    fn update_ab(&mut self) -> Option<Point> {
        while let Some(point) = self.line_a.next().or_else(|| self.line_b.next()) {
            if point.y != self.current_y {
                self.current_y = point.y;
                return Some(point);
            }
        }
        None
    }

    fn update_c(&mut self) -> Option<Point> {
        while let Some(point) = self.line_c.next() {
            if point.y == self.current_y {
                return Some(point);
            }
        }
        None
    }

    fn next_scanline(&mut self) -> bool {
        let a = self.update_ab();
        let b = self.update_c();

        if let (Some(a), Some(b)) = (a, b) {
            let (a, b) = sort_two_x(a, b);
            self.scan_points = Line::new(a, b).points();
            true
        } else {
            false
        }
    }
}

impl Iterator for FillScanlineIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(point) = self.scan_points.next() {
                return Some(point);
            } else {
                if !self.next_scanline() {
                    return None;
                }
            }
        }
    }
}
