//! The arc primitive

use crate::{
    drawable::{Drawable, Pixel},
    geometry::{angle_consts::*, Angle, Dimensions, Point, Real, Size, Trigonometry},
    pixelcolor::PixelColor,
    primitives::{circle, circle::DistanceIterator, Primitive, Rectangle, Styled},
    style::PrimitiveStyle,
    transform::Transform,
    DrawTarget,
};

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
        let top_left = center - Size::new_equal(diameter).center_offset();

        Arc {
            top_left,
            diameter,
            angle_start,
            angle_sweep,
        }
    }

    /// Return the center point of the arc
    pub fn center(&self) -> Point {
        self.bounding_box().center()
    }

    /// Return the center point of the arc scaled by a factor of 2
    ///
    /// This method is used to accurately calculate the outside edge of the arc.
    /// The result is not equivalent to `self.center() * 2` because of rounding.
    fn center_2x(&self) -> Point {
        // The radius scaled up by a factor of 2 is equal to the diameter
        let radius = self.diameter.saturating_sub(1);

        self.top_left * 2 + Size::new(radius, radius)
    }

    pub(crate) fn expand(&self, offset: u32) -> Self {
        let diameter = self.diameter.saturating_add(2 * offset);

        Self::with_center(self.center(), diameter, self.angle_start, self.angle_sweep)
    }

    pub(crate) fn shrink(&self, offset: u32) -> Self {
        let diameter = self.diameter.saturating_sub(2 * offset);

        Self::with_center(self.center(), diameter, self.angle_start, self.angle_sweep)
    }
}

impl Primitive for Arc {
    type PointsIter = Points;

    fn points(&self) -> Self::PointsIter {
        Points::new(self)
    }
}

impl Dimensions for Arc {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(self.top_left, Size::new(self.diameter, self.diameter))
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

/// Iterator over all points on the arc line.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Points {
    iter: DistanceIterator<PlaneSectorIterator>,

    outer_threshold: u32,
    inner_threshold: u32,
}

impl Points {
    fn new(arc: &Arc) -> Self {
        let outer_diameter = arc.diameter;
        let inner_diameter = outer_diameter.saturating_sub(2);

        let inner_threshold = circle::diameter_to_threshold(inner_diameter);
        let outer_threshold = circle::diameter_to_threshold(outer_diameter);

        let iter = DistanceIterator::new(
            arc.center_2x(),
            PlaneSectorIterator::new(arc, arc.center(), arc.angle_start, arc.angle_sweep),
        );

        Self {
            iter,
            outer_threshold,
            inner_threshold,
        }
    }
}

impl Iterator for Points {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let outer_threshold = self.outer_threshold;
        let inner_threshold = self.inner_threshold;

        self.iter
            .find(|(_, distance)| *distance < outer_threshold && *distance >= inner_threshold)
            .map(|(point, _)| point)
    }
}

/// Pixel iterator for each pixel in the arc border
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct StyledArcIterator<C>
where
    C: PixelColor,
{
    iter: DistanceIterator<PlaneSectorIterator>,

    outer_threshold: u32,
    inner_threshold: u32,

    stroke_color: Option<C>,
}

impl<C> StyledArcIterator<C>
where
    C: PixelColor,
{
    fn new(styled: &Styled<Arc, PrimitiveStyle<C>>) -> Self {
        let Styled { primitive, style } = styled;

        let stroke_area = primitive.expand(style.outside_stroke_width());
        let fill_area = primitive.shrink(style.inside_stroke_width());

        let inner_threshold = circle::diameter_to_threshold(fill_area.diameter);
        let outer_threshold = circle::diameter_to_threshold(stroke_area.diameter);

        let iter = if !styled.style.is_transparent() {
            DistanceIterator::new(
                stroke_area.center_2x(),
                PlaneSectorIterator::new(
                    &stroke_area,
                    stroke_area.center(),
                    stroke_area.angle_start,
                    stroke_area.angle_sweep,
                ),
            )
        } else {
            DistanceIterator::new(Point::zero(), PlaneSectorIterator::empty())
        };

        Self {
            iter,
            outer_threshold,
            inner_threshold,
            stroke_color: styled.style.stroke_color,
        }
    }
}

