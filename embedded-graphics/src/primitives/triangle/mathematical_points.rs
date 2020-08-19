use crate::{
    geometry::{Dimensions, Point},
    primitives::{rectangle, triangle::Triangle, Primitive},
};

// // TODO: Pub in crate primitives
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

// /// TESTING VERSION: outputs wireframe triangles so it's easier to debug.
// #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
// pub struct MathematicalPoints {
//     line1: line::Points,
//     line2: line::Points,
//     line3: line::Points,
// }

// impl MathematicalPoints {
//     /// Make a new one
//     pub fn new(triangle: &Triangle) -> Self {
//         Self {
//             line1: Line::new(triangle.p1, triangle.p2).points(),
//             line2: Line::new(triangle.p2, triangle.p3).points(),
//             line3: Line::new(triangle.p3, triangle.p1).points(),
//         }
//     }

//     /// Empty variant
//     pub fn empty() -> Self {
//         Self {
//             line1: line::Points::empty(),
//             line2: line::Points::empty(),
//             line3: line::Points::empty(),
//         }
//     }
// }

// impl Iterator for MathematicalPoints {
//     type Item = Point;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.line1
//             .next()
//             .or_else(|| self.line2.next())
//             .or_else(|| self.line3.next())
//     }
// }
