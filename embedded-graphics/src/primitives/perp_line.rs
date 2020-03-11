//! TODO: Docs

use super::thick_line::Side;
use crate::geometry::Point;

/// Pixel iterator for each pixel in the line
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct JoinerIterator {
    start: Point,
    end: Point,
    direction: Point,
    iters: u32,
    threshold: i32,
    e_diag: i32,
    e_square: i32,
    dx: i32,
    dy: i32,
    error: i32,
    step_major: Point,
    step_minor: Point,
    side: Side,
}

impl JoinerIterator {
    /// Create a new line iterator from a `Line`
    pub(crate) fn new(
        start: Point,
        end: Point,
        dx: i32,
        dy: i32,
        e_square: i32,
        e_diag: i32,
        threshold: i32,
        direction: Point,
        step_major: Point,
        step_minor: Point,
        initial_error: i32,
        side: Side,
    ) -> Self {
        Self {
            start,
            end,
            dx,
            dy,
            error: initial_error,
            threshold,
            e_diag,
            e_square,
            direction,
            iters: 0,
            step_major,
            step_minor,
            side,
        }
    }
}

// [Bresenham's line algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm)
impl Iterator for JoinerIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        self.iters += 1;

        if self.iters <= self.dx as u32 {
            match self.side {
                Side::Left => {
                    if self.error > self.threshold {
                        self.start += self.step_minor;
                        self.error += self.e_diag;
                    }

                    self.start += self.step_major;
                    self.error += self.e_square;
                }
                Side::Right => {
                    if self.error < -self.threshold {
                        self.start += self.step_minor;
                        self.error -= self.e_diag;
                    }

                    self.start += self.step_major;
                    self.error -= self.e_square;
                }
            }

            Some(self.start)
        } else {
            None
        }
    }
}
