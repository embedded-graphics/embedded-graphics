use crate::{
    geometry::Point,
    primitives::{
        line::{self, Line},
        triangle::{sort_two_yx, sort_yx, IterState, Triangle},
        Primitive,
    },
};
use core::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum PointType {
    Border,
    Inside,
}

/// Iterator over all points inside the triangle.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ScanlineIterator {
    /// Left-most edge of the triangle
    line_a: line::Points,

    /// Right-most edge of the triangle
    line_b: line::Points,

    /// Bottom edge of the triangle
    line_c: line::Points,
    cur_ac: Option<Point>,
    cur_b: Option<Point>,
    next_ac: Option<Point>,
    next_b: Option<Point>,
    x: i32,
}

impl ScanlineIterator {
    pub(in crate::primitives) fn new(triangle: &Triangle) -> Self {
        let (v1, v2, v3) = sort_yx(triangle.p1, triangle.p2, triangle.p3);

        let mut line_a = Line::new(v1, v2).points();
        let mut line_b = Line::new(v1, v3).points();
        let mut line_c = Line::new(v2, v3).points();

        // Skip first point of line C as this overlaps with the last point of line A
        line_c.next();

        let next_ac = line_a.next().or_else(|| line_c.next());
        let next_b = line_b.next();

        Self {
            line_a,
            line_b,
            line_c,
            cur_ac: None,
            cur_b: None,
            next_ac,
            next_b,
            x: 0,
        }
    }

    pub(in crate::primitives) fn empty() -> Self {
        Self {
            line_a: line::Points::empty(),
            line_b: line::Points::empty(),
            line_c: line::Points::empty(),
            cur_ac: None,
            cur_b: None,
            next_ac: None,
            next_b: None,
            x: 0,
        }
    }

    fn update_ac(&mut self) -> IterState {
        if let Some(ac) = self.next_ac {
            self.cur_ac = Some(ac);
            self.next_ac = self.line_a.next().or_else(|| self.line_c.next());
            self.x = 0;

            IterState::Border(ac)
        } else {
            IterState::None
        }
    }

    fn update_b(&mut self) -> IterState {
        if let Some(b) = self.next_b {
            self.cur_b = Some(b);
            self.next_b = self.line_b.next();
            self.x = 0;

            IterState::Border(b)
        } else {
            IterState::None
        }
    }

    pub(in crate::primitives::triangle) fn points(&mut self) -> IterState {
        match (self.cur_ac, self.cur_b) {
            // Point of ac line or b line is missing
            (None, _) => self.update_ac(),
            (_, None) => self.update_b(),
            // Both points are present
            (Some(ac), Some(b)) => {
                match (self.next_ac, self.next_b) {
                    (Some(n_ac), Some(n_b)) => {
                        // If y component differs, take new points from edge until both side have
                        // the same y
                        match n_ac.y.cmp(&n_b.y) {
                            Ordering::Less => self.update_ac(),
                            Ordering::Greater => self.update_b(),
                            Ordering::Equal => {
                                let (l, r) = sort_two_yx(n_ac, n_b);
                                IterState::LeftRight(l, r)
                            }
                        }
                    }
                    (None, Some(_)) => self.update_b(),
                    (Some(_), None) => self.update_ac(),
                    (None, None) => {
                        let (l, r) = sort_two_yx(ac, b);
                        IterState::LeftRight(l, r)
                    }
                }
            }
        }
    }
}

impl Iterator for ScanlineIterator {
    type Item = (PointType, Point);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.points() {
                IterState::Border(point) => {
                    // Skip overlapping left/right border points
                    if Some(point) != self.next_b {
                        // Draw edges of the triangle
                        self.x += 1;
                        return Some((PointType::Border, point));
                    }
                }
                IterState::LeftRight(l, r) => {
                    // Fill the space between the left and right points
                    if l.x + self.x < r.x {
                        let point = Point::new(l.x + self.x, l.y);
                        self.x += 1;
                        return Some((PointType::Inside, point));
                    } else if l.x + self.x >= r.x {
                        // We reached the right edge, move on to next row
                        self.cur_ac = None;
                        self.cur_b = None;
                    }
                }
                IterState::None => return None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        drawable::{Drawable, Pixel},
        geometry::Dimensions,
        mock_display::MockDisplay,
        pixel_iterator::IntoPixels,
        pixelcolor::BinaryColor,
        primitives::ContainsPoint,
        style::PrimitiveStyle,
        transform::Transform,
    };

    #[test]
    fn points_by_scanline_match_triangle_contains() {
        fn check(triangle: Triangle) {
            let mut mock_display1 = MockDisplay::new();
            let mut mock_display2 = MockDisplay::new();

            // FIXME: right now, ScanlineIterator does not guarantee unique points
            mock_display2.set_allow_overdraw(true);

            triangle
                .bounding_box()
                .points()
                .filter(|&p| triangle.contains(p))
                .for_each(|p| Pixel(p, BinaryColor::On).draw(&mut mock_display1).unwrap());

            ScanlineIterator::new(&triangle)
                .for_each(|(_, p)| Pixel(p, BinaryColor::On).draw(&mut mock_display2).unwrap());

            assert_eq!(mock_display1, mock_display2, "{:?}", triangle);
        }

        check(Triangle::new(
            Point::new(5, 10),
            Point::new(10, 15),
            Point::new(15, 10),
        ));
        check(Triangle::new(
            Point::new(5, 5),
            Point::new(15, 0),
            Point::new(10, 10),
        ));
        check(Triangle::new(
            Point::new(30, 0),
            Point::new(40, 10),
            Point::new(42, 10),
        ));
    }

    #[test]
    fn points_iter() {
        let triangle = Triangle::new(Point::new(5, 10), Point::new(15, 20), Point::new(10, 15));

        let styled_points = triangle
            .clone()
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .into_pixels()
            .map(|Pixel(p, _)| p);

        assert!(triangle.points().eq(styled_points));
    }

    #[test]
    fn off_screen_still_draws_points() {
        let off_screen = Triangle::new(Point::new(10, 10), Point::new(20, 20), Point::new(30, -30));
        let on_screen = off_screen.translate(Point::new(0, 35));

        ScanlineIterator::new(&off_screen)
            .map(|(_, p)| p)
            .eq(on_screen.points().map(|p| p - Point::new(0, 35)));
    }
}
