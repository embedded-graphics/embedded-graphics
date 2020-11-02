//! The sector primitive

mod points;
mod styled;

use crate::{
    geometry::{Angle, Dimensions, Point, Real, Size, Trigonometry},
    primitives::{
        arc::PlaneSector, line::Line, Arc, Circle, ContainsPoint, OffsetOutline, Primitive,
        Rectangle,
    },
    transform::Transform,
};
pub use points::Points;
pub use styled::StyledPixels;

/// Sector primitive
///
/// # Examples
///
/// ## Create some sectors with different styles
///
/// ```rust
/// use embedded_graphics::{
///     pixelcolor::Rgb565,
///     prelude::*,
///     primitives::Sector,
///     style::{PrimitiveStyle, PrimitiveStyleBuilder},
/// };
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::default();
/// # display.set_allow_overdraw(true);
///
/// // Sector with 1 pixel wide white stroke with top-left point at (10, 20) with a diameter of 30
/// Sector::new(Point::new(10, 20), 30, 0.0.deg(), 90.0.deg())
///     .into_styled(PrimitiveStyle::with_stroke(Rgb565::WHITE, 1))
///     .draw(&mut display)?;
///
/// // Sector with styled stroke and fill with top-left point at (10, 20) with a diameter of 30
/// let style = PrimitiveStyleBuilder::new()
///     .stroke_color(Rgb565::RED)
///     .stroke_width(3)
///     .fill_color(Rgb565::GREEN)
///     .build();
///
/// Sector::new(Point::new(10, 20), 30, 180.0.deg(), -90.0.deg())
///     .into_styled(style)
///     .draw(&mut display)?;
///
/// // Sector with blue fill and no stroke with a translation applied
/// Sector::new(Point::new(10, 20), 30, 0.0.deg(), 90.0.deg())
///     .translate(Point::new(15, 5))
///     .into_styled(PrimitiveStyle::with_fill(Rgb565::BLUE))
///     .draw(&mut display)?;
/// # Ok::<(), core::convert::Infallible>(())
/// ```
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Sector {
    /// Top-left point of the bounding-box of the circle supporting the sector
    pub top_left: Point,

    /// Diameter of the circle supporting the sector
    pub diameter: u32,

    /// Angle at which the sector starts
    pub angle_start: Angle,

    /// Angle defining the sector sweep starting at angle_start
    pub angle_sweep: Angle,
}

impl Sector {
    /// Create a new sector delimited with a top-left point with a specific diameter and start and sweep angles
    pub const fn new(
        top_left: Point,
        diameter: u32,
        angle_start: Angle,
        angle_sweep: Angle,
    ) -> Self {
        Sector {
            top_left,
            diameter,
            angle_start,
            angle_sweep,
        }
    }

    /// Create a new sector centered around a given point with a specific diameter and start and sweep angles
    pub fn with_center(
        center: Point,
        diameter: u32,
        angle_start: Angle,
        angle_sweep: Angle,
    ) -> Self {
        let top_left = center - Size::new(diameter, diameter).center_offset();

        Sector {
            top_left,
            diameter,
            angle_start,
            angle_sweep,
        }
    }

    /// Creates an arc based on a circle.
    ///
    /// The resulting sector will match the `top_left` and `diameter` of the base circle.
    pub fn from_circle(circle: Circle, angle_start: Angle, angle_sweep: Angle) -> Self {
        Sector {
            top_left: circle.top_left,
            diameter: circle.diameter,
            angle_start,
            angle_sweep,
        }
    }

    /// Returns a circle with the same `top_left` and `diameter` as this sector.
    pub fn to_circle(&self) -> Circle {
        Circle::new(self.top_left, self.diameter)
    }

    /// Return the center point of the sector
    pub fn center(&self) -> Point {
        self.to_circle().center()
    }

    /// Return the end angle of the sector
    fn angle_end(&self) -> Angle {
        self.angle_start + self.angle_sweep
    }

    /// Return a `Line` between the sector center and a point on the circumference following a given angle
    fn line_from_angle(&self, angle: Angle) -> Line {
        let center = self.center();
        let radius = Real::from(self.diameter.saturating_sub(1)) / 2.into();

        let point = Point::new(
            center.x + i32::from(angle.cos() * radius),
            center.y - i32::from(angle.sin() * radius),
        );

        Line::new(center, point)
    }
}

impl OffsetOutline for Sector {
    fn offset(&self, offset: i32) -> Self {
        let circle = self.to_circle().offset(offset);

        Self::from_circle(circle, self.angle_start, self.angle_sweep)
    }
}

impl Primitive for Sector {
    type PointsIter = Points;

    fn points(&self) -> Self::PointsIter {
        Points::new(self)
    }
}

impl ContainsPoint for Sector {
    fn contains(&self, point: Point) -> bool {
        if self.to_circle().contains(point) {
            PlaneSector::new(self.center(), self.angle_start, self.angle_sweep).contains(point)
        } else {
            false
        }
    }
}

