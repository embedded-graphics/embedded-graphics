use crate::geometry::Point;
use crate::primitives::Line;

/// Which side of the center line to draw on
///
/// Imagine standing on `start`, looking ahead to where `end` is. `Left` is to your left, `Right` to
/// your right.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum Side {
    Left,
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

/// Current state of each parallel line drawn
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct ParallelLineState {
    /// Current point along the line
    current_point: Point,

    /// Length accumulator
    ///
    /// Checked against `parallel_length` of the line to know when to stop iterating
    current_length: u32,

    /// Error accumulator
    error: i32,
}

impl ParallelLineState {
    fn new(start_point: Point, initial_length: u32, initial_error: i32) -> Self {
        Self {
            current_point: start_point,
            current_length: initial_length,
            error: initial_error,
        }
    }

    fn next(&mut self, parameters: &ThickLineParameters) -> Option<Point> {
        if self.current_length > parameters.parallel_length {
            return None;
        }

        self.current_length += 1;

        let p = self.current_point;

        if self.error > parameters.threshold {
            self.current_point += parameters.step_minor;
            self.error += parameters.e_diag;
        }

        self.current_point += parameters.step_major;
        self.error += parameters.e_square;
        Some(p)
    }
}

/// Current side state
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct SideState {
    /// Parallel line start point
    parallel_start: Point,

    /// Error accumulator
    error: i32,

    /// Perpendicular error accumulator
    p_error: i32,
}

impl SideState {
    fn new(parallel_start: Point) -> Self {
        Self {
            parallel_start,
            error: 0,
            p_error: 0,
        }
    }

