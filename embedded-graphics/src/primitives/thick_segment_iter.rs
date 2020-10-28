//! Thick line joint.

use crate::{
    geometry::Point,
    primitives::{line_joint::JointKind, line_joint::LineJoint, Line},
    style::StrokeAlignment,
};

use super::{line::Side, thick_segment::ThickSegment};

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
    stop: bool,
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
                stop: false,
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
                stop: false,
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
            stop: true,
        }
    }
}

impl<'a> Iterator for ThickSegmentIter<'a> {
    type Item = ThickSegment;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stop {
            return None;
        }

        let segment = ThickSegment::new(self.start_joint, self.end_joint);

        self.start_joint = self.end_joint;

        if let Some([start, mid, end]) = self.windows.next() {
            self.end_joint = LineJoint::from_points(*start, *mid, *end, self.width, self.alignment);
        } else if self.end_joint.kind != JointKind::End {
            let start = *self.points.get(self.points.len() - 2)?;
            let end = *self.points.last()?;

            self.end_joint = LineJoint::end(start, end, self.width, self.alignment);
        } else {
            self.stop = true;
        }

        Some(segment)
    }
}
