//! The rectangle primitive. Also good for drawing squares.

use crate::{
    drawable::{Drawable, Pixel},
    geometry::{Dimensions, Point, Size},
    pixelcolor::PixelColor,
    primitives::{
        ellipse::{self, Ellipse, StyledEllipseIterator},
        rectangle::{self, Rectangle, StyledRectangleIterator},
        ContainsPoint, Primitive, Quadrant,
    },
    style::{PrimitiveStyle, Styled},
    transform::Transform,
    DrawTarget,
};
use core::cmp::min;

/// Rounded rectangle primitive
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct RoundedRectangle {
    /// The base rectangle
    pub rectangle: Rectangle,

    /// The X and Y corner radius of each corner of the rectangle
    pub corner_radii: [Size; 4],
}

impl Primitive for RoundedRectangle {
    type PointsIter = Points;

    fn points(&self) -> Self::PointsIter {
        Points::new(self)
    }
}

impl ContainsPoint for RoundedRectangle {
    fn contains(&self, point: Point) -> bool {
        unimplemented!()
    }
}

impl Dimensions for RoundedRectangle {
    fn bounding_box(&self) -> Rectangle {
        self.rectangle
    }
}

impl RoundedRectangle {
    /// Creates a new rounded rectangle from a base rectangle and equal corner XY radius for  all
    /// corners.
    pub const fn new(rectangle: Rectangle, corner_radius: Size) -> Self {
        Self {
            rectangle,
            corner_radii: [corner_radius, corner_radius, corner_radius, corner_radius],
        }
    }

    /// Creates a new rounded rectangle with different corner radii.
    ///
    /// Corner radii are specified from the top-left corner in a clockwise direction
    pub fn with_corners(rectangle: Rectangle, corner_radii: [Size; 4]) -> Self {
        Self {
            rectangle,
            corner_radii,
        }
    }

    /// Creates a new rounded rectangle from the center point and the size.
    ///
    /// For rectangles with even width and/or height the top left corner doesn't
    /// align with the pixel grid. Because of this the coordinates of the top left
    /// corner will be rounded up to the nearest integer coordinate.
    pub fn with_center(center: Point, size: Size) -> Self {
        unimplemented!()
    }

    /// Returns the center of this rectangle.
    ///
    /// For rectangles with even width and/or height the returned value is rounded down
    /// to the nearest integer coordinate.
    pub fn center(&self) -> Point {
        self.rectangle.center()
    }
}

impl Transform for RoundedRectangle {
    /// Translate the rect from its current position to a new position by (x, y) pixels, returning
    /// a new `Rectangle`. For a mutating transform, see `translate_mut`.
    ///
    /// TODO: Example
    fn translate(&self, by: Point) -> Self {
        Self {
            rectangle: self.rectangle.translate(by),
            ..*self
        }
    }

    /// Translate the rect from its current position to a new position by (x, y) pixels.
    ///
    /// TODO: Example
    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.rectangle.translate_mut(by);

        self
    }
}

impl<C> IntoIterator for &Styled<RoundedRectangle, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = StyledRoundedRectangleIterator<C>;

    fn into_iter(self) -> Self::IntoIter {
        StyledRoundedRectangleIterator::new(self)
    }
}

/// Iterator over all points inside the rectangle.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Points {
    rect: rectangle::Points,
    top_left: ellipse::Points,
    top_right: ellipse::Points,
    bottom_right: ellipse::Points,
    bottom_left: ellipse::Points,
}

