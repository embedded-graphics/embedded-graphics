//! A variant of the bresenham algorithm that is used to select
//! points from a bresenham line in order to draw a dotted line.

use super::{bresenham::MajorMinor, Line, Points};
use crate::geometry::Point;

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
struct DottedBresenhamParameters {
    /// Error threshold.
    ///
    /// If the accumulated error exceeds the threshold a minor move is made.
    error_threshold: i32,

    /// Change in error for major and minor steps.
    error_step: MajorMinor<i32>,

    /// Change in index for major and minor steps.
    index_step: MajorMinor<usize>,
}

impl DottedBresenhamParameters {
    /// Create a new bresenham parameters object.
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
        }
    }

    /// Increases the error by a major step.
    ///
    /// If the error threshold is reached the error is reduced by a minor step and
    /// `true` is returned.
    fn increase_error(&self, error: &mut i32) -> bool {
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
struct DottedBresenham {
    /// Current index increment.
    ///
    /// It is used to retrieve the next dot from a [`Points`] iterator.
    index_nth: Option<usize>,

    /// Error accumulator.
    error: i32,
}

impl DottedBresenham {
    /// Create a new bresenham object.
    const fn new() -> Self {
        Self {
            index_nth: Some(0),
            error: 0,
        }
    }

    /// Return the increment to the next point on the line.
    /// This iterator is infinite, except if `parameters.index_step.major = 0`.
    fn next(&mut self, parameters: &DottedBresenhamParameters) -> Option<usize> {
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
pub(super) struct DottedLinePoints {
    points: Points,
    index_parameters: DottedBresenhamParameters,
    index_bresenham: DottedBresenham,
}

impl DottedLinePoints {
    /// Creates an iterator over all points on the given line
    /// taking into account the desired number of dots.
    pub(super) fn new(line: &Line, nb_dots_desired: i32) -> Self {
        let points = Points::new(line);
        let index_parameters = DottedBresenhamParameters::new(line, nb_dots_desired);
        let index_bresenham = DottedBresenham::new();

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
