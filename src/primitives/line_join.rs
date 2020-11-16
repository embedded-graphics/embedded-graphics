//! Thick line join.

use crate::{
    geometry::Point,
    primitives::{
        line::{Intersection, Side, StrokeOffset},
        Line,
    },
};

/// Join kind
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum JoinKind {
    /// Mitered (sharp point)
    Miter,

    /// Bevelled (flattened point)
    Bevel {
        /// Left side or right side?
        outer_side: Side,
    },

    /// Degenerate (angle between lines is too small to properly render stroke).
    ///
    /// Degenerate corners are rendered with a bevel.
    Degenerate {
        /// Left side or right side?
        outer_side: Side,
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
pub struct EdgeCorners {
    /// Left side point.
    pub left: Point,

    /// Right side point.
    pub right: Point,
}

/// A join between two lines.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(in crate::primitives) struct LineJoin {
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
    pub fn empty() -> Self {
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

        // Miter length limit is double the line width (but squared to avoid sqrt() costs)
        let miter_limit = (width * 2).pow(2);

        // Left and right edges of thick first segment
        let (first_edge_left, first_edge_right) = first_line.extents(width, stroke_offset);
        // Left and right edges of thick second segment
        let (second_edge_left, second_edge_right) = second_line.extents(width, stroke_offset);

        if let (
            Intersection::Point {
                point: l_intersection,
                outer_side,
                ..
            },
            Intersection::Point {
                point: r_intersection,
                ..
            },
        ) = (
            second_edge_left.line_intersection(&first_edge_left),
            second_edge_right.line_intersection(&first_edge_right),
        ) {
            let first_segment_start_edge = Line::new(first_edge_left.start, first_edge_right.start);
            let second_segment_end_edge = Line::new(second_edge_left.end, second_edge_right.end);

            let self_intersection_l = first_segment_start_edge
                .segment_intersection(&second_edge_left)
                || second_segment_end_edge.segment_intersection(&first_edge_left);

            let self_intersection_r = first_segment_start_edge
                .segment_intersection(&second_edge_right)
                || second_segment_end_edge.segment_intersection(&first_edge_right);

            // Normal line: non-overlapping line end caps
            if !self_intersection_r && !self_intersection_l {
                // Distance from midpoint to miter outside end point.
                let miter_length_squared = Line::new(
                    mid,
                    match outer_side {
                        Side::Left => l_intersection,
                        Side::Right => r_intersection,
                    },
                )
                .delta()
                .length_squared() as u32;

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
                        Side::Right => Self {
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
                        Side::Left => Self {
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
                    kind: match outer_side {
                        Side::Left => JoinKind::Degenerate { outer_side },
                        Side::Right => JoinKind::Degenerate { outer_side },
                    },
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
    fn filler_line(&self) -> Option<Line> {
        match self.kind {
            JoinKind::Bevel { outer_side, .. } | JoinKind::Degenerate { outer_side, .. } => {
                let line = match outer_side {
                    Side::Left => Line::new(self.first_edge_end.left, self.second_edge_start.left),
                    Side::Right => {
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
    pub fn is_degenerate(&self) -> bool {
        // MSRV: Use matches!() macro when we're at 1.42.0 or greater.
        if let JoinKind::Degenerate { .. } = self.kind {
            true
        } else {
            false
        }
    }
}
