//! Scanline iterator.

use crate::{
    geometry::Dimensions,
    pixelcolor::PixelColor,
    primitives::{polyline::scanline_intersections::ScanlineIntersections, Line, Polyline},
    style::{PrimitiveStyle, Styled},
};

/// Iterate over every scanline in the polyline's bounding box.
///
/// Each scanline produces multiple actual `Line`s for each intersection of the thick polyline.
#[derive(Clone, Debug)]
pub struct ScanlineIterator<'a> {
    scanline_y: i32,
    scanline_limit: i32,
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

        let bb = styled.bounding_box();

        let scanline_y = bb.top_left.y;
        let scanline_limit = scanline_y + bb.size.height as i32;

        let intersections = ScanlineIntersections::new(
            styled.primitive.vertices,
            styled.style.stroke_width,
            scanline_y,
        );

        Self {
            scanline_y,
            scanline_limit,
            intersections,
        }
    }
}

impl<'a> Iterator for ScanlineIterator<'a> {
    type Item = Line;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.scanline_y >= self.scanline_limit {
                break None;
            }

            if let Some(next) = self.intersections.next() {
                break Some(next);
            } else {
                self.scanline_y += 1;

                self.intersections.reset_with_new_scanline(self.scanline_y);
            }
        }
    }
}