impl<C> Iterator for StyledArcIterator<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        let stroke_color = self.stroke_color?;
        let outer_threshold = self.outer_threshold;
        let inner_threshold = self.inner_threshold;

        self.iter
            .find(|(_, distance)| *distance < outer_threshold && *distance >= inner_threshold)
            .map(|(point, _)| Pixel(point, stroke_color))
    }
}

impl<'a, C: 'a> Drawable<C> for &Styled<Arc, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    fn draw<D: DrawTarget<Color = C>>(self, display: &mut D) -> Result<(), D::Error> {
        display.draw_iter(self)
    }
}

impl<'a, C> IntoIterator for &'a Styled<Arc, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = StyledArcIterator<C>;

    fn into_iter(self) -> Self::IntoIter {
        StyledArcIterator::new(self)
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub(in crate::primitives) struct PlaneSector {
    line_a: LinearEquation,
    line_b: LinearEquation,
    draw_above_a: bool,
    draw_above_b: bool,
    sweep: Angle,
}

impl PlaneSector {
    pub(in crate::primitives) fn new(
        center: Point,
        angle_start: Angle,
        angle_sweep: Angle,
    ) -> Self {
        let angle_end = angle_start + angle_sweep;

        let angle_start_norm = angle_start.normalize_from(-ANGLE_90DEG);
        let angle_end_norm = angle_end.normalize_from(-ANGLE_90DEG);
        let negative_sweep = angle_sweep < Angle::zero();

        Self {
            line_a: LinearEquation::from_point_angle(center, angle_start),
            line_b: LinearEquation::from_point_angle(center, angle_end),
            draw_above_a: (angle_start_norm < ANGLE_90DEG) ^ negative_sweep,
            draw_above_b: (angle_end_norm >= ANGLE_90DEG) ^ negative_sweep,
            sweep: angle_sweep.abs(),
        }
    }

    fn empty() -> Self {
        Self {
            line_a: LinearEquation::flat(),
            line_b: LinearEquation::flat(),
            draw_above_a: true,
            draw_above_b: true,
            sweep: Angle::zero(),
        }
    }

    pub(in crate::primitives) fn contains(&self, point: &Point) -> bool {
        let side_a = self.line_a.side(point);
        let side_b = self.line_b.side(point);

        let correct_a_side = self.draw_above_a ^ (side_a == LineSide::Below);
        let correct_b_side = self.draw_above_b ^ (side_b == LineSide::Below);

        if self.sweep < ANGLE_180DEG {
            correct_a_side && correct_b_side
        } else if self.sweep < ANGLE_360DEG {
            correct_a_side || correct_b_side
        } else {
            true
        }
    }
}

/// Iterator that returns only the points which are inside a plane sector defined by two lines.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub(in crate::primitives) struct PlaneSectorIterator {
    plane_sector: PlaneSector,
    points: super::rectangle::Points,
}

impl PlaneSectorIterator {
    pub(in crate::primitives) fn new<D: Dimensions>(
        primitive: &D,
        center: Point,
        angle_start: Angle,
        angle_sweep: Angle,
    ) -> Self {
        Self {
            plane_sector: PlaneSector::new(center, angle_start, angle_sweep),
            points: primitive.bounding_box().points(),
        }
    }

    pub(in crate::primitives) fn empty() -> Self {
        Self {
            plane_sector: PlaneSector::empty(),
            points: Rectangle::new(Point::zero(), Size::zero()).points(),
        }
    }
}

impl Iterator for PlaneSectorIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let plane_sector = self.plane_sector;
        self.points.find(|p| plane_sector.contains(p))
    }
}

/// Define one side of a line
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum LineSide {
    Above,
    Below,
}

/// Linear equation representation
///
/// The equation is stored as the a, b and c coefficients of the ax + by + c = 0 equation
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
struct LinearEquation {
    a: Real,
    b: Real,
    c: Real,
}

impl LinearEquation {
    /// Create a new linear equation based on one point and one angle
    fn from_point_angle(point: Point, angle: Angle) -> Self {
        let (a, b) = match angle.tan() {
            None => (Real::from(1.0), Real::from(0.0)),
            Some(a) => (-a, Real::from(-1.0)),
        };
        let c = -(a * point.x.into() + b * point.y.into());
        LinearEquation { a, b, c }
    }

