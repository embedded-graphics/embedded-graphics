use crate::{
    prelude::Point,
    primitives::{
        polyline::triangle_iterator::TriangleIterator,
        triangle::{MathematicalPoints, Triangle},
        ContainsPoint,
    },
    style::StrokeAlignment,
};

// TODO: Generalise name, move into more common folder path
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(in crate::primitives) struct ThickPoints<'a> {
    triangle_iter: TriangleIterator<'a>,
    prev_triangle: Option<Triangle>,
    triangle: Triangle,
    points_iter: MathematicalPoints,
}

impl<'a> ThickPoints<'a> {
    pub fn new(points: &'a [Point], width: u32, alignment: StrokeAlignment) -> Self {
        let mut triangle_iter = TriangleIterator::new(points, width, alignment);

            let triangle = triangle_iter.next().unwrap_or_else(Triangle::empty);
            let points_iter = triangle.mathematical_points();

            Self {
                prev_triangle: None,
                triangle,
                triangle_iter,
                points_iter,
            }
        }
    }

    pub fn empty() -> Self {
        Self {
            prev_triangle: None,
            triangle: Triangle::empty(),
            triangle_iter: TriangleIterator::empty(),
            points_iter: MathematicalPoints::empty(),
        }
    }
}

impl<'a> Iterator for ThickPoints<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(point) = self.points_iter.next() {
                if !ContainsPoint::contains(&self.prev_triangle, point) {
                    return Some(point);
                }
            } else {
                self.prev_triangle = Some(self.triangle);
                self.triangle = self.triangle_iter.next()?;
                self.points_iter = self.triangle.mathematical_points();
            }
        }
    }
}
