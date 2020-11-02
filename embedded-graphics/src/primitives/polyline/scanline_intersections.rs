//! An iterator over all line intersections with a given scanline.

use crate::{
    geometry::Point,
    primitives::line::StrokeOffset,
    primitives::{line_join::LineJoin, thick_segment::ThickSegment, Line},
};

/// Scanline intersections iterator.
///
/// This iterator returns multiple `Line`s corresponding to the filled in areas of a polyline
/// defined by the `points` parameter.
///
/// The result is one line of a filled polygon.
#[derive(Clone, Debug)]
pub struct ScanlineIntersections<'a> {
    points: &'a [Point],
    remaining_points: &'a [Point],
    scanline_y: i32,
    next_start_join: Option<LineJoin>,
    width: u32,
    prev_line: Option<Line>,
}

impl<'a> ScanlineIntersections<'a> {
    /// New
    pub fn new(points: &'a [Point], width: u32, scanline_y: i32) -> Self {
        let next_start_join = match points {
            [first, second, ..] => {
                Some(LineJoin::start(*first, *second, width, StrokeOffset::None))
            }
            _ => None,
        };

        Self {
            next_start_join,
            width,
            points,
            remaining_points: points,
            scanline_y,
            prev_line: None,
        }
    }

    /// Reset scanline iterator with a new scanline.
    pub(in crate::primitives) fn reset_with_new_scanline(&mut self, scanline_y: i32) {
        *self = Self::new(self.points, self.width, scanline_y);
    }

    fn next_segment(&mut self) -> Option<ThickSegment> {
        let start_join = self.next_start_join?;

        let end_join = match self.remaining_points {
            [start, mid, end, ..] => {
                LineJoin::from_points(*start, *mid, *end, self.width, StrokeOffset::None)
            }
            [mid, end] => LineJoin::end(*mid, *end, self.width, StrokeOffset::None),
            _ => return None,
        };

        self.remaining_points = self.remaining_points.get(1..)?;

        let segment = ThickSegment::new(start_join, end_join);

        self.next_start_join = Some(end_join);

        Some(segment)
    }
}

// PERF: There may be some performance to be gained by merging adjacent line segments - this would
// shrink the number of draw calls around polyline joints and any line segments that are directly
// next to each other.
impl<'a> Iterator for ScanlineIntersections<'a> {
    type Item = Line;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let segment = self.next_segment()?;

            if let Some(mut line) = segment.intersection(self.scanline_y) {
                // Do some checks to prevent adjacent line overdraw at end/start points.
                if let Some(prev_line) = self.prev_line {
                    // NOTE: Lines returned from intersections are always X-sorted with start point
                    // to the left.
                    if let Some(fixed_overlap) = fix_overlap(prev_line, line) {
                        line = fixed_overlap;
                    }
                    // Current line doesn't need to be drawn. Skip onto the next line that may
                    // extend prev_line.
                    else {
                        continue;
                    }
                }

                self.prev_line = Some(line);

                return Some(line);
            }
        }
    }
}

/// Fix any overlap between two lines.
///
/// If the second line start point lies inside the first line, the second line start point will be
/// shifted to 1px before or 1px after the first line so it doesn't overlap.
/// If the second line doesn't need to be drawn at all, this function will return `None`.
fn fix_overlap(l1: Line, l2: Line) -> Option<Line> {
    let l1_range = l1.start.x..=l1.end.x;

    let is_completely_contained = l1_range.contains(&l2.start.x) && l1_range.contains(&l2.end.x);

    // Don't need to draw the second line if it will be completely obscured by first.
    if l1 == l2 || is_completely_contained {
        return None;
    }

    let mut l2 = l2;

    // If one line contains the start of the other, push second line to start/end of first so
    // they're adjacent without overlap.
    if l1_range.contains(&l2.start.x) {
        // Second line extends to the left of the first line. Clamp second end point to just before
        // first line start.
        if l2.start.x < l1.start.x {
            l2.end.x = l1.start.x - 1;
        }
        // Second line extends to the right. Clamp start point to just after first line end.
        else if l2.end.x > l1.end.x {
            l2.start.x = l1.end.x + 1;
        }
    }

    Some(l2)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_overlap_test(
        s1: i32,
        e1: i32,
        s2: i32,
        e2: i32,
        expected: Option<[i32; 2]>,
        ident: &str,
    ) {
        let expected = expected.map(|[expected_start, expected_end]| {
            Line::new(Point::new(expected_start, 0), Point::new(expected_end, 0))
        });

        let l1 = Line::new(Point::new(s1, 0), Point::new(e1, 0));
        let l2 = Line::new(Point::new(s2, 0), Point::new(e2, 0));

        assert_eq!(fix_overlap(l1, l2), expected, "{}", ident);
    }

    #[test]
    fn test_fix_overlap() {
        run_overlap_test(30, 40, 5, 15, Some([5, 15]), "Reversed");
        run_overlap_test(0, 6, 5, 10, Some([7, 10]), "Contained");
        run_overlap_test(11, 13, 11, 14, Some([14, 14]), "Contained 2");
        run_overlap_test(10, 15, 25, 35, Some([25, 35]), "Separated");
        run_overlap_test(10, 10, 10, 10, None, "Zero size");
        run_overlap_test(10, 20, 10, 20, None, "Equal");
        run_overlap_test(10, 20, 20, 10, None, "Equal reversed");
    }
}
