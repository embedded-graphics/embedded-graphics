use crate::{
    prelude::Point,
    primitives::{polyline::triangle_iterator::TriangleIterator, triangle::MathematicalPoints},
    style::StrokeAlignment,
};

// TODO: Generalise name, move into more common folder path
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(in crate::primitives) struct ThickPoints<'a> {
    triangle_iter: TriangleIterator<'a>,
    points_iter: MathematicalPoints,
}

impl<'a> ThickPoints<'a> {
    pub fn new(points: &'a [Point], width: u32, alignment: StrokeAlignment) -> Self {
        if points.len() < 2 {
            Self::empty()
        } else {
            let mut triangle_iter = TriangleIterator::new(points, width, alignment);

            let points_iter = triangle_iter
                .next()
                .map(|t| t.mathematical_points())
                .unwrap_or_else(MathematicalPoints::empty);

            Self {
                triangle_iter,
                points_iter,
            }
        }
    }

    pub fn empty() -> Self {
        Self {
            triangle_iter: TriangleIterator::empty(),
            points_iter: MathematicalPoints::empty(),
        }
    }
}

impl<'a> Iterator for ThickPoints<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(point) = self.points_iter.next() {
            Some(point)
        } else {
            self.points_iter = self.triangle_iter.next().map(|t| t.mathematical_points())?;

            self.next()
        }
    }
}
