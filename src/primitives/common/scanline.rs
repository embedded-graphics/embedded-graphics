use core::ops::Range;

use crate::{
    geometry::{Dimensions, Point},
    primitives::{ContainsPoint, Line, Primitive, Rectangle},
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Scanline {
    pub y: i32,
    pub x: Range<i32>,
}

impl Scanline {
    pub const fn new(y: i32) -> Self {
        Self { y, x: 0..0 }
    }

    pub fn is_empty(&self) -> bool {
        // MSRV: use `Range::is_empty` on version >= 1.47.0
        !(self.x.start < self.x.end)
    }

    fn extend(&mut self, x: i32) {
        if self.is_empty() {
            self.x = x..x + 1;
        } else {
            if x < self.x.start {
                self.x.start = x;
            } else if x >= self.x.end {
                self.x.end = x + 1;
            }
        }
    }

    /// Intersect a horizontal scan line with the Bresenham representation of this line segment.
    ///
    /// Intersection lines produced by this function are sorted so that the start always lies to the
    /// left of the end.
    pub fn bresenham_intersection(&mut self, line: &Line) {
        if !line
            .bounding_box()
            .contains(Point::new(line.start.x, self.y))
        {
            return;
        }

        let y = self.y;
        let mut points = line.points().filter(|p| p.y == y);

        if let Some(first) = points.next() {
            self.extend(first.x);
        }

        if let Some(last) = points.last() {
            self.extend(last.x);
        }
    }

    /// Check for lines that are adjacent or overlapping.
    ///
    /// This assumes that both lines have the same y coordinate.
    fn touches(&self, other: &Scanline) -> bool {
        debug_assert_eq!(
            self.y, other.y,
            "try_extend must be called with scanlines with equal y coordinate"
        );

        if self.is_empty() || other.is_empty() {
            return false;
        }

        let range = self.x.start - 1..self.x.end + 1;
        range.contains(&(other.x.start)) || range.contains(&(other.x.end - 1))
    }

    /// Tries to extend this line by another line.
    ///
    /// The line is only extended when the other line overlaps this line or is directly adjacent
    /// to this line.
    ///
    /// Returns `true` if the line was extended and `false` otherwise.
    pub fn try_extend(&mut self, other: &Scanline) -> bool {
        debug_assert_eq!(
            self.y, other.y,
            "try_extend must be called with scanlines with equal y coordinate"
        );

        if self.touches(other) {
            self.x.start = self.x.start.min(other.x.start);
            self.x.end = self.x.end.max(other.x.end);

            true
        } else {
            false
        }
    }

    pub fn try_to_rectangle(&self) -> Option<Rectangle> {
        if !self.is_empty() {
            Some(Rectangle::with_corners(
                Point::new(self.x.start, self.y),
                Point::new(self.x.end - 1, self.y),
            ))
        } else {
            None
        }
    }

    /// Renamed to try_take to because of conflict with Iterator::take
    pub fn try_take(&mut self) -> Option<Self> {
        if !self.is_empty() {
            let ret = self.clone();
            self.x = 0..0;
            Some(ret)
        } else {
            None
        }
    }
}

impl Iterator for Scanline {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        self.x.next().map(|x| Point::new(x, self.y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_touches_test(s1: i32, e1: i32, s2: i32, e2: i32, expected: bool, ident: &str) {
        let mut l1 = Scanline::new(0);
        l1.extend(s1);
        l1.extend(e1);

        let mut l2 = Scanline::new(0);
        l2.extend(s2);
        l2.extend(e2);

        assert_eq!(l1.touches(&l2), expected, "{}", ident);
    }

    #[test]
    fn check_touches() {
        run_touches_test(30, 40, 5, 15, false, "Reversed");
        run_touches_test(0, 6, 5, 10, true, "Contained");
        run_touches_test(11, 13, 11, 14, true, "Contained 2");
        run_touches_test(10, 15, 25, 35, false, "Separated");
        run_touches_test(10, 10, 10, 10, true, "Zero size");
        run_touches_test(10, 20, 10, 20, true, "Equal");
        run_touches_test(10, 20, 20, 10, true, "Equal reversed");
        run_touches_test(79, 82, 82, 92, true, "Overlapping lines 1");
        run_touches_test(82, 92, 79, 82, true, "Overlapping lines 1, reversed");
        run_touches_test(80, 83, 83, 94, true, "Overlapping lines 2");
        run_touches_test(83, 94, 80, 83, true, "Overlapping lines 2, reversed");
        run_touches_test(83, 94, 94, 100, true, "Adjacent");
    }
}
