use core::ops::Range;

use crate::{
    geometry::{Dimensions, Point, Size},
    primitives::{
        common::Scanline,
        ellipse::{compute_threshold, is_point_inside_ellipse, Ellipse},
    },
};

/// Iterator over all points inside the ellipse
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Points {
    scanlines: Scanlines,
    current_scanline: Scanline,
}

impl Points {
    pub(in crate::primitives) fn new(ellipse: &Ellipse) -> Self {
        Self {
            scanlines: Scanlines::new(ellipse),
            current_scanline: Scanline::new_empty(0),
        }
    }
}

impl Iterator for Points {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_scanline.next().or_else(|| {
            self.current_scanline = self.scanlines.next()?;
            self.current_scanline.next()
        })
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Scanlines {
    rows: Range<i32>,
    columns: Range<i32>,
    pub(super) center_2x: Point,
    size_sq: Size,
    threshold: u32,
}

impl Scanlines {
    pub fn new(ellipse: &Ellipse) -> Self {
        let bounding_box = ellipse.bounding_box();
        let (size_sq, threshold) = compute_threshold(ellipse.size);

        Self {
            rows: bounding_box.rows(),
            columns: bounding_box.columns(),
            center_2x: ellipse.center_2x(),
            size_sq,
            threshold,
        }
    }
}

impl Iterator for Scanlines {
    type Item = Scanline;

    fn next(&mut self) -> Option<Self::Item> {
        let y = self.rows.next()?;

        self.columns
            .clone()
            // find first pixel that is inside the ellipse
            .find(|x| {
                is_point_inside_ellipse(
                    self.size_sq,
                    Point::new(*x, y) * 2 - self.center_2x,
                    self.threshold,
                )
            })
            // shorten the scanline by right side of the same amount as the left side
            .map(|x| Scanline::new(y, x..self.columns.end - (x - self.columns.start)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::{Circle, PointsIter};

    #[test]
    fn matches_circles_points() {
        for diameter in 0..50 {
            let circle_points = Circle::new(Point::new(0, 0), diameter).points();

            let ellipse_points =
                Ellipse::new(Point::new(0, 0), Size::new(diameter, diameter)).points();

            assert!(circle_points.eq(ellipse_points), "diameter = {}", diameter);
        }
    }
}
