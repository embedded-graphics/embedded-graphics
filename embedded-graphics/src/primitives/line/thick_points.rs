use crate::{
    geometry::Point,
    primitives::line::{
        bresenham::{self, Bresenham, BresenhamParameters, BresenhamPoint},
        Line,
    },
};

const HORIZONTAL_LINE: Line = Line::new(Point::zero(), Point::new(1, 0));

/// Which side of the center line to draw on
///
/// Imagine standing on `start`, looking ahead to where `end` is. `Left` is to your left, `Right` to
/// your right.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Side {
    /// Left side of the line
    Left,

    /// Right side of the line
    Right,
}

impl Side {
    fn swap(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

/// Iterator over the parallel lines used to draw a thick line.
///
/// Thick lines are drawn using multiple 1px wide lines, which are parallel to
/// the original primitive line. The lines returned by the iterator are alternating
/// between the left and right side of original line to keep the resulting thick
/// line symmetric.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(in crate::primitives) struct ParallelsIterator {
    /// Parameters used for moves along the parallel lines.
    pub parallel_parameters: BresenhamParameters,

    /// Parameters used for moves perpendicular to the parallel lines.
    perpendicular_parameters: BresenhamParameters,

    /// Accumulated thickness.
    ///
    /// The thickness accumulator is increased each time a parallel line is returned.
    thickness_accumulator: i32,

    /// Thickness threshold.
    ///
    /// The thickness threshold is compared with the thickness accumulator to stop the iterator once
    /// the desired line thickness is reached.
    thickness_threshold: i32,

    /// Changes the sign of initial error variables.
    ///
    /// To keep the parallel lines in phase the sign of the error variables needs to be flipped in
    /// some quadrants.
    flip: bool,

    /// Starting point for parallels on the left side.
    left: Bresenham,

    /// Initial error for parallels on the left side.
    ///
    /// The initial error for the parallels is used to keep adjacent parallels in phase and prevent
    /// overlapping pixels.
    left_error: i32,

    /// Starting point for parallels on the right side.
    right: Bresenham,

    /// Initial error for parallels on the right side.
    ///
    /// The initial error for the parallels is used to keep adjacent parallels in phase and prevent
    /// overlapping pixels.
    right_error: i32,

    /// The next side which will be drawn.
    next_side: Side,
}

impl ParallelsIterator {
    pub fn new(mut line: &Line, thickness: i32) -> Self {
        let start_point = line.start;

        // The lines orientation is undefined if start and end point are equal.
        // To provide valid parameters a horizontal line is used to determine the
        // parameters instead of the original line.
        if line.start == line.end {
            line = &HORIZONTAL_LINE;
        }

        let parallel_parameters = BresenhamParameters::new(line);
        let perpendicular_parameters = BresenhamParameters::new(&line.perpendicular());

        // Thickness threshold, taking into account that fewer pixels are required to draw a
        // diagonal line of the same perceived width.
        let delta = (line.end - line.start).abs();
        let thickness_threshold = 4 * thickness.pow(2) * delta.length_squared();
        let thickness_accumulator =
            (parallel_parameters.error_step.minor + parallel_parameters.error_step.major) / 2;

        // Determine if the signs in the error calculation should be flipped.
        let flip = perpendicular_parameters.position_step.minor
            == -parallel_parameters.position_step.major;

        let mut self_ = Self {
            parallel_parameters,
            perpendicular_parameters,
            thickness_accumulator,
            thickness_threshold,
            flip,
            left: Bresenham::new(start_point),
            left_error: 0,
            right: Bresenham::new(start_point),
            right_error: 0,
            next_side: Side::Right,
        };

        // Skip center line on left side iterator
        self_.next_parallel(Side::Left);

        self_
    }

    /// Returns the next parallel on the given side.
    pub fn next_parallel(&mut self, side: Side) -> (BresenhamPoint, i32) {
        let (error, decrease_error) = match side {
            Side::Left => (&mut self.left_error, self.flip),
            Side::Right => (&mut self.right_error, !self.flip),
        };

        loop {
            let point = match side {
                Side::Left => self.left.next_all(&self.perpendicular_parameters),
                Side::Right => self.right.previous_all(&self.perpendicular_parameters),
            };

            match point {
                BresenhamPoint::Normal(_) => {
                    return (point, *error);
                }
                BresenhamPoint::Extra(_) => {
                    if decrease_error {
                        let error_before_decrease = *error;

                        if self.parallel_parameters.decrease_error(error) {
                            return (point, error_before_decrease);
                        }
                    } else if self.parallel_parameters.increase_error(error) {
                        return (point, *error);
                    };
                }
            }
        }
    }
}

impl Iterator for ParallelsIterator {
    /// The bresenham state (`Bresenham`) and the reduction in line length (`u32`).
    type Item = (Bresenham, u32, Side);

