//! TODO: Docs

use crate::geometry::Point;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum Side {
    Left,
    Right,
}

/// TODO: Docs
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PerpLineIterator {
    error: i32,
    start: Point,
    point: Point,
    threshold: i32,
    e_diag: i32,
    e_square: i32,
    width: u32,
    tk: i32,
    dx: i32,
    dy: i32,
    direction: Point,
    step_major: Point,
    step_minor: Point,
    side: Side,
    winit: i32,
    initial_error: i32,
    q: u32,
    p: u32,
}

impl PerpLineIterator {
    /// TODO: Docs
    pub fn new(
        start: Point,
        dx: i32,
        dy: i32,
        width: u32,
        error: i32,
        winit: i32,
        direction: Point,
        step_minor: Point,
        step_major: Point,
    ) -> Self {
        Self {
            start,
            error,
            initial_error: error,
            direction,
            dx,
            dy,
            point: start,
            threshold: dx - 2 * dy,
            e_diag: -2 * dx,
            e_square: 2 * dy,
            width,
            winit,
            tk: dx + dy + winit,
            side: Side::Left,
            step_major,
            step_minor,
            q: 0,
            p: 0,
        }
    }
}

impl Iterator for PerpLineIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.tk > self.width as i32 {
            match self.side {
                // Left side is complete, swap to right side now
                Side::Left => {
                    self.tk = self.dx + self.dy - self.winit;
                    self.point = self.start;
                    self.error = self.initial_error;
                    self.side = Side::Right;

                    // Skip first pixel on right side
                    Self::next(self);
                    Self::next(self)
                    // None
                }

                Side::Right => {
                    // Handle 1px thick lines
                    if self.q == 0 && self.p < 2 {
                        // Crappily set some exit conditions or we get an infinite loop next time we
                        // hit this branch.
                        self.q = 1;
                        self.p = 2;

                        Some(self.start)
                    } else {
                        // Right side is complete, we're done
                        None
                    }
                }
            }
        } else {
            let point = self.point;

            match self.side {
                Side::Left => {
                    if self.error < -self.threshold {
                        self.point += self.step_major;
                        self.error -= self.e_diag;
                        self.tk += 2 * self.dy;
                    }

                    self.error -= self.e_square;
                    self.point -= self.step_minor;
                    self.tk += 2 * self.dx;

                    self.q += 1;
                }
                Side::Right => {
                    if self.error > self.threshold {
                        self.point -= self.step_major;
                        self.error += self.e_diag;
                        self.tk += 2 * self.dy;
                    }

                    self.error += self.e_square;
                    self.point += self.step_minor;
                    self.tk += 2 * self.dx;

                    self.p += 1;
                }
            }

            Some(point)
        }
    }
}
