//! The line primitive

use super::super::drawable::*;
use super::super::transform::*;
use coord::Coord;

// TODO: Impl Default so people can leave the color bit out
/// Line primitive
#[derive(Debug, Copy, Clone)]
pub struct Line {
    /// Start point
    pub start: Coord,

    /// End point
    pub end: Coord,

    /// Line color
    pub color: Color,
}

impl Line {
    /// Create a new line
    pub fn new(start: Coord, end: Coord, color: u8) -> Self {
        Line { start, end, color }
    }
}

impl<'a> IntoIterator for &'a Line {
    type Item = Pixel;
    type IntoIter = LineIterator<'a>;

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
pub struct LineIterator<'a> {
    line: &'a Line,

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
impl<'a> Iterator for LineIterator<'a> {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        if self.quick > self.end {
            return None;
        }

        // Get the next point
        let &Line { color, .. } = self.line;
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

        // Return
        Some((coord, color))
    }
}

impl Drawable for Line {}

impl Transform for Line {
    /// Translate the line from its current position to a new position by (x, y) pixels, returning
    /// a new `Line`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Line;
    /// # use embedded_graphics::transform::Transform;
    /// # use embedded_graphics::coord::Coord;
    ///
    /// let line = Line::new(Coord::new(5, 10), Coord::new(15, 20), 1);
    /// let moved = line.translate(Coord::new(10, 10));
    ///
    /// assert_eq!(moved.start, Coord::new(15, 20));
    /// assert_eq!(moved.end, Coord::new(25, 30));
    /// ```
    fn translate(&self, by: Coord) -> Self {
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
    /// # use embedded_graphics::transform::Transform;
    /// # use embedded_graphics::coord::Coord;
    ///
    /// let mut line = Line::new(Coord::new(5, 10), Coord::new(15, 20), 1);
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

    fn test_expected_line(start: Coord, end: Coord, expected: &[Coord]) {
        let line = Line::new(start, end, 1);
        for (idx, (coord, _)) in line.into_iter().enumerate() {
            assert_eq!(coord, expected[idx]);
        }
    }

    #[test]
    fn draws_octant_1_correctly() {
        let start = Coord::new(10, 10);
        let end = Coord::new(15, 13);
        let expected = [
            Coord::new(10, 10),
            Coord::new(11, 11),
            Coord::new(12, 11),
            Coord::new(13, 12),
            Coord::new(14, 12),
            Coord::new(15, 13),
        ];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_2_correctly() {
        let start = Coord::new(10, 10);
        let end = Coord::new(13, 15);
        let expected = [
            Coord::new(10, 10),
            Coord::new(11, 11),
            Coord::new(11, 12),
            Coord::new(12, 13),
            Coord::new(12, 14),
            Coord::new(13, 15),
        ];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_3_correctly() {
        let start = Coord::new(10, 10);
        let end = Coord::new(7, 15);
        let expected = [
            Coord::new(10, 10),
            Coord::new(9, 11),
            Coord::new(9, 12),
            Coord::new(8, 13),
            Coord::new(8, 14),
            Coord::new(7, 15),
        ];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_4_correctly() {
        let start = Coord::new(10, 10);
        let end = Coord::new(5, 13);
        let expected = [
            Coord::new(5, 13),
            Coord::new(6, 12),
            Coord::new(7, 12),
            Coord::new(8, 11),
            Coord::new(9, 11),
            Coord::new(10, 10),
        ];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_5_correctly() {
        let start = Coord::new(10, 10);
        let end = Coord::new(5, 7);
        let expected = [
            Coord::new(5, 7),
            Coord::new(6, 8),
            Coord::new(7, 8),
            Coord::new(8, 9),
            Coord::new(9, 9),
            Coord::new(10, 10),
        ];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_6_correctly() {
        let start = Coord::new(10, 10);
        let end = Coord::new(7, 5);
        let expected = [
            Coord::new(7, 5),
            Coord::new(8, 6),
            Coord::new(8, 7),
            Coord::new(9, 8),
            Coord::new(9, 9),
            Coord::new(10, 10),
        ];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_7_correctly() {
        let start = Coord::new(10, 10);
        let end = Coord::new(13, 5);
        let expected = [
            Coord::new(13, 5),
            Coord::new(12, 6),
            Coord::new(12, 7),
            Coord::new(11, 8),
            Coord::new(11, 9),
            Coord::new(10, 10),
        ];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_8_correctly() {
        let start = Coord::new(10, 10);
        let end = Coord::new(15, 7);
        let expected = [
            Coord::new(10, 10),
            Coord::new(11, 9),
            Coord::new(12, 9),
            Coord::new(13, 8),
            Coord::new(14, 8),
            Coord::new(15, 7),
        ];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn it_truncates_lines_out_of_bounds() {
        let start = Coord::new(-2, -2);
        let end = Coord::new(2, 2);
        let expected = [Coord::new(0, 0), Coord::new(1, 1), Coord::new(2, 2)];
        test_expected_line(start, end, &expected);
    }
}
