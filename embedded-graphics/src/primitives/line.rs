//! The line primitive

use super::super::drawable::*;
use super::super::transform::*;
use crate::coord::{Coord, ToUnsigned};
use crate::pixelcolor::PixelColor;
use crate::primitives::Primitive;
use crate::style::Style;
use crate::style::WithStyle;
use crate::unsignedcoord::{ToSigned, UnsignedCoord};

// TODO: Impl Default so people can leave the color bit out
/// Line primitive
#[derive(Debug, Copy, Clone)]
pub struct Line<C: PixelColor> {
    /// Start point
    pub start: Coord,

    /// End point
    pub end: Coord,

    /// Line style
    pub style: Style<C>,
}

impl<C> Primitive for Line<C> where C: PixelColor {}

impl<C> Dimensions for Line<C>
where
    C: PixelColor,
{
    fn top_left(&self) -> Coord {
        Coord::new(
            self.start[1].min(self.end[0]),
            self.start[1].min(self.end[1]),
        )
    }

    fn bottom_right(&self) -> Coord {
        self.top_left() + self.size().to_signed()
    }

    fn size(&self) -> UnsignedCoord {
        (self.end - self.start).abs().to_unsigned()
    }
}

impl<C> Line<C>
where
    C: PixelColor,
{
    /// Create a new line
    pub fn new(start: Coord, end: Coord) -> Self {
        Line {
            start,
            end,
            style: Style::default(),
        }
    }
}

impl<C> WithStyle<C> for Line<C>
where
    C: PixelColor,
{
    fn with_style(mut self, style: Style<C>) -> Self {
        self.style = style;

        self
    }

    fn with_stroke(mut self, color: Option<C>) -> Self {
        self.style.stroke_color = color;

        self
    }

    fn with_stroke_width(mut self, width: u8) -> Self {
        self.style.stroke_width = width;

        self
    }

    fn with_fill(mut self, color: Option<C>) -> Self {
        self.style.fill_color = color;

        self
    }
}

impl<'a, C: PixelColor> IntoIterator for &'a Line<C> {
    type Item = Pixel<C>;
    type IntoIter = LineIterator<'a, C>;

    fn into_iter(self) -> Self::IntoIter {
        let x0 = self.start[0].max(0);
        let y0 = self.start[1].max(0);
        let x1 = self.end[0].max(0);
        let y1 = self.end[1].max(0);

        // Find out if our line is steep or shallow
        let is_steep = (y1 - y0).abs() > (x1 - x0).abs();

        // Determine if endpoints should be switched
        // based on the "quick" direction
        let (x0, y0, x1, y1) = if is_steep {
            if y0 > y1 {
                (x1, y1, x0, y0)
            } else {
                (x0, y0, x1, y1)
            }
        } else {
            if x0 > x1 {
                (x1, y1, x0, y0)
            } else {
                (x0, y0, x1, y1)
            }
        };

        // Setup our pre-calculated values
        let (dquick, mut dslow) = if is_steep {
            (y1 - y0, x1 - x0)
        } else {
            (x1 - x0, y1 - y0)
        };

        // Determine how we should increment the slow direction
        let increment = if dslow < 0 {
            dslow = -dslow;
            -1
        } else {
            1
        };

        // Compute the default error
        let error = 2 * dslow - dquick;

        // Set our inital quick & slow
        let (quick, slow, end) = if is_steep { (y0, x0, y1) } else { (x0, y0, x1) };

        LineIterator {
            line: self,

            is_steep,
            dquick,
            dslow,
            increment,
            error,

            quick,
            slow,
            end,
        }
    }
}

/// Pixel iterator for each pixel in the line
#[derive(Debug)]
pub struct LineIterator<'a, C: 'a>
where
    C: PixelColor,
{
    line: &'a Line<C>,

    dquick: i32,
    dslow: i32,
    increment: i32,
    error: i32,
    is_steep: bool,

    quick: i32,
    slow: i32,
    end: i32,
}

// [Bresenham's line algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm)
impl<'a, C: PixelColor> Iterator for LineIterator<'a, C> {
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.quick > self.end {
            return None;
        }

        // Get the next point
        // let &Line { ref color, .. } = self.line;
        let coord = if self.is_steep {
            Coord::new(self.slow, self.quick)
        } else {
            Coord::new(self.quick, self.slow)
        };

        // Update error and increment slow direction
        if self.error > 0 {
            self.slow = self.slow + self.increment;
            self.error -= 2 * self.dquick;
        }
        self.error += 2 * self.dslow;

        // Increment fast direction
        self.quick += 1;

