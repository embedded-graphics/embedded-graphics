//! The line primitive

use crate::{
    drawable::{Drawable, Pixel},
    geometry::{Dimensions, Point, Size},
    pixelcolor::PixelColor,
    primitives::Primitive,
    style::{PrimitiveStyle, Styled},
    transform::Transform,
    DrawTarget,
};

/// Line primitive
///
/// # Examples
///
/// The [macro examples](../../macro.egline.html) make for more concise code.
///
/// ## Create some lines with different styles
///
/// ```rust
/// use embedded_graphics::{
///     pixelcolor::Rgb565, prelude::*, primitives::Line, style::PrimitiveStyle,
/// };
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::default();
///
/// // Red 1 pixel wide line from (50, 20) to (60, 35)
/// Line::new(Point::new(50, 20), Point::new(60, 35))
///     .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 1))
///     .draw(&mut display)?;
///
/// // Green 1 pixel wide line with translation applied
/// Line::new(Point::new(50, 20), Point::new(60, 35))
///     .translate(Point::new(65, 35))
///     .into_styled(PrimitiveStyle::with_stroke(Rgb565::GREEN, 1))
///     .draw(&mut display)?;
/// # Ok::<(), core::convert::Infallible>(())
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Line {
    /// Start point
    pub start: Point,

    /// End point
    pub end: Point,
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

impl Line {
    /// Create a new line
    pub const fn new(start: Point, end: Point) -> Self {
        Line { start, end }
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
        let mut delta = self.primitive.end - self.primitive.start;
        if delta.x < 0 {
            delta = Point::new(-delta.x, delta.y);
        }
        if delta.y > 0 {
            delta = Point::new(delta.x, -delta.y);
        }

        let direction = match (
            self.primitive.start.x >= self.primitive.end.x,
            self.primitive.start.y >= self.primitive.end.y,
        ) {
            (false, false) => Point::new(1, 1),
            (false, true) => Point::new(1, -1),
            (true, false) => Point::new(-1, 1),
            (true, true) => Point::new(-1, -1),
        };

        StyledLineIterator {
            style: self.style,

            start: self.primitive.start,
            end: self.primitive.end,
            delta,
            direction,
            err: delta.x + delta.y,
            stop: self.primitive.start == self.primitive.end, /* if line length is zero, draw nothing */
        }
    }
}

/// Pixel iterator for each pixel in the line
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct StyledLineIterator<C>
where
    C: PixelColor,
{
    style: PrimitiveStyle<C>,

    start: Point,
    end: Point,
    delta: Point,
    /// in which quadrant is the line drawn (upper-left=(-1, -1), lower-right=(1, 1), ...)
    direction: Point,
    err: i32,
    stop: bool,
}

// [Bresenham's line algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm)
impl<C: PixelColor> Iterator for StyledLineIterator<C> {
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        // return none if stroke color is none
        self.style.stroke_color?;

        if !self.stop {
            let point = self.start;

            if self.start == self.end {
                self.stop = true;
            }
            let err_double = 2 * self.err;
            if err_double > self.delta.y {
                self.err += self.delta.y;
                self.start += Point::new(self.direction.x, 0);
            }
            if err_double < self.delta.x {
                self.err += self.delta.x;
                self.start += Point::new(0, self.direction.y);
            }

            Some(Pixel(point, self.style.stroke_color.unwrap()))
        } else {
            None
        }
    }
}

impl<'a, C: 'a> Drawable<C> for &Styled<Line, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        display.draw_line(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{drawable::Pixel, pixelcolor::BinaryColor};

    fn test_expected_line(start: Point, end: Point, expected: &[(i32, i32)]) {
        let line =
            Line::new(start, end).into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1));
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

        let line: Line = Line::new(start, end);
        let backwards_line: Line = Line::new(end, start);

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
