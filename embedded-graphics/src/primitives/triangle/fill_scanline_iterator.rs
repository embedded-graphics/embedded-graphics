//!
use crate::{
    geometry::Point,
    primitives::{
        line::{self, Line},
        triangle::{sort_yx, Triangle},
        Primitive,
    },
};

/// Iterator over all points inside the triangle.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct FillScanlineIterator {
    /// Left-most edge of the triangle
    line_a: line::Points,

    /// Right-most edge of the triangle
    line_b: line::Points,

    /// Bottom edge of the triangle
    line_c: line::Points,

    /// Horizontal line
    scan_points: line::Points,

    /// y
    current_y: i32,
}

impl FillScanlineIterator {
    pub(in crate::primitives) fn new(triangle: &Triangle) -> Self {

        let (v1, v2, v3) = sort_yx(triangle.p1, triangle.p2, triangle.p3);

        let mut line_a = if v1.y == v2.y { line::Points::empty() } else { Line::new(v1, v2).points() };
        let mut line_b = if v2.y == v3.y { line::Points::empty() } else { Line::new(v2, v3).points() };
        let mut line_c = Line::new(v1, v3).points();

        let self_ = Self {
            line_a,
            line_b,
            line_c,
            scan_points: line::Points::empty(),
            current_y: v1.y-1,
        };

        self_
    }

    fn update_ab(&mut self) -> Option<Point> {
        while let Some(point) = self.line_a.next().or_else(|| self.line_b.next()) {
            if point.y != self.current_y {
                self.current_y = point.y;
                return Some(point);
            }
        }
        None
    }

    fn update_c(&mut self) -> Option<Point> {
        while let Some(point) = self.line_c.next() {
            if point.y == self.current_y {
                return Some(point);
            }
        }
        None
    }

    fn next_scanline(&mut self) -> bool {
        let a = self.update_ab();
        let b = self.update_c();

        if let (Some(a), Some(b)) = (a, b) {
            self.scan_points = Line::new(a, b).points();
            true
        } else {
            false
        }
    }
}

impl Iterator for FillScanlineIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(point) = self.scan_points.next() {
                return Some(point);
            } else {
                if !self.next_scanline() {
                    return None;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        drawable::{Drawable, Pixel}, geometry::Dimensions, pixel_iterator::IntoPixels, pixelcolor::BinaryColor,
        primitives::ContainsPoint, style::PrimitiveStyle, transform::Transform,
        mock_display::MockDisplay
    };

    #[test]
    fn points_are_part_of_triangle() {
        fn check(triangle: Triangle) {
            let mut mock_display1 = MockDisplay::new();
            let mut mock_display2 = MockDisplay::new();

            triangle
                .bounding_box()
                .points()
                .filter(|&p| triangle.contains(p))
                .for_each(|p| Pixel(p, BinaryColor::On).draw(&mut mock_display1).unwrap());

            triangle
                .all_points()
                .for_each(|p| Pixel(p, BinaryColor::On).draw(&mut mock_display2).unwrap());

            assert_eq!(mock_display1, mock_display2, "{:?}", triangle);
        }

        check(Triangle::new(Point::new(5, 10), Point::new(10, 15), Point::new(15, 10)));
        check(Triangle::new(Point::new(5, 5), Point::new(15, 0), Point::new(10, 10)));
        check(Triangle::new(Point::new(30, 0), Point::new(40, 10), Point::new(42, 10)));

        /*
        for x in 10..15 {
            for y in 10..15 {
                for z in 10..15 {
                    for w in 10..15 {
                        check(Triangle::new(Point::new(30, 30), Point::new(30, 30) + Point::new(x, y), Point::new(30, 30)+ Point::new(z, w)));
                    }
                }
            }
        }
        */
    }

    #[test]
    fn off_screen_still_draws_points() {
        let off_screen = Triangle::new(Point::new(10, 10), Point::new(20, 20), Point::new(30, -30));
        let on_screen = off_screen.translate(Point::new(0, 35));

        assert!(off_screen
            .points()
            .eq(on_screen.all_points().map(|p| p - Point::new(0, 35))));
    }
}
