//! The rounded rectangle primitive.

use crate::{
    drawable::{Drawable, Pixel},
    geometry::{Dimensions, Point, Size},
    pixelcolor::PixelColor,
    primitives::{
        corner_radii::CornerRadii,
        ellipse_quadrant::{self, EllipseQuadrant, Quadrant},
        rectangle::{self, Rectangle},
        ContainsPoint, Primitive,
    },
    style::{PrimitiveStyle, Styled},
    transform::Transform,
    DrawTarget,
};

/// Rounded rectangle primitive.
///
/// Creates a rectangle with rounded corners. Corners can be circular or elliptical in shape, and
/// each corner may have a separate radius applied to it. Corner radii can be specified either by
/// creating an instance of [`CornerRadii`](../struct.CornerRadii.html), or using the
/// [`CornerRadiiBuilder`](../struct.CornerRadiiBuilder.html) builder.
///
/// # Overlapping corners
///
/// It is possible to create a `RoundedRectangle` with corner radii too large to be contained within
/// its edges. When this happens, the corner radii will be confined to fit within the rounded
/// rectangle before use by other parts of embedded-graphics.
///
/// This is similar but not identical to
/// [how the CSS specification works](https://www.w3.org/TR/css-backgrounds-3/#corner-overlap) as it
/// relies on floating point calculations.
///
/// # Examples
///
/// ## Create a uniform rounded rectangle
///
/// This example creates a rounded rectangle 50px wide by 60px tall. Using
/// [`with_equal_corners`](#method.new), all corners are given the same 10px circular radius.
/// The rectangle is drawn using a solid green fill with a 5px red stroke.
///
/// ```rust
/// use embedded_graphics::{
///     pixelcolor::Rgb565,
///     prelude::*,
///     primitives::{Rectangle, RoundedRectangle},
///     style::{PrimitiveStyle, PrimitiveStyleBuilder},
/// };
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::default();
///
/// let style = PrimitiveStyleBuilder::new()
///     .stroke_width(5)
///     .stroke_color(Rgb565::RED)
///     .fill_color(Rgb565::GREEN)
///     .build();
///
/// RoundedRectangle::with_equal_corners(
///     Rectangle::new(Point::new(5, 5), Size::new(50, 60)),
///     Size::new(10, 10),
/// )
/// .into_styled(style)
/// .draw(&mut display)?;
/// # Ok::<(), core::convert::Infallible>(())
/// ```
///
/// ## Different corner radii
///
/// This example creates a rounded rectangle 50px wide by 60px tall. Each corner is given a distinct
/// radius in the x and y direction by creating a [`CornerRadii`](../struct.CornerRadiiBuilder.html)
/// object and passing that to [`RoundedRectangle::new`](#method.new).
///
/// ```rust
/// use embedded_graphics::{
///     pixelcolor::Rgb565,
///     prelude::*,
///     primitives::{CornerRadii, Rectangle, RoundedRectangle},
///     style::{PrimitiveStyle, PrimitiveStyleBuilder},
/// };
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::default();
///
/// let style = PrimitiveStyleBuilder::new()
///     .stroke_width(5)
///     .stroke_color(Rgb565::RED)
///     .fill_color(Rgb565::GREEN)
///     .build();
///
/// let radii = CornerRadii {
///     top_left: Size::new(10, 12),
///     top_right: Size::new(14, 16),
///     bottom_right: Size::new(18, 20),
///     bottom_left: Size::new(22, 24),
/// };
///
/// RoundedRectangle::new(Rectangle::new(Point::new(5, 5), Size::new(50, 60)), radii)
///     .into_styled(style)
///     .draw(&mut display)?;
/// # Ok::<(), core::convert::Infallible>(())
/// ```
///
/// ## Using `CornerRadiiBuilder`
///
/// This example creates a rounded rectangle 50px wide by 60px tall. Corner radii are set using the
/// [`CornerRadiiBuilder`](../struct.CornerRadiiBuilder.html) builder.
///
/// ```rust
/// use embedded_graphics::{
///     pixelcolor::Rgb565,
///     prelude::*,
///     primitives::{CornerRadii, CornerRadiiBuilder, Rectangle, RoundedRectangle},
///     style::{PrimitiveStyle, PrimitiveStyleBuilder},
/// };
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::default();
///
/// let style = PrimitiveStyleBuilder::new()
///     .stroke_width(5)
///     .stroke_color(Rgb565::RED)
///     .fill_color(Rgb565::GREEN)
///     .build();
///
/// let radii = CornerRadiiBuilder::new()
///     // Set the top left and top right corner radii to 10 x 20px
///     .top(Size::new(10, 20))
///     // Set the bottom right corner radius to 5 x 8px
///     .bottom_right(Size::new(5, 8))
///     .build();
///
/// RoundedRectangle::new(Rectangle::new(Point::new(5, 5), Size::new(50, 60)), radii)
///     .into_styled(style)
///     .draw(&mut display)?;
/// # Ok::<(), core::convert::Infallible>(())
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct RoundedRectangle {
    /// The base rectangle
    pub rectangle: Rectangle,

    /// The radius of each corner
    pub corners: CornerRadii,
}

