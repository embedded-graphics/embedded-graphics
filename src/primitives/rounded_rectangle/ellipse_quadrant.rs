use crate::{
    geometry::{Dimensions, Point, Size},
    primitives::{
        ellipse::{self, EllipseContains},
        rectangle::{self, Rectangle},
        ContainsPoint, PointsIter, Primitive,
    },
};

/// A quadrant around an origin
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Quadrant {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(in crate::primitives) struct EllipseQuadrant {
    bounding_box: Rectangle,
    center_2x: Point,
    ellipse: EllipseContains,
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
            bounding_box: Rectangle::new(top_left, radius),
            center_2x: ellipse::center_2x(ellipse_top_left, radius * 2),
            ellipse: EllipseContains::new(radius * 2),
        }
    }
}

impl Dimensions for EllipseQuadrant {
    fn bounding_box(&self) -> Rectangle {
        self.bounding_box
    }
}

impl Primitive for EllipseQuadrant {}

impl PointsIter for EllipseQuadrant {
    type Iter = Points;

    fn points(&self) -> Self::Iter {
        Points::new(self)
    }
}

impl ContainsPoint for EllipseQuadrant {
    fn contains(&self, point: Point) -> bool {
        self.ellipse.contains(point * 2 - self.center_2x)
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub(in crate::primitives) struct Points {
    iter: rectangle::Points,
    center_2x: Point,
    ellipse: EllipseContains,
}

impl Points {
    pub fn new(ellipse_quadrant: &EllipseQuadrant) -> Self {
        Self {
            iter: ellipse_quadrant.bounding_box().points(),
            center_2x: ellipse_quadrant.center_2x,
            ellipse: ellipse_quadrant.ellipse,
        }
    }

    pub(in crate::primitives) const fn empty() -> Self {
        Self {
            iter: rectangle::Points::empty(),
            center_2x: Point::zero(),
            ellipse: EllipseContains::empty(),
        }
    }
}

impl Iterator for Points {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        for point in &mut self.iter {
            if self.ellipse.contains(point * 2 - self.center_2x) {
                return Some(point);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::{Point, Size},
        iterator::PixelIteratorExt,
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        Pixel,
    };

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

            display.assert_pattern(*expected);
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

        display.assert_pattern(&[
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
        ]);
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

        display.assert_pattern(&[
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
        ]);
    }
}
