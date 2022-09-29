//! The ellipse primitive

use crate::{
    geometry::{Dimensions, Point, Size},
    primitives::{circle, ContainsPoint, OffsetOutline, PointsIter, Primitive, Rectangle},
    transform::Transform,
};

mod points;
mod styled;

pub use points::Points;
pub use styled::StyledPixelsIterator;

/// Ellipse primitive
///
/// # Examples
///
/// ## Create some ellipses with different styles
///
/// ```rust
/// use embedded_graphics::{
///     pixelcolor::Rgb565,
///     prelude::*,
///     primitives::{Ellipse, PrimitiveStyle, PrimitiveStyleBuilder},
/// };
/// # use embedded_graphics::mock_display::MockDisplay;
///
/// // Ellipse with 1 pixel wide white stroke with top-left point at (10, 20) with a size of (30, 40)
/// # let mut display = MockDisplay::default();
/// Ellipse::new(Point::new(10, 20), Size::new(30, 40))
///     .into_styled(PrimitiveStyle::with_stroke(Rgb565::WHITE, 1))
///     .draw(&mut display)?;
///
/// // Ellipse with styled stroke and fill with top-left point at (20, 30) with a size of (40, 30)
/// let style = PrimitiveStyleBuilder::new()
///     .stroke_color(Rgb565::RED)
///     .stroke_width(3)
///     .fill_color(Rgb565::GREEN)
///     .build();
///
/// # let mut display = MockDisplay::default();
/// Ellipse::new(Point::new(20, 30), Size::new(40, 30))
///     .into_styled(style)
///     .draw(&mut display)?;
///
/// // Ellipse with blue fill and no stroke with a translation applied
/// # let mut display = MockDisplay::default();
/// Ellipse::new(Point::new(10, 20), Size::new(20, 40))
///     .translate(Point::new(10, -15))
///     .into_styled(PrimitiveStyle::with_fill(Rgb565::BLUE))
///     .draw(&mut display)?;
/// # Ok::<(), core::convert::Infallible>(())
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct Ellipse {
    /// Top-left point of ellipse's bounding box
    pub top_left: Point,

    /// Size of the ellipse
    pub size: Size,
}

impl Ellipse {
    /// Create a new ellipse delimited with a top-left point with a specific size
    pub const fn new(top_left: Point, size: Size) -> Self {
        Ellipse { top_left, size }
    }

    /// Create a new ellipse centered around a given point with a specific size
    pub const fn with_center(center: Point, size: Size) -> Self {
        let top_left = Rectangle::with_center(center, size).top_left;

        Ellipse { top_left, size }
    }

    /// Return the center point of the ellipse
    pub fn center(&self) -> Point {
        self.bounding_box().center()
    }

    /// Return the center point of the ellipse scaled by a factor of 2
    ///
    /// This method is used to accurately calculate the outside edge of the ellipse.
    /// The result is not equivalent to `self.center() * 2` because of rounding.
    fn center_2x(&self) -> Point {
        center_2x(self.top_left, self.size)
    }
}

impl OffsetOutline for Ellipse {
    fn offset(&self, offset: i32) -> Self {
        let size = if offset >= 0 {
            self.size.saturating_add(Size::new_equal(2 * offset as u32))
        } else {
            self.size
                .saturating_sub(Size::new_equal(2 * (-offset) as u32))
        };

        Self::with_center(self.center(), size)
    }
}

/// Return the center point of the ellipse scaled by a factor of 2
///
/// This method is used to accurately calculate the outside edge of the ellipse.
/// The result is not equivalent to `Ellipse::center() * 2` because of rounding.
pub(in crate::primitives) fn center_2x(top_left: Point, size: Size) -> Point {
    let radius = size.saturating_sub(Size::new(1, 1));

    top_left * 2 + radius
}

impl Primitive for Ellipse {}

impl PointsIter for Ellipse {
    type Iter = Points;

    fn points(&self) -> Self::Iter {
        Points::new(self)
    }
}