    fn next(&mut self) -> Option<Self::Item> {
        if self.thickness_accumulator.pow(2) > self.thickness_threshold {
            return None;
        }

        let (point, error) = self.next_parallel(self.next_side);

        let ret = match point {
            BresenhamPoint::Normal(point) => {
                self.thickness_accumulator += self.perpendicular_parameters.error_step.minor;

                // Normal lines are the same length as the original primitive line.
                (
                    Bresenham::with_initial_error(point, error),
                    0,
                    self.next_side,
                )
            }
            BresenhamPoint::Extra(point) => {
                self.thickness_accumulator += self.perpendicular_parameters.error_step.major;

                // Extra lines are 1 pixel shorter than normal lines.
                (
                    Bresenham::with_initial_error(point, error),
                    1,
                    self.next_side,
                )
            }
        };

        self.next_side = self.next_side.swap();

        Some(ret)
    }
}

/// Iterator over all pixels in the stroke of a thick line.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ThickPoints {
    parallel: Bresenham,
    parallel_length: u32,
    parallel_points_remaining: u32,

    iter: ParallelsIterator,
}

impl ThickPoints {
    /// Creates a new iterator over the points in the stroke of a thick line.
    pub(in crate::primitives) fn new(line: &Line, thickness: i32) -> Self {
        Self {
            parallel: Bresenham::new(line.start),
            parallel_length: bresenham::major_length(line),
            parallel_points_remaining: 0,
            iter: ParallelsIterator::new(line, thickness),
        }
    }
}

impl Iterator for ThickPoints {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.parallel_points_remaining > 0 {
                self.parallel_points_remaining -= 1;

