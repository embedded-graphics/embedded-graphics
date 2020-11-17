//! Thick segment iterator.

use crate::{
    geometry::Point,
    primitives::common::{JoinKind, LineJoin, StrokeOffset, ThickSegment},
};

/// Thick segments iterator.
///
/// Iterates over all line segments in the polyline, returning a 6-sided shape as a [`ThickSegment`]
/// for each segment. These are tessellated and are used to produce scanline intersections.
///
/// [`ThickSegment`]: ../thick_segment/struct.ThickSegment.html
#[derive(Clone, Debug)]
pub struct ThickSegmentIter<'a> {
    windows: core::slice::Windows<'a, Point>,
    start_join: LineJoin,
    end_join: LineJoin,
    width: u32,
    stroke_offset: StrokeOffset,
    points: &'a [Point],
    stop: bool,
}

static EMPTY: &[Point; 0] = &[];

impl<'a> ThickSegmentIter<'a> {
    /// Create a new thick segments iterator.
    pub fn new(points: &'a [Point], width: u32, _stroke_offset: StrokeOffset) -> Self {
        // Fix stroke alignment to None. There are issues with degenerate joints when using
        // Inside/Outside stroke alignment on polylines, so this is disabled for now.
        let stroke_offset = StrokeOffset::None;

        let mut windows = points.windows(3);

        if let Some([start, mid, end]) = windows.next() {
            let start_join = LineJoin::start(*start, *mid, width, stroke_offset);
            let end_join = LineJoin::from_points(*start, *mid, *end, width, stroke_offset);

            Self {
                windows,
                start_join,
                end_join,
                width,
                stroke_offset,
                points,
                stop: false,
            }
        } else if let [start, end] = points {
            // Single line segment.
            let start_join = LineJoin::start(*start, *end, width, stroke_offset);
            let end_join = LineJoin::end(*start, *end, width, stroke_offset);

            Self {
                windows: EMPTY.windows(3),
                start_join,
                end_join,
                width,
                stroke_offset,
                points,
                stop: false,
            }
        } else {
            // Points must be at least 2 in length to make a polyline iterator out of.
            Self::empty()
        }
    }

    /// Empty
    fn empty() -> Self {
        Self {
            windows: EMPTY.windows(3),
            start_join: LineJoin::empty(),
            end_join: LineJoin::empty(),
            width: 0,
            stroke_offset: StrokeOffset::None,
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

        let segment = ThickSegment::new(self.start_join, self.end_join);

        self.start_join = self.end_join;

        if let Some([start, mid, end]) = self.windows.next() {
            self.end_join =
                LineJoin::from_points(*start, *mid, *end, self.width, self.stroke_offset);
        } else if self.end_join.kind != JoinKind::End {
            let start = *self.points.get(self.points.len() - 2)?;
            let end = *self.points.last()?;

            self.end_join = LineJoin::end(start, end, self.width, self.stroke_offset);
        } else {
            self.stop = true;
        }

        Some(segment)
    }
}
