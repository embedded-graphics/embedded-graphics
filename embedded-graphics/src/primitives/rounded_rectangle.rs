//! The rectangle primitive. Also good for drawing squares.

use crate::{
    drawable::{Drawable, Pixel},
    geometry::{Dimensions, Point, Size},
    pixelcolor::PixelColor,
    primitives::{
        ellipse::{self, Ellipse, StyledEllipseIterator},
        ellipse_quadrant::Quadrant,
        rectangle::{self, Rectangle, StyledRectangleIterator},
        ContainsPoint, Primitive,
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
    pub fn new(rectangle: Rectangle, corner_radius: Size) -> Self {
        let size = rectangle.size;

        Self::with_corners(
            rectangle,
            [
                corner_radius.min(size),
                corner_radius.min(size),
                corner_radius.min(size),
                corner_radius.min(size),
            ],
        )
    }

    /// Creates a new rounded rectangle with different corner radii.
    ///
    /// Corner radii are specified from the top-left corner in a clockwise direction
    pub const fn with_corners(rectangle: Rectangle, corner_radii: [Size; 4]) -> Self {
        Self {
            rectangle,
            corner_radii,
        }
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
            .expand(style.outside_stroke_width())
            .quadrant(Quadrant::TopLeft);
        let top_right_corner = top_right_ellipse
            .bounding_box()
            .expand(style.outside_stroke_width())
            .quadrant(Quadrant::TopRight);
        let bottom_right_corner = bottom_right_ellipse
            .bounding_box()
            .expand(style.outside_stroke_width())
            .quadrant(Quadrant::BottomRight);
        let bottom_left_corner = bottom_left_ellipse
            .bounding_box()
            .expand(style.outside_stroke_width())
            .quadrant(Quadrant::BottomLeft);

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
    // TODO
}
