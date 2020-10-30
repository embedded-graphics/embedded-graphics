//! An iterator over all line intersections with a given scanline.

use crate::{
    geometry::Point,
    primitives::{
        line_join::{JoinKind, LineJoin},
        thick_segment::ThickSegment,
        Line,
    },
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
    windows: core::slice::Windows<'a, Point>,
    points: &'a [Point],
    scanline_y: i32,
    start_join: LineJoin,
    end_join: LineJoin,
    width: u32,
    alignment: StrokeAlignment,
    stop: bool,
    prev_line: Option<Line>,
}

static EMPTY: &[Point; 0] = &[];

impl<'a> ScanlineIntersections<'a> {
    /// New
    pub fn new(
        points: &'a [Point],
        width: u32,
        _alignment: StrokeAlignment,
        scanline_y: i32,
    ) -> Self {
        // Fix stroke alignment to Center. There are issues with degenerate joints when using
        // Inside/Outside stroke alignment on polylines, so this is disabled for now.
        let alignment = StrokeAlignment::Center;

        let mut windows = points.windows(3);

        if let Some([start, mid, end]) = windows.next() {
            let start_join = LineJoin::start(*start, *mid, width, alignment);
            let end_join = LineJoin::from_points(*start, *mid, *end, width, alignment);

            Self {
                windows,
                start_join,
                end_join,
                width,
                alignment,
                points,
                scanline_y,
                stop: false,
                prev_line: None,
            }
        } else if let [start, end] = points {
            // Single line segment.
            let start_join = LineJoin::start(*start, *end, width, alignment);
            let end_join = LineJoin::end(*start, *end, width, alignment);

            Self {
                windows: EMPTY.windows(3),
                start_join,
                end_join,
                width,
                alignment,
                points,
                scanline_y,
                stop: false,
                prev_line: None,
            }
        } else {
            // Points must be at least 2 in length to make a polyline iterator out of.
            Self::empty()
        }
    }

    /// Empty scanline iterator.
    fn empty() -> Self {
        Self {
            windows: EMPTY.windows(3),
            start_join: LineJoin::empty(),
            end_join: LineJoin::empty(),
            width: 0,
            alignment: StrokeAlignment::Center,
            points: EMPTY,
            scanline_y: 0,
            stop: true,
            prev_line: None,
        }
    }

    /// Reset scanline iterator with a new scanline.
    pub(in crate::primitives) fn reset_with_new_scanline(&mut self, scanline_y: i32) {
        *self = Self::new(self.points, self.width, self.alignment, scanline_y);
    }
}

// PERF: There may be some performance to be gained by merging adjacent line segments - this would
// shrink the number of draw calls around polyline joints and any line segments that are directly
// next to each other.
impl<'a> Iterator for ScanlineIntersections<'a> {
    type Item = Line;

    #[allow(unused)]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.stop {
                break None;
            }

            if self.end_join.kind == JoinKind::End {
                self.stop = true;
            }

            let line =
                ThickSegment::new(self.start_join, self.end_join).intersection(self.scanline_y);

            // Move window of joints along the line by 1 pair.
            {
                self.start_join = self.end_join;

                if let Some([start, mid, end]) = self.windows.next() {
                    self.end_join =
                        LineJoin::from_points(*start, *mid, *end, self.width, self.alignment);
                } else {
                    let start = *self.points.get(self.points.len() - 2)?;
                    let end = *self.points.last()?;

                    self.end_join = LineJoin::end(start, end, self.width, self.alignment);
                }
            }

            if let Some(mut line) = line {
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

                break Some(line);
            }

            // At this point, there was no intersection on this segment, so skip to the next one and
            // check that.
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
