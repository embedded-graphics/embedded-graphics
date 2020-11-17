//! An iterator over all line intersections with a given scanline.

use crate::{
    geometry::Point,
    primitives::{
        common::{LineJoin, StrokeOffset, ThickSegment},
        Line,
    },
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
    accum: Option<Line>,
}

const EMPTY: &[Point; 3] = &[Point::zero(); 3];

impl<'a> ScanlineIntersections<'a> {
    /// New
    pub fn new(points: &'a [Point], width: u32, scanline_y: i32) -> Self {
        // let next_start_join = if let Some([first, second]) = points.get(0..1) {
        //     Some(LineJoin::start(*first, *second, width, StrokeOffset::None))
        // } else {
        //     None
        // };

        // MSRV: Use subslice patterns when we bump to at least 1.42.0
        let next_start_join = match points.get(0..2) {
            Some([first, second]) => {
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
            accum: None,
        }
    }

    /// Empty scanline iterator.
    pub(in crate::primitives) fn empty() -> Self {
        Self {
            next_start_join: None,
            width: 0,
            points: EMPTY,
            remaining_points: EMPTY,
            scanline_y: 0,
            accum: None,
        }
    }

    /// Reset scanline iterator with a new scanline.
    pub(in crate::primitives) fn reset_with_new_scanline(&mut self, scanline_y: i32) {
        *self = Self::new(self.points, self.width, scanline_y);
    }

    fn next_segment(&mut self) -> Option<ThickSegment> {
        let start_join = self.next_start_join?;

        // let end_join = match self.remaining_points {
        //     [start, mid, end, ..] => {
        //         LineJoin::from_points(*start, *mid, *end, self.width, StrokeOffset::None)
        //     }
        //     [start, end] => LineJoin::end(*start, *end, self.width, StrokeOffset::None),
        //     _ => return None,
        // };

        // MSRV: Use subslice patterns when we bump to at least 1.42.0
        let end_join = self
            .remaining_points
            .get(0..3)
            .or_else(|| self.remaining_points.get(0..2))?;

        let end_join = match end_join {
            [start, mid, end] => {
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

/// This iterator loops through all scanline intersections for all segments. If two intersections
/// are adjacent or overlapping, an accumulator line is extended. This repeats until the next
/// intersection does not touch the current accumulator. At this point, the accumulated line
/// segment is returned, and is reset to the next segment.
///
/// This process reduces the number of draw calls for adjacent scanlines, whilst preventing overdraw
/// from overlapping scanline segments.
///
/// ```text
/// # Adjacent - merge
/// A---AB+++B
///     ⇓
/// A--------A
///
/// # Overlapping - merge
/// A---B+++A+++B
///      ⇓
/// A-----------A
///
/// # Separate - leave alone
/// A---A B---B
///      ⇓
/// A---A B---B
/// ```
impl<'a> Iterator for ScanlineIntersections<'a> {
    type Item = Line;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(segment) = self.next_segment() {
            if let Some(next_segment) = segment.intersection(self.scanline_y) {
                if let Some(accum) = self.accum {
                    // If next segment doesn't touch, return current accum state and reset accum
                    // to new line.
                    if !touches(accum, next_segment) {
                        self.accum = Some(next_segment);
                        return Some(accum);
                    }

                    // If next segment touches current accum, extend accum and continue.
                    self.accum = Some(extend(accum, next_segment));
                }
                // Initialize accumulator with a single line. Next iteration will return it if
                // next intersection doesn't touch.
                else {
                    self.accum = Some(next_segment);
                }
            }
        }

        // No more segments - return the final accumulated line.
        self.accum.take()
    }
}

/// Check for lines that are adjacent or overlapping.
///
/// This assumes that both lines have the same y coordinate and are horizontal.
pub(in crate::primitives) fn touches(l1: Line, l2: Line) -> bool {
    let first_range = (l1.start.x - 1)..=(l1.end.x + 1);

    first_range.contains(&l2.start.x) || first_range.contains(&l2.end.x)
}

/// Merge to lines into one longer line.
///
/// This assumes the lines are adjacent or touching, which is guaranteed by the iterator logic
/// around where this function is called.
pub(in crate::primitives) fn extend(l1: Line, l2: Line) -> Line {
    // Lines are scanlines, so we can reuse the same Y coordinate for everything.
    let y = l1.start.y;

    Line::new(
        Point::new(l1.start.x.min(l2.start.x), y),
        Point::new(l1.end.x.max(l2.end.x), y),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_touches_test(s1: i32, e1: i32, s2: i32, e2: i32, expected: bool, ident: &str) {
        let l1 = Line::new(Point::new(s1, 0), Point::new(e1, 0));
        let l2 = Line::new(Point::new(s2, 0), Point::new(e2, 0));

        assert_eq!(touches(l1, l2), expected, "{}", ident);
    }

    #[test]
    fn check_touches() {
        run_touches_test(30, 40, 5, 15, false, "Reversed");
        run_touches_test(0, 6, 5, 10, true, "Contained");
        run_touches_test(11, 13, 11, 14, true, "Contained 2");
        run_touches_test(10, 15, 25, 35, false, "Separated");
        run_touches_test(10, 10, 10, 10, true, "Zero size");
        run_touches_test(10, 20, 10, 20, true, "Equal");
        run_touches_test(10, 20, 20, 10, true, "Equal reversed");
        run_touches_test(79, 82, 82, 92, true, "Overlapping lines 1");
        run_touches_test(82, 92, 79, 82, true, "Overlapping lines 1, reversed");
        run_touches_test(80, 83, 83, 94, true, "Overlapping lines 2");
        run_touches_test(83, 94, 80, 83, true, "Overlapping lines 2, reversed");
        run_touches_test(83, 94, 94, 100, true, "Adjacent");
    }
}
