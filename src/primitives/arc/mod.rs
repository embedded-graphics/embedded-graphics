//! The arc primitive

use crate::{
    geometry::{Angle, Dimensions, Point, Size},
    primitives::{Circle, PointsIter, Primitive, Rectangle},
    transform::Transform,
};

mod points;
mod styled;

use crate::geometry::{Real, Trigonometry};
pub use points::Points;
pub use styled::StyledPixelsIterator;

/// Arc primitive
///
/// # Examples
///
/// ## Create some arcs with different styles
///
/// ```rust
/// use embedded_graphics::{
///     pixelcolor::Rgb565,
///     prelude::*,
///     primitives::{Arc, PrimitiveStyle, PrimitiveStyleBuilder},
/// };
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::default();
///
/// // Arc with 1 pixel wide white stroke with top-left point at (10, 20) with a diameter of 30
/// Arc::new(Point::new(10, 20), 30, 0.0.deg(), 90.0.deg())
///     .into_styled(PrimitiveStyle::with_stroke(Rgb565::WHITE, 1))
///     .draw(&mut display)?;
///
/// // Arc with styled stroke with top-left point at (15, 25) with a diameter of 20
/// let style = PrimitiveStyleBuilder::new()
///     .stroke_color(Rgb565::RED)
///     .stroke_width(3)
///     .build();
///
/// Arc::new(Point::new(15, 25), 20, 180.0.deg(), -90.0.deg())
///     .into_styled(style)
///     .draw(&mut display)?;
/// # Ok::<(), core::convert::Infallible>(())
/// ```
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct Arc {
    /// Top-left point of the bounding-box of the circle supporting the arc
    pub top_left: Point,

    /// Diameter of the circle supporting the arc
    pub diameter: u32,

    /// Angle at which the arc starts
    pub angle_start: Angle,

    /// Angle defining the arc sweep starting at angle_start
    pub angle_sweep: Angle,
}

impl Arc {
    /// Create a new arc delimited with a top-left point with a specific diameter and start and sweep angles
    pub const fn new(
        top_left: Point,
        diameter: u32,
        angle_start: Angle,
        angle_sweep: Angle,
    ) -> Self {
        Arc {
            top_left,
            diameter,
            angle_start,
            angle_sweep,
        }
    }

    /// Create a new arc centered around a given point with a specific diameter and start and sweep angles
    pub const fn with_center(
        center: Point,
        diameter: u32,
        angle_start: Angle,
        angle_sweep: Angle,
    ) -> Self {
        Self::from_circle(
            Circle::with_center(center, diameter),
            angle_start,
            angle_sweep,
        )
    }

    /// Creates an arc based on a circle.
    ///
    /// The resulting arc will match the `top_left` and `diameter` of the base circle.
    pub const fn from_circle(circle: Circle, angle_start: Angle, angle_sweep: Angle) -> Self {
        Self {
            top_left: circle.top_left,
            diameter: circle.diameter,
            angle_start,
            angle_sweep,
        }
    }

    /// Returns a circle with the same `top_left` and `diameter` as this arc.
    pub const fn to_circle(&self) -> Circle {
        Circle::new(self.top_left, self.diameter)
    }

    /// Returns the center point of the arc.
    pub fn center(&self) -> Point {
        self.to_circle().center()
    }

    fn radius(&self) -> Real {
        Real::from(self.diameter.saturating_sub(1)) / 2.into()
    }

    /// Returns the end angle of the arc.
    pub fn angle_end(&self) -> Angle {
        self.angle_start + self.angle_sweep
    }

    /// Returns the Point from a certain angle.
    pub fn point_from_angle(&self, angle: &Angle) -> Point {
        let center = self.center();
        let radius = self.radius();

        Point::new(
            center.x + i32::from(angle.cos() * radius),
            center.y - i32::from(angle.sin() * radius),
        )
    }

    /// Returns the Point from the start angle.
    pub fn point_start(&self) -> Point {
        self.point_from_angle(&self.angle_start)
    }

    /// Returns the Point from the end angle.
    pub fn point_end(&self) -> Point {
        self.point_from_angle(&self.angle_end())
    }

