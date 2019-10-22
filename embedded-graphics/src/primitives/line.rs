//! The line primitive

use super::super::drawable::{Drawable, Pixel};
use super::super::transform::Transform;
use crate::geometry::{Dimensions, Point, Size};
use crate::pixelcolor::PixelColor;
use crate::pixelcolor::{Rgb888, RgbColor};
use crate::primitives::Primitive;
use crate::style::Style;
use crate::style::WithStyle;
use integer_sqrt::IntegerSquareRoot;

/// Line primitive
///
/// # Examples
///
/// The [macro examples](../../macro.egline.html) make for more concise code.
///
/// ## Create some lines with different styles
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::primitives::Line;
/// use embedded_graphics::pixelcolor::Rgb565;
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::default();
///
/// // Default line from (10, 20) to (30, 40)
/// let l1 = Line::new(Point::new(10, 20), Point::new(30, 40));
///
/// // Line with styled stroke from (50, 20) to (60, 35)
/// let l2 = Line::new(Point::new(50, 20), Point::new(60, 35))
///     .stroke_color(Some(Rgb565::RED));
///
/// // Line with translation applied
/// let l3 = Line::new(Point::new(50, 20), Point::new(60, 35))
///     .translate(Point::new(65, 35));
///
/// display.draw(l1);
/// display.draw(l2);
/// display.draw(l3);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Line<C: PixelColor> {
    /// Start point
    pub start: Point,

    /// End point
    pub end: Point,

    /// Line style
    pub style: Style<C>,

    /// DELETEME
    show_extra_perp: bool,
}

impl<C> Primitive for Line<C> where C: PixelColor {}

impl<C> Dimensions for Line<C>
where
    C: PixelColor,
{
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

impl<C> Line<C>
where
    C: PixelColor,
{
    /// Create a new line
    pub fn new(start: Point, end: Point) -> Self {
        Line {
            start,
            end,
            style: Style::default(),
            show_extra_perp: false,
        }
    }

    /// DELETEME
    pub fn show_extra_perp(&mut self, show: bool) -> &mut Self {
        self.show_extra_perp = show;

        self
    }
}

impl<C> WithStyle<C> for Line<C>
where
    C: PixelColor,
{
    fn style(mut self, style: Style<C>) -> Self {
        self.style = style;

        self
    }

    fn stroke_color(mut self, color: Option<C>) -> Self {
        self.style.stroke_color = color;

        self
    }

    fn stroke_width(mut self, width: u8) -> Self {
        self.style.stroke_width = width;

        self
    }

    fn fill_color(mut self, color: Option<C>) -> Self {
        self.style.fill_color = color;

        self
    }
}

impl<C> IntoIterator for Line<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = LineIterator<C>;

    fn into_iter(self) -> Self::IntoIter {
        (&self).into_iter()
    }
}

impl<'a, C: PixelColor> IntoIterator for &'a Line<C> {
    type Item = Pixel<C>;
    type IntoIter = LineIterator<C>;

    /// Create a line iterator
    ///
    /// ### Quadrants
    ///
    /// ```
    /// 3 | 0
    /// --+--
    /// 2 | 1
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        let mut delta = (self.end - self.start).abs();

        // // Ensure delta points into quadrant 1 so signs are always positive
        // if delta.x < 0 {
        //     delta = Point::new(-delta.x, delta.y);
        // }
        // if delta.y > 0 {
        //     delta = Point::new(delta.x, -delta.y);
        // }

        let direction = match (self.start.x >= self.end.x, self.start.y >= self.end.y) {
            // Quadrant 0
            (false, true) => Point::new(1, -1),
            // Quadrant 1
            (false, false) => Point::new(1, 1),
            // Quadrant 2
            (true, false) => Point::new(-1, 1),
            // Quadrant 3
            (true, true) => Point::new(-1, -1),
        };

        let perp_direction = match (self.start.x >= self.end.x, self.start.y >= self.end.y) {
            // Quadrant 0
            (false, true) => Point::new(-1, -1),
            // Quadrant 1
            (false, false) => Point::new(1, -1),
            // Quadrant 2
            (true, false) => Point::new(-1, -1),
            // Quadrant 3
            (true, true) => Point::new(1, -1),
        };

        // let len = (delta.x.pow(2) + delta.y.pow(2)).integer_sqrt();

