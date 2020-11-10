//! Scanline iterator.

use crate::primitives::{
    line::StrokeOffset,
    triangle::scanline_intersections::{PointType, ScanlineIntersections},
    Line, Rectangle, Triangle,
};

/// Iterate over every scanline in the triangle's bounding box.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(in crate::primitives::triangle) struct ScanlineIterator {
    scanline_y: i32,
    scanline_limit: i32,
    intersections: ScanlineIntersections,
}

impl ScanlineIterator {
    /// New.
    pub fn new(
        triangle: &Triangle,
        stroke_width: u32,
        stroke_offset: StrokeOffset,
        has_fill: bool,
        bounding_box: &Rectangle,
    ) -> Self {
        let triangle = triangle.sorted_clockwise();

        let scanline_y = bounding_box.top_left.y;
        let scanline_limit = scanline_y + bounding_box.size.height as i32;

        let intersections = ScanlineIntersections::new(
            &triangle,
            stroke_width,
            stroke_offset,
            has_fill,
            scanline_y,
        );

        Self {
            scanline_y,
            scanline_limit,
            intersections,
        }
    }
}

impl Iterator for ScanlineIterator {
    type Item = (Line, PointType);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(border) = self.intersections.next() {
            Some(border)
        } else {
            self.scanline_y += 1;

            if self.scanline_y > self.scanline_limit {
                return None;
            }

            self.intersections.reset_with_new_scanline(self.scanline_y);

            self.intersections.next()
        }
    }
}
