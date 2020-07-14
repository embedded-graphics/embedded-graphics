use crate::{
    geometry::{Dimensions, Point, Size},
    primitives::{
        ellipse,
        rectangle::{self, Rectangle},
        ContainsPoint, Primitive,
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
    size_sq: Size,
    threshold: u32,
    center_2x: Point,
}

impl EllipseQuadrant {
    pub fn new(top_left: Point, radius: Size, quadrant: Quadrant) -> Self {
        let ellipse_top_left = match quadrant {
            Quadrant::TopLeft => top_left,
            Quadrant::TopRight => top_left - radius.x_axis(),
            Quadrant::BottomRight => top_left - radius,
            Quadrant::BottomLeft => top_left - radius.y_axis(),
        };

        let (size_sq, threshold) = ellipse::compute_threshold(radius * 2);

        Self {
            bounding_box: Rectangle::new(top_left, radius),
            size_sq,
            threshold,
            center_2x: ellipse::center_2x(ellipse_top_left, radius * 2),
        }
    }
}

impl Dimensions for EllipseQuadrant {
    fn bounding_box(&self) -> Rectangle {
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
        ellipse::is_point_inside_ellipse(self.size_sq, point * 2 - self.center_2x, self.threshold)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(in crate::primitives) struct Points {
    iter: rectangle::Points,
    size_sq: Size,
    threshold: u32,
    center_2x: Point,
}

impl Points {
    pub fn new(ellipse_quadrant: &EllipseQuadrant) -> Self {
        Self {
            iter: ellipse_quadrant.bounding_box().points(),
            size_sq: ellipse_quadrant.size_sq,
            threshold: ellipse_quadrant.threshold,
            center_2x: ellipse_quadrant.center_2x,
        }
    }

    pub(in crate::primitives) const fn empty() -> Self {
        Self {
            iter: rectangle::Points::empty(),
            size_sq: Size::zero(),
            threshold: 0,
            center_2x: Point::zero(),
        }
    }
}

impl Iterator for Points {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        for point in &mut self.iter {
            if ellipse::is_point_inside_ellipse(
                self.size_sq,
                point * 2 - self.center_2x,
                self.threshold,
            ) {
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
        drawable::Pixel,
        geometry::{Point, Size},
        mock_display::MockDisplay,
        pixel_iterator::PixelIteratorExt,
        pixelcolor::BinaryColor,
        primitives::Primitive,
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