    fn next(
        &mut self,
        parameters: &ThickLineParameters,
        side: Side,
        swapped_side: Side,
    ) -> (ParallelLineState, i32) {
        let parallel_start = self.parallel_start;

        let mut thickness_change = 0;

        if self.error > parameters.threshold {
            match side {
                Side::Left => self.parallel_start += parameters.perp_step_major,
                Side::Right => self.parallel_start -= parameters.perp_step_major,
            }

            self.error += parameters.e_diag;
            thickness_change += parameters.e_square;

            if self.p_error > parameters.threshold {
                let parallel = match swapped_side {
                    Side::Right => ParallelLineState::new(
                        parallel_start + parameters.step_minor,
                        1,
                        -self.p_error,
                    ),
                    Side::Left => {
                        ParallelLineState::new(parallel_start, 0, self.p_error + parameters.e_diag)
                    }
                };

                self.p_error += parameters.e_diag + parameters.e_square;

                return (parallel, thickness_change);
            } else {
                self.p_error += parameters.e_square;
            }
        }

        match side {
            Side::Left => self.parallel_start += parameters.perp_step_minor,
            Side::Right => self.parallel_start -= parameters.perp_step_minor,
        }

        self.error += parameters.e_square;
        thickness_change -= parameters.e_diag;

        let p_error = match swapped_side {
            Side::Left => self.p_error,
            Side::Right => -self.p_error,
        };

        let parallel = ParallelLineState::new(self.parallel_start, 0, p_error);

        (parallel, thickness_change)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct ThickLineParameters {
    /// Bresenham error threshold
    ///
    /// If this is exceeded, a "minor" move is made
    threshold: i32,

    /// "Major" error component
    e_diag: i32,

    /// "Minor" error component
    e_square: i32,

    /// Line thickness in arbitrary units
    ///
    /// Thickness is calculated according to the section titled "Fixing the Thickness" in [this
    /// article](http://kt8216.unixcab.org/murphy/index.html). The difference in this implementation
    /// is that both sides of the comparison are squared, removing the need for an expensive
    /// `sqrt()` call.
    thickness: i32,

    /// The "major" step
    ///
    /// The X or Y component with the larger delta is considered "major". This is the most common
    /// direction to move in.
    step_major: Point,

    /// The "minor" step
    ///
    /// The X or Y component with the smaller delta is considered "minor". This is the less common
    /// direction to move in.
    step_minor: Point,

    perp_step_major: Point,
    perp_step_minor: Point,
    swap_sides: bool,

    /// Length of parallel lines
    parallel_length: u32,
}

impl ThickLineParameters {
    fn new(line: &Line, stroke_width: i32) -> Self {
        let dx: i32 = line.end.x - line.start.x;
        let dy: i32 = line.end.y - line.start.y;

        let direction = match (dx >= 0, dy >= 0) {
            (true, true) => Point::new(1, 1),
            (true, false) => Point::new(1, -1),
            (false, true) => Point::new(-1, 1),
            (false, false) => Point::new(-1, -1),
        };

        // Left-hand perpendicular to the line between start and end
        let perp_direction = match (dx >= 0, dy >= 0) {
            (true, true) => Point::new(1, -1),
            (true, false) => Point::new(-1, -1),
            (false, true) => Point::new(1, 1),
            (false, false) => Point::new(-1, 1),
        };

        // Thickness threshold, taking into account that fewer pixels are required to draw a
        // diagonal line of the same perceived width.
        let thickness = 4 * stroke_width.pow(2) * (dx.pow(2) + dy.pow(2));

        let mut dx = dx.abs();
        let mut dy = dy.abs();

        // Force LHS to stay on left by swapping sides on some octants
        let swap_sides = match (dy > dx, direction.x, direction.y) {
            (false, 1, -1) | (true, 1, 1) | (false, -1, 1) | (true, -1, -1) => true,
            _ => false,
        };

        // Swap components if line is Y-major. dx is always the "major" direction delta.
        let (step_major, step_minor, perp_step_major, perp_step_minor) = if dy > dx {
            core::mem::swap(&mut dx, &mut dy);

            (
                Point::new(0, direction.y),
                Point::new(direction.x, 0),
                Point::new(0, perp_direction.y),
                Point::new(perp_direction.x, 0),
            )
        } else {
            (
                Point::new(direction.x, 0),
                Point::new(0, direction.y),
                Point::new(perp_direction.x, 0),
                Point::new(0, perp_direction.y),
            )
        };

        let threshold = dx - 2 * dy;
        let e_diag = -2 * dx;
        let e_square = 2 * dy;

        Self {
            step_major,
            step_minor,
            perp_step_major,
            perp_step_minor,
            threshold,
            e_diag,
            e_square,
            thickness,
            swap_sides,
            parallel_length: dx as u32,
        }
    }
}

/// Pixel iterator for each pixel in the line
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct ThickLineIterator {
    /// Thick line parameters.
    parameters: ThickLineParameters,

    /// Thickness of pixels drawn so far
    ///
    /// Compared against `thickness` for width limit
    thickness_accum: i32,

    /// Which side the _next_ parallel line will be on
    ///
    /// Lines start down the center, then alternate between left, then right. For lines with an even
    /// width, the line is unbalanced by 1px to the left.
    next_side: Side,

    /// State of the parallel line currently being iterated over
    parallel: ParallelLineState,

    /// Left side state
    left: SideState,

    /// Right side state
    right: SideState,
}

impl ThickLineIterator {
    /// Create a new line iterator from a `Line` and a stroke width
    ///
    /// Lines with a thickness greater than 1px are filled using multiple parallel lines to the
    /// left/right of the central original line.
    pub(crate) fn new(line: &Line, stroke_width: i32) -> Self {
        let parameters = ThickLineParameters::new(line, stroke_width);

        let thickness_accum = (parameters.e_square - parameters.e_diag) / 2;

        Self {
            parameters,
            thickness_accum,
            // Next side to draw after center line
            next_side: Side::Left,
            parallel: ParallelLineState::new(line.start, 0, 0),
            left: SideState::new(line.start),
            right: SideState::new(line.start),
        }
    }
}

impl Iterator for ThickLineIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        // Quit iterator if width threshold is reached or the line has no length
        if self.thickness_accum.pow(2) > self.parameters.thickness
            || self.parameters.parallel_length == 0
        {
            return None;
        }

        if let Some(p) = self.parallel.next(&self.parameters) {
            Some(p)
        } else {
            let swapped_side = if self.parameters.swap_sides {
                self.next_side.swap()
            } else {
                self.next_side
            };

            let (parallel, thickness_change) = match self.next_side {
                Side::Left => self.left.next(&self.parameters, Side::Left, swapped_side),
                Side::Right => self.right.next(&self.parameters, Side::Right, swapped_side),
            };

            self.thickness_accum += thickness_change;
            self.parallel = parallel;

            // Switch to opposite side of line to keep it balanced
            self.next_side = self.next_side.swap();

            Self::next(self)
        }
    }
}
