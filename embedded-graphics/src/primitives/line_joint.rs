//! Thick line joint.

use crate::{
    geometry::Point,
    primitives::{line::Intersection, Line, Triangle},
    style::StrokeAlignment,
};

/// Joint kind
#[derive(Copy, Clone, Debug)]
pub enum JointKind {
    /// Mitered (sharp point)
    Miter,

    /// Bevelled (flattened point)
    Bevel {
        /// The triangle used to draw the bevel itself.
        filler_triangle: Triangle,
    },

    /// Degenerate (angle between lines is too small to properly render stroke).
    ///
    /// Degenerate corners are rendered with a bevel.
    Degenerate {
        /// The triangle used to fill in the corner.
        filler_triangle: Triangle,
    },

    /// Essentially no joint (both lines are colinear)
    Colinear,

    /// The starting "joint" of a line
    Start,

    /// The ending "joint" of a line
    End,
}

// DELETEME: Remove when debugging in demo is no longer required.
use std::fmt;
impl fmt::Display for JointKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Miter => f.write_str("Miter"),
            Self::Bevel { .. } => f.write_str("Bevel"),
            Self::Degenerate { .. } => f.write_str("Degenerate"),
            Self::Colinear => f.write_str("Colinear"),
            Self::Start => f.write_str("Start"),
            Self::End => f.write_str("End"),
        }
    }
}

/// The left/right corners that make up the start or end edge of a thick line.
#[derive(Copy, Clone, Debug)]
pub struct EdgeCorners {
    /// Left side point.
    pub left: Point,

    /// Right side point.
    pub right: Point,
}

/// A joint between two lines.
#[derive(Copy, Clone, Debug)]
pub struct LineJoint {
    /// Joint kind.
    pub kind: JointKind,

    /// Corners comprising the ending edge of the line that ends at this joint.
    pub first_edge_end: EdgeCorners,

    /// Corners comprising the start edge of the line that begins at this joint.
    pub second_edge_start: EdgeCorners,
}

impl LineJoint {
    /// Create a starting joint
    pub fn start(start: Point, mid: Point, width: u32, alignment: StrokeAlignment) -> Self {
        let line = Line::new(start, mid);

        let (l, r) = line.extents(width as i32, alignment);

        let points = EdgeCorners {
            left: l.start,
            right: r.start,
        };

        Self {
            kind: JointKind::Start,
            first_edge_end: points,
            second_edge_start: points,
        }
    }

    /// Compute a joint.
    pub fn from_points(
        start: Point,
        mid: Point,
        end: Point,
        width: u32,
        alignment: StrokeAlignment,
    ) -> Self {
        let first_line = Line::new(start, mid);
        let second_line = Line::new(mid, end);

        // Miter length limit is dobule the line width (but squared to avoid sqrt() costs)
        let miter_limit = (width * 2).pow(2);

        // Left and right edges of thick first segment
        let (first_edge_left, first_edge_right) = first_line.extents(width as i32, alignment);
        // Left and right edges of thick second segment
        let (second_edge_left, second_edge_right) = second_line.extents(width as i32, alignment);

        if let (
            Intersection::Point {
                point: l_intersection,
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

            // Distance from midpoint to miter end point
            let miter_length_squared = Line::new(mid, l_intersection).length_squared();

            // Normal line: non-overlapping line end caps
            if !self_intersection_r && !self_intersection_l {
                // Intersection is within limit at which it will be chopped off into a bevel, so return
                // a miter.
                if miter_length_squared <= miter_limit {
                    let corners = EdgeCorners {
                        left: l_intersection,
                        right: r_intersection,
                    };

                    Self {
                        kind: JointKind::Miter,
                        first_edge_end: corners,
                        second_edge_start: corners,
                    }
                }
                // Miter is too long, chop it into bevel-style corner
                else {
                    let kind = JointKind::Bevel {
                        filler_triangle: Triangle::new(
                            first_edge_left.end,
                            second_edge_left.start,
                            r_intersection,
                        ),
                    };

                    Self {
                        kind,
                        first_edge_end: EdgeCorners {
                            left: first_edge_left.end,
                            right: r_intersection,
                        },
                        second_edge_start: EdgeCorners {
                            left: second_edge_left.start,
                            right: r_intersection,
                        },
                    }
                }
            }
            // Line segments overlap (degenerate)
            else {
                Self {
                    kind: JointKind::Degenerate {
                        filler_triangle: Triangle::new(
                            first_edge_left.end,
                            second_edge_left.start,
                            mid,
                        ),
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
                kind: JointKind::Colinear,
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

    /// Check whether the joint is degenerate (where the end edge of one line intersects the edge
    /// of the other).
    pub fn is_degenerate(&self) -> bool {
        match self.kind {
            JointKind::Degenerate { .. } => true,
            _ => false,
        }
    }

    /// Check whether the two base lines are colinear.
    ///
    /// This also checks for parallelism, but since both lines share a vertex, they can only ever
    /// be colinear.
    pub fn is_colinear(&self) -> bool {
        match self.kind {
            JointKind::Colinear => true,
            _ => false,
        }
    }
}
