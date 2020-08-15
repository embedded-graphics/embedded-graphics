use super::triangle_iterator::Item;
use crate::{
    prelude::Point,
    primitives::{
        polyline::triangle_iterator::TriangleIterator, trapezium::TrapeziumIterator,
        triangle::MathematicalPoints,
    },
    style::StrokeAlignment,
};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum PointsIter {
    Trapezium(TrapeziumIterator),
    Triangle(MathematicalPoints),
}

// TODO: Generalise name, move into more common folder path
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(in crate::primitives) struct ThickPoints<'a> {
    triangle_iter: TriangleIterator<'a>,
    points_iter: PointsIter,
}

impl<'a> ThickPoints<'a> {
    pub fn new(points: &'a [Point], width: u32, alignment: StrokeAlignment) -> Self {
        if points.len() < 2 {
            Self::empty()
        } else {
            let mut triangle_iter = TriangleIterator::new(points, width, alignment);

            let points_iter = triangle_iter
                .next()
                .map(|t| match t {
                    Item::Trapezium(t) => PointsIter::Trapezium(TrapeziumIterator::new(t)),
                    Item::Triangle(t) => PointsIter::Triangle(t.mathematical_points()),
                })
                .unwrap_or_else(|| PointsIter::Triangle(MathematicalPoints::empty()));

            Self {
                triangle_iter,
                points_iter,
            }
        }
    }

    pub fn empty() -> Self {
        Self {
            triangle_iter: TriangleIterator::empty(),
            points_iter: PointsIter::Triangle(MathematicalPoints::empty()),
        }
    }
}

impl<'a> Iterator for ThickPoints<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(point) = match self.points_iter {
            PointsIter::Trapezium(ref mut it) => it.next(),
            PointsIter::Triangle(ref mut it) => it.next(),
        } {
            Some(point)
        } else {
            self.points_iter = self.triangle_iter.next().map(|t| match t {
                Item::Trapezium(t) => PointsIter::Trapezium(TrapeziumIterator::new(t)),
                Item::Triangle(t) => PointsIter::Triangle(t.mathematical_points()),
            })?;

            self.next()
        }
    }
}
