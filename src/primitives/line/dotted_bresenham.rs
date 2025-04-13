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
