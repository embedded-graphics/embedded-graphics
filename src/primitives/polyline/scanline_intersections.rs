//! An iterator over all line intersections with a given scanline.

use crate::{
    geometry::Point,
    primitives::common::{LineJoin, Scanline, StrokeOffset, ThickSegment},
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
    next_start_join: Option<LineJoin>,
    width: u32,
    scanline: Scanline,
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
            scanline: Scanline::new_empty(scanline_y),
        }
    }

    /// Empty scanline iterator.
    pub(in crate::primitives) fn empty() -> Self {
        Self {
            next_start_join: None,
            width: 0,
            points: EMPTY,
            remaining_points: EMPTY,
            scanline: Scanline::new_empty(0),
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
    type Item = Scanline;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(segment) = self.next_segment() {
            let next_scanline = segment.intersection(self.scanline.y);

            if !self.scanline.try_extend(&next_scanline) {
                let ret = self.scanline.clone();
                self.scanline = next_scanline;

                return Some(ret);
            }
        }

        // No more segments - return the final accumulated line.
        self.scanline.try_take()
    }
}
