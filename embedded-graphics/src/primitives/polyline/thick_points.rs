use crate::{
    prelude::Point,
    primitives::{
        polyline::triangle_iterator::TriangleIterator,
        triangle::{Points, Triangle},
        ContainsPoint, Primitive,
    },
    style::StrokeAlignment,
};

// TODO: Generalise name, move into more common folder path
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(in crate::primitives) struct ThickPoints<'a> {
    triangle_iter: TriangleIterator<'a>,
    prev_triangle: Triangle,
    prev_triangle2: Triangle,
    prev_triangle3: Triangle,
    triangle: Triangle,
    points_iter: Points,
}

impl<'a> ThickPoints<'a> {
    pub fn new(points: &'a [Point], width: u32, alignment: StrokeAlignment) -> Self {
        let mut triangle_iter = TriangleIterator::new(points, width, alignment);

        let triangle = triangle_iter.next().unwrap_or_else(Triangle::empty);
        let points_iter = triangle.points();

        Self {
            prev_triangle: Triangle::empty(),
            prev_triangle2: Triangle::empty(),
            prev_triangle3: Triangle::empty(),
            triangle,
            triangle_iter,
            points_iter,
        }
    }

    pub fn empty() -> Self {
        Self {
            prev_triangle: Triangle::empty(),
            prev_triangle2: Triangle::empty(),
            prev_triangle3: Triangle::empty(),
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
                // We need to check previous triangles so we don't overdraw them
                // TODO: depending on the joint, not all 3 checks are necessary - optimize this
                if !ContainsPoint::contains(&self.prev_triangle, point)
                    && !ContainsPoint::contains(&self.prev_triangle2, point)
                    && !ContainsPoint::contains(&self.prev_triangle3, point)
                {
                    return Some(point);
                }
            } else {
                self.prev_triangle3 = self.prev_triangle2;
                self.prev_triangle2 = self.prev_triangle;
                self.prev_triangle = self.triangle;
                self.triangle = self.triangle_iter.next()?;
                self.points_iter = self.triangle.points();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        drawable::Drawable, mock_display::MockDisplay, pixelcolor::BinaryColor,
        primitives::Polyline, style::PrimitiveStyle,
    };

    #[test]
    fn test_no_overwrite_on_sharp_corner() {
        let mut mock_display = MockDisplay::new();

        Polyline::new(&[Point::new(35, 5), Point::new(25, 35), Point::new(15, 5)])
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 10))
            .draw(&mut mock_display)
            .unwrap();

        let mut mock_display = MockDisplay::new();

        Polyline::new(&[Point::new(15, 5), Point::new(25, 35), Point::new(35, 5)])
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 10))
            .draw(&mut mock_display)
            .unwrap();
    }
}
