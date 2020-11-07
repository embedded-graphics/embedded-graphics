//! The arc primitive

mod linear_equation;
mod plane_sector;
mod points;
mod styled;

use crate::{
    geometry::{Angle, Dimensions, Point, Real, Trigonometry},
    primitives::{Circle, OffsetOutline, Primitive, Rectangle},
    transform::Transform,
};
pub(in crate::primitives) use linear_equation::LinearEquation;
pub(in crate::primitives) use plane_sector::{PlaneSector, PlaneSectorIterator};
pub use points::Points;
pub use styled::StyledPixels;

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
///     primitives::Arc,
///     style::{PrimitiveStyle, PrimitiveStyleBuilder},
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
    pub fn with_center(
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
    pub fn from_circle(circle: Circle, angle_start: Angle, angle_sweep: Angle) -> Self {
        Self {
            top_left: circle.top_left,
            diameter: circle.diameter,
            angle_start,
            angle_sweep,
        }
    }

    /// Returns a circle with the same `top_left` and `diameter` as this arc.
    pub fn to_circle(&self) -> Circle {
        Circle::new(self.top_left, self.diameter)
    }

    /// Return the center point of the arc.
    pub fn center(&self) -> Point {
        self.to_circle().center()
    }

    /// Return the end angle of the arc
    fn angle_end(&self) -> Angle {
        self.angle_start + self.angle_sweep
    }

    /// Return a point on the arc from a given angle
    pub(in crate::primitives) fn point_from_angle(&self, angle: Angle) -> Point {
        let center = self.center();
        let radius = Real::from(self.diameter.saturating_sub(1)) / 2.into();

        Point::new(
            center.x + i32::from(angle.cos() * radius),
            center.y - i32::from(angle.sin() * radius),
        )
    }
}

impl OffsetOutline for Arc {
    fn offset(&self, offset: i32) -> Self {
        let circle = self.to_circle().offset(offset);

        Self::from_circle(circle, self.angle_start, self.angle_sweep)
    }
}

impl Primitive for Arc {
    type PointsIter = Points;

    fn points(&self) -> Self::PointsIter {
        Points::new(self)
    }
}

impl Dimensions for Arc {
    // https://stackoverflow.com/questions/1336663/2d-bounding-box-of-a-sector
    fn bounding_box(&self) -> Rectangle {
        let quadrants = [
            self.point_from_angle(Angle::from_degrees(0.0)),
            self.point_from_angle(Angle::from_degrees(90.0)),
            self.point_from_angle(Angle::from_degrees(180.0)),
            self.point_from_angle(Angle::from_degrees(270.0)),
            self.point_from_angle(Angle::from_degrees(360.0)),
        ];

        let start = self.point_from_angle(self.angle_start);
        let end = self.point_from_angle(self.angle_end());
        let center = self.center();

        let plane_sector = PlaneSector::new(center, self.angle_start, self.angle_sweep);

        let (min, mut max) = quadrants
            .iter()
            .filter(|quadrant| plane_sector.contains(**quadrant))
            .chain([&start, &end].iter().cloned())
            .fold(
                (start.component_min(end), start.component_max(end)),
                |acc, point| (acc.0.component_min(*point), acc.1.component_max(*point)),
            );

        if min != max {
            if max.x > center.x {
                max.x += 1;
            }

            if max.y > center.y {
                max.y += 1;
            }
        }

        Rectangle::with_corners(min, max)
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
    use crate::geometry::{AngleUnit, Size};

    #[test]
    fn negative_dimensions() {
        let arc = Arc::new(Point::new(-15, -15), 10, 0.0.deg(), 90.0.deg());

        assert_eq!(
            arc.bounding_box(),
            Rectangle::new(Point::new(-11, -15), Size::new(6, 5))
        );
    }

    #[test]
    fn dimensions() {
        let arc = Arc::new(Point::new(5, 15), 10, 0.0.deg(), 90.0.deg());

        assert_eq!(
            arc.bounding_box(),
            Rectangle::new(Point::new(9, 15), Size::new(6, 5))
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
