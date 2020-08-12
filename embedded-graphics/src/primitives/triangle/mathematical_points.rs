use crate::{
    geometry::{Dimensions, Point},
    primitives::{
        line, rectangle,
        triangle::{self, Triangle},
        Primitive,
    },
    transform::Transform,
};
use line::Line;

// TODO: Pub in crate primitives
/// Iterator over all points inside the triangle.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct MathematicalPoints {
    lines: [Line; 3],
    right: Point,
    pos: Point,
    scanline: Line,
}

impl MathematicalPoints {
    /// Make a new one
    pub fn new(triangle: &Triangle) -> Self {
        // TODO: Add method to `Triangle` to get YX sorted points
        let (p1, p2, p3) = triangle::sort_yx(triangle.p1, triangle.p2, triangle.p3);

        let lines = [Line::new(p1, p2), Line::new(p1, p3), Line::new(p2, p3)];

        let bb = triangle.bounding_box();

        let scanline = Line::new(bb.top_left, bb.top_left + bb.size.x_axis());

        if let Some((pos, right)) = Self::intersections(&lines, &scanline) {
            Self {
                lines,
                right,
                pos,
                scanline,
            }
        } else {
            Self::empty()
        }
    }

    fn intersections(lines: &[Line], scanline: &Line) -> Option<(Point, Point)> {
        let mut intersections = lines
            .iter()
            .filter_map(|l| l.segment_intersection_point(&scanline))
            .take(2);

        if let (Some(a), Some(b)) = (intersections.next(), intersections.next()) {
            // Sort by increasing X order so fill line always travels left -> right
            let (a, b) = if a.x > b.x { (b, a) } else { (a, b) };

            Some((a, b))
        } else {
            None
        }
    }

    /// Empty variant
    pub fn empty() -> Self {
        let l = Line::new(Point::zero(), Point::zero());

        Self {
            lines: [l, l, l],
            pos: Point::zero(),
            right: Point::zero(),
            scanline: Line::new(Point::zero(), Point::zero()),
        }
    }
}

impl Iterator for MathematicalPoints {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let point = self.pos;

        self.pos.x += 1;

        // Reached end of line, step down to next line
        if self.pos.x > self.right.x {
            self.scanline.translate_mut(Point::new(0, 1));

            let (new_pos, new_right) = Self::intersections(&self.lines, &self.scanline)?;

            self.pos = new_pos;
            self.right = new_right;
        }

        Some(point)
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
