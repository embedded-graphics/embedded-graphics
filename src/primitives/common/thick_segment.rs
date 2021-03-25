//! A line segment constructed from two line joints.

use crate::{
    geometry::Dimensions,
    primitives::{
        common::{LineJoin, Scanline},
        Line, Rectangle,
    },
};

#[derive(Debug, Clone, Copy)]
pub struct ThickSegment {
    start_join: LineJoin,
    end_join: LineJoin,
}

impl ThickSegment {
    /// Create a new thick segment from two joints.
    pub fn new(start_join: LineJoin, end_join: LineJoin) -> Self {
        Self {
            start_join,
            end_join,
        }
    }

    /// Check whether the thick segment is thick or not.
    pub fn is_skeleton(&self) -> bool {
        self.start_join.first_edge_end.left == self.start_join.first_edge_end.right
    }

    /// Get the right/left edges of this line segment.
    fn edges(&self) -> (Line, Line) {
        (
            Line::new(
                self.start_join.second_edge_start.right,
                self.end_join.first_edge_end.right,
            ),
            Line::new(
                self.end_join.first_edge_end.left,
                self.start_join.second_edge_start.left,
            ),
        )
    }

    /// Get the bounding box containing the left/right edges of the segment.
    ///
    /// Note that this does not include any bevel/cap lines as returned by `perimeter` which is why
    /// this is not `impl Dimensions`. These lines don't need to be included as other segments
    /// in the polyline will expand the bounding box to the right place anyway.
    pub fn edges_bounding_box(&self) -> Rectangle {
        let (right, left) = self.edges();

        if self.is_skeleton() {
            return left.bounding_box();
        }

        Rectangle::with_corners(
            right
                .start
                .component_min(right.end)
                .component_min(left.start)
                .component_min(left.end),
            right
                .start
                .component_max(right.end)
                .component_max(left.start)
                .component_max(left.end),
        )
    }

    pub fn intersection(&self, scanline_y: i32) -> Scanline {
        let mut scanline = Scanline::new_empty(scanline_y);

        // Single 1px line
        if self.is_skeleton() {
            scanline.bresenham_intersection(&self.edges().0);
        } else {
            let (line1, line2) = self.start_join.start_cap_lines();
            scanline.bresenham_intersection(&line1);
            if let Some(line2) = line2 {
                scanline.bresenham_intersection(&line2);
            }

            let (line1, line2) = self.end_join.end_cap_lines();
            scanline.bresenham_intersection(&line1);
            if let Some(line2) = line2 {
                scanline.bresenham_intersection(&line2);
            }

            let (line1, line2) = self.edges();
            scanline.bresenham_intersection(&line1);
            scanline.bresenham_intersection(&line2);
        }

        scanline
    }
}
