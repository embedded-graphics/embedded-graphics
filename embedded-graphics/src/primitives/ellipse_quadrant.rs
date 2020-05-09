use crate::draw_target::DrawTarget;
use crate::drawable::Drawable;
use crate::drawable::Pixel;
use crate::geometry::Dimensions;
use crate::geometry::{Point, Size};
use crate::pixelcolor::PixelColor;
use crate::primitives::ellipse::Ellipse;
use crate::primitives::ellipse::StyledEllipseIterator;
use crate::primitives::rectangle::{self, Rectangle};
use crate::primitives::ContainsPoint;
use crate::primitives::Primitive;
use crate::style::PrimitiveStyle;
use crate::style::Styled;

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

    /// Expand the ellipse quadrant by a given offset
    ///
    /// This method will expand the ellipse quadrant in the direction of its outer edge. The center
    /// point will not be moved.
    ///
    /// If the radius of the ellipse quadrant is zero, no expand will be performed.
    pub(crate) fn expand(&self, offset: u32) -> Self {
        if self.radius == Size::zero() {
            return *self;
        }

        let offset = Size::new_equal(offset);

        let Self {
            bounding_box,
            quadrant,
            ..
        } = self;

        let top_left = bounding_box.top_left;

        let top_left = match quadrant {
            Quadrant::TopLeft => top_left - offset,
            Quadrant::TopRight => top_left - offset.y_axis(),
            Quadrant::BottomRight => top_left,
            Quadrant::BottomLeft => top_left - offset.x_axis(),
        };

        Self::new(top_left, self.radius + offset, self.quadrant)
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

impl<'a, C> IntoIterator for &'a Styled<EllipseQuadrant, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = StyledEllipseQuadrantIterator<C>;

    fn into_iter(self) -> Self::IntoIter {
        StyledEllipseQuadrantIterator::new(self)
    }
}

/// Pixel iterator for each pixel in the ellipse border
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct StyledEllipseQuadrantIterator<C>
where
    C: PixelColor,
{
    iter: StyledEllipseIterator<C>,
    bounding_box: Rectangle,
}

impl<C> StyledEllipseQuadrantIterator<C>
where
    C: PixelColor,
{
    pub(crate) fn new(styled: &Styled<EllipseQuadrant, PrimitiveStyle<C>>) -> Self {
        let Styled { primitive, style } = styled;

        Self {
            iter: primitive.ellipse.into_styled(*style).into_iter(),
            bounding_box: primitive
                .expand(style.outside_stroke_width())
                .bounding_box(),
        }
    }
}

impl<C> Iterator for StyledEllipseQuadrantIterator<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        for pixel in &mut self.iter {
            if self.bounding_box.contains(pixel.0) {
                return Some(pixel);
            }
        }

        None
    }
}

impl<'a, C: 'a> Drawable<C> for &Styled<EllipseQuadrant, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        display.draw_iter(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        style::{PrimitiveStyleBuilder, StrokeAlignment},
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

    #[test]
    fn fill_and_stroke() {
        let mut display = MockDisplay::new();

        let style = PrimitiveStyleBuilder::new()
            .stroke_width(3)
            .stroke_color(BinaryColor::Off)
            .stroke_alignment(StrokeAlignment::Inside)
            .fill_color(BinaryColor::On)
            .build();

        let radius = Size::new(10, 5);
        let top_left = Point::new(0, 0);

        EllipseQuadrant::new(top_left, radius, Quadrant::TopLeft)
            .into_styled(style)
            .into_iter()
            .chain(
                EllipseQuadrant::new(top_left + radius.x_axis(), radius, Quadrant::TopRight)
                    .into_styled(style)
                    .into_iter(),
            )
            .chain(
                EllipseQuadrant::new(top_left + radius, radius, Quadrant::BottomRight)
                    .into_styled(style)
                    .into_iter(),
            )
            .chain(
                EllipseQuadrant::new(top_left + radius.y_axis(), radius, Quadrant::BottomLeft)
                    .into_styled(style)
                    .into_iter(),
            )
            .draw(&mut display)
            .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "      ........      ",
                "   ..............   ",
                " .................. ",
                ".....##########.....",
                "...##############...",
                "...##############...",
                ".....##########.....",
                " .................. ",
                "   ..............   ",
                "      ........      ",
            ])
        );
    }

    #[test]
    fn non_circular() {
        let mut display = MockDisplay::new();

        let style = PrimitiveStyleBuilder::new()
            .stroke_width(3)
            .stroke_color(BinaryColor::Off)
            .stroke_alignment(StrokeAlignment::Inside)
            .fill_color(BinaryColor::On)
            .build();

        EllipseQuadrant::new(Point::new(0, 0), Size::new(10, 5), Quadrant::TopLeft)
            .into_styled(style)
            .draw(&mut display)
            .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "      ....",
                "   .......",
                " .........",
                ".....#####",
                "...#######",
            ])
        );
    }
}
