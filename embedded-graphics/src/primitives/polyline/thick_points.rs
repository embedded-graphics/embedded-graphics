use crate::{
    prelude::Point,
    primitives::{
        polyline::triangle_iterator::TriangleIterator,
        triangle::{self, FillScanlineIterator, Points, Triangle},
        ContainsPoint, Primitive,
    },
    style::StrokeAlignment,
};

// TODO: Generalise name, move into more common folder path
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(in crate::primitives) struct ThickPoints<'a> {
    triangle_iter: TriangleIterator<'a>,
    prev_triangle: Option<Triangle>,
    prev_triangle2: Option<Triangle>,
    prev_triangle3: Option<Triangle>,
    triangle: Triangle,
    points_iter: Points,
}

impl<'a> ThickPoints<'a> {
    pub fn new(points: &'a [Point], width: u32, alignment: StrokeAlignment) -> Self {
        if points.len() < 2 {
            Self::empty()
        } else {
            let mut triangle_iter = TriangleIterator::new(points, width, alignment);

            let triangle = triangle_iter.next().unwrap_or_else(Triangle::empty);
            let points_iter = triangle.points();

            Self {
                prev_triangle: None,
                prev_triangle2: None,
                prev_triangle3: None,
                triangle,
                triangle_iter,
                points_iter,
            }
        }
    }

    pub fn empty() -> Self {
        Self {
            prev_triangle: None,
            prev_triangle2: None,
            prev_triangle3: None,
            triangle: Triangle::empty(),
            triangle_iter: TriangleIterator::empty(),
            points_iter: Triangle::empty().points(),
        }
    }
}

impl<'a> Iterator for ThickPoints<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(point) = self.points_iter.next() {
                if !ContainsPoint::contains(&self.prev_triangle, point)
                    && !ContainsPoint::contains(&self.prev_triangle2, point)
                    && !ContainsPoint::contains(&self.prev_triangle3, point)
                {
                    return Some(point);
                }
            } else {
                self.prev_triangle3 = self.prev_triangle2;
                self.prev_triangle2 = self.prev_triangle;
                self.prev_triangle = Some(self.triangle);
                self.triangle = self.triangle_iter.next()?;
                self.points_iter = self.triangle.points();
            }
        }
    }
}
