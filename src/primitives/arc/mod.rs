//! The arc primitive

use crate::{
    geometry::{Angle, Dimensions, Point, Size},
    primitives::{Circle, PointsIter, Primitive, Rectangle},
    transform::Transform,
};

mod points;
mod styled;

use crate::geometry::Trigonometry;
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

    /// Return the center point of the arc.
    pub fn center(&self) -> Point {
        self.bounding_box().center()
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
        // Center
        let bb = Rectangle::new(self.top_left, Size::new(self.diameter, self.diameter));
        let cx: f32 = bb.center().x as f32;
        let cy: f32 = bb.center().y as f32;
        let chord = (self.diameter as f32) / 2.0;

        // Starting point position
        let start_cos: f32 = self.angle_start.cos().into();
        let start_sin: f32 = self.angle_start.sin().into();

        let start_x = cx + chord * start_cos;
        let start_y = cy - chord * start_sin;

        // End point position
        let end_cos: f32 = (self.angle_start + self.angle_sweep).cos().into();
        let end_sin: f32 = (self.angle_start + self.angle_sweep).sin().into();

        let end_x = cx + ((self.diameter as f32) / 2.0) * end_cos;
        let end_y = cy - ((self.diameter as f32) / 2.0) * end_sin;

        let mut top: Option<f32> = None;
        let mut right: Option<f32> = None;
        let mut bottom: Option<f32> = None;
        let mut left: Option<f32> = None;

        let start_angle = self.angle_start.normalize().to_degrees();
        let end_angle = (self.angle_sweep + self.angle_start)
            .normalize()
            .to_degrees();

        if end_angle <= start_angle {
            right = Some(cx + chord);
        }

        if (start_angle < 90.0 && (end_angle > 90.0 || end_angle <= start_angle))
            || (start_angle > 90.0 && end_angle > 90.0 && end_angle <= start_angle)
        {
            top = Some(cy - chord);
        }

        if (start_angle < 180.0 && (end_angle > 180.0 || end_angle <= start_angle))
            || (start_angle > 180.0 && end_angle > 180.0 && end_angle <= start_angle)
        {
            left = Some(cx - chord);
        }

        if (start_angle < 270.0 && (end_angle > 270.0 || end_angle <= start_angle))
            || (start_angle > 270.0 && end_angle > 270.0 && end_angle <= start_angle)
        {
            bottom = Some(cy + chord);
        }

        let rect = Rectangle::with_corners(
            Point::new(start_x as i32, start_y as i32),
            Point::new(end_x as i32, end_y as i32),
        );

        let mut p1 = rect.top_left;
        let mut p2 = p1 + rect.size.x_axis() + rect.size.y_axis();

        if let Some(top) = top {
            p1 = Point::new(p1.x, top as i32);
        }

        if let Some(left) = left {
            p1 = Point::new(left as i32, p1.y);
        }

        if let Some(bottom) = bottom {
            p2 = Point::new(p2.x, bottom as i32);
        }

        if let Some(right) = right {
            p2 = Point::new(right as i32, p2.y);
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
