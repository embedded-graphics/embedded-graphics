//! The circle primitive

use crate::{
    geometry::{Dimensions, Point, PointExt, Size},
    primitives::{
        common::DistanceIterator, ContainsPoint, OffsetOutline, PointsIter, Primitive, Rectangle,
    },
    transform::Transform,
};

mod points;
mod styled;

pub use points::Points;
pub use styled::StyledPixelsIterator;

/// Circle primitive
///
/// # Examples
///
/// ## Create some circles with different styles
///
/// ```rust
/// use embedded_graphics::{
///     pixelcolor::Rgb565,
///     prelude::*,
///     primitives::{Circle,  PrimitiveStyle, PrimitiveStyleBuilder},
/// };
/// # use embedded_graphics::mock_display::MockDisplay;
///
/// // Circle with 1 pixel wide white stroke with top-left point at (10, 20) with a diameter of 30
/// # let mut display = MockDisplay::default();
/// Circle::new(Point::new(10, 20), 30)
///     .into_styled(PrimitiveStyle::with_stroke(Rgb565::WHITE, 1))
///     .draw(&mut display)?;
///
/// // Circle with styled stroke and fill with top-left point at (50, 20) with a diameter of 30
/// let style = PrimitiveStyleBuilder::new()
///     .stroke_color(Rgb565::RED)
///     .stroke_width(3)
///     .fill_color(Rgb565::GREEN)
///     .build();
///
/// # let mut display = MockDisplay::default();
/// Circle::new(Point::new(50, 20), 10)
///     .into_styled(style)
///     .draw(&mut display)?;
///
/// // Circle with blue fill and no stroke with a translation applied
/// # let mut display = MockDisplay::default();
/// Circle::new(Point::new(10, 20), 30)
///     .translate(Point::new(20, 10))
///     .into_styled(PrimitiveStyle::with_fill(Rgb565::BLUE))
///     .draw(&mut display)?;
/// # Ok::<(), core::convert::Infallible>(())
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Circle {
    /// Top-left point of circle's bounding box
    pub top_left: Point,

    /// Diameter of the circle
    pub diameter: u32,
}

impl Circle {
    /// Create a new circle delimited with a top-left point with a specific diameter
    pub const fn new(top_left: Point, diameter: u32) -> Self {
        Circle { top_left, diameter }
    }

    /// Create a new circle centered around a given point with a specific diameter
    pub fn with_center(center: Point, diameter: u32) -> Self {
        let top_left = Rectangle::with_center(center, Size::new_equal(diameter)).top_left;

        Circle { top_left, diameter }
    }

    /// Return the center point of the circle
    pub fn center(&self) -> Point {
        self.bounding_box().center()
    }

    /// Return the center point of the circle scaled by a factor of 2
    ///
    /// This method is used to accurately calculate the outside edge of the circle.
    /// The result is not equivalent to `self.center() * 2` because of rounding.
    pub(in crate::primitives) fn center_2x(&self) -> Point {
        // The radius scaled up by a factor of 2 is equal to the diameter
        let radius = self.diameter.saturating_sub(1);

        self.top_left * 2 + Size::new(radius, radius)
    }

    /// Returns the threshold for this circles diameter.
    pub(in crate::primitives) fn threshold(&self) -> u32 {
        diameter_to_threshold(self.diameter)
    }

    /// Returns the squared distance for every point in the bounding box.
    pub(in crate::primitives) fn distances(&self) -> DistanceIterator {
        DistanceIterator::new(self.center_2x(), &self.bounding_box())
    }
}

impl OffsetOutline for Circle {
    fn offset(&self, offset: i32) -> Self {
        let diameter = if offset >= 0 {
            self.diameter.saturating_add(2 * offset as u32)
        } else {
            self.diameter.saturating_sub(2 * (-offset) as u32)
        };

        Self::with_center(self.center(), diameter)
    }
}

impl Primitive for Circle {}

impl PointsIter for Circle {
    type Iter = Points;

    fn points(&self) -> Self::Iter {
        Points::new(self)
    }
}

impl ContainsPoint for Circle {
    fn contains(&self, point: Point) -> bool {
        let delta = self.center_2x() - point * 2;
        let distance = delta.length_squared() as u32;

        distance < self.threshold()
    }
}

impl Dimensions for Circle {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(self.top_left, Size::new_equal(self.diameter))
    }
}

impl Transform for Circle {
    /// Translate the circle from its current position to a new position by (x, y) pixels,
    /// returning a new `Circle`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Circle;
    /// # use embedded_graphics::prelude::*;
    /// let circle = Circle::new(Point::new(5, 10), 10);
    /// let moved = circle.translate(Point::new(10, 10));
    ///
    /// assert_eq!(moved.top_left, Point::new(15, 20));
    /// ```
    fn translate(&self, by: Point) -> Self {
        Self {
            top_left: self.top_left + by,
            ..*self
        }
    }

    /// Translate the circle from its current position to a new position by (x, y) pixels.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Circle;
    /// # use embedded_graphics::prelude::*;
    /// let mut circle = Circle::new(Point::new(5, 10), 10);
    /// circle.translate_mut(Point::new(10, 10));
    ///
    /// assert_eq!(circle.top_left, Point::new(15, 20));
    /// ```
    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.top_left += by;

        self
    }
}

pub(in crate::primitives) fn diameter_to_threshold(diameter: u32) -> u32 {
    if diameter <= 4 {
        diameter.pow(2) - diameter / 2
    } else {
        diameter.pow(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::{Dimensions, Point, Size},
        primitives::ContainsPoint,
    };

    #[test]
    fn negative_dimensions() {
        let circle = Circle::new(Point::new(-15, -15), 20);

        assert_eq!(
            circle.bounding_box(),
            Rectangle::new(Point::new(-15, -15), Size::new(20, 20))
        );
    }

    #[test]
    fn dimensions() {
        let circle = Circle::new(Point::new(5, 15), 10);

        assert_eq!(
            circle.bounding_box(),
            Rectangle::new(Point::new(5, 15), Size::new(10, 10))
        );
    }

    #[test]
    fn center_is_correct() {
        // odd diameter
        let circle = Circle::new(Point::new(10, 10), 5);
        assert_eq!(circle.center(), Point::new(12, 12));

        // even diameter
        let circle = Circle::new(Point::new(10, 10), 6);
        assert_eq!(circle.center(), Point::new(12, 12));

        // odd diameter
        let circle = Circle::with_center(Point::new(10, 10), 5);
        assert_eq!(circle.center(), Point::new(10, 10));

        // even diameter
        let circle = Circle::with_center(Point::new(10, 10), 6);
        assert_eq!(circle.center(), Point::new(10, 10));
    }

    #[test]
    fn contains() {
        let circle = Circle::new(Point::zero(), 5);

        let contained_points = Rectangle::new(Point::new(-10, -10), Size::new(20, 20))
            .points()
            .filter(|p| circle.contains(*p));

        assert!(contained_points.eq(circle.points()));
    }

    #[test]
    fn offset() {
        let center = Point::new(1, 2);
        let circle = Circle::with_center(center, 3);

        assert_eq!(circle.offset(0), circle);

        assert_eq!(circle.offset(1), Circle::with_center(center, 5));
        assert_eq!(circle.offset(2), Circle::with_center(center, 7));

        assert_eq!(circle.offset(-1), Circle::with_center(center, 1));
        assert_eq!(circle.offset(-2), Circle::with_center(center, 0));
        assert_eq!(circle.offset(-3), Circle::with_center(center, 0));
    }
}