impl Dimensions for Sector {
    fn bounding_box(&self) -> Rectangle {
        let arc = Arc::new(
            self.top_left,
            self.diameter,
            self.angle_start,
            self.angle_sweep,
        );

        let quadrants = [
            arc.point_from_angle(Angle::from_degrees(0.0)),
            arc.point_from_angle(Angle::from_degrees(90.0)),
            arc.point_from_angle(Angle::from_degrees(180.0)),
            arc.point_from_angle(Angle::from_degrees(270.0)),
            arc.point_from_angle(Angle::from_degrees(360.0)),
        ];

        let start = arc.point_from_angle(self.angle_start);
        let end = arc.point_from_angle(self.angle_end());
        let center = self.center();

        let plane_sector = PlaneSector::new(center, self.angle_start, self.angle_sweep);

        let (min, mut max) = quadrants
            .iter()
            .filter(|quadrant| plane_sector.contains(**quadrant))
            .chain([&start, &end, &center].iter().cloned())
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

impl Transform for Sector {
    /// Translate the sector from its current position to a new position by (x, y) pixels,
    /// returning a new `Sector`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Sector;
    /// # use embedded_graphics::prelude::*;
    /// let sector = Sector::new(Point::new(5, 10), 10, 0.0.deg(), 90.0.deg());
    /// let moved = sector.translate(Point::new(10, 10));
    ///
    /// assert_eq!(moved.top_left, Point::new(15, 20));
    /// ```
    fn translate(&self, by: Point) -> Self {
        Self {
            top_left: self.top_left + by,
            ..*self
        }
    }

    /// Translate the sector from its current position to a new position by (x, y) pixels.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Sector;
    /// # use embedded_graphics::prelude::*;
    /// let mut sector = Sector::new(Point::new(5, 10), 10, 0.0.deg(), 90.0.deg());
    /// sector.translate_mut(Point::new(10, 10));
    ///
    /// assert_eq!(sector.top_left, Point::new(15, 20));
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
        let sector = Sector::new(Point::new(-15, -15), 10, 0.0.deg(), 90.0.deg());

        assert_eq!(
            sector.bounding_box(),
            Rectangle::new(Point::new(-11, -15), Size::new(6, 5))
        );
    }

    #[test]
    fn dimensions() {
        let sector = Sector::new(Point::new(5, 15), 10, 0.0.deg(), 90.0.deg());

        assert_eq!(
            sector.bounding_box(),
            Rectangle::new(Point::new(9, 15), Size::new(6, 5))
        );
    }

    #[test]
    fn it_handles_negative_coordinates() {
        let positive = Sector::new(Point::new(10, 10), 5, 0.0.deg(), 90.0.deg()).points();

        let negative = Sector::new(Point::new(-10, -10), 5, 0.0.deg(), 90.0.deg()).points();

        assert!(negative.eq(positive.map(|p| p - Point::new(20, 20))));
    }

    #[test]
    fn center_is_correct() {
        // odd diameter
        let sector = Sector::new(Point::new(10, 10), 5, 0.0.deg(), 90.0.deg());
        assert_eq!(sector.center(), Point::new(12, 12));

        // even diameter
        let sector = Sector::new(Point::new(10, 10), 6, 0.0.deg(), 90.0.deg());
        assert_eq!(sector.center(), Point::new(12, 12));

        // odd diameter
        let sector = Sector::with_center(Point::new(10, 10), 5, 0.0.deg(), 90.0.deg());
        assert_eq!(sector.center(), Point::new(10, 10));

        // even diameter
        let sector = Sector::with_center(Point::new(10, 10), 6, 0.0.deg(), 90.0.deg());
        assert_eq!(sector.center(), Point::new(10, 10));
    }

    #[test]
    fn contains() {
        let sector = Sector::new(Point::zero(), 10, 0.0.deg(), 90.0.deg());

        let contained_points = Rectangle::new(Point::new(-10, -10), Size::new(30, 30))
            .points()
            .filter(|p| sector.contains(*p));

        assert!(contained_points.eq(sector.points()));
    }

    #[test]
    fn offset() {
        let center = Point::new(5, 7);
        let sector = Sector::with_center(center, 3, 0.0.deg(), 90.0.deg());

        assert_eq!(sector.offset(0), sector);

        assert_eq!(
            sector.offset(1),
            Sector::with_center(center, 5, 0.0.deg(), 90.0.deg())
        );
        assert_eq!(
            sector.offset(2),
            Sector::with_center(center, 7, 0.0.deg(), 90.0.deg())
        );

        assert_eq!(
            sector.offset(-1),
            Sector::with_center(center, 1, 0.0.deg(), 90.0.deg())
        );
        assert_eq!(
            sector.offset(-2),
            Sector::with_center(center, 0, 0.0.deg(), 90.0.deg())
        );
        assert_eq!(
            sector.offset(-3),
            Sector::with_center(center, 0, 0.0.deg(), 90.0.deg())
        );
    }
}
