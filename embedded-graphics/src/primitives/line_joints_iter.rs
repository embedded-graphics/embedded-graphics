//! Thick line joint.

use crate::{
    geometry::Point,
    primitives::{line_joint::JointKind, line_joint::LineJoint, Line},
    style::StrokeAlignment,
};

use super::line::Side;

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
pub struct LineJointsIter<'a> {
    windows: core::slice::Windows<'a, Point>,
    state: State,
    start_joint: LineJoint,
    end_joint: LineJoint,
    width: u32,
    alignment: StrokeAlignment,
    points: &'a [Point],
    swap_sides: bool,
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
                swap_sides: Line::new(*start, *mid).sign_y() < 0,
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
                swap_sides: Line::new(*start, *end).sign_y() < 0,
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
            swap_sides: false,
        }
    }
}

impl<'a> Iterator for LineJointsIter<'a> {
    type Item = (Line, LineType);

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            State::StartEdge => {
                if !self.swap_sides {
                    self.state = State::FirstEdgeL;
                } else {
                    self.state = State::FirstEdgeR;
                }

                Some((
                    Line::new(
                        self.start_joint.first_edge_end.left,
                        self.start_joint.first_edge_end.right,
                    ),
                    LineType::StartCap,
                ))
            }
            State::FirstEdgeL => {
                if !self.swap_sides {
                    self.state = match self.end_joint.kind {
                        JointKind::End => State::EndEdge,
                        _ => State::FirstEdgeR,
                    };
                } else {
                    self.state = match self.end_joint.kind {
                        JointKind::Bevel { .. } | JointKind::Degenerate { .. } => State::Bevel,
                        JointKind::End => State::Done,
                        _ => State::NextSegment,
                    };
                }

                Some((
                    Line::new(
                        self.start_joint.second_edge_start.left,
                        self.end_joint.first_edge_end.left,
                    ),
                    LineType::LeftSide,
                ))
            }
            State::FirstEdgeR => {
                if !self.swap_sides {
                    self.state = match self.end_joint.kind {
                        JointKind::Bevel { .. } | JointKind::Degenerate { .. } => State::Bevel,
                        JointKind::End => State::Done,
                        _ => State::NextSegment,
                    };
                } else {
                    self.state = match self.end_joint.kind {
                        JointKind::End => State::EndEdge,
                        _ => State::FirstEdgeL,
                    };
                }

                Some((
                    Line::new(
                        self.start_joint.second_edge_start.right,
                        self.end_joint.first_edge_end.right,
                    ),
                    LineType::RightSide,
                ))
            }
            State::Bevel => {
                self.state = State::NextSegment;

                match self.end_joint.kind {
                    JointKind::Bevel {
                        filler_line, side, ..
                    }
                    | JointKind::Degenerate {
                        filler_line, side, ..
                    } => Some((
                        filler_line,
                        match side {
                            Side::Left => LineType::LeftSide,
                            Side::Right => LineType::RightSide,
                        },
                    )),
                    _ => None,
                }
            }
            State::NextSegment => {
                self.start_joint = self.end_joint;

                if let Some([start, mid, end]) = self.windows.next() {
                    self.end_joint =
                        LineJoint::from_points(*start, *mid, *end, self.width, self.alignment);

                    self.swap_sides = Line::new(
                        self.start_joint.second_edge_start.left,
                        self.end_joint.first_edge_end.left,
                    )
                    .sign_y()
                        > 0;
                } else {
                    let start = *self.points.get(self.points.len() - 2)?;
                    let end = *self.points.last()?;

                    self.swap_sides = Line::new(start, end).sign_y() > 0;
                    self.end_joint = LineJoint::end(start, end, self.width, self.alignment);
                }

                self.state = if !self.swap_sides {
                    State::FirstEdgeL
                } else {
                    State::FirstEdgeR
                };

                self.next()
            }
            State::EndEdge => {
                if !self.swap_sides {
                    self.state = State::FirstEdgeR;
                } else {
                    self.state = State::FirstEdgeL;
                }

                Some((
                    Line::new(
                        self.end_joint.second_edge_start.left,
                        self.end_joint.second_edge_start.right,
                    ),
                    LineType::EndCap,
                ))
            }
            State::Done => None,
        }
        // TODO: Check if I can delete this forever
        // .map(|(l, s)| (l.sorted_x(), s))
    }
}
