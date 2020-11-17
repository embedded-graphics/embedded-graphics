//! Thick line join.

use crate::{
    geometry::Point,
    primitives::{
        common::StrokeOffset,
        line::{Intersection, Side},
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

/// Line coefficients.
struct Coefficients {
    a1: i32,
    b1: i32,
    c1: i32,
    a2: i32,
    b2: i32,
    c2: i32,
    denom: i32,
}

/// A join between two lines.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
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

    const fn coefficients(l1: &Line, l2: &Line) -> Coefficients {
        let Point { x: x1, y: y1 } = l1.start;
        let Point { x: x2, y: y2 } = l1.end;
        let Point { x: x3, y: y3 } = l2.start;
        let Point { x: x4, y: y4 } = l2.end;

        // First line coefficients where "a1 x  +  b1 y  +  c1  =  0"
        let a1 = y2 - y1;
        let b1 = x1 - x2;
        let c1 = x2 * y1 - x1 * y2;

        // Second line coefficients
        let a2 = y4 - y3;
        let b2 = x3 - x4;
        let c2 = x4 * y3 - x3 * y4;

        let denom = a1 * b2 - a2 * b1;

        Coefficients {
            a1,
            b1,
            c1,
            a2,
            b2,
            c2,
            denom,
        }
    }

    /// Integer-only line intersection
    ///
    /// Inspired from https://stackoverflow.com/a/61485959/383609, which links to
    /// https://webdocs.cs.ualberta.ca/~graphics/books/GraphicsGems/gemsii/xlines.c
    pub(in crate::primitives) fn line_intersection(l1: &Line, l2: &Line) -> Intersection {
        let Coefficients {
            a1,
            b1,
            c1,
            a2,
            b2,
            c2,
            denom,
        } = Self::coefficients(l1, l2);

        // Lines are colinear or parallel
        if denom == 0 {
            return Intersection::Colinear;
        }

        // If we got here, line segments intersect. Compute intersection point using method similar
        // to that described here: http://paulbourke.net/geometry/pointlineplane/#i2l

        // The denom/2 is to get rounding instead of truncating.
        let offset = denom.abs() / 2;

        let num = b1 * c2 - b2 * c1;
        let x = if num < 0 { num - offset } else { num + offset } / denom;

        let num = a2 * c1 - a1 * c2;
        let y = if num < 0 { num - offset } else { num + offset } / denom;

        Intersection::Point {
            point: Point::new(x, y),
            outer_side: if denom > 0 { Side::Right } else { Side::Left },
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
            Self::line_intersection(&second_edge_left, &first_edge_left),
            Self::line_intersection(&second_edge_right, &first_edge_right),
        ) {
            // Check if the inside end point of the second line lies inside the first segment.
            let self_intersection = match outer_side {
                Side::Right => first_edge_left.side(second_edge_left.end) <= 0,
                Side::Left => first_edge_right.side(second_edge_right.end) >= 0,
            };

            // Normal line: non-overlapping line end caps
            if !self_intersection {
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
