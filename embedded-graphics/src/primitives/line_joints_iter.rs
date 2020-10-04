//! Thick line joint.

use crate::{
    geometry::Point,
    primitives::{line_joint::JointKind, line_joint::LineJoint, Line},
    style::StrokeAlignment,
};

#[derive(Debug, Copy, Clone)]
enum State {
    StartEdge,
    FirstEdgeL,
    FirstEdgeR,
    Bevel,
    NextSegment,
    EndEdge,
    Done,
}

/// Line joints iter
#[derive(Debug, Clone)]
pub struct LineJointsIter<'a> {
    windows: core::slice::Windows<'a, Point>,
    state: State,
    start_joint: LineJoint,
    end_joint: LineJoint,
    width: u32,
    alignment: StrokeAlignment,
    points: &'a [Point],
}

static EMPTY: &[Point; 0] = &[];

impl<'a> LineJointsIter<'a> {
    /// New
    pub fn new(points: &'a [Point], width: u32, alignment: StrokeAlignment) -> Self {
        let mut windows = points.windows(3);

        if let Some([start, mid, end]) = windows.next() {
            let start_joint = LineJoint::start(*start, *mid, width, alignment);
            let end_joint = LineJoint::from_points(*start, *mid, *end, width, alignment);

            Self {
                state: State::StartEdge,
                windows,
                start_joint,
                end_joint,
                width,
                alignment,
                points,
            }
        } else if let [start, end] = points {
            // Single line segment.
            let start_joint = LineJoint::start(*start, *end, width, alignment);
            let end_joint = LineJoint::end(*start, *end, width, alignment);

            Self {
                state: State::StartEdge,
                windows: EMPTY.windows(3),
                start_joint,
                end_joint,
                width,
                alignment,
                points,
            }
        } else {
            // Points must be at least 2 in length to make a polyline iterator out of.
            Self::empty()
        }
    }

    /// Empty
    fn empty() -> Self {
        Self {
            state: State::Done,
            windows: EMPTY.windows(3),
            start_joint: LineJoint::empty(),
            end_joint: LineJoint::empty(),
            width: 0,
            alignment: StrokeAlignment::Center,
            points: EMPTY,
        }
    }
}

impl<'a> Iterator for LineJointsIter<'a> {
    type Item = Line;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            State::StartEdge => {
                self.state = State::FirstEdgeL;

                Some(Line::new(
                    self.start_joint.first_edge_end.left,
                    self.start_joint.first_edge_end.right,
                ))
            }
            State::FirstEdgeL => {
                self.state = State::FirstEdgeR;

                Some(Line::new(
                    self.start_joint.second_edge_start.left,
                    self.end_joint.first_edge_end.left,
                ))
            }
            State::FirstEdgeR => {
                self.state = match self.end_joint.kind {
                    JointKind::Bevel { .. } | JointKind::Degenerate { .. } => State::Bevel,
                    JointKind::End => State::EndEdge,
                    _ => State::NextSegment,
                };

                Some(Line::new(
                    self.start_joint.second_edge_start.right,
                    self.end_joint.first_edge_end.right,
                ))
            }
            State::Bevel => {
                self.state = State::NextSegment;

                match self.end_joint.kind {
                    JointKind::Bevel { filler_line, .. }
                    | JointKind::Degenerate { filler_line, .. } => Some(filler_line),
                    _ => None,
                }
            }
            State::NextSegment => {
                self.start_joint = self.end_joint;
                self.state = State::FirstEdgeL;

                if let Some([start, mid, end]) = self.windows.next() {
                    self.end_joint =
                        LineJoint::from_points(*start, *mid, *end, self.width, self.alignment);
                } else {
                    self.end_joint = LineJoint::end(
                        *self.points.get(self.points.len() - 2)?,
                        *self.points.last()?,
                        self.width,
                        self.alignment,
                    );
                }

                self.next()
            }
            State::EndEdge => {
                self.state = State::Done;

                Some(Line::new(
                    self.end_joint.second_edge_start.left,
                    self.end_joint.second_edge_start.right,
                ))
            }
            State::Done => None,
        }
    }
}
