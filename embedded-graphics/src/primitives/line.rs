//! TODO: Docs

use crate::draw_target::DrawTarget;
use crate::drawable::Drawable;
use crate::drawable::Pixel;
use crate::geometry::Dimensions;
use crate::geometry::Point;
use crate::geometry::Size;
use crate::pixelcolor::PixelColor;
use crate::primitives::Primitive;
use crate::style::PrimitiveStyle;
use crate::style::Styled;
use crate::transform::Transform;

/// TODO: Docs
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    /// TODO: Docs
    pub const fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }
}

impl Primitive for Line {}

impl Dimensions for Line {
    fn top_left(&self) -> Point {
        Point::new(self.start.x.min(self.end.x), self.start.y.min(self.end.y))
    }

    fn bottom_right(&self) -> Point {
        self.top_left() + self.size()
    }

    fn size(&self) -> Size {
        Size::from_bounding_box(self.start, self.end)
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
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct LineIterator {
    error_l: i32,
    error_r: i32,
    threshold: i32,
    e_diag: i32,
    e_square: i32,

    /// The "major" (greater) delta. Swapped with `dy` if dy is greater than dx
    dx: u32,

    /// The "minor" (lesser) delta. Swapped with `dx` if dx is greater than dy
    dy: u32,
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

impl LineIterator {
    /// TODO: Docs
    pub fn new(line: &Line, stroke_width: u32) -> Self {
        let dx: i32 = line.end.x - line.start.x;
        let dy: i32 = line.end.y - line.start.y;

        let direction = match (dx >= 0, dy >= 0) {
            (true, true) => Point::new(1, 1),
            (true, false) => Point::new(1, -1),
            (false, true) => Point::new(-1, 1),
            (false, false) => Point::new(-1, -1),
        };

        // Originally contained a `sqrt()` call. Removed by squaring all components
        let thickness = 4 * stroke_width.pow(2) * (dx.pow(2) as u32 + dy.pow(2) as u32);

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

impl Iterator for LineIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        // Quit iterator if width threshold is reached or the line has no length
        if self.thickness_accum.pow(2) > self.thickness || self.dx == 0 {
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

            Some(self.state.start)
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

impl Transform for Line {
    /// Translate the line from its current position to a new position by (x, y) pixels, returning
    /// a new `Line`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Line;
    /// # use embedded_graphics::prelude::*;
    /// let line = Line::new(Point::new(5, 10), Point::new(15, 20));
    /// let moved = line.translate(Point::new(10, 10));
    ///
    /// assert_eq!(moved.start, Point::new(15, 20));
    /// assert_eq!(moved.end, Point::new(25, 30));
    /// ```
    fn translate(&self, by: Point) -> Self {
        Self {
            start: self.start + by,
            end: self.end + by,
            ..*self
        }
    }

    /// Translate the line from its current position to a new position by (x, y) pixels.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Line;
    /// # use embedded_graphics::prelude::*;
    /// let mut line = Line::new(Point::new(5, 10), Point::new(15, 20));
    /// line.translate_mut(Point::new(10, 10));
    ///
    /// assert_eq!(line.start, Point::new(15, 20));
    /// assert_eq!(line.end, Point::new(25, 30));
    /// ```
    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.start += by;
        self.end += by;

        self
    }
}

impl<'a, C> IntoIterator for &'a Styled<Line, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = StyledLineIterator<C>;

    fn into_iter(self) -> Self::IntoIter {
        StyledLineIterator {
            style: self.style,

            line_iter: LineIterator::new(&self.primitive, self.style.stroke_width),
        }
    }
}

/// Pixel iterator for each pixel in the line
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct StyledLineIterator<C>
where
    C: PixelColor,
{
    style: PrimitiveStyle<C>,

    line_iter: LineIterator,
}

// [Bresenham's line algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm)
impl<C: PixelColor> Iterator for StyledLineIterator<C> {
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        // Break if stroke width is zero
        if self.style.stroke_width == 0 {
            return None;
        }

        // Return none if stroke color is none
        let stroke_color = self.style.stroke_color?;

        self.line_iter
            .next()
            .map(|point| Pixel(point, stroke_color))
    }
}

impl<'a, C: 'a> Drawable<C> for &Styled<Line, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        // display.draw_line(self)
        display.draw_iter(self)
    }
}