        // let normal = Point::new(
        //     (delta.x * self.style.stroke_width as i32) / len,
        //     (delta.y * self.style.stroke_width as i32) / len,
        // );

        // let width: i32 = normal.x.abs().max(normal.y.abs());

        // let width = dbg!(((delta.x.pow(2) + delta.y.pow(2)) as f32).sqrt() / 2.0) as i32;

        // let width = 5;

        // let perp_err = delta.x + delta.y;

        LineIterator {
            style: self.style,

            start: self.start,
            end: self.end,
            delta,
            direction,
            err: 0,
            stop: self.start == self.end, // if line length is zero, draw nothing
            num_iter: 0,
            show_extra_perp: self.show_extra_perp,
            perp: PerpLineIterator {
                start: self.start,
                color: self.style.test_color,
                width: self.style.stroke_width as u32,
                delta,
                direction: perp_direction,
                err: 0,
                current_iter: 0,
                stop: false,
            },
        }
    }
}

/// Pixel iterator for each pixel in the line
#[derive(Debug, Clone, Copy)]
pub struct LineIterator<C>
where
    C: PixelColor,
{
    style: Style<C>,

    start: Point,
    end: Point,
    delta: Point,
    /// in which quadrant is the line drawn (upper-left=(-1, -1), lower-right=(1, 1), ...)
    direction: Point,
    err: i32,
    stop: bool,
    num_iter: u32,
    // width: u32,
    perp: PerpLineIterator<C>,
    // extra_perp: PerpLineIterator<C>,
    show_extra_perp: bool,
    // perp_err: i32,
    // is_diag: bool
}

// [Bresenham's line algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm)
impl<C: PixelColor> Iterator for LineIterator<C> {
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        // return none if stroke color is none
        self.style.stroke_color?;

        if let Some(perp) = self.perp.next() {
            return Some(perp);
        }

        if !self.stop {
            let start = self.start;

            if self.start == self.end || self.num_iter > 500 {
                self.stop = true;
            }

            if self.delta.x >= self.delta.y {
                let threshold = self.delta.x - 2 * self.delta.y;
                let e_diag = -2 * self.delta.x;
                let e_square = 2 * self.delta.y;

                if self.err > threshold {
                    self.start += Point::new(0, self.direction.y);

                    self.err += e_diag;
                }

                self.err += e_square;

                self.start += Point::new(self.direction.x, 0);
            } else {
                let threshold = self.delta.y - 2 * self.delta.x;
                let e_diag = -2 * self.delta.y;
                let e_square = 2 * self.delta.x;

                if self.err > threshold {
                    self.start += Point::new(self.direction.x, 0);

                    self.err += e_diag;
                }

                self.err += e_square;

                self.start += Point::new(0, self.direction.y);
            }

            self.num_iter += 1;

            self.perp = PerpLineIterator {
                start,
                color: if self.perp.color == self.style.stroke_color {
                    self.style.fill_color
                } else {
                    self.style.stroke_color
                },
                width: self.style.stroke_width as u32,
                err: 0,
                stop: false,
                current_iter: 0,
                ..self.perp
                // delta,
                // direction: perp_direction,
                // err: 0,
                // current_iter: 0,
                // stop: false,
            };

            self.perp.next()

        // Some(Pixel(self.start, self.style.stroke_color.unwrap()))
        } else {
            None
        }
    }
}

/// TODO: Docs
#[derive(Debug, Clone, Copy)]
pub struct PerpLineIterator<C>
where
    C: PixelColor,
{
    color: Option<C>,
    start: Point,
    width: u32,
    delta: Point,
    direction: Point,
    err: i32,
    current_iter: u32,
    stop: bool,
}

impl<C: PixelColor> Iterator for PerpLineIterator<C> {
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        // Noop if color is none (line is transparent)
        self.color?;

        if !self.stop {
            if self.current_iter >= self.width {
                self.stop = true;
            }

            let point = self.start;

            if self.delta.x >= self.delta.y {
                if self.err > self.delta.x - 2 * self.delta.y {
                    self.start += Point::new(self.direction.y, 0);

                    self.err -= 2 * self.delta.x;
                }

                self.err += 2 * self.delta.y;

                self.start += Point::new(0, self.direction.x);
            } else {
                if self.err > self.delta.y - 2 * self.delta.x {
                    self.start += Point::new(0, self.direction.x);

                    self.err -= 2 * self.delta.y;
                }

                self.err += 2 * self.delta.x;

                self.start += Point::new(self.direction.y, 0);
            }

            self.current_iter += 1;

            Some(Pixel(point, self.color.unwrap()))
        } else {
            None
        }
    }
}

