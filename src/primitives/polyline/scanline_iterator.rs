//! Scanline iterator.

use core::ops::Range;

use crate::{
    pixelcolor::PixelColor,
    primitives::{
        common::Scanline, polyline::scanline_intersections::ScanlineIntersections, Polyline,
    },
    style::{PrimitiveStyle, Styled},
};

/// Iterate over every scanline in the polyline's bounding box.
///
/// Each scanline produces multiple actual `Line`s for each intersection of the thick polyline.
#[derive(Clone, Debug)]
pub struct ScanlineIterator<'a> {
    rows: Range<i32>,
    scanline_y: i32,
    intersections: ScanlineIntersections<'a>,
}

impl<'a> ScanlineIterator<'a> {
    /// New.
    pub fn new<C>(styled: &Styled<Polyline<'a>, PrimitiveStyle<C>>) -> Self
    where
        C: PixelColor,
    {
        debug_assert!(
            styled.style.stroke_width > 1,
            "Polyline ScanlineIterator should only be used for stroke widths greater than 1"
        );

        let mut rows = styled.original_bounding_box().rows();

        if let Some(scanline_y) = rows.next() {
            let intersections = ScanlineIntersections::new(
                styled.primitive.vertices,
                styled.style.stroke_width,
                scanline_y,
            );

            Self {
                rows,
                scanline_y,
                intersections,
            }
        } else {
            Self::empty()
        }
    }

    fn empty() -> Self {
        Self {
            rows: 0i32..0,
            scanline_y: 0,
            intersections: ScanlineIntersections::empty(),
        }
    }
}

impl<'a> Iterator for ScanlineIterator<'a> {
    type Item = Scanline;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(next) = self.intersections.next() {
                if !next.is_empty() {
                    break Some(next);
                }
            } else {
                self.scanline_y = self.rows.next()?;

                self.intersections.reset_with_new_scanline(self.scanline_y);
            }
        }
    }
}
