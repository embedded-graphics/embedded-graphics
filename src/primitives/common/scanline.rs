use crate::{
    draw_target::DrawTarget,
    geometry::{Point, Size},
    primitives::{Line, PointsIter, Rectangle},
};
use core::ops::Range;

#[cfg(feature = "async_draw")]
use crate::draw_target::AsyncDrawTarget;

/// Scanline.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct Scanline {
    pub y: i32,
    pub x: Range<i32>,
}

impl Scanline {
    /// Creates a new scanline.
    pub const fn new(y: i32, x: Range<i32>) -> Self {
        Self { y, x }
    }

    /// Creates a new empty scanline.
    pub const fn new_empty(y: i32) -> Self {
        Self::new(y, 0..0)
    }

    /// Returns `true` if the x range of the scanline is empty.
    pub fn is_empty(&self) -> bool {
        self.x.is_empty()
    }

    /// Extends the scanline to include the given x coordinate.
    fn extend(&mut self, x: i32) {
        if self.is_empty() {
            self.x = x..x + 1;
        } else if x < self.x.start {
            self.x.start = x;
        } else if x >= self.x.end {
            self.x.end = x + 1;
        }
    }

    /// Intersect a horizontal scan line with the Bresenham representation of this line segment.
    ///
    /// Intersection lines produced by this function are sorted so that the start always lies to the
    /// left of the end.
    pub fn bresenham_intersection(&mut self, line: &Line) {
        // Check if the scanline is in the y range of the line.
        let y_range = if line.start.y <= line.end.y {
            line.start.y..=line.end.y
        } else {
            line.end.y..=line.start.y
        };

        if !y_range.contains(&self.y) {
            return;
        }

        let y = self.y;

        line.points()
            .skip_while(|p| p.y != y)
            .take_while(|p| p.y == y)
            .for_each(|p| {
                self.extend(p.x);
            });
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

        let range = self.x.start - 1..=self.x.end;

        range.contains(&(other.x.start)) || range.contains(&(other.x.end - 1)) || {
            // PERF: If the other conditions short circuit, this won't be computed.
            let range = other.x.start - 1..=other.x.end;

            range.contains(&(self.x.start)) || range.contains(&(self.x.end - 1))
        }
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

    /// Converts the scanline into a 1px high rectangle.
    pub fn to_rectangle(&self) -> Rectangle {
        let width = if !self.is_empty() {
            (self.x.end - self.x.start) as u32
        } else {
            0
        };

        Rectangle::new(Point::new(self.x.start, self.y), Size::new(width, 1))
    }

    /// Returns a clone of the scanline if it isn't empty.
    ///
    /// This method is used similar to `Option::take`, but was renamed to `try_take` because
    /// of a naming conflict with `Iterator::take`.
    pub fn try_take(&mut self) -> Option<Self> {
        if !self.is_empty() {
            let ret = self.clone();
            self.x = 0..0;
            Some(ret)
        } else {
            None
        }
    }

    /// Draws the scanline.
    pub fn draw<T>(&self, target: &mut T, color: T::Color) -> Result<(), T::Error>
    where
        T: DrawTarget,
    {
        if self.is_empty() {
            return Ok(());
        }

        let width = (self.x.end - self.x.start) as u32;

        target.fill_solid(
            &Rectangle::new(Point::new(self.x.start, self.y), Size::new(width, 1)),
            color,
        )
    }

    #[cfg(feature = "async_draw")]
    /// Draws the scanline asynchronoulsy.
    pub async fn draw_async<T>(&self, target: &mut T, color: T::Color) -> Result<(), T::Error>
    where
        T: AsyncDrawTarget,
    {
        if self.is_empty() {
            return Ok(());
        }

        let width = (self.x.end - self.x.start) as u32;

        target
            .fill_solid_async(
                &Rectangle::new(Point::new(self.x.start, self.y), Size::new(width, 1)),
                color,
            )
            .await
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
        let mut l1 = Scanline::new_empty(0);
        l1.extend(s1);
        l1.extend(e1);

        let mut l2 = Scanline::new_empty(0);
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

    #[test]
    fn issue_489_filled_triangle_bug() {
        let mut l1 = Scanline { y: 5, x: 18..20 };
        let l2 = Scanline { y: 5, x: 11..26 };

        assert_eq!(l1.touches(&l2), true, "l1 touches l2");

        let result = l1.try_extend(&l2);

        assert_eq!(result, true);
        assert_eq!(l1, Scanline { y: 5, x: 11..26 });
    }
}
