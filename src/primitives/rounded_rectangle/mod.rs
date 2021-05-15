//! The rounded rectangle primitive.

use core::ops::Range;

use crate::{
    geometry::{Dimensions, Point, Size},
    primitives::{rectangle::Rectangle, ContainsPoint, OffsetOutline, PointsIter, Primitive},
    transform::Transform,
};

mod corner_radii;
mod ellipse_quadrant;
mod points;
mod styled;

pub use corner_radii::{CornerRadii, CornerRadiiBuilder};
use ellipse_quadrant::{EllipseQuadrant, Quadrant};
pub use points::Points;
pub use styled::StyledPixelsIterator;

/// Rounded rectangle primitive.
///
/// Creates a rectangle with rounded corners. Corners can be circular or elliptical in shape, and
/// each corner may have a separate radius applied to it. To create a rounded rectangle with the same
/// radius for each corner, use the [`with_equal_corners`](RoundedRectangle::with_equal_corners()) method.
///
/// Rounded rectangles with different radii for each corner can be created by passing a
/// [`CornerRadii`](super::CornerRadii) configuration struct to the [`new`](RoundedRectangle::new())
/// method.
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
/// [`with_equal_corners`](RoundedRectangle::with_equal_corners()), all corners are given the same 10px circular
/// radius. The rectangle is drawn using a solid green fill with a 5px red stroke.
///
/// ```rust
/// use embedded_graphics::{
///     pixelcolor::Rgb565,
///     prelude::*,
///     primitives::{Rectangle, RoundedRectangle, PrimitiveStyle, PrimitiveStyleBuilder},
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
///     Rectangle::new(Point::new(5, 5), Size::new(40, 50)),
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
/// radius in the x and y direction by creating a [`CornerRadii`](super::CornerRadii)
/// object and passing that to [`RoundedRectangle::new`](RoundedRectangle::new()).
///
/// ```rust
/// use embedded_graphics::{
///     pixelcolor::Rgb565,
///     prelude::*,
///     primitives::{CornerRadiiBuilder, Rectangle, RoundedRectangle, PrimitiveStyle, PrimitiveStyleBuilder},
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
///     .top_left(Size::new(5, 6))
///     .top_right(Size::new(7, 8))
///     .bottom_right(Size::new(9, 10))
///     .bottom_left(Size::new(11, 12))
///     .build();
///
/// RoundedRectangle::new(Rectangle::new(Point::new(5, 5), Size::new(40, 50)), radii)
///     .into_styled(style)
///     .draw(&mut display)?;
/// # Ok::<(), core::convert::Infallible>(())
/// ```
///
/// ## Using `CornerRadiiBuilder`
///
/// This example creates a rounded rectangle 50px wide by 60px tall. Corner radii are set using the
/// [`CornerRadiiBuilder`](super::CornerRadiiBuilder) builder.
///
/// ```rust
/// use embedded_graphics::{
///     pixelcolor::Rgb565,
///     prelude::*,
///     primitives::{CornerRadii, CornerRadiiBuilder, Rectangle, RoundedRectangle, PrimitiveStyle, PrimitiveStyleBuilder},
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
/// RoundedRectangle::new(Rectangle::new(Point::new(5, 5), Size::new(40, 50)), radii)
///     .into_styled(style)
///     .draw(&mut display)?;
/// # Ok::<(), core::convert::Infallible>(())
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
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
    pub const fn new(rectangle: Rectangle, corners: CornerRadii) -> Self {
        Self { rectangle, corners }
    }

    /// Creates a new rounded rectangle with equal corner radius for all corners.
    ///
    /// The size and position of the rounded rectangle is determined by the given base
    /// rectangle.
    pub const fn with_equal_corners(rectangle: Rectangle, corner_radius: Size) -> Self {
        Self::new(rectangle, CornerRadii::new(corner_radius))
    }

    /// Return the rounded rectangle with confined corner radii.
    ///
    /// This method will return a rounded rectangle of the same width and height, but with all
    /// corner radii confined to fit within its base rectangle.
    ///
    /// Calling this method is not necessary when using operations provided by embedded-graphics
    /// (`.into_styled()`, `.contains()`, etc) as these confine the corner radii internally.
    ///
    /// # Examples
    ///
    /// ## Confine corner radii that are too large
    ///
    /// This example creates a rounded rectangle 50px x 60px in size. Each corner is set to an equal
    /// radius of 40px x 40px. Each edge of the rectangle would thus need to be at least 80px long
    /// to contain all corner radii completely. By using `confine_radii`, the corner radii are
    /// reduced to 25px x 25px so that they fit within the 50px x 60px base rectangle.
    ///
    /// ```rust
    /// use embedded_graphics::{
    ///     geometry::{Point, Size},
    ///     primitives::{CornerRadii, CornerRadiiBuilder, Rectangle, RoundedRectangle},
    /// };
    ///
    /// let radii = CornerRadiiBuilder::new().all(Size::new(40, 40)).build();
    ///
    /// let base_rectangle = Rectangle::new(Point::zero(), Size::new(50, 60));
    ///
    /// let rounded_rectangle = RoundedRectangle::new(base_rectangle, radii);
    ///
    /// let confined = rounded_rectangle.confine_radii();
    ///
    /// assert_eq!(
    ///     confined.corners,
    ///     CornerRadii {
    ///         top_left: Size::new(25, 25),
    ///         top_right: Size::new(25, 25),
    ///         bottom_right: Size::new(25, 25),
    ///         bottom_left: Size::new(25, 25),
    ///     }
    /// );
    /// ```
    pub fn confine_radii(&self) -> Self {
        Self::new(self.rectangle, self.corners.confine(self.rectangle.size))
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
}

