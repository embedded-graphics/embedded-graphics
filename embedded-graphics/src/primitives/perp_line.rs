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
    point_l: Point,
    point_r: Point,
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
}

impl PerpLineIterator {
    /// TODO: Docs
    pub fn new(
        start: Point,
        dx: i32,
        dy: i32,
        width: u32,
        initial_error: i32,
        winit: i32,
        direction: Point,
        step_minor: Point,
        step_major: Point,
        is_extra: bool,
    ) -> Self {
        Self {
            start,
            error: -initial_error,
            initial_error,
            direction,
            dx,
            dy,
            point_l: start,
            point_r: start,
            threshold: dx - 2 * dy,
            e_diag: -2 * dx,
            e_square: 2 * dy,
            width: width,
            winit,
            tk: dx + dy + winit,
            side: Side::Left,
            step_major,
            step_minor,
        }
    }
}

impl Iterator for PerpLineIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.tk > self.width as i32 {
            // match self.side {
            //     // Left side is complete, swap to right side now
            //     Side::Left => {
            //         self.tk = self.dx + self.dy - self.winit;
            //         self.point = self.start;
            //         self.error = self.initial_error;
            //         self.side = Side::Right;

            //         Self::next(self)
            //     }

            //     Side::Right => None,
            // }
            None
        } else {
            match self.side {
                Side::Left => {
                    let point = self.point_l;

                    if self.error > self.threshold {
                        self.point_l += self.step_major;
                        self.error += self.e_diag;
                        self.tk += 2 * self.dy;
                    }

                    self.error += self.e_square;
                    self.point_l -= self.step_minor;
                    self.tk += 2 * self.dx;

                    self.side = Side::Right;

                    Some(point)
                }
                Side::Right => {
                    if self.error > self.threshold {
                        self.point_r -= self.step_major;
                        self.error += self.e_diag;
                        self.tk += 2 * self.dy;
                    }

                    self.error += self.e_square;
                    self.point_r += self.step_minor;
                    self.tk += 2 * self.dx;

                    self.side = Side::Left;

                    Some(self.point_r)
                }
            }
        }
    }
}
