use crate::{
    prelude::Point,
    primitives::{
        line::Side,
        line_joint::{EdgeCorners, JointKind, LineJoint},
        Triangle,
    },
    style::StrokeAlignment,
};

/// The internal state of `TriangleIterator`
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(in crate::primitives) enum TriangleIteratorState {
    /// Calculate the next joint
    NextJoint,

    /// Return the first triangle of a joint
    First,

    /// Return the second triangle of a joint
    Second,

    /// Return the filler triangle for bevelled / degenerate joints
    Filler,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(in crate::primitives) struct TriangleIterator<'a> {
    /// Contains which iterator state will run next.
    pub state: TriangleIteratorState,

    points: &'a [Point],
    width: u32,
    alignment: StrokeAlignment,
    start_joint: EdgeCorners, // it's not necessary to store the whole joint so save some memory
    start_joint_kind: JointKind,
    end_joint: LineJoint,
}

impl<'a> TriangleIterator<'a> {
    pub fn new(points: &'a [Point], width: u32, alignment: StrokeAlignment) -> Self {
        if points.len() < 2 {
            Self::empty()
        } else {
            let start_joint = LineJoint::start(points[0], points[1], width, alignment);

            // If there are enough points to compute first joint, do so. Otherwise the line is two
            // points long and should just be a straight segment.
            let end_joint = if points.len() >= 3 {
                LineJoint::from_points(points[0], points[1], points[2], width, alignment)
            } else {
                LineJoint::end(points[0], points[1], width, alignment)
            };

            Self {
                state: TriangleIteratorState::First,
                points,
                width,
                alignment,
                start_joint: start_joint.second_edge_start,
                start_joint_kind: start_joint.kind,
                end_joint,
            }
        }
    }

    pub fn empty() -> Self {
        Self {
            state: TriangleIteratorState::NextJoint,
            points: &[],
            width: 0,
            alignment: StrokeAlignment::Center,
            start_joint: EdgeCorners {
                left: Point::zero(),
                right: Point::zero(),
            },
            start_joint_kind: JointKind::Start,
            end_joint: LineJoint::empty(),
        }
    }

    fn edge_triangle1(start_joint_corners: EdgeCorners, end_joint: LineJoint) -> Option<Triangle> {
        let LineJoint {
            first_edge_end: EdgeCorners { left: left_end, .. },
            ..
        } = end_joint;

        // Point order matters so that the next triangle's new point is p3
        Some(Triangle::new(
            start_joint_corners.left,
            start_joint_corners.right,
            left_end,
        ))
    }

    fn edge_triangle2(start_joint_corners: EdgeCorners, end_joint: LineJoint) -> Option<Triangle> {
        let LineJoint {
            first_edge_end:
                EdgeCorners {
                    left: left_end,
                    right: right_end,
                },
            ..
        } = end_joint;

        // Point order matters so that the next triangle's new point is p3
        Some(Triangle::new(
            start_joint_corners.right,
            left_end,
            right_end,
        ))
    }

    /// Returns the type (`JointKind`) of the starting joint.
    pub const fn joint_kind(&self) -> JointKind {
        self.start_joint_kind
    }
}

impl<'a> Iterator for TriangleIterator<'a> {
    type Item = Triangle;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.state {
                TriangleIteratorState::NextJoint => {
                    if self.end_joint.is_end_joint() {
                        return None;
                    }

                    self.start_joint = self.end_joint.second_edge_start;
                    self.start_joint_kind = self.end_joint.kind;

                    // Compute next end joint. The iterator will stop if the `points.get()` calls below
                    // return `None`, denoting that we've gone past the end of the points array.
                    self.end_joint = if self.points.len() >= 3 {
                        LineJoint::from_points(
                            self.points[0],
                            self.points[1],
                            self.points[2],
                            self.width,
                            self.alignment,
                        )
                    } else {
                        LineJoint::end(
                            *self.points.get(0)?,
                            *self.points.get(1)?,
                            self.width,
                            self.alignment,
                        )
                    };

                    // Skip redundant data
                    if self.start_joint != self.end_joint.first_edge_end {
                        self.state = TriangleIteratorState::First;
                    } else {
                        self.state = TriangleIteratorState::NextJoint;
                    }

                    // Shift the points.
                    self.points = &self.points[1..];
                }

                TriangleIteratorState::First => {
                    self.state = TriangleIteratorState::Second;
                    return Self::edge_triangle1(self.start_joint, self.end_joint);
                }

                TriangleIteratorState::Second => {
                    self.state = TriangleIteratorState::Filler;
                    return Self::edge_triangle2(self.start_joint, self.end_joint);
                }

                TriangleIteratorState::Filler => {
                    self.state = TriangleIteratorState::NextJoint;
                    if let Some(t) = self.end_joint.filler() {
                        return Some(t);
                    }
                }
            }
        }
    }
}
