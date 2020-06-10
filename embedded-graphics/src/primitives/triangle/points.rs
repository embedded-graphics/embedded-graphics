use crate::{
    geometry::Point,
    primitives::triangle::{scanline_iterator::ScanlineIterator, Triangle},
};

/// Iterator over all points inside the triangle.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Points(ScanlineIterator);

impl Points {
    pub(in crate::primitives) fn new(triangle: &Triangle) -> Self {
        Self(ScanlineIterator::new(triangle))
    }
}

impl Iterator for Points {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(_, p)| p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        drawable::Pixel, pixelcolor::BinaryColor, primitives::Primitive, style::PrimitiveStyle,
        transform::Transform,
    };

    #[test]
    fn points_iter() {
        let triangle = Triangle::new(Point::new(5, 10), Point::new(15, 20), Point::new(10, 15));

        let styled_points = triangle
            .clone()
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .into_iter()
            .map(|Pixel(p, _)| p);

        assert!(triangle.points().eq(styled_points));
    }

    #[test]
    fn off_screen_still_draws_points() {
        let off_screen = Triangle::new(Point::new(10, 10), Point::new(20, 20), Point::new(30, -30));
        let on_screen = off_screen.translate(Point::new(0, 35));

        assert!(off_screen
            .points()
            .eq(on_screen.points().map(|p| p - Point::new(0, 35))));
    }

    #[test]
    fn it_draws_unfilled_tri_line_y() {
        let mut tri = Triangle::new(Point::new(2, 2), Point::new(2, 4), Point::new(2, 4)).points();

        // Nodes are returned twice. first line a and b yield the same point.
        // After that line a ends where line c starts.
        assert_eq!(tri.next(), Some(Point::new(2, 2)));
        assert_eq!(tri.next(), Some(Point::new(2, 2)));
        assert_eq!(tri.next(), Some(Point::new(2, 3)));
        assert_eq!(tri.next(), Some(Point::new(2, 3)));
        assert_eq!(tri.next(), Some(Point::new(2, 4)));
        assert_eq!(tri.next(), Some(Point::new(2, 4)));
        assert_eq!(tri.next(), Some(Point::new(2, 4)));
        assert_eq!(tri.next(), None);
    }

    #[test]
    fn it_draws_unfilled_tri_line_x() {
        let mut tri = Triangle::new(Point::new(2, 2), Point::new(4, 2), Point::new(4, 2)).points();

        assert_eq!(tri.next(), Some(Point::new(2, 2)));
        assert_eq!(tri.next(), Some(Point::new(2, 2)));
        assert_eq!(tri.next(), Some(Point::new(3, 2)));
        assert_eq!(tri.next(), Some(Point::new(3, 2)));
        assert_eq!(tri.next(), Some(Point::new(4, 2)));
        assert_eq!(tri.next(), Some(Point::new(4, 2)));
        assert_eq!(tri.next(), Some(Point::new(4, 2)));
        assert_eq!(tri.next(), None);
    }
}
