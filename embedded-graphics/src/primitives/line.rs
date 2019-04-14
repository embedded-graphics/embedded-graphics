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
    type IntoIter = LineIterator<C>;

    fn into_iter(self) -> Self::IntoIter {
        let mut d = self.end - self.start;
        if d[0] < 0 {
            d = Coord::new(-d[0], d[1]);
        }
        if d[1] > 0 {
            d = Coord::new(d[0], -d[1]);
        }

        let s = match (self.start[0] >= self.end[0], 
                       self.start[1] >= self.end[1]) {
            (false, false) => Coord::new(1, 1),
            (false, true) => Coord::new(1, -1),
            (true, false) => Coord::new(-1, 1),
            (true, true) => Coord::new(-1, -1),
        };

        LineIterator {
            style: self.style,
            
            start: self.start,
            end: self.end,
            d,
            s,
            err: d[0] + d[1],
            e2: 0,
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

    start: Coord,
    end: Coord,
    d: Coord,
    s: Coord,
    err: i32,
    e2: i32,
}

// [Bresenham's line algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm)
impl<C: PixelColor> Iterator for LineIterator<C> {
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        self.style.stroke_color?;

        loop {
            let p_coord = self.start;

            if self.start == self.end {
                return None;
            }
            self.e2 = 2 * self.err;
            if self.e2 > self.d[1] {
                self.err += self.d[1];
                self.start += Coord::new(self.s[0], 0);
            }
            if self.e2 < self.d[0] {
                self.err += self.d[0];
                self.start += Coord::new(0, self.s[1]);
            }
            if p_coord[0] >= 0 && p_coord[1] >= 0 {
                return Some(Pixel(p_coord.to_unsigned(), 
                                  self.style.stroke_color.unwrap()));
            }
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
        let expected = [(10, 10), (9, 11), (8, 11), (7, 12), (6, 12), (5, 13)];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_5_correctly() {
        let start = Coord::new(10, 10);
        let end = Coord::new(5, 7);
        let expected = [(10, 10), (9, 9), (8, 9), (7, 8), (6, 8), (5, 7)];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_6_correctly() {
        let start = Coord::new(10, 10);
        let end = Coord::new(7, 5);
        let expected = [(10, 10), (9, 9), (9, 8), (8, 7), (8, 6), (7, 5)];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_7_correctly() {
        let start = Coord::new(10, 10);
        let end = Coord::new(13, 5);
        let expected = [(10, 10), (11, 9), (11, 8), (12, 7), (12, 6), (13, 5)];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_8_correctly() {
        let start = Coord::new(10, 10);
        let end = Coord::new(15, 7);
        let expected = [(10, 10), (11, 9), (12, 9), (13, 8), (14, 8), (15, 7),];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn it_truncates_lines_out_of_bounds() {
        let start = Coord::new(-2, -2);
        let end = Coord::new(2, 2);
        let expected = [(0, 0), (1, 1), (2, 2)];
        test_expected_line(start, end, &expected);
    }
}
