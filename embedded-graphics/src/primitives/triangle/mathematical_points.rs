use crate::{
    geometry::{Dimensions, Point},
    primitives::{
        rectangle,
        triangle::{scanline_iterator::ScanlineIterator, Triangle},
        Primitive,
    },
};

/// Iterator over all points inside the triangle.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct MathematicalPoints {
    rect: rectangle::Points,
    triangle: Triangle,
}

impl MathematicalPoints {
    /// Make a new one
    pub fn new(triangle: &Triangle) -> Self {
        Self {
            rect: triangle.bounding_box().points(),
            triangle: *triangle,
        }
    }
}

impl Iterator for MathematicalPoints {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let Self { triangle, .. } = self;

        self.rect
            .find(|point| triangle.mathematical_contains(point))
    }
}
