//! The rectangle primitive. Also good for drawing squares.

use crate::{
    drawable::{Drawable, Pixel},
    geometry::{Dimensions, Point, Size},
    pixelcolor::PixelColor,
    primitives::{
        ellipse_quadrant::{self, EllipseQuadrant, Quadrant, StyledEllipseQuadrantIterator},
        rectangle::{self, Rectangle, StyledRectangleIterator},
        ContainsPoint, Primitive,
    },
    style::{PrimitiveStyle, Styled},
    transform::Transform,
    DrawTarget,
};

/// The definition of each corner radius for a rounded rectangle.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CornerRadii {
    /// Top left corner radius
    pub top_left: Size,

    /// Top right corner radius
    pub top_right: Size,

    /// Bottom right corner radius
    pub bottom_right: Size,

    /// Bottom left corner radius
    pub bottom_left: Size,
}

impl CornerRadii {
    /// Create a new set of corner radii with all corners having equal values.
    pub fn new_equal(radius: Size) -> Self {
        Self {
            top_left: radius,
            top_right: radius,
            bottom_right: radius,
            bottom_left: radius,
        }
    }
}

/// Rounded rectangle primitive
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct RoundedRectangle {
    /// The base rectangle
    pub rectangle: Rectangle,

    /// The radius of each corner
    pub corners: CornerRadii,
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
        Self::with_radii(rectangle, CornerRadii::new_equal(corner_radius))
    }

    /// Creates a new rounded rectangle with different corner radii.
    ///
    /// Corner radii are specified from the top-left corner in a clockwise direction
    pub fn with_radii(rectangle: Rectangle, corners: CornerRadii) -> Self {
        let Rectangle { size, top_left, .. } = rectangle;

        let corners = CornerRadii {
            top_left: corners.top_left.component_min(size / 2),
            top_right: corners.top_right.component_min(size / 2),
            bottom_right: corners.bottom_right.component_min(size / 2),
            bottom_left: corners.bottom_left.component_min(size / 2),
        };

        // let top_left_radius = corner_radii[0].component_min(size / 2);
        // let top_right_radius = corner_radii[1].component_min(size / 2);
        // let bottom_right_radius = corner_radii[2].component_min(size / 2);
        // let bottom_left_radius = corner_radii[3].component_min(size / 2);

        Self {
            rectangle,
            corners
            // corners: [
            //     EllipseQuadrant::new(top_left, top_left_radius, Quadrant::TopLeft),
            //     EllipseQuadrant::new(
            //         top_left + size.x_axis() - top_left_radius.x_axis(),
            //         top_right_radius,
            //         Quadrant::TopRight,
            //     ),
            //     EllipseQuadrant::new(
            //         top_left + size - bottom_right_radius,
            //         bottom_right_radius,
            //         Quadrant::BottomRight,
            //     ),
            //     EllipseQuadrant::new(
            //         top_left + size.y_axis() - bottom_left_radius.y_axis(),
            //         bottom_left_radius,
            //         Quadrant::BottomLeft,
            //     ),
            // ],
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
    rect_iter: rectangle::Points,

    top_left_corner: Rectangle,
    top_right_corner: Rectangle,
    bottom_right_corner: Rectangle,
    bottom_left_corner: Rectangle,

    top_left_iter: ellipse_quadrant::Points,
    top_right_iter: ellipse_quadrant::Points,
    bottom_right_iter: ellipse_quadrant::Points,
    bottom_left_iter: ellipse_quadrant::Points,
}

impl Points {
    fn new(shape: &RoundedRectangle) -> Self {
        let RoundedRectangle {
            rectangle, corners, ..
        } = shape;

        let Rectangle { top_left, size, .. } = *rectangle;

        let top_left_ellipse = EllipseQuadrant::new(top_left, corners.top_left, Quadrant::TopLeft);
        let top_right_ellipse = EllipseQuadrant::new(
            top_left + size.x_axis() - corners.top_left.x_axis(),
            corners.top_right,
            Quadrant::TopRight,
        );
        let bottom_right_ellipse = EllipseQuadrant::new(
            top_left + size - corners.bottom_right,
            corners.bottom_right,
            Quadrant::BottomRight,
        );
        let bottom_left_ellipse = EllipseQuadrant::new(
            top_left + size.y_axis() - corners.bottom_left.y_axis(),
            corners.bottom_left,
            Quadrant::BottomLeft,
        );

        Self {
            rect_iter: rectangle.points(),

            top_left_iter: top_left_ellipse.points(),
            top_right_iter: top_right_ellipse.points(),
            bottom_right_iter: bottom_right_ellipse.points(),
            bottom_left_iter: bottom_left_ellipse.points(),

            top_left_corner: top_left_ellipse.bounding_box(),
            top_right_corner: top_right_ellipse.bounding_box(),
            bottom_right_corner: bottom_right_ellipse.bounding_box(),
            bottom_left_corner: bottom_left_ellipse.bounding_box(),
        }
    }
}

impl Iterator for Points {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let Self {
            top_left_corner,
            top_right_corner,
            bottom_right_corner,
            bottom_left_corner,
            ..
        } = self;

        self.rect_iter
            .find(|p| {
                !top_left_corner.contains(*p)
                    && !top_right_corner.contains(*p)
                    && !bottom_right_corner.contains(*p)
                    && !bottom_left_corner.contains(*p)
            })
            .or_else(|| self.top_left_iter.next())
            .or_else(|| self.top_right_iter.next())
            .or_else(|| self.bottom_right_iter.next())
            .or_else(|| self.bottom_left_iter.next())
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

    top_left_iter: StyledEllipseQuadrantIterator<C>,
    top_right_iter: StyledEllipseQuadrantIterator<C>,
    bottom_right_iter: StyledEllipseQuadrantIterator<C>,
    bottom_left_iter: StyledEllipseQuadrantIterator<C>,
}

impl<C> StyledRoundedRectangleIterator<C>
where
    C: PixelColor,
{
    fn new(styled: &Styled<RoundedRectangle, PrimitiveStyle<C>>) -> Self {
        // let top_left_iter = corners[0].into_styled(*style).into_iter();
        // let top_right_iter = corners[1].into_styled(*style).into_iter();
        // let bottom_right_iter = corners[2].into_styled(*style).into_iter();
        // let bottom_left_iter = corners[3].into_styled(*style).into_iter();

        // let top_left_corner = corners[0].expand_curved_edge(sw).bounding_box();
        // let top_right_corner = corners[1].expand_curved_edge(sw).bounding_box();
        // let bottom_right_corner = corners[2].expand_curved_edge(sw).bounding_box();
        // let bottom_left_corner = corners[3].expand_curved_edge(sw).bounding_box();

        // Self {
        //     rect_iter: if !style.is_transparent() {
        //         *rectangle
        //     } else {
        //         Rectangle::new(Point::zero(), Size::zero())
        //     }
        //     .into_styled(*style)
        //     .into_iter(),

        //     top_left_iter,
        //     top_right_iter,
        //     bottom_right_iter,
        //     bottom_left_iter,

        //     top_left_corner,
        //     top_right_corner,
        //     bottom_right_corner,
        //     bottom_left_corner,
        // }

        let Styled { style, primitive } = styled;

        let Rectangle { top_left, size, .. } = primitive.rectangle;
        let corners = primitive.corners;

        let sw = style.outside_stroke_width();

        let top_left_ellipse = EllipseQuadrant::new(top_left, corners.top_left, Quadrant::TopLeft);
        let top_right_ellipse = EllipseQuadrant::new(
            top_left + size.x_axis() - corners.top_left.x_axis(),
            corners.top_right,
            Quadrant::TopRight,
        );
        let bottom_right_ellipse = EllipseQuadrant::new(
            top_left + size - corners.bottom_right,
            corners.bottom_right,
            Quadrant::BottomRight,
        );
        let bottom_left_ellipse = EllipseQuadrant::new(
            top_left + size.y_axis() - corners.bottom_left.y_axis(),
            corners.bottom_left,
            Quadrant::BottomLeft,
        );

        Self {
            rect_iter: if !style.is_transparent() {
                primitive.rectangle
            } else {
                Rectangle::new(Point::zero(), Size::zero())
            }
            .into_styled(*style)
            .into_iter(),

            top_left_iter: top_left_ellipse.into_styled(*style).into_iter(),
            top_right_iter: top_right_ellipse.into_styled(*style).into_iter(),
            bottom_right_iter: bottom_right_ellipse.into_styled(*style).into_iter(),
            bottom_left_iter: bottom_left_ellipse.into_styled(*style).into_iter(),

            top_left_corner: top_left_ellipse.expand_curved_edge(sw).bounding_box(),
            top_right_corner: top_right_ellipse.expand_curved_edge(sw).bounding_box(),
            bottom_right_corner: bottom_right_ellipse.expand_curved_edge(sw).bounding_box(),
            bottom_left_corner: bottom_left_ellipse.expand_curved_edge(sw).bounding_box(),
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
        pixelcolor::{BinaryColor, Rgb565},
        prelude::*,
        style::PrimitiveStyleBuilder,
    };

    #[test]
    fn transparent_style_no_render() {
        let rounded_rect = RoundedRectangle::new(
            Rectangle::new(Point::zero(), Size::new(10, 20)),
            Size::new(4, 8),
        )
        .into_styled(PrimitiveStyleBuilder::<BinaryColor>::new().build());

        assert!(rounded_rect.into_iter().eq(core::iter::empty()));
    }

    #[test]
    fn points_equals_filled() {
        let rounded_rect = RoundedRectangle::new(
            Rectangle::new(Point::zero(), Size::new(10, 20)),
            Size::new(4, 8),
        );

        assert!(rounded_rect.points().eq(rounded_rect
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .into_iter()
            .map(|pixel| pixel.0)));
    }

    #[test]
    fn styled_unequal_corners() {
        //
    }

    #[test]
    fn zero_radius_equals_rectangle() {
        let style = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb565::RED)
            .stroke_width(8)
            .fill_color(Rgb565::RED)
            .build();

        let rounded_rect = RoundedRectangle::new(
            Rectangle::new(Point::zero(), Size::new(20, 30)),
            Size::zero(),
        )
        .into_styled(style);

        let rect = Rectangle::new(Point::zero(), Size::new(20, 30)).into_styled(style);

        assert!(rounded_rect.into_iter().eq(rect.into_iter()));
    }

    #[test]
    fn clamp_radius_at_rect_size() {
        //
    }
}
