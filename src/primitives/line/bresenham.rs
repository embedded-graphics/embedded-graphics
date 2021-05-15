use crate::{geometry::Point, primitives::Line};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
/// Struct to hold major and minor values.
pub struct MajorMinor<T> {
    /// Major value.
    ///
    /// Used to describe the change of a value when a major step is taken.
    pub major: T,

    /// Minor value.
    ///
    /// Used to describe the change of a value when a minor step is taken.
    pub minor: T,
}

impl<T> MajorMinor<T> {
    /// Creates a new struct holding a major and a minor value.
    pub const fn new(major: T, minor: T) -> Self {
        Self { major, minor }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct BresenhamParameters {
    /// Error threshold.
    ///
    /// If the accumulated error exceeds the threshold a minor move is made.
    pub error_threshold: i32,

    /// Change in error for major and minor steps.
    pub error_step: MajorMinor<i32>,

    /// Change in position for major and minor steps.
    pub position_step: MajorMinor<Point>,
}

impl BresenhamParameters {
    /// Creates a new bresenham parameters object.
    pub fn new(line: &Line) -> Self {
        let delta = line.end - line.start;

        let direction = Point::new(
            if delta.x >= 0 { 1 } else { -1 },
            if delta.y >= 0 { 1 } else { -1 },
        );

        let delta = delta.abs();

        // Determine major and minor directions.
        let (delta, position_step) = if delta.y >= delta.x {
            (
                MajorMinor::new(delta.y, delta.x),
                MajorMinor::new(direction.y_axis(), direction.x_axis()),
            )
        } else {
            (
                MajorMinor::new(delta.x, delta.y),
                MajorMinor::new(direction.x_axis(), direction.y_axis()),
            )
        };

        Self {
            error_threshold: delta.major,
            error_step: MajorMinor::new(2 * delta.minor, 2 * delta.major),
            position_step,
        }
    }

    /// Increases the error by a major step.
    ///
    /// If the error threshold is reached the error is reduced by a minor step and
    /// `true` is returned.
    pub fn increase_error(&self, error: &mut i32) -> bool {
        *error += self.error_step.major;
        if *error > self.error_threshold {
            *error -= self.error_step.minor;

            true
        } else {
            false
        }
    }

    /// Decreases the error by a major step.
    ///
    /// If the error threshold is reached the error is increased by a minor step and
    /// `true` is returned.
    pub fn decrease_error(&self, error: &mut i32) -> bool {
        *error -= self.error_step.major;
        if *error <= -self.error_threshold {
            *error += self.error_step.minor;

            true
        } else {
            false
        }
    }

    /// Returns if extra points need to be mirrored along the line.
    ///
    /// Extra points should always be added to the right side of a line.
    const fn mirror_extra_points(&self) -> bool {
        if self.position_step.major.x != 0 {
            self.position_step.major.x == self.position_step.minor.y
        } else {
            self.position_step.major.y == -self.position_step.minor.x
        }
    }
}

/// Implementation of the bresenham algorithm.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct Bresenham {
    /// Current point.
    pub point: Point,

    /// Error accumulator.
    error: i32,
}

impl Bresenham {
    /// Creates a new bresenham object.
    pub const fn new(start_point: Point) -> Self {
        Self::with_initial_error(start_point, 0)
    }

    /// Creates a new bresenham object with the initial error.
    pub const fn with_initial_error(start_point: Point, initial_error: i32) -> Self {
        Self {
            point: start_point,
            error: initial_error,
        }
    }

    /// Returns the next point on the line.
    pub fn next(&mut self, parameters: &BresenhamParameters) -> Point {
        if self.error > parameters.error_threshold {
            self.point += parameters.position_step.minor;
            self.error -= parameters.error_step.minor;
        }

        let ret = self.point;

        self.point += parameters.position_step.major;
        self.error += parameters.error_step.major;

        ret
    }

    /// Returns the next point on the line, including extra points.
    pub fn next_all(&mut self, parameters: &BresenhamParameters) -> BresenhamPoint {
        let mut point = self.point;

        if self.error > parameters.error_threshold {
            self.point += parameters.position_step.minor;
            self.error -= parameters.error_step.minor;

            if parameters.mirror_extra_points() {
                point += parameters.position_step.minor;
                point -= parameters.position_step.major;
            }

            BresenhamPoint::Extra(point)
        } else {
            self.point += parameters.position_step.major;
            self.error += parameters.error_step.major;

            BresenhamPoint::Normal(point)
        }
    }

    /// Returns the previous point on the line, including extra points.
    pub fn previous_all(&mut self, parameters: &BresenhamParameters) -> BresenhamPoint {
        let mut point = self.point;

        if self.error <= -parameters.error_threshold {
            self.point -= parameters.position_step.minor;
            self.error += parameters.error_step.minor;

            if !parameters.mirror_extra_points() {
                point -= parameters.position_step.minor;
                point += parameters.position_step.major;
            }

            BresenhamPoint::Extra(point)
        } else {
            self.point -= parameters.position_step.major;
            self.error -= parameters.error_step.major;

            BresenhamPoint::Normal(point)
        }
    }
}

/// Point returned by `next_all` and `previous_all` to distinguish the point type.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub enum BresenhamPoint {
    /// Normal point.
    Normal(Point),