impl<C> Drawable for Line<C> where C: PixelColor {}

impl<C> Transform for Line<C>
where
    C: PixelColor,
{
    /// Translate the line from its current position to a new position by (x, y) pixels, returning
    /// a new `Line`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Line;
    /// # use embedded_graphics::prelude::*;
    /// # use embedded_graphics::pixelcolor::BinaryColor;
    /// #
    /// # let style = Style::stroke_color(BinaryColor::On);
    /// #
    /// let line = Line::new(Point::new(5, 10), Point::new(15, 20))
    /// #    .style(style);
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
    /// # use embedded_graphics::pixelcolor::BinaryColor;
    /// #
    /// # let style = Style::stroke_color(BinaryColor::On);
    /// #
    /// let mut line = Line::new(Point::new(5, 10), Point::new(15, 20))
    /// #    .style(style);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::drawable::Pixel;
    use crate::pixelcolor::BinaryColor;
    use crate::style::Style;

    fn test_expected_line(start: Point, end: Point, expected: &[(i32, i32)]) {
        let line = Line::new(start, end).style(Style::stroke_color(BinaryColor::On));
        let mut expected_iter = expected.iter();
        for Pixel(coord, _) in line.into_iter() {
            match expected_iter.next() {
                Some(point) => assert_eq!(coord, Point::from(*point)),
                // expected runs out of points before line does
                None => unreachable!(),
            }
        }
        // check that expected has no points left
        assert!(expected_iter.next().is_none())
    }

    #[test]
    fn bounding_box() {
        let start = Point::new(10, 10);
        let end = Point::new(20, 20);

        let line: Line<BinaryColor> = Line::new(start, end);
        let backwards_line: Line<BinaryColor> = Line::new(end, start);

        assert_eq!(line.top_left(), start);
        assert_eq!(line.bottom_right(), end);
        assert_eq!(line.size(), Size::new(10, 10));

        assert_eq!(backwards_line.top_left(), start);
        assert_eq!(backwards_line.bottom_right(), end);
        assert_eq!(backwards_line.size(), Size::new(10, 10));
    }

    #[test]
    fn draws_no_dot() {
        let start = Point::new(10, 10);
        let end = Point::new(10, 10);
        let expected = [];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_short_correctly() {
        let start = Point::new(2, 3);
        let end = Point::new(3, 2);
        let expected = [(2, 3), (3, 2)];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_1_correctly() {
        let start = Point::new(10, 10);
        let end = Point::new(15, 13);
        let expected = [(10, 10), (11, 11), (12, 11), (13, 12), (14, 12), (15, 13)];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_2_correctly() {
        let start = Point::new(10, 10);
        let end = Point::new(13, 15);
        let expected = [(10, 10), (11, 11), (11, 12), (12, 13), (12, 14), (13, 15)];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_3_correctly() {
        let start = Point::new(10, 10);
        let end = Point::new(7, 15);
        let expected = [(10, 10), (9, 11), (9, 12), (8, 13), (8, 14), (7, 15)];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_4_correctly() {
        let start = Point::new(10, 10);
        let end = Point::new(5, 13);
        let expected = [(10, 10), (9, 11), (8, 11), (7, 12), (6, 12), (5, 13)];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_5_correctly() {
        let start = Point::new(10, 10);
        let end = Point::new(5, 7);
        let expected = [(10, 10), (9, 9), (8, 9), (7, 8), (6, 8), (5, 7)];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_6_correctly() {
        let start = Point::new(10, 10);
        let end = Point::new(7, 5);
        let expected = [(10, 10), (9, 9), (9, 8), (8, 7), (8, 6), (7, 5)];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_7_correctly() {
        let start = Point::new(10, 10);
        let end = Point::new(13, 5);
        let expected = [(10, 10), (11, 9), (11, 8), (12, 7), (12, 6), (13, 5)];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_8_correctly() {
        let start = Point::new(10, 10);
        let end = Point::new(15, 7);
        let expected = [(10, 10), (11, 9), (12, 9), (13, 8), (14, 8), (15, 7)];
        test_expected_line(start, end, &expected);
    }
}
