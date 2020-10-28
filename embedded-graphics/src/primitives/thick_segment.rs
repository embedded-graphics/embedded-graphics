//! A line segment constructed from two line joints.
//!
//! Segment always has one open side, so scanline intersections may return one or two points, but
//! no more.

use crate::{
    geometry::{Dimensions, Point},
    primitives::{line_joint::JointKind, line_joint::LineJoint, Line, Rectangle},
    style::StrokeAlignment,
};

// /// TODO: Doc
// #[derive(Debug, Clone, Copy)]
// pub enum SegmentIntersection {
//     /// Start and end intersection.
//     Closed(Line),

//     /// Only one intersection, could be start or end. Usually 1px long but for shallow lines this
//     /// can be longer.
//     Open(Line),
// }

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

    // TODO: Un-pub
    /// Get the right/left edges of this line segment.
    pub fn edges(&self) -> (Line, Line) {
        (
            Line::new(
                self.start_joint.second_edge_start.right,
                self.end_joint.first_edge_end.right,
            ),
            Line::new(
                self.end_joint.first_edge_end.left,
                self.start_joint.second_edge_start.left,
            ),
        )
    }

    /// Get the bounding box containing the left/right edges of the segment.
    ///
    /// Note that this does not include any bevel/cap lines as returned by `perimiter` which is why
    /// this is not `impl Dimensions`. These lines don't need to be included as  other segments
    /// in the polyline will expand the bounding box to the right place anyway.
    pub fn edges_bounding_box(&self) -> Rectangle {
        let (right, left) = self.edges();

        let left = left.bounding_box();
        let right = right.bounding_box();

        let tl = left.top_left.component_min(right.top_left);

        let left_br = left.bottom_right().unwrap_or(tl);
        let right_br = right.bottom_right().unwrap_or(tl);

        let br = left_br.component_max(right_br);

        Rectangle::with_corners(tl, br)
    }

    /// Get up to 6 lines comprising the perimiter of this segment.
    ///
    /// Note: This array could be stored as a `ThickSegment` property, but it's calculated on the fly
    /// for memory reasons.
    fn perimiter(&self) -> [Option<Line>; 6] {
        let start_cap = self.start_joint.start_cap_lines();
        let end_cap = self.end_joint.end_cap_lines();
        let edges = self.edges();

        [
            start_cap[0],
            start_cap[1],
            end_cap[0],
            end_cap[1],
            edges.0.into(),
            edges.1.into(),
        ]
    }

    /// TODO: DOc. Un-pub?
    pub fn intersection(&self, scanline_y: i32) -> Option<Line> {
        let perimiter = self.perimiter();

        // Loop through perimiter and get any intersections
        let it = perimiter.iter().filter_map(|l| {
            l.and_then(|l| {
                l.bresenham_scanline_intersection(scanline_y)
                    .map(|(intersection, _)| intersection.as_line().sorted_x())
            })
        });

        // Loop through intersections and collect min/max bounds of all of them into a single line
        let line = it.fold(None, |acc: Option<Line>, line| {
            if let Some(acc) = acc {
                Some(Line::new(
                    acc.start.component_min(line.start),
                    acc.end.component_max(line.end),
                ))
            } else {
                Some(line)
            }
        })?;

        Some(line)
    }
}

// #[derive(Debug, Copy, Clone)]
// enum State {
//     StartEdge1,
//     StartEdge2,
//     RightEdge,
//     EndEdge1,
//     EndEdge2,
//     LeftEdge,
//     NextSegment,
//     Done,
// }

// /// Line joints iter.
// ///
// /// Iterates over all line segments that make up the closed boundary polygon of a thick polyline.
// /// Lines must be returned in increasing X order so that scanline intersections work correctly.
// #[derive(Clone, Copy, Debug)]
// pub struct Segments {
//     state: State,
//     start_joint: LineJoint,
//     end_joint: LineJoint,
// }

// impl Segments {
//     /// TODO: Unpub me
//     pub fn new(start_joint: LineJoint, end_joint: LineJoint) -> Self {
//         Self {
//             start_joint,
//             end_joint,
//             state: State::StartEdge,
//         }
//     }
// }

// impl Iterator for Segments {
//     type Item = Line;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.state {
//             State::StartEdge => {
//                 self.state = State::LeftEdge;

//                 Some(Line::new(
//                     self.start_joint.first_edge_end.left,
//                     self.start_joint.first_edge_end.right,
//                 ))
//             }
//             State::LeftEdge => {
//                 self.state = State::RightEdge;

//                 Some(Line::new(
//                     self.start_joint.second_edge_start.left,
//                     self.end_joint.first_edge_end.left,
//                 ))
//             }
//             State::RightEdge => {
//                 self.state = match self.end_joint.kind {
//                     JointKind::Bevel { .. } | JointKind::Degenerate { .. } => State::Bevel,
//                     _ => State::EndEdge,
//                 };

//                 Some(Line::new(
//                     self.start_joint.second_edge_start.right,
//                     self.end_joint.first_edge_end.right,
//                 ))
//             }
//             State::Bevel => {
//                 self.state = State::ClosingLine;

//                 match self.end_joint.kind {
//                     JointKind::Bevel { filler_line, .. }
//                     | JointKind::Degenerate { filler_line, .. } => Some(dbg!(filler_line)),
//                     _ => None,
//                 }
//             }
//             State::EndEdge => {
//                 self.state = State::Done;

//                 Some(Line::new(
//                     self.end_joint.second_edge_start.left,
//                     self.end_joint.second_edge_start.right,
//                 ))
//             }
//             State::ClosingLine => {
//                 self.state = State::Done;

//                 match self.end_joint.kind {
//                     JointKind::Bevel {
//                         filler_line, side, ..
//                     }
//                     | JointKind::Degenerate {
//                         filler_line, side, ..
//                     } => {
//                         let end = match side {
//                             Side::Left => self.end_joint.second_edge_start.right,
//                             Side::Right => self.end_joint.second_edge_start.left,
//                         };

//                         Some(Line::new(filler_line.end, end))
//                     }
//                     _ => None,
//                 }
//             }
//             State::Done => None,
//         }
//     }
// }