impl RoundedRectangle {
    /// Creates a new rounded rectangle with the given corner radii.
    ///
    /// The size and position of the rounded rectangle is determined by the given base
    /// rectangle.
    pub fn new(rectangle: Rectangle, corners: CornerRadii) -> Self {
        Self { rectangle, corners }
    }

    /// Creates a new rounded rectangle with equal corner radius for all corners.
    ///
    /// The size and position of the rounded rectangle is determined by the given base
    /// rectangle.
    pub fn with_equal_corners(rectangle: Rectangle, corner_radius: Size) -> Self {
        Self::new(rectangle, CornerRadii::new(corner_radius))
    }

    fn get_confined_corner_quadrant(&self, quadrant: Quadrant) -> EllipseQuadrant {
        let Self {
            rectangle, corners, ..
        } = self;

        let Rectangle { top_left, size, .. } = *rectangle;

        let corners = corners.confine(size);

        match quadrant {
            Quadrant::TopLeft => {
                EllipseQuadrant::new(top_left, corners.top_left, Quadrant::TopLeft)
            }
            Quadrant::TopRight => EllipseQuadrant::new(
                top_left + size.x_axis() - corners.top_right.x_axis(),
                corners.top_right,
                Quadrant::TopRight,
            ),
            Quadrant::BottomRight => EllipseQuadrant::new(
                top_left + size - corners.bottom_right,
                corners.bottom_right,
                Quadrant::BottomRight,
            ),
            Quadrant::BottomLeft => EllipseQuadrant::new(
                top_left + size.y_axis() - corners.bottom_left.y_axis(),
                corners.bottom_left,
                Quadrant::BottomLeft,
            ),
        }
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

impl Primitive for RoundedRectangle {
    type PointsIter = Points;

    fn points(&self) -> Self::PointsIter {
        Points::new(self)
    }
}

impl ContainsPoint for RoundedRectangle {
    fn contains(&self, point: Point) -> bool {
        if !self.rectangle.contains(point) {
            return false;
        }

        let tl = self.get_confined_corner_quadrant(Quadrant::TopLeft);

        if tl.bounding_box().contains(point) {
            return tl.contains(point);
        }

        let tr = self.get_confined_corner_quadrant(Quadrant::TopRight);

        if tr.bounding_box().contains(point) {
            return tr.contains(point);
        }

        let br = self.get_confined_corner_quadrant(Quadrant::BottomRight);

        if br.bounding_box().contains(point) {
            return br.contains(point);
        }

        let bl = self.get_confined_corner_quadrant(Quadrant::BottomLeft);

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

/// Iterator over all points inside the rounded rectangle.
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
        let top_left_ellipse = shape.get_confined_corner_quadrant(Quadrant::TopLeft);
        let top_right_ellipse = shape.get_confined_corner_quadrant(Quadrant::TopRight);
        let bottom_right_ellipse = shape.get_confined_corner_quadrant(Quadrant::BottomRight);
        let bottom_left_ellipse = shape.get_confined_corner_quadrant(Quadrant::BottomLeft);

        Self {
            rect_iter: shape.rectangle.points(),

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
pub struct StyledRoundedRectangleIterator<C>
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
            Size::new(10, 10),
        )
        .into_styled(style);

        assert!(clamped.into_iter().eq(expected.into_iter()));
    }

    #[test]
    fn full_height_corners() {
        let mut display = MockDisplay::new();

        RoundedRectangle::new(
            Rectangle::new(Point::zero(), Size::new(40, 20)),
            CornerRadii {
                top_left: Size::new(20, 20),
                top_right: Size::new(20, 20),
                bottom_right: Size::new(0, 0),
                bottom_left: Size::new(0, 0),
            },
        )
        .into_styled(PrimitiveStyleBuilder::new().fill_color(Rgb888::RED).build())
        .draw(&mut display)
        .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "                RRRRRRRR                ",
                "            RRRRRRRRRRRRRRRR            ",
                "          RRRRRRRRRRRRRRRRRRRR          ",
                "         RRRRRRRRRRRRRRRRRRRRRR         ",
                "       RRRRRRRRRRRRRRRRRRRRRRRRRR       ",
                "      RRRRRRRRRRRRRRRRRRRRRRRRRRRR      ",
                "     RRRRRRRRRRRRRRRRRRRRRRRRRRRRRR     ",
                "    RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR    ",
                "    RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR    ",
                "   RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR   ",
                "  RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR  ",
                "  RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR  ",
                " RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR ",
                " RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR ",
                " RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR ",
                " RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR ",
                "RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR",
                "RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR",
                "RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR",
                "RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR",
            ])
        );
    }

    #[test]
    fn corner_radii_exact_size() {
        let corners = CornerRadii {
            top_left: Size::new(10, 15),
            top_right: Size::new(10, 15),
            bottom_right: Size::new(10, 15),
            bottom_left: Size::new(10, 15),
        };

        assert_eq!(corners.confine(Size::new(20, 30)), corners);
    }

    #[test]
    fn corner_radii_single_overlap() {
        let corners = CornerRadii {
            // Create an overlap of 5px in the Y direction
            top_left: Size::new(10, 20),
            top_right: Size::new(10, 15),
            bottom_right: Size::new(10, 15),
            bottom_left: Size::new(10, 15),
        };

        assert_eq!(
            corners.confine(Size::new(20, 30)),
            // All corners should be shrunk by half the overlap, rounded up
            CornerRadii {
                top_left: Size::new(7, 17),
                top_right: Size::new(7, 12),
                bottom_right: Size::new(7, 12),
                bottom_left: Size::new(7, 12),
            }
        );
    }

    #[test]
    fn corner_radii_1px_overlap() {
        let corners = CornerRadii {
            // 1px overlap in Y
            top_left: Size::new(10, 16),
            // 1px overlap in X
            top_right: Size::new(11, 15),
            bottom_right: Size::new(10, 15),
            bottom_left: Size::new(10, 15),
        };

        assert_eq!(
            corners.confine(Size::new(20, 30)),
            CornerRadii {
                top_left: Size::new(9, 15),
                top_right: Size::new(10, 14),
                bottom_right: Size::new(9, 14),
                bottom_left: Size::new(9, 14),
            }
        );
    }

    #[test]
    fn corner_radii_multiple_overlap() {
        let corners = CornerRadii {
            // Create an overlap of 5px in the Y direction
            top_left: Size::new(10, 20),
            top_right: Size::new(10, 15),
            // Create an overlap of 8px in the X direction
            bottom_right: Size::new(18, 15),
            bottom_left: Size::new(10, 15),
        };

        assert_eq!(
            corners.confine(Size::new(20, 30)),
            // Reduce all corners by (8px / 2) = 4px
            CornerRadii {
                top_left: Size::new(6, 16),
                top_right: Size::new(6, 11),
                bottom_right: Size::new(14, 11),
                bottom_left: Size::new(6, 11),
            }
        );
    }
}
