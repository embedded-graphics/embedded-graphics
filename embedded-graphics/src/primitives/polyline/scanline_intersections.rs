//! An iterator over all line intersections with a given scanline.

use crate::{
    geometry::Point,
    primitives::{line::BresenhamIntersection, line_joints_iter::LineJointsIter, Line},
    style::StrokeAlignment,
};

/// Scanline intersections iterator.
///
/// This iterator returns multiple `Line`s corresponding to the filled in areas of a polyline
/// defined by the `points` parameter.
///
/// The result is one line of a filled polygon.
#[derive(Clone, Debug)]
pub struct ScanlineIntersections<'a> {
    lines: LineJointsIter<'a>,
    scanline_y: i32,
    prev_end: Point,
}

impl<'a> ScanlineIntersections<'a> {
    /// New
    pub fn new(
        points: &'a [Point],
        width: u32,
        alignment: StrokeAlignment,
        scanline_y: i32,
    ) -> Self {
        let lines = LineJointsIter::new(points, width, alignment);

        Self {
            lines,
            scanline_y,
            prev_end: Point::zero(),
        }
    }

    pub fn reset_with_new_scanline(&mut self, scanline_y: i32) {
        self.lines.reset();
        self.scanline_y = scanline_y;
        self.prev_end = Point::zero();
    }

    fn next_intersection(&mut self) -> Option<BresenhamIntersection> {
        let scanline_y = self.scanline_y;

        self.lines
            .find_map(|l| l.bresenham_scanline_intersection(scanline_y))
    }
}

impl<'a> Iterator for ScanlineIntersections<'a> {
    type Item = Line;

    #[allow(unused)]
    fn next(&mut self) -> Option<Self::Item> {
        let mut first = self.next_intersection()?;
        let mut second = self.next_intersection()?;

        match first {
            BresenhamIntersection::Point(p) if p == self.prev_end => {
                first = second;
                second = self.next_intersection()?;
            }
            _ => (),
        }

        let mut start = match first {
            BresenhamIntersection::Colinear(l) => l.start,
            BresenhamIntersection::Line(l) => l.start,
            BresenhamIntersection::Point(p) => p,
        };

        let end = match second {
            BresenhamIntersection::Colinear(l) => l.end,
            BresenhamIntersection::Line(l) => l.end,
            BresenhamIntersection::Point(p) => p,
        };

        self.prev_end = end;

        Some(Line::new(start, end))
    }
}