impl OffsetOutline for RoundedRectangle {
    fn offset(&self, offset: i32) -> Self {
        let rectangle = self.rectangle.offset(offset);

        let corners = if offset >= 0 {
            let corner_offset = Size::new_equal(offset as u32);

            CornerRadii {
                top_left: self.corners.top_left.saturating_add(corner_offset),
                top_right: self.corners.top_right.saturating_add(corner_offset),
                bottom_right: self.corners.bottom_right.saturating_add(corner_offset),
                bottom_left: self.corners.bottom_left.saturating_add(corner_offset),
            }
        } else {
            let corner_offset = Size::new_equal((-offset) as u32);

            CornerRadii {
                top_left: self.corners.top_left.saturating_sub(corner_offset),
                top_right: self.corners.top_right.saturating_sub(corner_offset),
                bottom_right: self.corners.bottom_right.saturating_sub(corner_offset),
                bottom_left: self.corners.bottom_left.saturating_sub(corner_offset),
            }
        };

        Self::new(rectangle, corners)
    }
}

impl Primitive for RoundedRectangle {}

impl PointsIter for RoundedRectangle {
    type Iter = Points;

    fn points(&self) -> Self::Iter {
        Points::new(self)
    }
}

impl ContainsPoint for RoundedRectangle {
    fn contains(&self, point: Point) -> bool {
        let rounded_rectangle_contains = RoundedRectangleContains::new(self);
        rounded_rectangle_contains.contains(point)
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

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub(in crate::primitives) struct RoundedRectangleContains {
    /// Bounding box rows.
    rows: Range<i32>,
    /// Bounding box columns.
    columns: Range<i32>,

    /// Rows that don't belong to a corner radius on the left side.
    straight_rows_left: Range<i32>,
    /// Rows that don't belong to a corner radius on the right side.
    straight_rows_right: Range<i32>,

    /// Confined top left corner ellipse.
    top_left: EllipseQuadrant,
    /// Confined top right corner ellipse.
    top_right: EllipseQuadrant,
    /// Confined bottom left corner ellipse.
    bottom_left: EllipseQuadrant,
    /// Confined bottom right corner ellipse.
    bottom_right: EllipseQuadrant,
}

impl RoundedRectangleContains {
    pub fn new(rounded_rectangle: &RoundedRectangle) -> Self {
        let top_left = rounded_rectangle.get_confined_corner_quadrant(Quadrant::TopLeft);
        let top_right = rounded_rectangle.get_confined_corner_quadrant(Quadrant::TopRight);
        let bottom_left = rounded_rectangle.get_confined_corner_quadrant(Quadrant::BottomLeft);
        let bottom_right = rounded_rectangle.get_confined_corner_quadrant(Quadrant::BottomRight);

        let rows = rounded_rectangle.rectangle.rows();
        let columns = rounded_rectangle.rectangle.columns();

        let straight_rows_left = (rows.start + top_left.bounding_box().size.height as i32)
            ..(rows.end - bottom_left.bounding_box().size.height as i32);
        let straight_rows_right = (rows.start + top_right.bounding_box().size.height as i32)
            ..(rows.end - bottom_right.bounding_box().size.height as i32);

        Self {
            rows,
            columns,

            straight_rows_left,
            straight_rows_right,

            top_left,
            top_right,
            bottom_left,
            bottom_right,
        }
    }

    pub fn contains(&self, point: Point) -> bool {
        if !(self.rows.contains(&point.y) && self.columns.contains(&point.x)) {
            return false;
        }

        if point.y < self.straight_rows_left.start
            && point.x < self.top_left.bounding_box().columns().end
        {
            return self.top_left.contains(point);
        }

        if point.y < self.straight_rows_right.start
            && point.x >= self.top_right.bounding_box().columns().start
        {
            return self.top_right.contains(point);
        }

        if point.y >= self.straight_rows_left.end
            && point.x < self.bottom_left.bounding_box().columns().end
        {
            return self.bottom_left.contains(point);
        }

        if point.y >= self.straight_rows_right.end
            && point.x >= self.bottom_right.bounding_box().columns().start
        {
            return self.bottom_right.contains(point);
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::{Point, Size},
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        primitives::CornerRadiiBuilder,
    };

    #[test]
    fn clamp_radius_at_rect_size() {
        let clamped = RoundedRectangle::with_equal_corners(
            Rectangle::new(Point::zero(), Size::new(20, 30)),
            Size::new_equal(50),
        )
        .points();

        let expected = RoundedRectangle::with_equal_corners(
            Rectangle::new(Point::zero(), Size::new(20, 30)),
            Size::new_equal(10),
        )
        .points();

        assert!(clamped.eq(expected));
    }

    #[test]
    fn large_bottom_right_corner() {
        let radii = CornerRadiiBuilder::new()
            .all(Size::new_equal(20))
            .bottom_right(Size::new(200, 200))
            .build();

        let base_rectangle = Rectangle::with_corners(Point::new_equal(20), Point::new_equal(100));

        let rounded_rectangle = RoundedRectangle::new(base_rectangle, radii);

        let confined = rounded_rectangle.confine_radii();

        assert_eq!(
            confined,
            RoundedRectangle {
                rectangle: base_rectangle,
                corners: CornerRadii {
                    top_left: Size::new_equal(7),
                    top_right: Size::new_equal(7),
                    bottom_right: Size::new_equal(73),
                    bottom_left: Size::new_equal(7),
                }
            }
        );
    }

    #[test]
    fn offset() {
        let center = Point::new(10, 20);
        let rect = Rectangle::with_center(center, Size::new(3, 4));
        let rounded = RoundedRectangle::with_equal_corners(rect, Size::new(2, 3));

        assert_eq!(rounded.offset(0), rounded);

        assert_eq!(
            rounded.offset(1),
            RoundedRectangle::with_equal_corners(
                Rectangle::with_center(center, Size::new(5, 6)),
                Size::new(3, 4)
            ),
        );
        assert_eq!(
            rounded.offset(2),
            RoundedRectangle::with_equal_corners(
                Rectangle::with_center(center, Size::new(7, 8)),
                Size::new(4, 5)
            ),
        );

        assert_eq!(
            rounded.offset(-1),
            RoundedRectangle::with_equal_corners(
                Rectangle::with_center(center, Size::new(1, 2)),
                Size::new(1, 2)
            ),
        );
        assert_eq!(
            rounded.offset(-2),
            RoundedRectangle::with_equal_corners(
                Rectangle::with_center(center, Size::new(0, 0)),
                Size::new(0, 1)
            ),
        );
        assert_eq!(
            rounded.offset(-3),
            RoundedRectangle::with_equal_corners(
                Rectangle::with_center(center, Size::new(0, 0)),
                Size::new(0, 0)
            ),
        );
    }

    #[test]
    fn contains_equal_corners() {
        let rounded_rectangle = RoundedRectangle::with_equal_corners(
            Rectangle::new(Point::new(1, 2), Size::new(20, 10)),
            Size::new(8, 4),
        );

        let expected = MockDisplay::from_points(rounded_rectangle.points(), BinaryColor::On);

        let display = MockDisplay::from_points(
            rounded_rectangle
                .rectangle
                .offset(10)
                .points()
                .filter(|p| rounded_rectangle.contains(*p)),
            BinaryColor::On,
        );
        display.assert_eq(&expected);
    }

    #[test]
    fn contains_different_corners() {
        let rounded_rectangle = RoundedRectangle::new(
            Rectangle::new(Point::new(1, 2), Size::new(25, 10)),
            CornerRadiiBuilder::new()
                .top_left(Size::new_equal(10))
                .bottom_right(Size::new_equal(10))
                .build(),
        );

        let expected = MockDisplay::from_points(rounded_rectangle.points(), BinaryColor::On);

        let display = MockDisplay::from_points(
            rounded_rectangle
                .rectangle
                .offset(10)
                .points()
                .filter(|p| rounded_rectangle.contains(*p)),
            BinaryColor::On,
        );
        display.assert_eq(&expected);
    }
}