impl Points {
    fn new(shape: &RoundedRectangle) -> Self {
        let top_left = Ellipse::new(shape.rectangle.top_left, shape.corner_radii[0]);

        let top_right = Ellipse::new(
            shape.rectangle.top_left + shape.rectangle.size.x_axis()
                - shape.corner_radii[1].x_axis(),
            shape.corner_radii[1],
        );

        let bottom_right = Ellipse::new(
            shape.rectangle.bottom_right().unwrap() - shape.corner_radii[2],
            shape.corner_radii[2],
        );

        let bottom_left = Ellipse::new(
            shape.rectangle.top_left + shape.rectangle.size.y_axis()
                - shape.corner_radii[3].y_axis(),
            shape.corner_radii[3],
        );

        Self {
            rect: shape.rectangle.points(),
            top_left: ellipse::Points::with_quadrant(&top_left, Quadrant::TopLeft),
            top_right: ellipse::Points::with_quadrant(&top_right, Quadrant::TopRight),
            bottom_right: ellipse::Points::with_quadrant(&bottom_right, Quadrant::BottomRight),
            bottom_left: ellipse::Points::with_quadrant(&bottom_left, Quadrant::BottomLeft),
        }
    }

    fn empty() -> Self {
        Self {
            rect: rectangle::Points::empty(),
            top_left: ellipse::Points::empty(),
            top_right: ellipse::Points::empty(),
            bottom_right: ellipse::Points::empty(),
            bottom_left: ellipse::Points::empty(),
        }
    }
}

impl Iterator for Points {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        self.rect
            .next()
            .or_else(|| self.top_left.next())
            .or_else(|| self.top_right.next())
            .or_else(|| self.bottom_right.next())
            .or_else(|| self.bottom_left.next())
    }
}

/// Pixel iterator for each pixel in the rect border
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct StyledRoundedRectangleIterator<C: PixelColor>
where
    C: PixelColor,
{
    rect_iter: StyledRectangleIterator<C>,
    top_left_corner: Rectangle,
    top_right_corner: Rectangle,
    bottom_right_corner: Rectangle,
    bottom_left_corner: Rectangle,
    top_left_iter: StyledEllipseIterator<C>,
    top_right_iter: StyledEllipseIterator<C>,
    bottom_right_iter: StyledEllipseIterator<C>,
    bottom_left_iter: StyledEllipseIterator<C>,
    corner_radii: [Size; 4],
    top_left: Point,
}

impl<C> StyledRoundedRectangleIterator<C>
where
    C: PixelColor,
{
    fn new(styled: &Styled<RoundedRectangle, PrimitiveStyle<C>>) -> Self {
        let Styled { style, primitive } = styled;

        // let iter = if !style.is_transparent() {
        //     Points::empty()
        // } else {
        //     Points::empty()
        // };

        let rect = primitive.rectangle;

        let top_left_ellipse = Ellipse::new(rect.top_left, primitive.corner_radii[0]);
        let top_right_ellipse = Ellipse::new(
            rect.top_left + rect.size.x_axis() - primitive.corner_radii[1].x_axis(),
            primitive.corner_radii[1],
        );
        let bottom_right_ellipse = Ellipse::new(
            rect.bottom_right().unwrap() + Size::new_equal(1) - primitive.corner_radii[2],
            primitive.corner_radii[2],
        );
        let bottom_left_ellipse = Ellipse::new(
            rect.top_left + rect.size.y_axis() - primitive.corner_radii[3].y_axis(),
            primitive.corner_radii[3],
        );

        let top_left_iter = StyledEllipseIterator::with_quadrant(
            &top_left_ellipse.into_styled(*style),
            Quadrant::TopLeft,
        );
        let top_right_iter = StyledEllipseIterator::with_quadrant(
            &top_right_ellipse.into_styled(*style),
            Quadrant::TopRight,
        );
        let bottom_right_iter = StyledEllipseIterator::with_quadrant(
            &bottom_right_ellipse.into_styled(*style),
            Quadrant::BottomRight,
        );
        let bottom_left_iter = StyledEllipseIterator::with_quadrant(
            &bottom_left_ellipse.into_styled(*style),
            Quadrant::BottomLeft,
        );

        let top_left_corner = top_left_ellipse
            .bounding_box()
            .expand(style.outside_stroke_width());
        let top_right_corner = top_right_ellipse
            .bounding_box()
            .expand(style.outside_stroke_width());
        let bottom_right_corner = bottom_right_ellipse
            .bounding_box()
            .expand(style.outside_stroke_width());
        let bottom_left_corner = bottom_left_ellipse
            .bounding_box()
            .expand(style.outside_stroke_width());

        Self {
            rect_iter: primitive.rectangle.into_styled(*style).into_iter(),

            top_left_iter,
            top_right_iter,
            bottom_right_iter,
            bottom_left_iter,

            top_left: primitive.rectangle.top_left,
            corner_radii: primitive.corner_radii,

            top_left_corner,
            top_right_corner,
            bottom_right_corner,
            bottom_left_corner,
        }
    }
}

