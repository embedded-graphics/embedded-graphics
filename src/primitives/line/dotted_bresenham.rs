//! A variant of the bresenham algorithm that is used to select
//! points from a bresenham line in order to draw a dotted line.

use super::{bresenham::MajorMinor, Line, Points};
use crate::geometry::{Point, PointExt};
use integer_sqrt::IntegerSquareRoot;

/// Bresenham algorithm for dotted lines.
///
/// [`super::bresenham::BresenhamParameters`] describes a major and
/// a minor step, classically vectors along opposing axes.
///
/// [`DottedBresenham`] describes a scalar major step and a
/// scalar minor step. It is used to draw a dotted line with the
/// desired number of dots by selecting the appropriate indices in a
/// bresenham line.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
struct DottedBresenham {
    /// Error threshold.
    ///
    /// If the accumulated error exceeds the threshold a minor move is made.
    error_threshold: i32,

    /// Change in error for major and minor steps.
    error_step: MajorMinor<i32>,

    /// Change in index for major and minor steps.
    index_step: MajorMinor<usize>,

    /// Current index increment.
    ///
    /// It is used to retrieve the next dot from a [`Points`] iterator.
    index_nth: Option<usize>,

    /// Error accumulator.
    error: i32,
}

impl DottedBresenham {
    /// Create a new bresenham object.
    fn new(line: &Line, nb_dots_desired: i32) -> Self {
        let delta = line.delta().abs();
        let line_max_index = delta.x.max(delta.y);
        let nb_pixels_in_line = line_max_index + 1;

        // Enforce the use of at least 2 dots to prevent division by 0 when the line is reduced to a point.
        // `clamp` can't be used here (`nb_pixels_in_line` is less than 2 when the line is reduced to a point).
        let nb_dots = nb_dots_desired.min(nb_pixels_in_line).max(2);

        let integer_quotient = line_max_index / (nb_dots - 1);
        let remainder = line_max_index - integer_quotient * (nb_dots - 1);

        let index_step = MajorMinor::new(integer_quotient as usize, 1);
        let error_threshold = nb_dots - 1;

        Self {
            error_threshold,
            error_step: MajorMinor::new(remainder * 2, error_threshold * 2),
            index_step,
            index_nth: Some(0),
            error: 0,
        }
    }

    /// Increases the error by a major step.
    ///
    /// If the error threshold is reached the error is reduced by a minor step and
    /// `true` is returned.
    fn increase_error(&mut self) -> bool {
        self.error += self.error_step.major;
        if self.error >= self.error_threshold {
            self.error -= self.error_step.minor;

            true
        } else {
            false
        }
    }
}

impl Iterator for DottedBresenham {
    type Item = usize;

    /// Return the increment to the next point on the line.
    /// This iterator is infinite, except if `parameters.index_step.major = 0`.
    fn next(&mut self) -> Option<usize> {
        let ret = self.index_nth;

        if self.index_nth.is_some() {
            let mut increment = self.index_step.major;

            if self.increase_error() {
                increment += self.index_step.minor;
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
pub(super) struct DottedLinePoints {
    points: Points,
    index_bresenham: DottedBresenham,
}

impl DottedLinePoints {
    /// Creates an iterator over all points on the given line
    /// taking into account the size of the dots.
    pub(super) fn with_dot_size(line: &Line, dot_size: i32) -> Self {
        let mut length = line.delta().length_squared().integer_sqrt();
        // The gaps between dots ideally have the same size as the dots
        // If `dot_size <= 3`, only positive error is allowed,
        // otherwise both positive and negative error are allowed.
        if dot_size > 3 {
            length += dot_size;
        }
        // The 2 endpoint dots take half the space of a regular dot.
        let nb_dots_desired = length / (2 * dot_size) + 1;

        Self::new(line, nb_dots_desired)
    }

    /// Creates an iterator over all points on the given line
    /// taking into account the desired number of dots.
    fn new(line: &Line, nb_dots_desired: i32) -> Self {
        let points = Points::new(line);
        let index_bresenham = DottedBresenham::new(line, nb_dots_desired);

        Self {
            points,
            index_bresenham,
        }
    }
}

impl Iterator for DottedLinePoints {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        self.points.nth(self.index_bresenham.next()?)
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
