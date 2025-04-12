//! A variant of the bresenham algorithm that is used to select
//! points from a bresenham line in order to draw a dotted line.

use super::{bresenham::MajorMinor, Line, Points};
use crate::geometry::Point;
use core::cmp::{max, min};

/// Dotted bresenham parameters.
///
/// [`super::bresenham::BresenhamParameters`] describes a major and
/// a minor step, classically vectors along opposing axes.
///
/// [`DottedBresenhamParameters`] describes a scalar major step and a
/// scalar minor step. It is used to draw a dotted line with the
/// desired number of dots by selecting the appropriate points in a
/// bresenham line.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct DottedBresenhamParameters {
    /// Error threshold.
    ///
    /// If the accumulated error exceeds the threshold a minor move is made.
    pub error_threshold: i32,

    /// Change in error for major and minor steps.
    pub error_step: MajorMinor<i32>,

    /// Change in index for major and minor steps.
    pub index_step: MajorMinor<usize>,
}

impl DottedBresenhamParameters {
    /// Creates a new bresenham parameters object.
    pub fn new(line: &Line, nb_dots_desired: u32) -> Self {
        let delta = line.delta().abs();
        let line_max_index = delta.x.max(delta.y);

        // Enforce the use of at most `line_max_index + 1` dots.
        let mut nb_dots = min(nb_dots_desired as i32, line_max_index + 1);
        // Then enforce the use of at least 2 dots to prevent division by 0 when the line is reduced to a point.
        nb_dots = max(2, nb_dots);

        let integer_quotient = line_max_index / (nb_dots - 1);
        let remainder = line_max_index - integer_quotient * (nb_dots - 1);

        let index_step = MajorMinor::new(integer_quotient as usize, 1);
        let error_threshold = nb_dots - 1;

        Self {
            error_threshold,
            error_step: MajorMinor::new(remainder * 2, error_threshold * 2),
            index_step,
        }
    }

    /// Increases the error by a major step.
    ///
    /// If the error threshold is reached the error is reduced by a minor step and
    /// `true` is returned.
    pub fn increase_error(&self, error: &mut i32) -> bool {
        *error += self.error_step.major;
        if *error >= self.error_threshold {
            *error -= self.error_step.minor;

            true
        } else {
            false
        }
    }
}

/// Bresenham algorithm for dotted lines.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct DottedBresenham {
    /// Current index increment.
    ///
    /// It is used to retrieve the next dot from a [`Points`] iterator.
    pub index_nth: Option<usize>,

    /// Error accumulator.
    error: i32,
}

impl DottedBresenham {
    /// Creates a new bresenham object.
    pub const fn new(initial_index: usize) -> Self {
        Self::with_initial_error(initial_index, 0)
    }

    /// Creates a new extended bresenham object with the initial error.
    pub const fn with_initial_error(initial_index: usize, initial_error: i32) -> Self {
        Self {
            index_nth: Some(initial_index),
            error: initial_error,
        }
    }

    /// Returns the increment to the next point on the line.
    /// This iterator is infinite, except if `parameters.index_step.major = 0`.
    pub fn next(&mut self, parameters: &DottedBresenhamParameters) -> Option<usize> {
        let ret = self.index_nth;

        if self.index_nth.is_some() {
            let mut increment = parameters.index_step.major;

            if parameters.increase_error(&mut self.error) {
                increment += parameters.index_step.minor;
            }

            self.index_nth = if increment > 0 {
                Some(increment - 1)
            } else {
                None
            };
        }

        ret
    }
}

/// Iterator over all points on a dotted line.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct DottedLinePoints {
    points: Points,
    index_parameters: DottedBresenhamParameters,
    index_bresenham: DottedBresenham,
}

impl DottedLinePoints {
    /// Creates an iterator over all points on the given line
    /// taking into account the desired number of dots.
    pub(in crate::primitives) fn new(line: &Line, nb_dots_desired: u32) -> Self {
        let points = Points::new(line);
        let index_parameters = DottedBresenhamParameters::new(line, nb_dots_desired);
        let index_bresenham = DottedBresenham::new(0);

        Self {
            points,
            index_parameters,
            index_bresenham,
        }
    }
}

