//! A line segment constructed from two line joints.
//!
//! Segment always has one open side, so scanline intersections may return one or two points, but
//! no more.

use super::line::Side;
use crate::{
    geometry::Point,
    primitives::{line_joint::JointKind, line_joint::LineJoint, Line},
    style::StrokeAlignment,
};

/// TODO: Doc
#[derive(Debug, Clone, Copy)]
pub enum SegmentIntersection {
    /// Start and end intersection.
    Closed(Line),

    /// Only one intersection, could be start or end. Usually 1px long but for shallow lines this
    /// can be longer.
    Open(Line),
}

/// TODO: Doc
#[derive(Debug, Clone, Copy)]
pub struct ThickSegment {
    start_joint: LineJoint,
    end_joint: LineJoint,
}

impl ThickSegment {
    /// New
    pub fn new(start_joint: LineJoint, end_joint: LineJoint) -> Self {
        Self {
            start_joint,
            end_joint,
        }
    }

    /// TODO: DOc
    pub fn intersection(&self, scanline_y: i32) -> Option<Line> {
        // TODO: Broad phase bounding box collision detection

        let mut it = Segments::new(self.start_joint, self.end_joint).filter_map(|l| {
            l.bresenham_scanline_intersection(scanline_y)
                .map(|(intersection, base_line)| intersection.as_line().sorted_x())
        });

        let start = it.next()?;
        let end = it.next()?;

        Some(Line::new(start.start, end.end).sorted_x())

        // let (first, base) = it.next()?;

        // // Colinear
        // if first == base {
        //     return Some(SegmentIntersection::Closed(first));
        // }

        // // // On vertex
        // // if first.start == base.start
        // //     || first.start == base.end
        // //     || first.end == base.start
        // //     || first.end == base.end
        // // {
        // //     return Some(SegmentIntersection::Closed(first));
        // // }

        // // Second intersection as long as it doesn't lie on the same point (e.g. at nodes).
        // let second = it.next()/*.filter(|second| *second != first)*/;

        // let res = if let Some((second, _)) = second {
        //     let line = Line::new(first.start, second.end).sorted_x();

        //     SegmentIntersection::Closed(line)
        // } else {
        //     // FIXME: What about when this is a line?
        //     SegmentIntersection::Open(first)
        // };

        // Some(res)
    }
}

#[derive(Debug, Copy, Clone)]
enum State {
    StartEdge,
    LeftEdge,
    RightEdge,
    Bevel,
    EndEdge,
    Done,
    ClosingLine,
}

/// Line joints iter.
///
/// Iterates over all line segments that make up the closed boundary polygon of a thick polyline.
/// Lines must be returned in increasing X order so that scanline intersections work correctly.
#[derive(Clone, Copy, Debug)]
pub struct Segments {
    state: State,
    start_joint: LineJoint,
    end_joint: LineJoint,
}

impl Segments {
    fn new(start_joint: LineJoint, end_joint: LineJoint) -> Self {
        // let state = match start_joint.kind {
        //     JointKind::Start => State::StartEdge,
        //     _ => State::LeftEdge,
        // };

        Self {
            start_joint,
            end_joint,
            state: State::StartEdge,
        }
    }
}

impl Iterator for Segments {
    type Item = Line;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            State::StartEdge => {
                self.state = State::LeftEdge;

                Some(Line::new(
                    self.start_joint.first_edge_end.left,
                    self.start_joint.first_edge_end.right,
                ))
            }
            State::LeftEdge => {
                self.state = State::RightEdge;

                Some(Line::new(
                    self.start_joint.second_edge_start.left,
                    self.end_joint.first_edge_end.left,
                ))
            }
            State::RightEdge => {
                self.state = match self.end_joint.kind {
                    JointKind::Bevel { .. } | JointKind::Degenerate { .. } => State::Bevel,
                    // JointKind::End => State::EndEdge,
                    _ => State::EndEdge,
                };

                Some(Line::new(
                    self.start_joint.second_edge_start.right,
                    self.end_joint.first_edge_end.right,
                ))
            }
            State::Bevel => {
                self.state = State::ClosingLine;

                match self.end_joint.kind {
                    JointKind::Bevel { filler_line, .. }
                    | JointKind::Degenerate { filler_line, .. } => Some(filler_line),
                    _ => None,
                }
            }
            State::EndEdge => {
                self.state = State::Done;

                Some(Line::new(
                    self.end_joint.second_edge_start.left,
                    self.end_joint.second_edge_start.right,
                ))
            }
            State::ClosingLine => {
                self.state = State::Done;

                match self.end_joint.kind {
                    JointKind::Bevel {
                        filler_line, side, ..
                    }
                    | JointKind::Degenerate {
                        filler_line, side, ..
                    } => {
                        let end = match side {
                            Side::Left => self.end_joint.first_edge_end.right,
                            Side::Right => self.end_joint.first_edge_end.left,
                        };

                        Some(Line::new(filler_line.end, end))
                    }
                    _ => None,
                }
            }
            State::Done => None,
        }
    }
}
