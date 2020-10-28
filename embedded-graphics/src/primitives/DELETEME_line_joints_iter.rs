//! Thick line joint.

use crate::{
    geometry::Point,
    primitives::{line_joint::JointKind, line_joint::LineJoint, Line},
    style::StrokeAlignment,
};

use super::line::Side;

#[derive(Debug, Copy, Clone)]
enum State {
    StartEdge1,
    StartEdge2,
    RightEdge,
    EndEdge1,
    EndEdge2,
    LeftEdge,
    NextSegment,
    Done,
}

/// Line type.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LineType {
    /// Start Cap
    StartCap,
    /// Left Side
    LeftSide,
    /// Right Side
    RightSide,
    /// Bevel
    Bevel,
    /// End Cap
    EndCap,
}

/// Line joints iter.
///
/// Iterates over all line segments that make up the closed boundary polygon of a thick polyline.
/// Lines must be returned in increasing X order so that scanline intersections work correctly.
#[derive(Clone, Debug)]
pub struct ThickSegmentIter<'a> {
    windows: core::slice::Windows<'a, Point>,
    state: State,
    start_joint: LineJoint,
    end_joint: LineJoint,
    width: u32,
    alignment: StrokeAlignment,
    points: &'a [Point],
    right_side_first: bool,
}

static EMPTY: &[Point; 0] = &[];

impl<'a> ThickSegmentIter<'a> {
    /// New
    pub fn new(points: &'a [Point], width: u32, alignment: StrokeAlignment) -> Self {
        let mut windows = points.windows(3);

        if let Some([start, mid, end]) = windows.next() {
            let start_joint = LineJoint::start(*start, *mid, width, alignment);
            let end_joint = LineJoint::from_points(*start, *mid, *end, width, alignment);

            let joiner = Line::new(
                start_joint.second_edge_start.left,
                end_joint.first_edge_end.left,
            );

            // dbg!(joiner, joiner.sign_y());
            let dir = joiner.direction();

            Self {
                state: State::StartEdge1,
                windows,
                start_joint,
                end_joint,
                width,
                alignment,
                points,
                // TODO: Can I simplify this by sorting points and checking for a certain Y slope?
                // right_side_first: dir == Point::new(1, -1) || dir == Point::new(-1, 1),
                // right_side_first: joiner.sign_y() == 1,
                // right_side_first: dir == Point::new(-1, -1) || dir == Point::new(1, 1),
                right_side_first: false,
            }
        } else if let [start, end] = points {
            // Single line segment.
            let start_joint = LineJoint::start(*start, *end, width, alignment);
            let end_joint = LineJoint::end(*start, *end, width, alignment);

            Self {
                state: State::StartEdge1,
                windows: EMPTY.windows(3),
                start_joint,
                end_joint,
                width,
                alignment,
                points,
                right_side_first: Line::new(*start, *end).sign_y() < 0,
            }
        } else {
            // Points must be at least 2 in length to make a polyline iterator out of.
            Self::empty()
        }
    }

    /// Reset
    pub fn reset(&mut self) {
        *self = Self::new(self.points, self.width, self.alignment);
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
            right_side_first: false,
        }
    }
}

impl<'a> Iterator for ThickSegmentIter<'a> {
    type Item = ThickSegment;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            State::StartEdge1 => {
                let start = self.start_joint.second_edge_start.left;

                let end = match self.start_joint.kind {
                    JointKind::Bevel { filler_line, .. }
                    | JointKind::Degenerate { filler_line, .. } => {
                        self.state = State::StartEdge2;

                        filler_line.midpoint()
                    }
                    _ => {
                        self.state = State::RightEdge;

                        self.start_joint.second_edge_start.right
                    }
                };

                Some(Line::new(start, end))
            }
            State::StartEdge2 => {
                let start = match self.start_joint.kind {
                    JointKind::Bevel { filler_line, .. }
                    | JointKind::Degenerate { filler_line, .. } => {
                        self.state = State::RightEdge;

                        filler_line.midpoint()
                    }
                    _ => unreachable!(
                        "StartEdge2: Bevelled end caps always comprise two line segments."
                    ),
                };

                let end = self.start_joint.second_edge_start.right;

                Some(Line::new(start, end))
            }
            State::RightEdge => {
                self.state = State::EndEdge1;

                let start = self.start_joint.second_edge_start.right;
                let end = self.end_joint.first_edge_end.right;

                Some(Line::new(start, end))
            }
            State::EndEdge1 => {
                let start = self.end_joint.first_edge_end.right;

                let end = match self.end_joint.kind {
                    JointKind::Bevel { filler_line, .. }
                    | JointKind::Degenerate { filler_line, .. } => {
                        self.state = State::EndEdge2;

                        filler_line.midpoint()
                    }
                    _ => {
                        self.state = State::LeftEdge;

                        self.end_joint.first_edge_end.left
                    }
                };

                Some(Line::new(start, end))
            }
            State::EndEdge2 => {
                let start = match self.end_joint.kind {
                    JointKind::Bevel { filler_line, .. }
                    | JointKind::Degenerate { filler_line, .. } => {
                        self.state = State::LeftEdge;

                        filler_line.midpoint()
                    }
                    _ => unreachable!(
                        "EndEdge2: Bevelled end caps always comprise two line segments."
                    ),
                };

                let end = self.end_joint.first_edge_end.left;

                Some(Line::new(start, end))
            }
            State::LeftEdge => {
                self.state = match self.end_joint.kind {
                    JointKind::End => State::Done,
                    _ => State::NextSegment,
                };

                let start = self.end_joint.first_edge_end.left;
                let end = self.start_joint.second_edge_start.left;

                Some(Line::new(start, end))
            }
            State::NextSegment => {
                self.start_joint = self.end_joint;

                if let Some([start, mid, end]) = self.windows.next() {
                    self.state = State::StartEdge1;

                    self.end_joint =
                        LineJoint::from_points(*start, *mid, *end, self.width, self.alignment);
                } else if self.end_joint.kind != JointKind::End {
                    self.state = State::StartEdge1;

                    let start = *self.points.get(self.points.len() - 2)?;
                    let end = *self.points.last()?;

                    self.end_joint = LineJoint::end(start, end, self.width, self.alignment);
                } else {
                    self.state = State::Done;
                }

                self.next()
            }
            State::Done => None,
        }
    }
}