impl ContainsPoint for Ellipse {
    fn contains(&self, point: Point) -> bool {
        let ellipse_contains = EllipseContains::new(self.size);
        ellipse_contains.contains(point * 2 - self.center_2x())
    }
}

impl Dimensions for Ellipse {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(self.top_left, self.size)
    }
}

impl Transform for Ellipse {
    /// Translate the ellipse from its current position to a new position by (x, y) pixels,
    /// returning a new `Ellipse`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Ellipse;
    /// # use embedded_graphics::prelude::*;
    /// let ellipse = Ellipse::new(Point::new(5, 10), Size::new(10, 15));
    /// let moved = ellipse.translate(Point::new(10, 10));
    ///
    /// assert_eq!(moved.top_left, Point::new(15, 20));
    /// ```
    fn translate(&self, by: Point) -> Self {
        Self {
            top_left: self.top_left + by,
            ..*self
        }
    }

    /// Translate the ellipse from its current position to a new position by (x, y) pixels.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Ellipse;
    /// # use embedded_graphics::prelude::*;
    /// let mut ellipse = Ellipse::new(Point::new(5, 10), Size::new(10, 15));
    /// ellipse.translate_mut(Point::new(10, 10));
    ///
    /// assert_eq!(ellipse.top_left, Point::new(15, 20));
    /// ```
    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.top_left += by;

        self
    }
}

/// Determines if a point is inside an ellipse.
// TODO: Make this available to the user as part of #343
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub(in crate::primitives) struct EllipseContains {
    a: u32,
    b: u32,
    threshold: u32,
}

impl EllipseContains {
    /// Creates an object to determine if a point is inside an ellipse.
    ///
    /// The ellipse is always located in the origin.
    pub const fn new(size: Size) -> Self {
        let Size { width, height } = size;

        let a = width.pow(2);
        let b = height.pow(2);

        // Special case for circles, where width and height are equal
        let threshold = if width == height {
            circle::diameter_to_threshold(width)
        } else {
            b * a
        };

        Self { a, b, threshold }
    }

    /// Returns `true` if the point is inside the ellipse.
    pub const fn contains(&self, point: Point) -> bool {
        let x = point.x.pow(2) as u32;
        let y = point.y.pow(2) as u32;

        // Special case for circles, where width and height are equal
        if self.a == self.b {
            x + y < self.threshold
        } else {
            self.b * x + self.a * y < self.threshold
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::{Point, Size},
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        primitives::ContainsPoint,
    };

    #[test]
    fn contains() {
        let ellipse = Ellipse::new(Point::zero(), Size::new(40, 20));

        let display = MockDisplay::from_points(
            ellipse
                .bounding_box()
                .points()
                .filter(|p| ellipse.contains(*p)),
            BinaryColor::On,
        );

        let expected = MockDisplay::from_points(ellipse.points(), BinaryColor::On);

        display.assert_eq(&expected);
    }

    #[test]
    fn translate() {
        let moved = Ellipse::new(Point::new(4, 6), Size::new(5, 8)).translate(Point::new(3, 5));

        assert_eq!(moved, Ellipse::new(Point::new(7, 11), Size::new(5, 8)));
    }

    #[test]
    fn offset() {
        let center = Point::new(5, 6);
        let ellipse = Ellipse::with_center(center, Size::new(3, 4));

        assert_eq!(ellipse.offset(0), ellipse);

        assert_eq!(
            ellipse.offset(1),
            Ellipse::with_center(center, Size::new(5, 6))
        );
        assert_eq!(
            ellipse.offset(2),
            Ellipse::with_center(center, Size::new(7, 8))
        );

        assert_eq!(
            ellipse.offset(-1),
            Ellipse::with_center(center, Size::new(1, 2))
        );
        assert_eq!(
            ellipse.offset(-2),
            Ellipse::with_center(center, Size::new(0, 0))
        );
        assert_eq!(
            ellipse.offset(-3),
            Ellipse::with_center(center, Size::new(0, 0))
        );
    }
}
