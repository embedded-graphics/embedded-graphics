//! The line primitive

use super::super::drawable::*;
use super::super::transform::*;

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
        let &Line {
            start: (x0, y0),
            end: (x1, y1),
            ..
        } = self;

        // Find out if our line is steep or shallow
        let is_steep = (y1 as i32 - y0 as i32).abs() > (x1 as i32 - x0 as i32).abs();

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
            (y1 as i32 - y0 as i32, x1 as i32 - x0 as i32)
        } else {
            (x1 as i32 - x0 as i32, y1 as i32 - y0 as i32)
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

    quick: u32,
    slow: u32,
    end: u32,
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
            (self.slow, self.quick)
        } else {
            (self.quick, self.slow)
        };

        // Update error and increment slow direction
        if self.error > 0 {
            self.slow = (self.slow as i32 + self.increment) as u32;
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
    ///
    /// let line = Line::new((5, 10), (15, 20), 1);
    /// let moved = line.translate((10, 10));
    ///
    /// assert_eq!(moved.start, (15, 20));
    /// assert_eq!(moved.end, (25, 30));
    /// ```
    fn translate(&self, by: Coord) -> Self {
        Self {
            start: (self.start.0 + by.0, self.start.1 + by.1),
            end: (self.end.0 + by.0, self.end.1 + by.1),
            ..*self
        }
    }

    /// Translate the line from its current position to a new position by (x, y) pixels.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Line;
    /// # use embedded_graphics::transform::Transform;
    ///
    /// let mut line = Line::new((5, 10), (15, 20), 1);
    /// line.translate_mut((10, 10));
    ///
    /// assert_eq!(line.start, (15, 20));
    /// assert_eq!(line.end, (25, 30));
    /// ```
    fn translate_mut(&mut self, by: Coord) -> &mut Self {
        self.start = (self.start.0 + by.0, self.start.1 + by.1);
        self.end = (self.end.0 + by.0, self.end.1 + by.1);

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_expected_line(
        start: Coord,
        end: Coord,
        expected: &[Coord],
        error_message: &'static str,
    ) {
        let line = Line::new(start, end, 1);
        for (idx, (coord, _)) in line.into_iter().enumerate() {
            assert!(coord == expected[idx], error_message);
        }
    }

    #[test]
    fn draws_octant_1_correctly() {
        let start = (10, 10);
        let end = (15, 13);
        let expected = [(10, 10), (11, 11), (12, 11), (13, 12), (14, 12), (15, 13)];
        test_expected_line(start, end, &expected, "Octant 1 failed to draw correctly");
    }

    #[test]
    fn draws_octant_2_correctly() {
        let start = (10, 10);
        let end = (13, 15);
        let expected = [(10, 10), (11, 11), (11, 12), (12, 13), (12, 14), (13, 15)];
        test_expected_line(start, end, &expected, "Octant 2 failed to draw correctly");
    }

    #[test]
    fn draws_octant_3_correctly() {
        let start = (10, 10);
        let end = (7, 15);
        let expected = [(10, 10), (9, 11), (9, 12), (8, 13), (8, 14), (7, 15)];
        test_expected_line(start, end, &expected, "Octant 3 failed to draw correctly");
    }

    #[test]
    fn draws_octant_4_correctly() {
        let start = (10, 10);
        let end = (5, 13);
        let expected = [(5, 13), (6, 12), (7, 12), (8, 11), (9, 11), (10, 10)];
        test_expected_line(start, end, &expected, "Octant 4 failed to draw correctly");
    }

    #[test]
    fn draws_octant_5_correctly() {
        let start = (10, 10);
        let end = (5, 7);
        let expected = [(5, 7), (6, 8), (7, 8), (8, 9), (9, 9), (10, 10)];
        test_expected_line(start, end, &expected, "Octant 5 failed to draw correctly");
    }

    #[test]
    fn draws_octant_6_correctly() {
        let start = (10, 10);
        let end = (7, 5);
        let expected = [(7, 5), (8, 6), (8, 7), (9, 8), (9, 9), (10, 10)];
        test_expected_line(start, end, &expected, "Octant 6 failed to draw correctly");
    }

    #[test]
    fn draws_octant_7_correctly() {
        let start = (10, 10);
        let end = (13, 5);
        let expected = [(13, 5), (12, 6), (12, 7), (11, 8), (11, 9), (10, 10)];
        test_expected_line(start, end, &expected, "Octant 7 failed to draw correctly");
    }

    #[test]
    fn draws_octant_8_correctly() {
        let start = (10, 10);
        let end = (15, 7);
        let expected = [(10, 10), (11, 9), (12, 9), (13, 8), (14, 8), (15, 7)];
        test_expected_line(start, end, &expected, "Octant 8 failed to draw correctly");
    }
}
