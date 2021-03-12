use core::ops::Range;

use crate::{
    geometry::{Dimensions, Point, PointExt},
    primitives::{circle::Circle, common::Scanline},
};

/// Iterator over all points inside the circle.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Points {
    scanlines: Scanlines,
    current_scanline: Scanline,
}

impl Points {
    pub(in crate::primitives) fn new(circle: &Circle) -> Self {
        Self {
            scanlines: Scanlines::new(circle),
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
    threshold: u32,
}

impl Scanlines {
    pub fn new(circle: &Circle) -> Self {
        let bounding_box = circle.bounding_box();

        Self {
            rows: bounding_box.rows(),
            columns: bounding_box.columns(),
            center_2x: circle.center_2x(),
            threshold: circle.threshold(),
        }
    }
}

impl Iterator for Scanlines {
    type Item = Scanline;

    fn next(&mut self) -> Option<Self::Item> {
        let y = self.rows.next()?;

        self.columns
            .clone()
            // find first pixel that is inside the threshold
            .find(|x| {
                let delta = Point::new(*x, y) * 2 - self.center_2x;
                (delta.length_squared() as u32) < self.threshold
            })
            // shorten the scanline by right side of the same amount as the left side
            .map(|x| Scanline::new(y, x..self.columns.end - (x - self.columns.start)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::Point, mock_display::MockDisplay, pixelcolor::BinaryColor, primitives::PointsIter,
    };

    fn test_circle(diameter: u32, pattern: &[&str]) {
        let display = MockDisplay::from_points(
            Circle::new(Point::new(0, 0), diameter).points(),
            BinaryColor::On,
        );

        display.assert_pattern(pattern);
    }

    #[test]
    fn circle_1() {
        #[rustfmt::skip]
        test_circle(1, &[
            "#",
        ],);
    }

    #[test]
    fn circle_2() {
        #[rustfmt::skip]
        test_circle(2, &[
            "##",
            "##",
        ],);
    }

    #[test]
    fn circle_3() {
        #[rustfmt::skip]
        test_circle(3, &[
            " # ",
            "###",
            " # ",
        ],);
    }

    #[test]
    fn circle_4() {
        #[rustfmt::skip]
        test_circle(4, &[
            " ## ",
            "####",
            "####",
            " ## ",
        ],);
    }

    #[test]
    fn circle_5() {
        #[rustfmt::skip]
        test_circle(5, &[
            " ### ",
            "#####",
            "#####",
            "#####",
            " ### ",
        ],);
    }

    #[test]
    fn circle_6() {
        #[rustfmt::skip]
        test_circle(6, &[
            " #### ",
            "######",
            "######",
            "######",
            "######",
            " #### ",
        ],);
    }

    #[test]
    fn circle_7() {
        #[rustfmt::skip]
        test_circle(7, &[
            "  ###  ",
            " ##### ",
            "#######",
            "#######",
            "#######",
            " ##### ",
            "  ###  ",
        ],);
    }

    #[test]
    fn circle_8() {
        #[rustfmt::skip]
        test_circle(8, &[
            "  ####  ",
            " ###### ",
            "########",
            "########",
            "########",
            "########",
            " ###### ",
            "  ####  ",
        ],);
    }

    #[test]
    fn circle_9() {
        #[rustfmt::skip]
        test_circle(9, &[
            "  #####  ",
            " ####### ",
            "#########",
            "#########",
            "#########",
            "#########",
            "#########",
            " ####### ",
            "  #####  ",
        ],);
    }
}
