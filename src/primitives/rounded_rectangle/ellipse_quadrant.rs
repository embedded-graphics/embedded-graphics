use crate::{
    geometry::{Dimensions, Point, Size},
    primitives::{
        ellipse::{self, EllipseContains},
        rectangle::Rectangle,
        ContainsPoint,
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

impl ContainsPoint for EllipseQuadrant {
    fn contains(&self, point: Point) -> bool {
        self.ellipse.contains(point * 2 - self.center_2x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        draw_target::DrawTarget,
        geometry::{Point, Size},
        iterator::PixelIteratorExt,
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        primitives::PointsIter,
        Pixel,
    };

    fn draw_quadrant<D: DrawTarget<Color = BinaryColor>>(
        quadrant: &EllipseQuadrant,
        target: &mut D,
    ) -> Result<(), D::Error> {
        quadrant
            .bounding_box
            .points()
            .filter(|p| quadrant.contains(*p))
            .map(|p| Pixel(p, BinaryColor::On))
            .draw(target)
    }

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
            let ellipse_quadrant =
                EllipseQuadrant::new(Point::new(0, 0), Size::new(10, 5), *quadrant);

            let mut display = MockDisplay::new();
            draw_quadrant(&ellipse_quadrant, &mut display).unwrap();
            display.assert_pattern(*expected);
        }
    }

    #[test]
    fn quadrants_equal_even_ellipse() {
        let mut display = MockDisplay::new();

        let radius = Size::new(10, 5);
        let top_left = Point::new(0, 0);

        draw_quadrant(
            &EllipseQuadrant::new(top_left, radius, Quadrant::TopLeft),
            &mut display,
        )
        .unwrap();
        draw_quadrant(
            &EllipseQuadrant::new(top_left + radius.x_axis(), radius, Quadrant::TopRight),
            &mut display,
        )
        .unwrap();
        draw_quadrant(
            &EllipseQuadrant::new(top_left + radius, radius, Quadrant::BottomRight),
            &mut display,
        )
        .unwrap();
        draw_quadrant(
            &EllipseQuadrant::new(top_left + radius.y_axis(), radius, Quadrant::BottomLeft),
            &mut display,
        )
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

        draw_quadrant(
            &EllipseQuadrant::new(top_left, radius, Quadrant::TopLeft),
            &mut display,
        )
        .unwrap();
        draw_quadrant(
            &EllipseQuadrant::new(top_left + radius.x_axis(), radius, Quadrant::TopRight),
            &mut display,
        )
        .unwrap();
        draw_quadrant(
            &EllipseQuadrant::new(top_left + radius, radius, Quadrant::BottomRight),
            &mut display,
        )
        .unwrap();
        draw_quadrant(
            &EllipseQuadrant::new(top_left + radius.y_axis(), radius, Quadrant::BottomLeft),
            &mut display,
        )
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
