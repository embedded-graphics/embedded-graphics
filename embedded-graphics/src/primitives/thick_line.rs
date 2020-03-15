//! TODO: Docs

use crate::drawable::Pixel;
use crate::geometry::Point;
use crate::pixelcolor::PixelColor;
use crate::style::PrimitiveStyle;

/// TODO: Docs
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ThickLine<C: PixelColor> {
    start: Point,
    end: Point,
    style: PrimitiveStyle<C>,
    offs: i32,
}

impl<C> ThickLine<C>
where
    C: PixelColor,
{
    /// TODO: Docs
    pub fn new(start: Point, end: Point, style: PrimitiveStyle<C>, offs: i32) -> Self {
        Self {
            start,
            end,
            style,
            offs,
        }
    }
}

impl<C> IntoIterator for ThickLine<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = ThickLineIterator<C>;

    fn into_iter(self) -> Self::IntoIter {
        ThickLineIterator::new(&self, self.style)
    }
}

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
struct ParallelLineState {
    start: Point,
    error: i32,
    dx_accum: u32,
    side: Side,
}

/// TODO: Docs
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ThickLineIterator<C: PixelColor> {
    error_l: i32,
    error_r: i32,
    threshold: i32,
    e_diag: i32,
    e_square: i32,

    /// The "major" (greater) delta. Swapped with `dy` if dy is greater than dx
    dx: u32,

    /// The "minor" (lesser) delta. Swapped with `dx` if dx is greater than dy
    dy: u32,
    style: PrimitiveStyle<C>,
    thickness: u32,
    direction: Point,
    start: Point,
    end: Point,

    start_l: Point,
    start_r: Point,
    p_error_l: i32,
    p_error_r: i32,

    /// The "major" step
    ///
    /// The X or Y component with the larger delta is considered "major". This is the most common
    /// direction to move in.
    step_major: Point,

    /// The "minor" step
    ///
    /// The X or Y component with the smaller delta is considered "minor". This is the less common
    /// direction to move in.
    step_minor: Point,

    thickness_accum: u32,
    side: Side,

    state: ParallelLineState,
}

impl<C> ThickLineIterator<C>
where
    C: PixelColor,
{
    /// TODO: Docs
    pub fn new(line: &ThickLine<C>, style: PrimitiveStyle<C>) -> Self {
        let dx: i32 = line.end.x - line.start.x;
        let dy: i32 = line.end.y - line.start.y;

        let direction = match (dx >= 0, dy >= 0) {
            (true, true) => Point::new(1, 1),
            (true, false) => Point::new(1, -1),
            (false, true) => Point::new(-1, 1),
            (false, false) => Point::new(-1, -1),
        };

        // Originally contained a `sqrt()` call. Removed by squaring all components
        let thickness = 4 * line.style.stroke_width.pow(2) * (dx.pow(2) as u32 + dy.pow(2) as u32);

        let mut dx = dx.abs();
        let mut dy = dy.abs();

        // Swap components if line is Y-major
        let (step_major, step_minor) = if dy > dx {
            core::mem::swap(&mut dx, &mut dy);

            (Point::new(0, direction.y), Point::new(direction.x, 0))
        } else {
            (Point::new(direction.x, 0), Point::new(0, direction.y))
        };

        let threshold = dx - 2 * dy;
        let e_diag = -2 * dx;
        let e_square = 2 * dy;

        // Safe due to abs() call above
        let dx = dx as u32;
        let dy = dy as u32;

        Self {
            step_major,
            step_minor,
            error_l: 0,
            error_r: 0,
            dx: dx,
            dy: dy,
            start: line.start,
            end: line.end,
            threshold,
            e_diag,
            e_square,
            style,
            thickness,
            p_error_l: 0,
            p_error_r: 0,
            direction,
            thickness_accum: dx + dy,
            // Next side to draw on will be left side
            side: Side::Left,
            start_l: line.start,
            start_r: line.start,
            state: ParallelLineState {
                dx_accum: 0,
                error: 0,
                start: line.start,
                side: Side::Left,
            },
        }
    }
}

impl<C> Iterator for ThickLineIterator<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        // Separate block here to remove call to unwrap()
        let color = if let Some(c) = self.style.stroke_color {
            c
        } else {
            // Don't draw line if no stroke color is set
            return None;
        };

        // Quit iterator if width threshold is reached or the line has no length/thickness
        if self.thickness_accum.pow(2) > self.thickness
            || self.dx == 0
            || self.style.stroke_width == 0
        {
            return None;
        }

        self.state.dx_accum += 1;

        if self.state.dx_accum <= self.dx {
            match self.state.side {
                Side::Left => {
                    if self.state.error > self.threshold {
                        self.state.start += self.step_minor;
                        self.state.error += self.e_diag;
                    }

                    self.state.start += self.step_major;
                    self.state.error += self.e_square;
                }
                Side::Right => {
                    if self.state.error < -self.threshold {
                        self.state.start += self.step_minor;
                        self.state.error -= self.e_diag;
                    }

                    self.state.start += self.step_major;
                    self.state.error -= self.e_square;
                }
            }

            Some(Pixel(self.state.start, color))
        } else {
            match self.side {
                Side::Left => {
                    let mut extra = false;
                    let start = self.start_l;

                    if self.error_l > self.threshold {
                        self.start_l += self.step_major;
                        self.error_l += self.e_diag;
                        self.thickness_accum += 2 * self.dy;

                        if self.p_error_l > self.threshold {
                            extra = true;

                            self.state = ParallelLineState {
                                dx_accum: 0,
                                start: start,
                                error: self.p_error_l + self.e_diag,
                                side: Side::Left,
                            };

                            self.side = Side::Right;

                            self.p_error_l += self.e_diag;
                        }

                        self.p_error_l += self.e_square;
                    }

                    if !extra {
                        self.start_l -= self.step_minor;
                        self.error_l += self.e_square;
                        self.thickness_accum += 2 * self.dx;

                        self.side = Side::Right;

                        self.state = ParallelLineState {
                            dx_accum: 0,
                            start: self.start_l,
                            error: self.p_error_l,
                            side: Side::Left,
                        };
                    }
                }
                Side::Right => {
                    let mut extra = false;

                    if self.error_r > self.threshold {
                        self.start_r -= self.step_major;
                        self.error_r += self.e_diag;
                        self.thickness_accum += 2 * self.dy;

                        if self.p_error_r > self.threshold {
                            extra = true;

                            self.state = ParallelLineState {
                                dx_accum: 0,
                                start: self.start_r,
                                error: self.p_error_r + self.e_diag + self.e_square,
                                side: Side::Right,
                            };

                            self.side = Side::Left;

                            self.p_error_r += self.e_diag;
                        }

                        self.p_error_r += self.e_square;
                    }

                    if !extra {
                        self.start_r += self.step_minor;
                        self.error_r += self.e_square;
                        self.thickness_accum += 2 * self.dx;

                        self.side = Side::Left;

                        self.state = ParallelLineState {
                            dx_accum: 0,
                            start: self.start_r,
                            error: self.p_error_r,
                            side: Side::Right,
                        };
                    }
                }
            }

            Self::next(self)
        }
    }
}
