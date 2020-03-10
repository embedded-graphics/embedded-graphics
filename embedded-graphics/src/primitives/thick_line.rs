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

/// TODO: Docs
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ThickLineIterator<C: PixelColor> {
    error: i32,
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
            error,
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
        }
    }
}

impl<C> Iterator for ThickLineIterator<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(point) = self.extra_perp.as_mut().and_then(|it| it.next()) {
            Some(Pixel(point, self.style.fill_color.unwrap()))
        } else if let Some(point) = self.perp.next() {
            self.extra_perp = None;

            Some(Pixel(point, self.style.stroke_color.unwrap()))
        } else if self.start == self.end
            || self.style.stroke_width == 0
            || self.style.stroke_color.is_none()
        {
            None
        } else {
            if self.error > self.threshold {
                self.start += self.step_minor;

                self.error += self.e_diag;

                if self.p_error >= self.threshold {
                    self.p_error += self.e_diag;

                    // Draw an extra perpendicular only if required. Lines with a stroke width of 1
                    // don't have any "double square" moves to compensate for, so the extra
                    // perpendicular is not required.
                    if self.draw_extra && self.style.stroke_width > 1 {
                        self.extra_perp = Some(PerpLineIterator::new(
                            self.start,
                            self.dx,
                            self.dy,
                            self.side_thickness,
                            self.p_error + self.e_square,
                            self.error,
                            self.direction,
                            self.step_minor,
                            self.step_major,
                        ));
                    }
                }

                self.p_error += self.e_square;
            }

            self.error += self.e_square;

            self.start += self.step_major;

            self.perp = PerpLineIterator::new(
                self.start,
                self.dx,
                self.dy,
                self.side_thickness,
                self.p_error,
                self.error,
                self.direction,
                self.step_minor,
                self.step_major,
            );

            Self::next(self)
        }
    }
}