    /// Whether or not the arc passes through a given angle.
    fn passes_through(&self, angle: &Angle) -> bool {
        let start_angle = self.angle_start.to_degrees();
        let end_angle = self.angle_end().to_degrees();

        (start_angle < angle.to_degrees()
            && (end_angle > angle.to_degrees() || end_angle <= start_angle))
            || (start_angle > angle.to_degrees()
                && end_angle > angle.to_degrees()
                && end_angle <= start_angle)
    }
}

impl Primitive for Arc {}

impl PointsIter for Arc {
    type Iter = Points;

    fn points(&self) -> Self::Iter {
        Points::new(self)
    }
}

// TODO Check rect.styled_bounding_box()
impl Dimensions for Arc {
    fn bounding_box(&self) -> Rectangle {
        // Create a rectangle between the start and the end angle points.
        let rect = Rectangle::with_corners(self.point_start(), self.point_end());
        let mut p1 = rect.top_left;
        let mut p2 = p1 + rect.size.x_axis() + rect.size.y_axis();

        // Extend the rectangle if the arc passes through the circle bounding box borders.
        let center = self.center();
        let radius = i32::from(self.radius());

        if self.passes_through(&Angle::from_degrees(90.0)) {
            p1 = Point::new(p1.x, center.y - radius);
        }

        if self.passes_through(&Angle::from_degrees(180.0)) {
            p1 = Point::new(center.x - radius, p1.y);
        }

        if self.passes_through(&Angle::from_degrees(270.0)) {
            p2 = Point::new(p2.x, center.y + radius);
        }

        if self.passes_through(&Angle::from_degrees(360.0)) {
            p2 = Point::new(center.x + radius, p2.y);
        }

        Rectangle::with_corners(p1, p2)
    }
}

impl Transform for Arc {
    /// Translate the arc from its current position to a new position by (x, y) pixels,
    /// returning a new `Arc`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Arc;
    /// # use embedded_graphics::prelude::*;
    /// let arc = Arc::new(Point::new(5, 10), 10, 0.0.deg(), 90.0.deg());
    /// let moved = arc.translate(Point::new(10, 10));
    ///
    /// assert_eq!(moved.top_left, Point::new(15, 20));
    /// ```
    fn translate(&self, by: Point) -> Self {
        Self {
            top_left: self.top_left + by,
            ..*self
        }
    }

    /// Translate the arc from its current position to a new position by (x, y) pixels.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Arc;
    /// # use embedded_graphics::prelude::*;
    /// let mut arc = Arc::new(Point::new(5, 10), 10, 0.0.deg(), 90.0.deg());
    /// arc.translate_mut(Point::new(10, 10));
    ///
    /// assert_eq!(arc.top_left, Point::new(15, 20));
    /// ```
    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.top_left += by;

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::AngleUnit;

    #[test]
    fn negative_dimensions() {
        let arc = Arc::new(Point::new(-15, -15), 10, 0.0.deg(), 90.0.deg());

        assert_eq!(
            arc.bounding_box(),
            Rectangle::new(Point::new(-15, -15), Size::new(10, 10))
        );
    }

    #[test]
    fn dimensions() {
        let arc = Arc::new(Point::new(5, 15), 10, 0.0.deg(), 90.0.deg());

        assert_eq!(
            arc.bounding_box(),
            Rectangle::new(Point::new(5, 15), Size::new(10, 10))
        );
    }

    #[test]
    fn it_handles_negative_coordinates() {
        let positive = Arc::new(Point::new(10, 10), 5, 0.0.deg(), 90.0.deg()).points();

        let negative = Arc::new(Point::new(-10, -10), 5, 0.0.deg(), 90.0.deg()).points();

        assert!(negative.eq(positive.map(|p| p - Point::new(20, 20))));
    }

    #[test]
    fn center_is_correct() {
        // odd diameter
        let arc = Arc::new(Point::new(10, 10), 5, 0.0.deg(), 90.0.deg());
        assert_eq!(arc.center(), Point::new(12, 12));

        // even diameter
        let arc = Arc::new(Point::new(10, 10), 6, 0.0.deg(), 90.0.deg());
        assert_eq!(arc.center(), Point::new(12, 12));

        // odd diameter
        let arc = Arc::with_center(Point::new(10, 10), 5, 0.0.deg(), 90.0.deg());
        assert_eq!(arc.center(), Point::new(10, 10));

        // even diameter
        let arc = Arc::with_center(Point::new(10, 10), 6, 0.0.deg(), 90.0.deg());
        assert_eq!(arc.center(), Point::new(10, 10));
    }
}
