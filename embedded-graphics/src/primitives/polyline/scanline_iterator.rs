//! Scanline iterator.

use crate::{
    geometry::Dimensions,
    pixelcolor::PixelColor,
    primitives::{polyline::scanline_intersections::ScanlineIntersections, Line, Polyline},
    style::{PrimitiveStyle, Styled},
};

/// Iterate over every scanline in the polyline's bounding box.
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
        let bb = styled.bounding_box();

        let scanline_y = bb.top_left.y;
        // FIXME: Return empty version of this iterator if this is `None`.
        let scanline_limit = bb.bottom_right().unwrap().y;

        let intersections = ScanlineIntersections::new(
            styled.primitive.vertices,
            styled.style.stroke_width,
            styled.style.stroke_alignment,
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
        if self.scanline_y == self.scanline_limit {
            return None;
        }

        if let Some(next) = self.intersections.next() {
            Some(next)
        } else {
            self.scanline_y += 1;

            self.intersections.reset_with_new_scanline(self.scanline_y);

            self.next()
        }
    }
}
