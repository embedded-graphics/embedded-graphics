//! Thick line join.

use crate::{
    geometry::{Point, PointExt},
    primitives::{
        common::{LineSide, LinearEquation, StrokeOffset},
        line::intersection_params::{Intersection, IntersectionParams},
        Line,
    },
};

/// Join kind
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub enum JoinKind {
    /// Mitered (sharp point)
    Miter,

    /// Bevelled (flattened point)
    Bevel {
        /// Left side or right side?
        outer_side: LineSide,
    },

    /// Degenerate (angle between lines is too small to properly render stroke).
    ///
    /// Degenerate corners are rendered with a bevel.
    Degenerate {
        /// Left side or right side?
        outer_side: LineSide,
    },

    /// Lines are colinear.
    ///
    /// Start and end points for this join will be equal.
    Colinear,

    /// The starting cap of a line.
    Start,

    /// The ending cap of a line.
    End,
}

/// The left/right corners that make up the start or end edge of a thick line.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct EdgeCorners {
    /// Left side point.
    pub left: Point,

    /// Right side point.
    pub right: Point,
}

/// A join between two lines.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct LineJoin {
    /// Join kind.
    pub kind: JoinKind,

    /// Corners comprising the ending edge of the line that ends at this join.
    pub first_edge_end: EdgeCorners,

    /// Corners comprising the start edge of the line that begins at this join.
    pub second_edge_start: EdgeCorners,
}

impl LineJoin {
    /// Create a starting join.
    ///
    /// `first_edge_end` and `second_edge_start` are set to the same points.
    pub fn start(start: Point, mid: Point, width: u32, stroke_offset: StrokeOffset) -> Self {
        let line = Line::new(start, mid);

        let (l, r) = line.extents(width, stroke_offset);

        let points = EdgeCorners {
            left: l.start,
            right: r.start,
        };

        Self {
            kind: JoinKind::Start,
            first_edge_end: points,
            second_edge_start: points,
        }
    }

    /// Create an ending join.
    ///
    /// `first_edge_end` and `second_edge_start` are set to the same points.
    pub fn end(mid: Point, end: Point, width: u32, stroke_offset: StrokeOffset) -> Self {
        let line = Line::new(mid, end);

        let (l, r) = line.extents(width, stroke_offset);

        let points = EdgeCorners {
            left: l.end,
            right: r.end,
        };

        Self {
            kind: JoinKind::End,
            first_edge_end: points,
            second_edge_start: points,
        }
    }

    /// Empty join
    pub const fn empty() -> Self {
        Self {
            kind: JoinKind::End,
            first_edge_end: EdgeCorners {
                left: Point::zero(),
                right: Point::zero(),
            },
            second_edge_start: EdgeCorners {
                left: Point::zero(),
                right: Point::zero(),
            },
        }
    }

