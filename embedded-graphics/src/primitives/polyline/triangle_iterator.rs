use crate::{
    prelude::Point,
    primitives::{
        line_joint::{EdgeCorners, LineJoint},
        Triangle,
    },
    style::StrokeAlignment,
};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(in crate::primitives) enum TriangleIteratorState {
    NextJoint,
    First,
    Secound,
    Filler,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(in crate::primitives) struct TriangleIterator<'a> {
    points: &'a [Point],
    state: TriangleIteratorState,
    width: u32,
    alignment: StrokeAlignment,
    start_joint: EdgeCorners, // it's not necessary to store the whole joint so save some memory
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
            end_joint: LineJoint::empty(),
        }
    }

    fn edge_triangle1(start_joint_corners: EdgeCorners, end_joint: LineJoint) -> Triangle {
        let LineJoint {
            first_edge_end: EdgeCorners { left: left_end, .. },
            ..
        } = end_joint;

        Triangle::new(start_joint_corners.left, left_end, start_joint_corners.right)
    }

    fn edge_triangle2(start_joint_corners: EdgeCorners, end_joint: LineJoint) -> Triangle {
        let LineJoint {
            first_edge_end:
                EdgeCorners {
                    left: left_end,
                    right: right_end,
                },
            ..
        } = end_joint;

        Triangle::new(left_end, right_end, start_joint_corners.right)
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

                    self.state = TriangleIteratorState::First;
                    self.start_joint = self.end_joint.second_edge_start;

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

                    // Shift the points.
                    self.points = &self.points[1..];
                }

                TriangleIteratorState::First => {
                    self.state = TriangleIteratorState::Secound;
                    return Some(Self::edge_triangle1(self.start_joint, self.end_joint));
                }

                TriangleIteratorState::Secound => {
                    self.state = TriangleIteratorState::Filler;
                    return Some(Self::edge_triangle2(self.start_joint, self.end_joint));
                }

                TriangleIteratorState::Filler => {
                    let t = self.end_joint.filler();
                    self.state = TriangleIteratorState::NextJoint;

                    if t.is_some() {
                        return t;
                    }
                }
            }
        }
    }
}