                return Some(self.parallel.next(&self.iter.parallel_parameters));
            } else {
                let (parallel, length_reduction, _) = self.iter.next()?;

                self.parallel = parallel;
                self.parallel_points_remaining = self.parallel_length - length_reduction;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{mock_display::MockDisplay, pixelcolor::Gray8};

    /// Draws the output of `ParallelsIterator` to a `MockDisplay`.
    ///
    /// Each parallel line is drawn using a different `Gray8` color, to allow testing
    /// of the drawing order. Points that are drawn multiple times are marked using
    /// `Gray8::new(0xFF)`.
    fn draw_parallels(line: Line, count: u8) -> MockDisplay<Gray8> {
        // The maximum number of lines is 0xE, because 0xF is used to mark overdraw
        assert!(count < 0xF);

        let mut parallels = ParallelsIterator::new(&line, 100);

        let mut display = MockDisplay::new();

        for line_number in 0..count {
            let (mut parallel, length_reduction, _) = parallels.next().unwrap();
            let length = bresenham::major_length(&line) - length_reduction;

            for _ in 0..length {
                let point = parallel.next(&parallels.parallel_parameters);

                let color = if display.get_pixel(point).is_some() {
                    // mark overdraw with `F`
                    Gray8::new(0xFF)
                } else {
                    Gray8::new(line_number * 0x11)
                };

                display.draw_pixel(point, color);
            }
        }

        display
    }

    #[test]
    fn equal_start_and_end() {
        let line = Line::new(Point::new(3, 3), Point::new(3, 3));
        let display = draw_parallels(line, 3);

        let expected = MockDisplay::from_pattern(&[
            "        ", //
            "        ", //
            "   1    ", //
            "   0    ", //
            "   2    ", //
        ]);
        assert_eq!(display, expected);
    }

    #[test]
    fn horizontal_1() {
        let line = Line::new(Point::new(1, 3), Point::new(4, 3));
        let display = draw_parallels(line, 3);

        let expected = MockDisplay::from_pattern(&[
            "        ", //
            "        ", //
            " 1111   ", //
            " 0000   ", //
            " 2222   ", //
        ]);
        assert_eq!(display, expected);
    }

    #[test]
    fn horizontal_2() {
        let line = Line::new(Point::new(4, 3), Point::new(1, 3));
        let display = draw_parallels(line, 3);

        let expected = MockDisplay::from_pattern(&[
            "        ", //
            "        ", //
            " 2222   ", //
            " 0000   ", //
            " 1111   ", //
        ]);
        assert_eq!(display, expected);
    }

    #[test]
    fn vertical_1() {
        let line = Line::new(Point::new(3, 3), Point::new(3, 0));
        let display = draw_parallels(line, 3);

        let expected = MockDisplay::from_pattern(&[
            "  102   ", //
            "  102   ", //
            "  102   ", //
            "  102   ", //
        ]);
        assert_eq!(display, expected);
    }

    #[test]
    fn vertical_2() {
        let line = Line::new(Point::new(3, 0), Point::new(3, 3));
        let display = draw_parallels(line, 3);

        let expected = MockDisplay::from_pattern(&[
            "  201   ", //
            "  201   ", //
            "  201   ", //
            "  201   ", //
        ]);
        assert_eq!(display, expected);
    }

    #[test]
    fn line_45_1() {
        let line = Line::new(Point::new(2, 4), Point::new(5, 1));
        let display = draw_parallels(line, 5);

        let expected = MockDisplay::from_pattern(&[
            "    3   ", //
            "   310  ", //
            "  31024 ", //
            " 31024  ", //
            "  024   ", //
            "   4    ", //
            "        ",
        ]);
        assert_eq!(display, expected);
    }

    #[test]
    fn line_45_2() {
        let line = Line::new(Point::new(5, 1), Point::new(2, 4));
        let display = draw_parallels(line, 5);

        let expected = MockDisplay::from_pattern(&[
            "    4   ", //
            "   420  ", //
            "  42013 ", //
            " 42013  ", //
            "  013   ", //
            "   3    ", //
            "        ",
        ]);
        assert_eq!(display, expected);
    }

    #[test]
    fn line_45_3() {
        let line = Line::new(Point::new(2, 2), Point::new(5, 5));
        let display = draw_parallels(line, 5);

        let expected = MockDisplay::from_pattern(&[
            "        ", //
            "   3    ", //
            "  013   ", //
            " 42013  ", //
            "  42013 ", //
            "   420  ", //
            "    4   ",
        ]);
        assert_eq!(display, expected);
    }

    #[test]
    fn line_45_4() {
        let line = Line::new(Point::new(5, 5), Point::new(2, 2));
        let display = draw_parallels(line, 5);

        let expected = MockDisplay::from_pattern(&[
            "        ", //
            "   4    ", //
            "  024   ", //
            " 31024  ", //
            "  31024 ", //
            "   310  ", //
            "    3   ",
        ]);
        assert_eq!(display, expected);
    }

    #[test]
    fn line_1() {
        let line = Line::new(Point::new(2, 2), Point::new(5, 4));
        let display = draw_parallels(line, 5);

        let expected = MockDisplay::from_pattern(&[
            "        ", //
            "   33   ", //
            "  0113  ", //
            " 420013 ", //
            "  4220  ", //
            "   44   ", //
            "        ",
        ]);
        assert_eq!(display, expected);
    }

    #[test]
    fn line_2() {
        let line = Line::new(Point::new(5, 4), Point::new(2, 2));
        let display = draw_parallels(line, 5);

        let expected = MockDisplay::from_pattern(&[
            "        ", //
            "   44   ", //
            "  0224  ", //
            " 310024 ", //
            "  3110  ", //
            "   33   ", //
            "        ",
        ]);
        assert_eq!(display, expected);
    }

    #[test]
    fn line_3() {
        let line = Line::new(Point::new(2, 4), Point::new(5, 2));
        let display = draw_parallels(line, 5);

        let expected = MockDisplay::from_pattern(&[
            "        ", //
            "   33   ", //
            "  3110  ", //
            " 310024 ", //
            "  0224  ", //
            "   44   ", //
            "        ",
        ]);
        assert_eq!(display, expected);
    }

    #[test]
    fn line_4() {
        let line = Line::new(Point::new(5, 2), Point::new(2, 4));
        let display = draw_parallels(line, 5);

        let expected = MockDisplay::from_pattern(&[
            "        ", //
            "   44   ", //
            "  4220  ", //
            " 420013 ", //
            "  0113  ", //
            "   33   ", //
            "        ",
        ]);
        assert_eq!(display, expected);
    }

    #[test]
    fn line_5() {
        let line = Line::new(Point::new(3, 3), Point::new(5, 2));
        let display = draw_parallels(line, 3);

        let expected = MockDisplay::from_pattern(&[
            "        ", //
            "     1  ", //
            "   110  ", //
            "   0022 ", //
            "    2   ", //
            "        ", //
            "        ",
        ]);
        assert_eq!(display, expected);
    }
}
