//! Thick line joint.

use crate::{
    geometry::Point,
    primitives::{
        line_joint::JointKind, line_joint::LineJoint, line_joints_iter::LineJointsIter, Line,
    },
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
pub struct ScanlineIntersections<'a> {
    lines: LineJointsIter<'a>,
    scanline: Line,
}

impl<'a> ScanlineIntersections<'a> {
    /// New
    pub fn new(
        points: &'a [Point],
        width: u32,
        alignment: StrokeAlignment,
        scanline: Line,
    ) -> Self {
        Self {
            lines: LineJointsIter::new(points, width, alignment),
            scanline,
        }
    }
}

impl<'a> Iterator for ScanlineIntersections<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let start_y = self.scanline.start.y;

        self.lines
            .find_map(|l| l.bresenham_scanline_intersection(start_y))
    }
}