    /// Extra point.
    ///
    /// The default `next` routine only outputs a single point if a step in the minor direction
    /// occurred. The `next_all` and `previous_all` methods will output an extra intermediate point
    /// that is always positioned on the right side of the line.
    Extra(Point),
}

/// Returns the length of a line in bresenham major direction steps.
pub fn major_length(line: &Line) -> u32 {
    let delta = (line.end - line.start).abs();

    delta.x.max(delta.y) as u32 + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{mock_display::MockDisplay, pixelcolor::BinaryColor, Drawable, Pixel};

    #[test]
    fn bresenham() {
        let line = Line::new(Point::new(1, 2), Point::new(5, 4));

        let mut bresenham = Bresenham::new(line.start);
        let parameters = BresenhamParameters::new(&line);

        let expected = &[(1, 2), (2, 2), (3, 3), (4, 3), (5, 4)];
        for point in expected.iter().copied().map(Point::from) {
            assert_eq!(bresenham.next(&parameters), point);
        }
    }

    /// Draws all lines in the iterator including extra points.
    ///
    /// Normal points and extra points are distinguished by drawing normal points using
    /// `BinaryColor::On` and extra points using `BinaryColor::Off`.
    fn draw_with_extras<'a>(lines: impl Iterator<Item = &'a Line>) -> MockDisplay<BinaryColor> {
        let mut display_next = MockDisplay::new();
        let mut display_previous = MockDisplay::new();

        for line in lines {
            let mut bresenham = Bresenham::new(line.start);
            let parameters = BresenhamParameters::new(&line);

            for point in core::iter::from_fn(|| Some(bresenham.next_all(&parameters))).take(7) {
                match point {
                    BresenhamPoint::Normal(point) => Pixel(point, BinaryColor::On),
                    BresenhamPoint::Extra(point) => Pixel(point, BinaryColor::Off),
                }
                .draw(&mut display_next)
                .unwrap();
            }

            let mut bresenham = Bresenham::new(line.end);
            for point in core::iter::from_fn(|| Some(bresenham.previous_all(&parameters))).take(7) {
                match point {
                    BresenhamPoint::Normal(point) => Pixel(point, BinaryColor::On),
                    BresenhamPoint::Extra(point) => Pixel(point, BinaryColor::Off),
                }
                .draw(&mut display_previous)
                .unwrap();
            }
        }

        display_next.assert_eq(&display_previous);

        display_next
    }

    #[test]
    fn lines_with_extra_points_1() {
        let lines = &[
            Line::new(Point::new(4, 2), Point::new(0, 0)),
            Line::new(Point::new(6, 2), Point::new(10, 0)),
            Line::new(Point::new(4, 4), Point::new(0, 6)),
            Line::new(Point::new(6, 4), Point::new(10, 6)),
        ];
        let display = draw_with_extras(lines.iter());

        display.assert_pattern(&[
            "#.        #", //
            " ##.    ##.", //
            "   ## ##.  ", //
            "           ", //
            "  .## ##   ", //
            ".##    .## ", //
            "#        .#", //
        ]);
    }

    #[test]
    fn lines_with_extra_points_2() {
        let lines = &[
            Line::new(Point::new(2, 4), Point::new(0, 0)),
            Line::new(Point::new(4, 4), Point::new(6, 0)),
            Line::new(Point::new(2, 6), Point::new(0, 10)),
            Line::new(Point::new(4, 6), Point::new(6, 10)),
        ];
        let display = draw_with_extras(lines.iter());

        display.assert_pattern(&[
            "#.    #", //
            " #   #.", //
            " #.  # ", //
            "  # #. ", //
            "  # #  ", //
            "       ", //
            "  # #  ", //
            " .# #  ", //
            " #  .# ", //
            ".#   # ", //
            "#    .#", //
        ]);
    }

    #[test]
    fn lines_with_extra_points_3() {
        let lines = &[
            Line::new(Point::new(3, 3), Point::new(0, 0)),
            Line::new(Point::new(5, 3), Point::new(8, 0)),
            Line::new(Point::new(3, 5), Point::new(0, 8)),
            Line::new(Point::new(5, 5), Point::new(8, 8)),
        ];
        let display = draw_with_extras(lines.iter());

        display.assert_pattern(&[
            "#.      #", //
            " #.    #.", //
            "  #.  #. ", //
            "   # #.  ", //
            "         ", //
            "  .# #   ", //
            " .#  .#  ", //
            ".#    .# ", //
            "#      .#", //
        ]);
    }
}