    /// Create a flat line equation
    fn flat() -> Self {
        LinearEquation {
            a: Real::from(0.0),
            b: Real::from(1.0),
            c: Real::from(0.0),
        }
    }

    /// Check on which side of the line a point is
    fn side(&self, point: &Point) -> LineSide {
        if self.a * point.x.into() + self.b * point.y.into() + self.c < Real::from(0.0) {
            LineSide::Below
        } else {
            LineSide::Above
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::AngleUnit,
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        style::{PrimitiveStyleBuilder, StrokeAlignment},
    };

    // Check the rendering of a simple arc
    #[test]
    fn tiny_arc() -> Result<(), core::convert::Infallible> {
        let mut display = MockDisplay::new();

        Arc::new(Point::zero(), 7, 30.0.deg(), 120.0.deg())
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut display)?;

        #[rustfmt::skip]
        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "  ###  ",
                " #   # ",
            ])
        );

        Ok(())
    }

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
        let positive = Arc::new(Point::new(10, 10), 5, 0.0.deg(), 90.0.deg())
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .into_iter();

        let negative = Arc::new(Point::new(-10, -10), 5, 0.0.deg(), 90.0.deg())
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .into_iter();

        assert!(negative.into_iter().eq(positive
            .into_iter()
            .map(|Pixel(p, c)| Pixel(p - Point::new(20, 20), c))));
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
    fn points_iter() {
        let arc = Arc::with_center(Point::new(10, 10), 5, 0.0.deg(), 90.0.deg());

        let styled_points = arc
            .clone()
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .into_iter()
            .map(|Pixel(p, _)| p);

        assert!(arc.points().eq(styled_points));
    }

    #[test]
    fn plane_arc_iter() {
        let arc = Arc::new(Point::zero(), 3, 0.0.deg(), 90.0.deg());

        let mut iter =
            PlaneSectorIterator::new(&arc, arc.center(), arc.angle_start, arc.angle_sweep);
        assert_eq!(iter.next(), Some(Point::new(1, 0)));
        assert_eq!(iter.next(), Some(Point::new(2, 0)));
        assert_eq!(iter.next(), Some(Point::new(1, 1)));
        assert_eq!(iter.next(), Some(Point::new(2, 1)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn plane_sector_iter_empty() {
        let mut iter = PlaneSectorIterator::empty();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn distance_iter() {
        let arc = Arc::new(Point::zero(), 3, 0.0.deg(), 90.0.deg());

        let mut iter = DistanceIterator::new(
            arc.center_2x(),
            PlaneSectorIterator::new(&arc, arc.center(), arc.angle_start, arc.angle_sweep),
        );
        assert_eq!(iter.next(), Some((Point::new(1, 0), 4)));
        assert_eq!(iter.next(), Some((Point::new(2, 0), 8)));
        assert_eq!(iter.next(), Some((Point::new(1, 1), 0)));
        assert_eq!(iter.next(), Some((Point::new(2, 1), 4)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn distance_iter_empty() {
        let mut iter = DistanceIterator::new(Point::zero(), PlaneSectorIterator::empty());
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn stroke_alignment() {
        const CENTER: Point = Point::new(15, 15);
        const SIZE: u32 = 10;

        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 3);

        let mut display_center = MockDisplay::new();
        Arc::with_center(CENTER, SIZE, 0.0.deg(), 90.0.deg())
            .into_styled(style)
            .draw(&mut display_center)
            .unwrap();

        let mut display_inside = MockDisplay::new();
        Arc::with_center(CENTER, SIZE + 2, 0.0.deg(), 90.0.deg())
            .into_styled(
                PrimitiveStyleBuilder::from(&style)
                    .stroke_alignment(StrokeAlignment::Inside)
                    .build(),
            )
            .draw(&mut display_inside)
            .unwrap();

        let mut display_outside = MockDisplay::new();
        Arc::with_center(CENTER, SIZE - 4, 0.0.deg(), 90.0.deg())
            .into_styled(
                PrimitiveStyleBuilder::from(&style)
                    .stroke_alignment(StrokeAlignment::Outside)
                    .build(),
            )
            .draw(&mut display_outside)
            .unwrap();

        assert_eq!(display_center, display_inside);
        assert_eq!(display_center, display_outside);
    }
}
