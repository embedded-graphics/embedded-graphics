//! TODO: Docs

use crate::drawable::Pixel;
use crate::geometry::Point;
use crate::pixelcolor::PixelColor;
use crate::primitives::perp_line::JoinerIterator;
use crate::style::PrimitiveStyle;

/// TODO: Docs
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ThickLine<C: PixelColor> {
    start: Point,
    end: Point,
    style: PrimitiveStyle<C>,
    draw_extra: bool,
    offs: i32,
}

impl<C> ThickLine<C>
where
    C: PixelColor,
{
    /// TODO: Docs
    pub fn new(
        start: Point,
        end: Point,
        style: PrimitiveStyle<C>,
        draw_extra: bool,
        offs: i32,
    ) -> Self {
        Self {
            start,
            end,
            style,
            draw_extra,
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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ThickLineIterator<C: PixelColor> {
    error_l: i32,
    error_r: i32,
    threshold: i32,
    e_diag: i32,
    e_square: i32,
    dx: i32,
    dy: i32,
    length: i32,
    style: PrimitiveStyle<C>,
    side_thickness: u32,
    draw_extra: bool,
    direction: Point,
    start: Point,
    end: Point,

    start_l: Point,
    start_r: Point,
    end_l: Point,
    end_r: Point,
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

    tk: u32,
    side: Side,

    joiner: JoinerIterator,
    extra_joiner: Option<JoinerIterator>,
}

impl<C> ThickLineIterator<C>
where
    C: PixelColor,
{
    /// TODO: Docs
    pub fn new(line: &ThickLine<C>, style: PrimitiveStyle<C>) -> Self {
        let dx = line.end.x - line.start.x;
        let dy = line.end.y - line.start.y;

        let direction = match (dx >= 0, dy >= 0) {
            (true, true) => Point::new(1, 1),
            (true, false) => Point::new(1, -1),
            (false, true) => Point::new(-1, 1),
            (false, false) => Point::new(-1, -1),
        };

        // Originally contained a `sqrt()` call. Removed by squaring all components
        let side_thickness =
            4 * line.style.stroke_width.pow(2) * (dx.pow(2) as u32 + dy.pow(2) as u32);

        let mut dx = dx.abs();
        let mut dy = dy.abs();

        let (step_major, step_minor) = if dy > dx {
            // Swap components if line is Y-major
            core::mem::swap(&mut dx, &mut dy);

            (Point::new(0, direction.y), Point::new(direction.x, 0))
        } else {
            (Point::new(direction.x, 0), Point::new(0, direction.y))
        };

        let threshold = dx - 2 * dy;
        let e_diag = -2 * dx;
        let e_square = 2 * dy;

        Self {
            step_major,
            step_minor,
            error_l: 0,
            error_r: 0,
            dx,
            dy,
            start: line.start,
            end: line.end,
            threshold,
            e_diag,
            e_square,
            length: dx,
            style,
            draw_extra: line.draw_extra,
            side_thickness,
            p_error_l: 0,
            p_error_r: 0,
            direction,
            tk: (dx + dy) as u32,
            // Next side to draw on will be left side
            side: Side::Left,
            start_l: line.start,
            start_r: line.start,
            end_l: line.end,
            end_r: line.end,
            // Initialise joiner iter to draw center line first
            joiner: JoinerIterator::new(
                line.start,
                line.end,
                dx,
                dy,
                e_square,
                e_diag,
                threshold,
                direction,
                step_major,
                step_minor,
                0,
                Side::Left,
            ),
            extra_joiner: None,
        }
    }
}

impl<C> Iterator for ThickLineIterator<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        let color = if let Some(c) = self.style.stroke_color {
            c
        } else {
            // Don't draw line if no stroke color is set
            return None;
        };

        // Quit iterator if width threshold is reached or the line has no length/thickness
        if self.tk.pow(2) > self.side_thickness || self.dx == 0 || self.style.stroke_width == 0 {
            return None;
        }

        if let Some(point) = self.extra_joiner.as_mut().and_then(|it| it.next()) {
            Some(Pixel(point, color))
        } else if let Some(point) = self.joiner.next() {
            Some(Pixel(point, color))
        } else {
            match self.side {
                Side::Left => {
                    let start = self.start_l;
                    let end = self.end_l;

                    if self.error_l > self.threshold {
                        self.start_l += self.step_major;
                        self.end_l += self.step_major;
                        self.error_l += self.e_diag;
                        self.tk += 2 * self.dy as u32;

                        if self.p_error_l > self.threshold {
                            self.extra_joiner = Some(JoinerIterator::new(
                                start,
                                end,
                                self.dx,
                                self.dy,
                                self.e_square,
                                self.e_diag,
                                self.threshold,
                                self.direction,
                                self.step_major,
                                self.step_minor,
                                self.p_error_l + self.e_diag,
                                Side::Left,
                            ));

                            self.p_error_l += self.e_diag;
                        }

                        self.p_error_l += self.e_square;
                    }

                    self.start_l -= self.step_minor;
                    self.end_l -= self.step_minor;
                    self.error_l += self.e_square;
                    self.tk += 2 * self.dx as u32;

                    self.side = Side::Right;

                    self.joiner = JoinerIterator::new(
                        self.start_l,
                        self.end_l,
                        self.dx,
                        self.dy,
                        self.e_square,
                        self.e_diag,
                        self.threshold,
                        self.direction,
                        self.step_major,
                        self.step_minor,
                        self.p_error_l,
                        Side::Left,
                    );

                    Self::next(self)
                }
                Side::Right => {
                    let start = self.start_r;
                    let end = self.end_r;

                    if self.error_r > self.threshold {
                        self.start_r -= self.step_major;
                        self.end_r -= self.step_major;
                        self.error_r += self.e_diag;
                        self.tk += 2 * self.dy as u32;

                        if self.p_error_r > self.threshold {
                            self.extra_joiner = Some(JoinerIterator::new(
                                start - self.step_major,
                                end,
                                self.dx,
                                self.dy,
                                self.e_square,
                                self.e_diag,
                                self.threshold,
                                self.direction,
                                self.step_major,
                                self.step_minor,
                                self.p_error_r + self.e_diag + self.e_square,
                                Side::Right,
                            ));

                            self.p_error_r += self.e_diag;
                        }

                        self.p_error_r += self.e_square;
                    }

                    self.start_r += self.step_minor;
                    self.end_r += self.step_minor;
                    self.error_r += self.e_square;
                    self.tk += 2 * self.dx as u32;

                    self.side = Side::Left;

                    self.joiner = JoinerIterator::new(
                        self.start_r,
                        self.end_r,
                        self.dx,
                        self.dy,
                        self.e_square,
                        self.e_diag,
                        self.threshold,
                        self.direction,
                        self.step_major,
                        self.step_minor,
                        self.p_error_r,
                        Side::Right,
                    );

                    Self::next(self)
                }
            }
        }
    }
}