impl<C> Iterator for StyledRoundedRectangleIterator<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        let Self {
            top_left_corner,
            top_right_corner,
            bottom_right_corner,
            bottom_left_corner,
            ..
        } = self;

        self.rect_iter
            .find(|Pixel(pos, _)| {
                !top_left_corner.contains(*pos)
                    && !top_right_corner.contains(*pos)
                    && !bottom_right_corner.contains(*pos)
                    && !bottom_left_corner.contains(*pos)
            })
            .or_else(|| self.top_left_iter.next())
            .or_else(|| self.top_right_iter.next())
            .or_else(|| self.bottom_right_iter.next())
            .or_else(|| self.bottom_left_iter.next())
    }
}

impl<C> Drawable<C> for &Styled<RoundedRectangle, PrimitiveStyle<C>>
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
        pixelcolor::{BinaryColor, Rgb565, RgbColor},
        style::{PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment},
    };

    #[test]
    fn dimensions() {
        let rect = Rectangle::new(Point::new(5, 10), Size::new(10, 20));
        let moved = rect.translate(Point::new(-10, -20));

        assert_eq!(
            rect.bounding_box(),
            Rectangle::new(Point::new(5, 10), Size::new(10, 20))
        );

        assert_eq!(
            moved.bounding_box(),
            Rectangle::new(Point::new(-5, -10), Size::new(10, 20))
        );
    }

    #[test]
    fn it_can_be_translated() {
        let rect = Rectangle::new(Point::new(5, 10), Size::new(10, 20));
        let moved = rect.translate(Point::new(10, 15));

        let bounding_box = moved.bounding_box();
        assert_eq!(bounding_box.top_left, Point::new(15, 25));
        assert_eq!(bounding_box.size, Size::new(10, 20));
    }

    #[test]
    fn it_draws_unfilled_rect() {
        let mut rect = Rectangle::new(Point::new(2, 2), Size::new(3, 3))
            .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 1))
            .into_iter();

        assert_eq!(rect.next(), Some(Pixel(Point::new(2, 2), Rgb565::RED)));
        assert_eq!(rect.next(), Some(Pixel(Point::new(3, 2), Rgb565::RED)));
        assert_eq!(rect.next(), Some(Pixel(Point::new(4, 2), Rgb565::RED)));

        assert_eq!(rect.next(), Some(Pixel(Point::new(2, 3), Rgb565::RED)));
        assert_eq!(rect.next(), Some(Pixel(Point::new(4, 3), Rgb565::RED)));

        assert_eq!(rect.next(), Some(Pixel(Point::new(2, 4), Rgb565::RED)));
        assert_eq!(rect.next(), Some(Pixel(Point::new(3, 4), Rgb565::RED)));
        assert_eq!(rect.next(), Some(Pixel(Point::new(4, 4), Rgb565::RED)));
    }

    #[test]
    fn it_can_be_negative() {
        let negative = Rectangle::new(Point::new(-2, -2), Size::new(4, 4))
            .into_styled(PrimitiveStyle::with_fill(Rgb565::GREEN))
            .into_iter();

        let positive = Rectangle::new(Point::new(2, 2), Size::new(4, 4))
            .into_styled(PrimitiveStyle::with_fill(Rgb565::GREEN))
            .into_iter();

        assert!(negative.eq(positive.map(|Pixel(p, c)| Pixel(p - Point::new(4, 4), c))));
    }

    #[test]
    fn points_iter_matches_filled_styled() {
        let rectangle = Rectangle::new(Point::new(10, 10), Size::new(20, 30));

        let styled_points = rectangle
            .clone()
            .into_styled(PrimitiveStyle::with_fill(Rgb565::WHITE))
            .into_iter()
            .map(|Pixel(p, _)| p);

        assert!(rectangle.points().eq(styled_points));
    }

    #[test]
    fn points_iter() {
        let rectangle = Rectangle::new(Point::new(10, 20), Size::new(2, 3));

        let mut points = rectangle.points();
        assert_eq!(points.next(), Some(Point::new(10, 20)));
        assert_eq!(points.next(), Some(Point::new(11, 20)));
        assert_eq!(points.next(), Some(Point::new(10, 21)));
        assert_eq!(points.next(), Some(Point::new(11, 21)));
        assert_eq!(points.next(), Some(Point::new(10, 22)));
        assert_eq!(points.next(), Some(Point::new(11, 22)));
        assert_eq!(points.next(), None);
    }

    #[test]
    fn points_iter_zero_size() {
        let rectangle = Rectangle::new(Point::new(1, 2), Size::zero());

        let mut points = rectangle.points();
        assert_eq!(points.next(), None);
    }

    #[test]
    fn points_iter_empty() {
        let mut points = Points::empty();
        assert_eq!(points.next(), None);
    }

    #[test]
    fn contains() {
        let outer = Rectangle::new(Point::zero(), Size::new(10, 10));
        let inner = Rectangle::new(Point::new(2, 4), Size::new(3, 5));

        for p in outer.points() {
            let expected = p.x >= 2 && p.x < 2 + 3 && p.y >= 4 && p.y < 4 + 5;

            assert_eq!(inner.contains(p), expected, "{:?}", p);
        }
    }

    #[test]
    fn center() {
        let odd = Rectangle::new(Point::new(10, 20), Size::new(5, 7));
        assert_eq!(odd.center(), Point::new(12, 23));

        let even = Rectangle::new(Point::new(20, 30), Size::new(4, 8));
        assert_eq!(even.center(), Point::new(21, 33));
    }

    #[test]
    fn bottom_right() {
        let zero = Rectangle::new(Point::new(10, 20), Size::zero());
        assert_eq!(zero.bottom_right(), None);

        let odd = Rectangle::new(Point::new(10, 20), Size::new(5, 7));
        assert_eq!(odd.bottom_right(), Some(Point::new(14, 26)));

        let even = Rectangle::new(Point::new(20, 30), Size::new(4, 8));
        assert_eq!(even.bottom_right(), Some(Point::new(23, 37)));
    }

    #[test]
    fn stroke_alignment() {
        const TOP_LEFT: Point = Point::new(5, 6);
        const SIZE: Size = Size::new(10, 5);

        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 3);

        let mut display_center = MockDisplay::new();
        Rectangle::new(TOP_LEFT, SIZE)
            .into_styled(style)
            .draw(&mut display_center)
            .unwrap();

        let mut display_inside = MockDisplay::new();
        Rectangle::new(TOP_LEFT - Point::new(1, 1), SIZE + Size::new(2, 2))
            .into_styled(
                PrimitiveStyleBuilder::from(&style)
                    .stroke_alignment(StrokeAlignment::Inside)
                    .build(),
            )
            .draw(&mut display_inside)
            .unwrap();

        let mut display_outside = MockDisplay::new();
        Rectangle::new(TOP_LEFT + Point::new(2, 2), SIZE - Size::new(4, 4))
            .into_styled(
                PrimitiveStyleBuilder::from(&style)
                    .stroke_alignment(StrokeAlignment::Outside)
                    .build(),
            )
            .draw(&mut display_outside)
            .unwrap();

        assert_eq!(display_center, display_inside);
        assert_eq!(display_center, display_outside);
    }
}
