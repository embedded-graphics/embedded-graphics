use crate::geometry::Dimensions;
use crate::geometry::{Point, Size};
use crate::primitives::ellipse::Ellipse;
use crate::primitives::rectangle::{self, Rectangle};
use crate::primitives::ContainsPoint;
use crate::primitives::Primitive;

/// A quadrant around an origin
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Quadrant {
    TopLeft = 0,
    TopRight = 1,
    BottomRight = 2,
    BottomLeft = 3,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct EllipseQuadrant {
    pub radius: Size,
    pub quadrant: Quadrant,
    pub bounding_box: Rectangle,
    pub ellipse: Ellipse,
}

impl EllipseQuadrant {
    pub fn new(top_left: Point, radius: Size, quadrant: Quadrant) -> Self {
        let ellipse_top_left = match quadrant {
            Quadrant::TopLeft => top_left,
            Quadrant::TopRight => top_left - radius.x_axis(),
            Quadrant::BottomRight => top_left - radius,
            Quadrant::BottomLeft => top_left - radius.y_axis(),
        };

        Self {
            radius,
            ellipse: Ellipse::new(ellipse_top_left, radius * 2),
            quadrant,
            bounding_box: Rectangle::new(top_left, radius),
        }
    }
}

impl Dimensions for EllipseQuadrant {
    fn bounding_box(&self) -> Rectangle {
        // TODO: Should we just calculate this on the fly?
        self.bounding_box
    }
}

impl Primitive for EllipseQuadrant {
    type PointsIter = Points;

    fn points(&self) -> Self::PointsIter {
        Points::new(self)
    }
}

impl ContainsPoint for EllipseQuadrant {
    fn contains(&self, point: Point) -> bool {
        // Broad phase: check if point is inside bounding box
        if !self.bounding_box.contains(point) {
            return false;
        }

        // Narrow phase: check if point is within ellipse. The bounding box check above constrains
        // this check to only the current quadrant.
        self.ellipse.contains(point)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct Points {
    ellipse: Ellipse,
    iter: rectangle::Points,
}

impl Points {
    pub fn new(ellipse_quadrant: &EllipseQuadrant) -> Self {
        Self {
            ellipse: ellipse_quadrant.ellipse,
            iter: ellipse_quadrant.bounding_box().points(),
        }
    }

    pub(crate) const fn empty() -> Self {
        Self {
            ellipse: Ellipse::new(Point::zero(), Size::zero()),
            iter: rectangle::Points::empty(),
        }
    }
}

impl Iterator for Points {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        for p in &mut self.iter {
            if self.ellipse.contains(p) {
                return Some(p);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{drawable::Pixel, mock_display::MockDisplay, pixelcolor::BinaryColor, prelude::*};

    #[test]
    fn quadrants_even_size() {
        let cases = [
            (
                Quadrant::TopLeft,
                &[
                    "      ####",
                    "   #######",
                    " #########",
                    "##########",
                    "##########",
                ],
            ),
            (
                Quadrant::TopRight,
                &[
                    "####      ",
                    "#######   ",
                    "######### ",
                    "##########",
                    "##########",
                ],
            ),
            (
                Quadrant::BottomRight,
                &[
                    "##########",
                    "##########",
                    "######### ",
                    "#######   ",
                    "####      ",
                ],
            ),
            (
                Quadrant::BottomLeft,
                &[
                    "##########",
                    "##########",
                    " #########",
                    "   #######",
                    "      ####",
                ],
            ),
        ];

        for (quadrant, expected) in cases.iter() {
            let mut display = MockDisplay::new();

            EllipseQuadrant::new(Point::new(0, 0), Size::new(10, 5), *quadrant)
                .points()
                .map(|p| Pixel(p, BinaryColor::On))
                .draw(&mut display)
                .unwrap();

            assert_eq!(display, MockDisplay::from_pattern(*expected));
        }
    }

    #[test]
    fn quadrants_equal_even_ellipse() {
        let mut display = MockDisplay::new();

        let radius = Size::new(10, 5);
        let top_left = Point::new(0, 0);

        EllipseQuadrant::new(top_left, radius, Quadrant::TopLeft)
            .points()
            .chain(
                EllipseQuadrant::new(top_left + radius.x_axis(), radius, Quadrant::TopRight)
                    .points(),
            )
            .chain(EllipseQuadrant::new(top_left + radius, radius, Quadrant::BottomRight).points())
            .chain(
                EllipseQuadrant::new(top_left + radius.y_axis(), radius, Quadrant::BottomLeft)
                    .points(),
            )
            .map(|p| Pixel(p, BinaryColor::On))
            .draw(&mut display)
            .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "      ########      ",
                "   ##############   ",
                " ################## ",
                "####################",
                "####################",
                "####################",
                "####################",
                " ################## ",
                "   ##############   ",
                "      ########      ",
            ])
        );
    }

    #[test]
    fn quadrants_equal_odd_ellipse() {
        let mut display = MockDisplay::new();

        let radius = Size::new(7, 9);
        let top_left = Point::new(0, 0);

        EllipseQuadrant::new(top_left, radius, Quadrant::TopLeft)
            .points()
            .chain(
                EllipseQuadrant::new(top_left + radius.x_axis(), radius, Quadrant::TopRight)
                    .points(),
            )
            .chain(EllipseQuadrant::new(top_left + radius, radius, Quadrant::BottomRight).points())
            .chain(
                EllipseQuadrant::new(top_left + radius.y_axis(), radius, Quadrant::BottomLeft)
                    .points(),
            )
            .map(|p| Pixel(p, BinaryColor::On))
            .draw(&mut display)
            .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "     ####     ",
                "   ########   ",
                "  ##########  ",
                " ############ ",
                " ############ ",
                " ############ ",
                "##############",
                "##############",
                "##############",
                "##############",
                "##############",
                "##############",
                " ############ ",
                " ############ ",
                " ############ ",
                "  ##########  ",
                "   ########   ",
                "     ####     ",
            ])
        );
    }
}