impl Iterator for DottedLinePoints {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        self.points
            .nth(self.index_bresenham.next(&self.index_parameters)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Real;

    #[test]
    fn dotted_line_has_correct_number_of_dots() {
        // The desired number of dots should be clamped between 2 and the number of dots in the line
        // (except if the line is reduced to a point).
        let line = Line::new(Point::new(-17, 8), Point::new(-38, 25)); // 22 pixels

        // Testing an appropriate number of dots.
        for i in [3, 8, 15, 17, 21] {
            let dotted_line_points = DottedLinePoints::new(&line, i);
            assert_eq!(dotted_line_points.count(), i as usize);
        }

        // Testing too few dots.
        for i in [0, 1, 2] {
            let dotted_line_points = DottedLinePoints::new(&line, i);
            assert_eq!(dotted_line_points.count(), 2);
        }

        // Testing too many dots and checking the resulting dotted line is a regular line.
        for i in [22, 23, 30] {
            let dotted_line_points = DottedLinePoints::new(&line, i);
            assert_eq!(dotted_line_points.count(), 22);
            assert!(dotted_line_points.eq(Points::new(&line)));
        }
    }

    #[test]
    fn one_pixel_dotted_line() {
        // When the line is reduced to a point, the iterator should exceptionally contain only one item.
        let p = Point::new(-35, 15);
        assert!(DottedLinePoints::new(&Line::new(p, p), 5).eq(core::iter::once(p)));
    }

    #[test]
    fn dotted_line_has_correct_start_and_end() {
        // The starting and ending items of the `DottedLinePoints` iterator should be the line endpoints.
        let start_end = [Point::new(5, -6), Point::new(23, 45)];

        let line = Line::new(start_end[0], start_end[1]);
        let opposite_line = Line::new(start_end[1], start_end[0]);

        // Testing with 2 dots.
        let mut dotted_endpoints = DottedLinePoints::new(&line, 2);
        assert!(dotted_endpoints.next().eq(&Some(line.start)));
        assert!(dotted_endpoints.next().eq(&Some(line.end)));

        let mut opposite_dotted_endpoints = DottedLinePoints::new(&opposite_line, 2);
        assert_eq!(opposite_dotted_endpoints.next(), Some(opposite_line.start));
        assert_eq!(opposite_dotted_endpoints.next(), Some(opposite_line.end));

        // Testing with 5 dots.
        let mut dotted_endpoints = DottedLinePoints::new(&line, 5);
        assert_eq!(dotted_endpoints.next(), Some(line.start));
        assert_eq!(dotted_endpoints.last(), Some(line.end));

        let mut opposite_dotted_endpoints = DottedLinePoints::new(&opposite_line, 5);
        assert_eq!(opposite_dotted_endpoints.next(), Some(opposite_line.start));
        assert_eq!(opposite_dotted_endpoints.last(), Some(opposite_line.end));
    }

    #[test]
    fn dotted_line_dots_are_correct() {
        // The dot indices should match those calculated with floats.
        let point = Point::new(37, 50).abs();
        let max = point.x.max(point.y);
        let nb_dots = 21; // should be between 2 and `max`

        let line = Line::new(Point::zero(), point);
        let line_points = Points::new(&line).into_iter();

        let dot_offset = Real::from(max) / Real::from(nb_dots - 1);
        let idx_iter = 0..nb_dots;
        let float_dotted_line_points = idx_iter.map(move |idx| {
            line_points
                .clone()
                .nth(Into::<i32>::into((dot_offset * Real::from(idx)).round()) as usize)
                .unwrap()
        });

        let dotted_line_points = DottedLinePoints::new(&line, nb_dots);

        for (bresenham, float) in core::iter::zip(dotted_line_points, float_dotted_line_points) {
            assert_eq!(bresenham, float);
        }
    }
}
