//! TODO: Docs

use crate::geometry::Point;

/// TODO: Docs
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Side {
    /// TODO: Docs
    Left,
    /// TODO: Docs
    Right,
}

/// TODO: Docs
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PerpLineIterator {
    error: i32,
    point: Point,
    threshold: i32,
    e_diag: i32,
    e_square: i32,
    width: u32,
    side: Side,
    tk: i32,
    dx: i32,
    dy: i32,
    direction: Point,
    x_major: bool,
}

impl PerpLineIterator {
    /// TODO: Docs
    pub fn new(
        start: Point,
        dx: i32,
        dy: i32,
        side: Side,
        width: u32,
        error: i32,
        winit: i32,
        direction: Point,
        x_major: bool,
    ) -> Self {
        Self {
            error,
            direction,
            dx,
            dy,
            point: start,
            threshold: dx - 2 * dy,
            e_diag: -2 * dx,
            e_square: 2 * dy,
            width,
            side,
            tk: match side {
                Side::Right => dx + dy - winit,
                Side::Left => dx + dy + winit,
            },
            x_major,
        }
    }
}

impl Iterator for PerpLineIterator {
    type Item = Point;

    // Octant 1 only
    fn next(&mut self) -> Option<Self::Item> {
        if self.tk > self.width as i32 {
            None
        } else {
            let point = self.point;

            match self.side {
                Side::Right => {
                    if self.error > self.threshold {
                        if self.x_major {
                            self.point -= Point::new(self.direction.x, 0);
                        } else {
                            self.point -= Point::new(0, self.direction.y);
                        }

                        self.error += self.e_diag;

                        self.tk += 2 * self.dy;
                    }

                    self.error += self.e_square;

                    if self.x_major {
                        self.point += Point::new(0, self.direction.y);
                    } else {
                        self.point += Point::new(self.direction.x, 0);
                    }

                    self.tk += 2 * self.dx;
                }
                Side::Left => {
                    if self.error < -self.threshold {
                        if self.x_major {
                            self.point += Point::new(self.direction.x, 0);
                        } else {
                            self.point += Point::new(0, self.direction.y);
                        }

                        self.error -= self.e_diag;

                        self.tk += 2 * self.dy;
                    }

                    self.error -= self.e_square;

                    if self.x_major {
                        self.point -= Point::new(0, self.direction.y);
                    } else {
                        self.point -= Point::new(self.direction.x, 0);
                    }

                    self.tk += 2 * self.dx;
                }
            }

            Some(point)
        }
    }
}
