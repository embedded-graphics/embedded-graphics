//! The rectangle primitive. Also good for drawing squares.

use crate::{
    drawable::{Drawable, Pixel},
    geometry::{Dimensions, Point, Size},
    pixelcolor::PixelColor,
    primitives::{
        ellipse_quadrant::{self, EllipseQuadrant, Quadrant},
        rectangle::{self, Rectangle},
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
        let Self {
            rectangle, corners, ..
        } = self;

        let Rectangle { top_left, size, .. } = *rectangle;

        if !rectangle.contains(point) {
            return false;
        }

        let tl = EllipseQuadrant::new(top_left, corners.top_left, Quadrant::TopLeft);
        let tr = EllipseQuadrant::new(
            top_left + size.x_axis() - corners.top_right.x_axis(),
            corners.top_right,
            Quadrant::TopRight,
        );
        let br = EllipseQuadrant::new(
            top_left + size - corners.bottom_right,
            corners.bottom_right,
            Quadrant::BottomRight,
        );
        let bl = EllipseQuadrant::new(
            top_left + size.y_axis() - corners.bottom_left.y_axis(),
            corners.bottom_left,
            Quadrant::BottomLeft,
        );

        if tl.bounding_box().contains(point) {
            return tl.contains(point);
        }

        if tr.bounding_box().contains(point) {
            return tr.contains(point);
        }

        if br.bounding_box().contains(point) {
            return br.contains(point);
        }

        if bl.bounding_box().contains(point) {
            return bl.contains(point);
        }

        // We're in the rest of the rectangle at this point
        true
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
    pub fn with_equal_corners(rectangle: Rectangle, corner_radius: Size) -> Self {
        Self::new(rectangle, CornerRadii::new_equal(corner_radius))
    }

    /// Creates a new rounded rectangle with different corner radii.
    pub fn new(rectangle: Rectangle, corners: CornerRadii) -> Self {
        let Rectangle { size, .. } = rectangle;

        let corners = CornerRadii {
            top_left: corners.top_left.component_min(size / 2),
            top_right: corners.top_right.component_min(size / 2),
            bottom_right: corners.bottom_right.component_min(size / 2),
            bottom_left: corners.bottom_left.component_min(size / 2),
        };

        Self { rectangle, corners }
    }

    /// Returns the center of this rectangle.
    ///
    /// For rectangles with even width and/or height the returned value is rounded down
    /// to the nearest integer coordinate.
    pub fn center(&self) -> Point {
        self.rectangle.center()
    }

    fn expand(&self, offset: u32) -> Self {
        let corner_offset = Size::new_equal(offset);

        Self::new(
            self.rectangle.expand(offset),
            CornerRadii {
                top_left: self.corners.top_left.saturating_add(corner_offset),
                top_right: self.corners.top_right.saturating_add(corner_offset),
                bottom_right: self.corners.bottom_right.saturating_add(corner_offset),
                bottom_left: self.corners.bottom_left.saturating_add(corner_offset),
            },
        )
    }

    fn shrink(&self, offset: u32) -> Self {
        let corner_offset = Size::new_equal(offset);

        Self::new(
            self.rectangle.shrink(offset),
            CornerRadii {
                top_left: self.corners.top_left.saturating_sub(corner_offset),
                top_right: self.corners.top_right.saturating_sub(corner_offset),
                bottom_right: self.corners.bottom_right.saturating_sub(corner_offset),
                bottom_left: self.corners.bottom_left.saturating_sub(corner_offset),
            },
        )
    }
}

impl Transform for RoundedRectangle {
    /// Translate the rounded rectangle from its current position to a new position by (x, y)
    /// pixels, returning a new `RoundedRectangle`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::prelude::*;
    /// use embedded_graphics::primitives::{Rectangle, RoundedRectangle};
    ///
    /// let original = RoundedRectangle::with_equal_corners(
    ///     Rectangle::new(Point::new(5, 10), Size::new(20, 30)),
    ///     Size::new(10, 15),
    /// );
    /// let moved = original.translate(Point::new(10, 12));
    ///
    /// assert_eq!(original.bounding_box().top_left, Point::new(5, 10));
    /// assert_eq!(moved.bounding_box().top_left, Point::new(15, 22));
    /// ```
    fn translate(&self, by: Point) -> Self {
        Self {
            rectangle: self.rectangle.translate(by),
            ..*self
        }
    }

    /// Translate the rounded rectangle from its current position to a new position by (x, y) pixels.
    ///
    /// ```
    /// # use embedded_graphics::prelude::*;
    /// use embedded_graphics::primitives::{Rectangle, RoundedRectangle};
    ///
    /// let mut shape = RoundedRectangle::with_equal_corners(
    ///     Rectangle::new(Point::new(5, 10), Size::new(20, 30)),
    ///     Size::new(10, 15),
    /// );
    ///
    /// shape.translate_mut(Point::new(10, 12));
    ///
    /// assert_eq!(shape.bounding_box().top_left, Point::new(15, 22));
    /// ```
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
            top_left + size.x_axis() - corners.top_right.x_axis(),
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

    fn empty() -> Self {
        Self {
            rect_iter: rectangle::Points::empty(),
            top_left_iter: ellipse_quadrant::Points::empty(),
            top_right_iter: ellipse_quadrant::Points::empty(),
            bottom_right_iter: ellipse_quadrant::Points::empty(),
            bottom_left_iter: ellipse_quadrant::Points::empty(),
            top_left_corner: Rectangle::new(Point::zero(), Size::zero()),
            top_right_corner: Rectangle::new(Point::zero(), Size::zero()),
            bottom_right_corner: Rectangle::new(Point::zero(), Size::zero()),
            bottom_left_corner: Rectangle::new(Point::zero(), Size::zero()),
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
    iter: Points,
    fill_area: RoundedRectangle,
    stroke_color: Option<C>,
    fill_color: Option<C>,
}

impl<C> StyledRoundedRectangleIterator<C>
where
    C: PixelColor,
{
    fn new(styled: &Styled<RoundedRectangle, PrimitiveStyle<C>>) -> Self {
        let Styled { style, primitive } = styled;

        let iter = if !styled.style.is_transparent() {
            let stroke_area = primitive.expand(style.outside_stroke_width());
            Points::new(&stroke_area)
        } else {
            Points::empty()
        };

        let fill_area = primitive.shrink(style.inside_stroke_width());

        Self {
            iter,
            fill_area,
            stroke_color: style.stroke_color,
            fill_color: style.fill_color,
        }
    }
}

impl<C> Iterator for StyledRoundedRectangleIterator<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        let point = self.iter.next()?;

        let color = if self.fill_area.contains(point) {
            self.fill_color?
        } else {
            self.stroke_color?
        };

        Some(Pixel(point, color))
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
        pixelcolor::{BinaryColor, Rgb565, Rgb888},
        prelude::*,
        style::PrimitiveStyleBuilder,
    };

    #[test]
    fn transparent_style_no_render() {
        let rounded_rect = RoundedRectangle::with_equal_corners(
            Rectangle::new(Point::zero(), Size::new(10, 20)),
            Size::new(4, 8),
        )
        .into_styled(PrimitiveStyleBuilder::<BinaryColor>::new().build());

        assert!(rounded_rect.into_iter().eq(core::iter::empty()));
    }

    #[test]
    fn points_equals_filled() {
        let rounded_rect = RoundedRectangle::with_equal_corners(
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
        let mut display = MockDisplay::new();

        RoundedRectangle::new(
            Rectangle::new(Point::new_equal(2), Size::new(20, 20)),
            CornerRadii {
                top_left: Size::new(3, 4),
                top_right: Size::new(5, 6),
                bottom_right: Size::new(7, 8),
                bottom_left: Size::new(9, 10),
            },
        )
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_width(5)
                .fill_color(Rgb888::RED)
                .stroke_color(Rgb888::GREEN)
                .build(),
        )
        .draw(&mut display)
        .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "   GGGGGGGGGGGGGGGG     ",
                "  GGGGGGGGGGGGGGGGGGG   ",
                " GGGGGGGGGGGGGGGGGGGGG  ",
                "GGGGGGGGGGGGGGGGGGGGGGG ",
                "GGGGGGGGGGGGGGGGGGGGGGG ",
                "GGGGGRRRRRRRRRRRRRGGGGGG",
                "GGGGGRRRRRRRRRRRRRRGGGGG",
                "GGGGGRRRRRRRRRRRRRRGGGGG",
                "GGGGGRRRRRRRRRRRRRRGGGGG",
                "GGGGGRRRRRRRRRRRRRRGGGGG",
                "GGGGGRRRRRRRRRRRRRRGGGGG",
                "GGGGGRRRRRRRRRRRRRRGGGGG",
                "GGGGGRRRRRRRRRRRRRRGGGGG",
                "GGGGGRRRRRRRRRRRRRRGGGGG",
                "GGGGGRRRRRRRRRRRRRRGGGGG",
                "GGGGGGRRRRRRRRRRRRRGGGGG",
                " GGGGGRRRRRRRRRRRRGGGGGG",
                " GGGGGGRRRRRRRRRRRGGGGG ",
                "  GGGGGGGRRRRRRRRGGGGGG ",
                "  GGGGGGGGGGGGGGGGGGGGG ",
                "   GGGGGGGGGGGGGGGGGGG  ",
                "    GGGGGGGGGGGGGGGGG   ",
                "      GGGGGGGGGGGGGG    ",
                "        GGGGGGGGGG      ",
            ])
        );
    }

    #[test]
    fn thin_line_zero_radius_equals_rectangle() {
        let style = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb565::RED)
            .stroke_width(1)
            .fill_color(Rgb565::RED)
            .build();

        let rounded_rect = RoundedRectangle::with_equal_corners(
            Rectangle::new(Point::zero(), Size::new(20, 30)),
            Size::zero(),
        )
        .into_styled(style);

        let rect = Rectangle::new(Point::zero(), Size::new(20, 30)).into_styled(style);

        assert!(rounded_rect.into_iter().eq(rect.into_iter()));
    }

    #[test]
    fn clamp_radius_at_rect_size() {
        let style = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb565::RED)
            .stroke_width(8)
            .fill_color(Rgb565::GREEN)
            .build();

        let clamped = RoundedRectangle::with_equal_corners(
            Rectangle::new(Point::zero(), Size::new(20, 30)),
            Size::new_equal(50),
        )
        .into_styled(style);

        let expected = RoundedRectangle::with_equal_corners(
            Rectangle::new(Point::zero(), Size::new(20, 30)),
            Size::new(10, 15),
        )
        .into_styled(style);

        assert!(clamped.into_iter().eq(expected.into_iter()));
    }
}
