//! TODO: Docs

use super::thick_line::Side;
use crate::geometry::Point;

/// Pixel iterator for each pixel in the line
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct JoinerIterator {
    start: Point,
    end: Point,
    // delta: Point,
    /// in which quadrant is the line drawn (upper-left=(-1, -1), lower-right=(1, 1), ...)
    direction: Point,
    // err: i32,
    stop: bool,
    iters: u32,

    threshold: i32,
    e_diag: i32,
    e_square: i32,
    dx: i32,
    dy: i32,
    error: i32,
    step_major: Point,
    step_minor: Point,
    is_extra: bool,
}

impl JoinerIterator {
    /// Create a new line iterator from a `Line`
    pub(crate) fn new(
        start: Point,
        end: Point,
        dx: i32,
        dy: i32,
        direction: Point,
        step_major: Point,
        step_minor: Point,
        initial_error: i32,
        is_extra: bool,
        side: Side,
    ) -> Self {
        // dbg!((side, start, is_extra));

        Self {
            start,
            end,
            // delta,
            // direction,
            // err: initial_error,
            // stop: start == end, // If line length is zero, draw nothing
            // iters: 0,
            dx,
            dy,
            error: initial_error,
            threshold: dx - 2 * dy,
            e_diag: -2 * dx,
            e_square: 2 * dy,
            direction,
            iters: 0,
            stop: false,
            step_major,
            step_minor,
            is_extra,
        }
    }
}

// [Bresenham's line algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm)
impl Iterator for JoinerIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        self.iters += 1;

        if !self.stop && self.iters <= self.dx as u32 {
            if self.error > self.threshold {
                self.start += self.step_minor;
                self.error += self.e_diag;
            }

            self.start += self.step_major;
            self.error += self.e_square;

            if self.start == self.end {
                self.stop = true;
            }

            Some(self.start)
        } else {
            None
        }
    }
}
