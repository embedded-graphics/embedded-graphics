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
    start_idx: usize,
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
            let start_idx = 0;

            let start_joint =
                LineJoint::start(points[start_idx], points[start_idx + 1], width, alignment);

            // If there are enough points to compute first joint, do so. Otherwise the line is two
            // points long and should just be a straight segment.
            let end_joint = if points.len() >= 3 {
                LineJoint::from_points(
                    points[start_idx],
                    points[start_idx + 1],
                    points[start_idx + 2],
                    width,
                    alignment,
                )
            } else {
                LineJoint::end(points[start_idx], points[start_idx + 1], width, alignment)
            };

            Self {
                state: TriangleIteratorState::First,
                points,
                start_idx,
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
            start_idx: 0,
            width: 0,
            alignment: StrokeAlignment::Center,
            start_joint: EdgeCorners {
                left: Point::zero(),
                right: Point::zero(),
            },
            end_joint: LineJoint::empty(),
        }
    }

    fn edge_triangle1(start_joint: EdgeCorners, end_joint: LineJoint) -> Triangle {
        let EdgeCorners {
            left: left_start,
            right: right_start,
        } = start_joint;
        let LineJoint {
            first_edge_end: EdgeCorners { left: left_end, .. },
            ..
        } = end_joint;

        // NOTE: Winding order is important here to prevent overdraw of the shared edge from
        // right_start to left_end.
        Triangle::new(left_start, left_end, right_start) // CW winding order
    }

    fn edge_triangle2(start_joint: EdgeCorners, end_joint: LineJoint) -> Triangle {
        let EdgeCorners {
            right: right_start, ..
        } = start_joint;
        let LineJoint {
            first_edge_end:
                EdgeCorners {
                    left: left_end,
                    right: right_end,
                },
            ..
        } = end_joint;

        // NOTE: Winding order is important here to prevent overdraw of the shared edge from
        // right_start to left_end.
        Triangle::new(left_end, right_end, right_start) // CCW winding order
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

                    self.start_idx += 1;
                    self.start_joint = self.end_joint.second_edge_start;

                    // Compute next end joint. The iterator will stop if the `points.get()` calls below
                    // return `None`, denoting that we've gone past the end of the points array.
                    let first_point = *self.points.get(self.start_idx)?;
                    let secound_point = *self.points.get(self.start_idx + 1)?;

                    self.end_joint = if let Some(third_point) = self.points.get(self.start_idx + 2)
                    {
                        LineJoint::from_points(
                            first_point,
                            secound_point,
                            *third_point,
                            self.width,
                            self.alignment,
                        )
                    } else {
                        LineJoint::end(first_point, secound_point, self.width, self.alignment)
                    };
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
