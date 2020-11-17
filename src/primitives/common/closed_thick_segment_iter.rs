//! Closed shape thick segment iterator.

use crate::{
    geometry::Point,
    primitives::common::{LineJoin, StrokeOffset, ThickSegment},
};

/// Closed shape thick segments iterator.
///
/// Iterates over all line segments in the polyline, returning a 6-sided shape as a [`ThickSegment`]
/// for each segment. These are tessellated and are used to produce scanline intersections.
///
/// Unlike [`ThickSegmentIter`], this iterator closes the shape with a final line between the
/// start and end points.
///
/// [`ThickSegment`]: ../thick_segment/struct.ThickSegment.html
/// [`ThickSegmentIter`]: ../thick_segment_iter/struct.ThickSegmentIter.html
#[derive(Clone, Debug)]
pub struct ClosedThickSegmentIter<'a> {
    windows: core::slice::Windows<'a, Point>,
    first_join: LineJoin,
    start_join: LineJoin,
    width: u32,
    stroke_offset: StrokeOffset,
    points: &'a [Point],
    stop: bool,
    idx: usize,
}

static EMPTY: &[Point; 0] = &[];

impl<'a> ClosedThickSegmentIter<'a> {
    /// Create a new thick segments iterator.
    pub fn new(points: &'a [Point], width: u32, stroke_offset: StrokeOffset) -> Self {
        if let [start, end] = points {
            // Single line segment.
            let start_join = LineJoin::start(*start, *end, width, stroke_offset);

            Self {
                windows: EMPTY.windows(3),
                start_join,
                width,
                stroke_offset,
                points,
                stop: false,
                first_join: start_join,
                idx: 1,
            }
        } else if points.is_empty() {
            Self::empty()
        } else {
            let windows = points.windows(3);

            let start_join = LineJoin::from_points(
                *points.last().unwrap(),
                points[0],
                points[1],
                width,
                stroke_offset,
            );

            Self {
                windows,
                start_join,
                width,
                stroke_offset,
                points,
                stop: false,
                first_join: start_join,
                idx: 1,
            }
        }
    }

    /// Empty
    fn empty() -> Self {
        Self {
            windows: EMPTY.windows(3),
            start_join: LineJoin::empty(),
            width: 0,
            stroke_offset: StrokeOffset::None,
            points: EMPTY,
            stop: true,
            first_join: LineJoin::empty(),
            idx: 1,
        }
    }
}

impl<'a> Iterator for ClosedThickSegmentIter<'a> {
    type Item = ThickSegment;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stop {
            return None;
        }

        self.idx += 1;

        let end_join = if let Some([start, mid, end]) = self.windows.next() {
            LineJoin::from_points(*start, *mid, *end, self.width, self.stroke_offset)
        } else if self.idx == self.points.len() {
            // The join at the end of the line. This will become the start join of the closing
            // segment.
            let start = self.points.get(self.points.len() - 2)?;
            let mid = self.points.last()?;
            let end = self.points.first()?;

            LineJoin::from_points(*start, *mid, *end, self.width, self.stroke_offset)
        } else {
            // Final closing line between start/end.
            self.stop = true;

            self.first_join
        };

        let segment = ThickSegment::new(self.start_join, end_join);

        self.start_join = end_join;

        Some(segment)
    }
}
