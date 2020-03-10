//! TODO: Docs

use crate::drawable::Pixel;
use crate::geometry::Point;
use crate::pixelcolor::PixelColor;
use crate::primitives::perp_line::PerpLineIterator;
use crate::style::PrimitiveStyle;
use integer_sqrt::IntegerSquareRoot;

/// TODO: Docs
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ThickLine<C: PixelColor> {
    start: Point,
    end: Point,
    style: PrimitiveStyle<C>,
    draw_extra: bool,
}

impl<C> ThickLine<C>
where
    C: PixelColor,
{
    /// TODO: Docs
    pub fn new(start: Point, end: Point, style: PrimitiveStyle<C>, draw_extra: bool) -> Self {
        Self {
            start,
            end,
            style,
            draw_extra,
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

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum Side {
    Left,
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
    perp: PerpLineIterator,
    extra_perp: Option<PerpLineIterator>,
    side_thickness: u32,
    p_error: i32,
    draw_extra: bool,
    direction: Point,
    start: Point,
    end: Point,

    point_l: Point,
    point_r: Point,

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

        let side_thickness =
            2 * line.style.stroke_width * (dx.pow(2) as u32 + dy.pow(2) as u32).integer_sqrt();

        let f_side_thickness =
            2.0 * line.style.stroke_width as f32 * (dx.pow(2) as f32 + dy.pow(2) as f32).sqrt();

        dbg!((side_thickness, f_side_thickness));

        let side_thickness = f_side_thickness.round() as u32;

        let error = 0;
        let p_error = 0;

        let mut dx = dx.abs();
        let mut dy = dy.abs();

        let (step_major, step_minor) = if dy > dx {
            // Swap components if line is Y-major
            core::mem::swap(&mut dx, &mut dy);

            (Point::new(0, direction.y), Point::new(direction.x, 0))
        } else {
            (Point::new(direction.x, 0), Point::new(0, direction.y))
        };

        Self {
            step_major,
            step_minor,
            error_l: 0,
            error_r: 0,
            dx,
            dy,
            start: line.start,
            end: line.end,
            threshold: dx - 2 * dy,
            e_diag: -2 * dx,
            e_square: 2 * dy,
            length: dx,
            style,
            draw_extra: line.draw_extra,
            perp: PerpLineIterator::new(
                line.start,
                dx,
                dy,
                side_thickness,
                p_error,
                error,
                direction,
                step_minor,
                step_major,
            ),
            extra_perp: None,
            side_thickness,
            p_error,
            direction,
            tk: (dx + dy) as u32,
            side: Side::Left,
            point_l: line.start,
            point_r: line.start,
        }
    }
}

impl<C> Iterator for ThickLineIterator<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.side {
            Side::Left if self.tk > self.side_thickness => None,
            Side::Right if self.tk > self.side_thickness => None,
            Side::Left => {
                let point = self.point_l;

                if self.error_l > self.threshold {
                    self.point_l += self.step_major;
                    self.error_l += self.e_diag;
                    self.tk += 2 * self.dy as u32;
                }

                self.point_l -= self.step_minor;
                self.error_l += self.e_square;
                self.tk += 2 * self.dx as u32;

                self.side = Side::Right;

                Some(Pixel(point, self.style.stroke_color.unwrap()))
            }
            Side::Right => {
                if self.error_r >= self.threshold {
                    self.point_r -= self.step_major;
                    self.error_r += self.e_diag;
                    self.tk += 2 * self.dy as u32;
                }

                self.point_r += self.step_minor;
                self.error_r += self.e_square;
                self.tk += 2 * self.dx as u32;

                self.side = Side::Left;

                Some(Pixel(self.point_r, self.style.stroke_color.unwrap()))
            }
        }
    }
}