    /// Compute a join.
    pub fn from_points(
        start: Point,
        mid: Point,
        end: Point,
        width: u32,
        stroke_offset: StrokeOffset,
    ) -> Self {
        let first_line = Line::new(start, mid);
        let second_line = Line::new(mid, end);

        // Left and right edges of thick first segment
        let (first_edge_left, first_edge_right) = first_line.extents(width, stroke_offset);
        // Left and right edges of thick second segment
        let (second_edge_left, second_edge_right) = second_line.extents(width, stroke_offset);

        if let Some((l_intersection, outer_side, r_intersection)) = intersections(
            &first_edge_left,
            &first_edge_right,
            &second_edge_left,
            &second_edge_right,
        ) {
            // Check if the inside end point of the second line lies inside the first segment.
            let self_intersection = match outer_side {
                LineSide::Right => LinearEquation::from_line(&first_edge_left)
                    .check_side(second_edge_left.end, LineSide::Right),
                LineSide::Left => LinearEquation::from_line(&first_edge_right)
                    .check_side(second_edge_right.end, LineSide::Left),
            };

            // Normal line: non-overlapping line end caps
            if !self_intersection {
                // Distance from midpoint to miter outside end point.
                let miter_length_squared = Line::new(
                    mid,
                    match outer_side {
                        LineSide::Left => l_intersection,
                        LineSide::Right => r_intersection,
                    },
                )
                .delta()
                .length_squared() as u32;

                // Miter length limit is double the line width (but squared to avoid sqrt() costs)
                let miter_limit = (width * 2).pow(2);

                // Intersection is within limit at which it will be chopped off into a bevel, so
                // return a miter.
                if miter_length_squared <= miter_limit {
                    let corners = EdgeCorners {
                        left: l_intersection,
                        right: r_intersection,
                    };

                    Self {
                        kind: JoinKind::Miter,
                        first_edge_end: corners,
                        second_edge_start: corners,
                    }
                }
                // Miter is too long, chop it into bevel-style corner
                else {
                    match outer_side {
                        LineSide::Right => Self {
                            kind: JoinKind::Bevel { outer_side },
                            first_edge_end: EdgeCorners {
                                left: l_intersection,
                                right: first_edge_right.end,
                            },
                            second_edge_start: EdgeCorners {
                                left: l_intersection,
                                right: second_edge_right.start,
                            },
                        },
                        LineSide::Left => Self {
                            kind: JoinKind::Bevel { outer_side },
                            first_edge_end: EdgeCorners {
                                left: first_edge_left.end,
                                right: r_intersection,
                            },
                            second_edge_start: EdgeCorners {
                                left: second_edge_left.start,
                                right: r_intersection,
                            },
                        },
                    }
                }
            }
            // Line segments overlap (degenerate)
            else {
                Self {
                    kind: JoinKind::Degenerate { outer_side },
                    first_edge_end: EdgeCorners {
                        left: first_edge_left.end,
                        right: first_edge_right.end,
                    },
                    second_edge_start: EdgeCorners {
                        left: second_edge_left.start,
                        right: second_edge_right.start,
                    },
                }
            }
        }
        // Lines are colinear
        else {
            Self {
                kind: JoinKind::Colinear,
                first_edge_end: EdgeCorners {
                    left: first_edge_left.end,
                    right: first_edge_right.end,
                },
                second_edge_start: EdgeCorners {
                    left: second_edge_left.start,
                    right: second_edge_right.start,
                },
            }
        }
    }

    /// The filler line (if any) for bevel and degenerate joints.
    const fn filler_line(&self) -> Option<Line> {
        match self.kind {
            JoinKind::Bevel { outer_side, .. } | JoinKind::Degenerate { outer_side, .. } => {
                let line = match outer_side {
                    LineSide::Left => {
                        Line::new(self.first_edge_end.left, self.second_edge_start.left)
                    }
                    LineSide::Right => {
                        Line::new(self.first_edge_end.right, self.second_edge_start.right)
                    }
                };

                Some(line)
            }
            _ => None,
        }
    }

    fn cap(&self, cap: &EdgeCorners) -> (Line, Option<Line>) {
        if let Some(filler) = self.filler_line() {
            let midpoint = filler.midpoint();

            let l1 = Line::new(cap.left, midpoint);
            let l2 = Line::new(midpoint, cap.right);

            (l1, l2.into())
        } else {
            (Line::new(cap.left, cap.right), None)
        }
    }

    /// Start node bevel line(s).
    ///
    /// If the join is a bevel join, this will return two lines, otherwise one.
    pub fn start_cap_lines(&self) -> (Line, Option<Line>) {
        self.cap(&self.second_edge_start)
    }

    /// End node bevel line(s).
    ///
    /// If the join is a bevel join, this will return two lines, otherwise one.
    pub fn end_cap_lines(&self) -> (Line, Option<Line>) {
        self.cap(&self.first_edge_end)
    }

    /// Whether the join is degenerate (segments self-intersect) or not.
    pub const fn is_degenerate(&self) -> bool {
        matches!(self.kind, JoinKind::Degenerate { .. })
    }
}

fn intersections(
    first_edge_left: &Line,
    first_edge_right: &Line,
    second_edge_left: &Line,
    second_edge_right: &Line,
) -> Option<(Point, LineSide, Point)> {
    let params = IntersectionParams::from_lines(second_edge_left, first_edge_left);

    let (l_intersection, outer_side) = if let Intersection::Point {
        point, outer_side, ..
    } = params.intersection()
    {
        if !params.nearly_colinear_has_error() {
            (point, outer_side)
        } else {
            (first_edge_left.end, outer_side)
        }
    } else {
        return None;
    };

    let params = IntersectionParams::from_lines(second_edge_right, first_edge_right);

    let r_intersection = if let Intersection::Point { point, .. } = params.intersection() {
        if !params.nearly_colinear_has_error() {
            point
        } else {
            first_edge_right.end
        }
    } else {
        return None;
    };

    Some((l_intersection, outer_side, r_intersection))
}