        // Return if there is a stroke on the line
        self.line
            .style
            .stroke_color
            .map(|color| Pixel(coord.to_unsigned(), color))
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
    /// # use embedded_graphics::dev::TestPixelColor;
    /// # use embedded_graphics::prelude::*;
    /// #
    /// # let style: Style<TestPixelColor> = Style::with_stroke(TestPixelColor(1));
    /// #
    /// let line = Line::new(Coord::new(5, 10), Coord::new(15, 20))
    /// #    .with_style(style);
    /// let moved = line.translate(Coord::new(10, 10));
    ///
    /// assert_eq!(moved.start, Coord::new(15, 20));
    /// assert_eq!(moved.end, Coord::new(25, 30));
    /// ```
    fn translate(&self, by: Coord) -> Self {
        Self {
            start: self.start + by,
            end: self.end + by,
            ..self.clone()
        }
    }

    /// Translate the line from its current position to a new position by (x, y) pixels.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Line;
    /// # use embedded_graphics::dev::TestPixelColor;
    /// # use embedded_graphics::prelude::*;
    /// #
    /// # let style: Style<TestPixelColor> = Style::with_stroke(TestPixelColor(1));
    /// #
    /// let mut line = Line::new(Coord::new(5, 10), Coord::new(15, 20))
    /// #    .with_style(style);
    /// line.translate_mut(Coord::new(10, 10));
    ///
    /// assert_eq!(line.start, Coord::new(15, 20));
    /// assert_eq!(line.end, Coord::new(25, 30));
    /// ```
    fn translate_mut(&mut self, by: Coord) -> &mut Self {
        self.start += by;
        self.end += by;

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dev::TestPixelColor;
    use crate::drawable::Pixel;
    use crate::pixelcolor::PixelColorU8;
    use crate::style::Style;
    use crate::unsignedcoord::UnsignedCoord;

    fn test_expected_line(start: Coord, end: Coord, expected: &[(u32, u32)]) {
        let line = Line::new(start, end).with_style(Style::with_stroke(PixelColorU8(1)));
        for (idx, Pixel(coord, _)) in line.into_iter().enumerate() {
            assert_eq!(coord, UnsignedCoord::new(expected[idx].0, expected[idx].1));
        }
    }

    #[test]
    fn bounding_box() {
        let start = Coord::new(10, 10);
        let end = Coord::new(20, 20);

        let line: Line<TestPixelColor> = Line::new(start, end);
        let backwards_line: Line<TestPixelColor> = Line::new(end, start);

        assert_eq!(line.top_left(), start);
        assert_eq!(line.bottom_right(), end);
        assert_eq!(line.size(), UnsignedCoord::new(10, 10));

        assert_eq!(backwards_line.top_left(), start);
        assert_eq!(backwards_line.bottom_right(), end);
        assert_eq!(backwards_line.size(), UnsignedCoord::new(10, 10));
    }

    #[test]
    fn draws_octant_1_correctly() {
        let start = Coord::new(10, 10);
        let end = Coord::new(15, 13);
        let expected = [(10, 10), (11, 11), (12, 11), (13, 12), (14, 12), (15, 13)];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_2_correctly() {
        let start = Coord::new(10, 10);
        let end = Coord::new(13, 15);
        let expected = [(10, 10), (11, 11), (11, 12), (12, 13), (12, 14), (13, 15)];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_3_correctly() {
        let start = Coord::new(10, 10);
        let end = Coord::new(7, 15);
        let expected = [(10, 10), (9, 11), (9, 12), (8, 13), (8, 14), (7, 15)];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_4_correctly() {
        let start = Coord::new(10, 10);
        let end = Coord::new(5, 13);
        let expected = [(5, 13), (6, 12), (7, 12), (8, 11), (9, 11), (10, 10)];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_5_correctly() {
        let start = Coord::new(10, 10);
        let end = Coord::new(5, 7);
        let expected = [(5, 7), (6, 8), (7, 8), (8, 9), (9, 9), (10, 10)];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_6_correctly() {
        let start = Coord::new(10, 10);
        let end = Coord::new(7, 5);
        let expected = [(7, 5), (8, 6), (8, 7), (9, 8), (9, 9), (10, 10)];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_7_correctly() {
        let start = Coord::new(10, 10);
        let end = Coord::new(13, 5);
        let expected = [(13, 5), (12, 6), (12, 7), (11, 8), (11, 9), (10, 10)];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_8_correctly() {
        let start = Coord::new(10, 10);
        let end = Coord::new(15, 7);
        let expected = [
            (10, 10).into(),
            (11, 9).into(),
            (12, 9).into(),
            (13, 8).into(),
            (14, 8).into(),
            (15, 7).into(),
        ];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn it_truncates_lines_out_of_bounds() {
        let start = Coord::new(-2, -2);
        let end = Coord::new(2, 2);
        let expected = [(0, 0).into(), (1, 1).into(), (2, 2).into()];
        test_expected_line(start, end, &expected);
    }
}
