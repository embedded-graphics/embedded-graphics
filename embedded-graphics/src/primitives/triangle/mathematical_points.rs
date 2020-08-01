use crate::{
    geometry::{Dimensions, Point},
    primitives::{rectangle, triangle::Triangle, Primitive},
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

    /// Empty variant
    pub fn empty() -> Self {
        Self {
            rect: rectangle::Points::empty(),
            triangle: Triangle::empty(),
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
