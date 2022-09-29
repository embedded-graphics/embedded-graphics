use crate::{
    geometry::Point,
    primitives::{
        line::{self, Line},
        polyline::Polyline,
        PointsIter,
    },
};

/// An iterator over all pixel positions on the polyline
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct Points<'a> {
    vertices: &'a [Point],
    translate: Point,
    segment_iter: line::Points,
}

impl<'a> Points<'a> {
    pub(in crate::primitives) fn new<'b>(polyline: &'b Polyline<'a>) -> Self
    where
        'a: 'b,
    {
        polyline
            .vertices
            .split_first()
            .and_then(|(start, rest)| {
                // Polyline is 2 or more vertices long, return an iterator for it
                rest.get(0).map(|end| Points {
                    vertices: rest,
                    translate: polyline.translate,
                    segment_iter: Line::new(*start + polyline.translate, *end + polyline.translate)
                        .points(),
                })
            })
            .unwrap_or_else(||
                // Polyline is less than 2 vertices long. Return a dummy iterator that will short
                // circuit
                Points {
                    vertices: &[],
                    translate: Point::zero(),
                    segment_iter: line::Points::empty(),
                })
    }
}

impl<'a> Iterator for Points<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(p) = self.segment_iter.next() {
            Some(p)
        } else {
            let (start, rest) = self.vertices.split_first()?;
            let end = rest.get(0)?;

            self.vertices = rest;

            self.segment_iter = Line::new(*start + self.translate, *end + self.translate).points();

            // Skip first point of next line, otherwise we overlap with the previous line
            self.nth(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::polyline::tests::SMALL;

    // Ensure that consecutive points are always different
    #[test]
    fn no_duplicate_points() {
        let expected: [Point; 14] = [
            Point::new(2, 5),
            Point::new(3, 4),
            Point::new(4, 3),
            Point::new(5, 2),
            Point::new(6, 3),
            Point::new(7, 3),
            Point::new(8, 4),
            Point::new(9, 4),
            Point::new(10, 5),
            Point::new(11, 4),
            Point::new(12, 4),
            Point::new(13, 3),
            Point::new(14, 3),
            Point::new(15, 2),
        ];

        assert!(Polyline::new(&SMALL).points().eq(expected.iter().copied()))
    }

    #[test]
    fn one_point() {
        let points = &[Point::zero()];

        let polyline = Polyline::new(points);

        assert!(polyline.points().eq(core::iter::empty()));
    }

    #[test]
    fn equal_points() {
        let points: [Point; 3] = [Point::new(2, 5), Point::new(2, 5), Point::new(2, 5)];

        assert!(Polyline::new(&points)
            .points()
            .eq(core::iter::once(Point::new(2, 5))));
    }
}
