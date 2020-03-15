//! TODO: Docs

use super::thick_line::Side;
use crate::geometry::Point;

/// Pixel iterator for each pixel in the line
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct JoinerIterator {
    start: Point,
    end: Point,
    direction: Point,
    dx_accum: u32,
    threshold: i32,
    e_diag: i32,
    e_square: i32,
    dx: u32,
    dy: u32,
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
        dx: u32,
        dy: u32,
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
            dx_accum: 0,
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
        self.dx_accum += 1;

        if self.dx_accum <= self.dx {
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
