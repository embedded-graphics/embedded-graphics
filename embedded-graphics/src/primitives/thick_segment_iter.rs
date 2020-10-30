//! Thick line joint.

use crate::{
    geometry::Point,
    primitives::line_join::{JoinKind, LineJoin},
    style::StrokeAlignment,
};

use super::thick_segment::ThickSegment;

/// Thick segments iterator.
///
/// Iterates over all line segments in the polyline, returning a 6-sided shape as a [`ThickSegment`]
/// for each segment. These are tesselated and are used to produce scanline intersections.
#[derive(Clone, Debug)]
pub(in crate::primitives) struct ThickSegmentIter<'a> {
    windows: core::slice::Windows<'a, Point>,
    start_join: LineJoin,
    end_join: LineJoin,
    width: u32,
    alignment: StrokeAlignment,
    points: &'a [Point],
    stop: bool,
}

static EMPTY: &[Point; 0] = &[];

impl<'a> ThickSegmentIter<'a> {
    /// Create a new thick segments iterator.
    pub fn new(points: &'a [Point], width: u32, _alignment: StrokeAlignment) -> Self {
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
                stop: false,
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
            alignment: StrokeAlignment::Center,
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
            self.end_join = LineJoin::from_points(*start, *mid, *end, self.width, self.alignment);
        } else if self.end_join.kind != JoinKind::End {
            let start = *self.points.get(self.points.len() - 2)?;
            let end = *self.points.last()?;

            self.end_join = LineJoin::end(start, end, self.width, self.alignment);
        } else {
            self.stop = true;
        }

        Some(segment)
    }
}
