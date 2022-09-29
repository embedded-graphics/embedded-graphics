use core::ops::Range;

use crate::{
    geometry::{Dimensions, Point},
    primitives::{
        common::Scanline,
        ellipse::{Ellipse, EllipseContains},
    },
};

/// Iterator over all points inside the ellipse
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
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
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct Scanlines {
    rows: Range<i32>,
    columns: Range<i32>,
    pub(super) center_2x: Point,
    ellipse_contains: EllipseContains,
}

impl Scanlines {
    pub fn new(ellipse: &Ellipse) -> Self {
        let bounding_box = ellipse.bounding_box();

        Self {
            rows: bounding_box.rows(),
            columns: bounding_box.columns(),
            center_2x: ellipse.center_2x(),
            ellipse_contains: EllipseContains::new(ellipse.size),
        }
    }
}

impl Iterator for Scanlines {
    type Item = Scanline;

    fn next(&mut self) -> Option<Self::Item> {
        let y = self.rows.next()?;

        let scaled_y = y * 2 - self.center_2x.y;

        self.columns
            .clone()
            // Find the first pixel that is inside the ellipse.
            .find(|x| {
                self.ellipse_contains
                    .contains(Point::new(*x * 2 - self.center_2x.x, scaled_y))
            })
            // Shorten the right side of the scanline by the same amount as the left side.
            .map(|x| Scanline::new(y, x..self.columns.end - (x - self.columns.start)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::Size,
        primitives::{Circle, PointsIter},
    };

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
