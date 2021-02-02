//! The arc primitive

mod points;
mod styled;

use crate::{
    geometry::{Angle, AngleUnit, Dimensions, Point, Real, Size, Trigonometry},
    primitives::{common::PlaneSector, Circle, OffsetOutline, PointsIter, Primitive, Rectangle},
    transform::Transform,
};
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

    /// Return the delta between a point on the arc at a given angle and the center point.
    pub(in crate::primitives) fn delta_from_angle(&self, angle: Angle) -> Point {
        let diameter = Real::from(self.diameter);

        Point::new(
            i32::from(angle.cos() * diameter),
            // NOTE: Y coordinate is top-down in e-g, but sine function is only in correct phase
            // with cosine with bottom-up Y axis, hence negation here.
            -i32::from(angle.sin() * diameter),
        )
    }

    /// Whether or not the arc passes through a given angle.
    fn passes_through(&self, angle: Angle) -> bool {
        PlaneSector::new(self.angle_start, self.angle_sweep).contains(self.delta_from_angle(angle))
    }
}

impl OffsetOutline for Arc {
    fn offset(&self, offset: i32) -> Self {
        let circle = self.to_circle().offset(offset);

        Self::from_circle(circle, self.angle_start, self.angle_sweep)
    }
}

impl Primitive for Arc {}

impl PointsIter for Arc {
    type Iter = Points;

    fn points(&self) -> Self::Iter {
        Points::new(self)
    }
}

impl Dimensions for Arc {
    fn bounding_box(&self) -> Rectangle {
        let quadrants = [
            Angle::from_degrees(0.0),
            Angle::from_degrees(90.0),
            Angle::from_degrees(180.0),
            Angle::from_degrees(270.0),
            Angle::from_degrees(360.0),
        ];

        let circle_bb = self.to_circle().bounding_box();

        // 3 quadrants consumed. Bounding box is same as circle
        if self.angle_sweep >= 270.0.deg() {
            return circle_bb;
        }

        // let center = self.center();
        let center = circle_bb.top_left + (circle_bb.size.saturating_sub(Size::new_equal(1)) / 2);

        let start = center + (self.delta_from_angle(self.angle_start)) / 2;
        let end = center + (self.delta_from_angle(self.angle_end())) / 2;

        dbg!(center, start, end);

        let mut tl = start.component_min(end);
        let mut br = start.component_max(end);

        for angle in quadrants.iter().copied() {
            if self.passes_through(angle) {
                tl = tl.component_min(center + self.delta_from_angle(angle) / 2);
                br = br.component_max(center + self.delta_from_angle(angle) / 2);
            }
        }

        dbg!(tl, br);

        // if tl != br {
        //     if tl.x < center.x {
        //         tl.x += 1;
        //     }

        //     if tl.y < center.y {
        //         tl.y += 1;
        //     }
        // }

        Rectangle::with_corners(tl, br)

        // let start = self.delta_from_angle(self.angle_start);
        // let end = self.delta_from_angle(self.angle_end());
        // let center = self.center();

        // let plane_sector = PlaneSector::new(self.angle_start, self.angle_sweep);

        // let (mut min, max) = quadrants
        //     .iter()
        //     .map(|q| self.delta_from_angle(*q))
        //     .filter(|quadrant| plane_sector.contains(*quadrant))
        //     .chain([start, end].iter().cloned())
        //     .fold(
        //         (start.component_min(end), start.component_max(end)),
        //         |acc, point| (acc.0.component_min(point), acc.1.component_max(point)),
        //     );

        // // if min != max {
        // //     if min.x < center.x {
        // //         min.x += 1;
        // //     }

        // //     if min.y < center.y {
        // //         min.y += 1;
        // //     }
        // // }

        // Rectangle::with_corners(min, max).translate(center)
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
            Rectangle::new(Point::new(-11, -16), Size::new(6, 6))
        );
    }

    #[test]
    fn dimensions() {
        let arc = Arc::new(Point::new(5, 15), 10, 0.0.deg(), 90.0.deg());

        assert_eq!(
            arc.bounding_box(),
            Rectangle::new(Point::new(10, 15), Size::new(5, 5))
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

    #[test]
    fn full_circle() {
        // Full circle
        assert_eq!(
            Arc::new(Point::new(10, 10), 10, 0.0.deg(), 360.0.deg()).bounding_box(),
            Rectangle::new(Point::new(10, 10), Size::new(10, 10))
        );

        // Greater than full circle
        assert_eq!(
            Arc::new(Point::new(10, 10), 10, 0.0.deg(), 380.0.deg()).bounding_box(),
            Rectangle::new(Point::new(10, 10), Size::new(10, 10))
        );

        // Two rotations
        assert_eq!(
            Arc::new(Point::new(10, 10), 10, 0.0.deg(), 720.0.deg()).bounding_box(),
            Rectangle::new(Point::new(10, 10), Size::new(10, 10))
        );
    }
}
