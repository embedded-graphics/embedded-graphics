//! The sector primitive

use crate::{
    drawable::{Drawable, Pixel},
    geometry::{Angle, Dimensions, Point, Real, Size, Trigonometry},
    pixelcolor::PixelColor,
    primitives::{
        arc::PlaneSector, arc::PlaneSectorIterator, circle, circle::DistanceIterator, line::Line,
        line::ThickPoints, ContainsPoint, Primitive, Rectangle, Styled,
    },
    style::PrimitiveStyle,
    transform::Transform,
    DrawTarget,
};

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

    /// Return the center point of the sector
    pub fn center(&self) -> Point {
        self.bounding_box().center()
    }

    /// Return the center point of the sector scaled by a factor of 2
    ///
    /// This method is used to accurately calculate the outside edge of the sector.
    /// The result is not equivalent to `self.center() * 2` because of rounding.
    fn center_2x(&self) -> Point {
        // The radius scaled up by a factor of 2 is equal to the diameter
        let radius = self.diameter.saturating_sub(1);

        self.top_left * 2 + Size::new(radius, radius)
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

    pub(crate) fn expand(&self, offset: u32) -> Self {
        let diameter = self.diameter.saturating_add(2 * offset);

        Self::with_center(self.center(), diameter, self.angle_start, self.angle_sweep)
    }

    pub(crate) fn shrink(&self, offset: u32) -> Self {
        let diameter = self.diameter.saturating_sub(2 * offset);

        Self::with_center(self.center(), diameter, self.angle_start, self.angle_sweep)
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
        let delta = self.center_2x() - point * 2;
        let distance = delta.length_squared() as u32;

        let threshold = circle::diameter_to_threshold(self.diameter);

        if distance >= threshold {
            return false;
        }

        PlaneSector::new(self.center(), self.angle_start, self.angle_sweep).contains(&point)
    }
}

impl Dimensions for Sector {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(self.top_left, Size::new_equal(self.diameter))
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

/// Iterator over all points inside the sector.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Points {
    iter: DistanceIterator<PlaneSectorIterator>,
    threshold: u32,
}

impl Points {
    fn new(sector: &Sector) -> Self {
        let threshold = circle::diameter_to_threshold(sector.diameter);

        Self {
            iter: DistanceIterator::new(
                sector.center_2x(),
                PlaneSectorIterator::new(
                    sector,
                    sector.center(),
                    sector.angle_start,
                    sector.angle_sweep,
                ),
            ),
            threshold,
        }
    }
}

impl Iterator for Points {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let threshold = self.threshold;
        self.iter
            .find(|(_, distance)| *distance < threshold)
            .map(|(point, _)| point)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum IterState {
    Arc,
    Lines,
    Done,
}

/// Pixel iterator for each pixel in the sector border
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct StyledSectorIterator<C>
where
    C: PixelColor,
{
    iter: DistanceIterator<PlaneSectorIterator>,

    outer_threshold: u32,
    outer_color: Option<C>,

    inner_threshold: u32,
    inner_color: Option<C>,

    line_a_iter: ThickPoints,
    line_b_iter: ThickPoints,

    state: IterState,
}

impl<C> StyledSectorIterator<C>
where
    C: PixelColor,
{
    fn new(styled: &Styled<Sector, PrimitiveStyle<C>>) -> Self {
        let Styled { primitive, style } = styled;

        let stroke_area = primitive.expand(style.outside_stroke_width());
        let fill_area = primitive.shrink(style.inside_stroke_width());

        let inner_threshold = circle::diameter_to_threshold(fill_area.diameter);
        let outer_threshold = circle::diameter_to_threshold(stroke_area.diameter);

        let line_a = stroke_area.line_from_angle(styled.primitive.angle_start);
        let line_b = stroke_area.line_from_angle(styled.primitive.angle_end());

        let line_a_iter = ThickPoints::new(&line_a, styled.style.stroke_width_i32());
        let line_b_iter = ThickPoints::new(&line_b, styled.style.stroke_width_i32());

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
            outer_color: styled.style.stroke_color,
            inner_threshold,
            inner_color: styled.style.fill_color,
            line_a_iter,
            line_b_iter,
            state: IterState::Arc,
        }
    }
}

impl<C> Iterator for StyledSectorIterator<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.state {
                IterState::Arc => {
                    if let Some((point, distance)) = self.iter.next() {
                        let color = if distance < self.inner_threshold {
                            self.inner_color
                        } else if distance < self.outer_threshold {
                            self.outer_color
                        } else {
                            None
                        };

                        if let Some(color) = color {
                            return Some(Pixel(point, color));
                        }
                    } else {
                        self.state = IterState::Lines;
                    }
                }
                IterState::Lines => {
                    if let Some(color) = self.outer_color {
                        if let Some(point) =
                            self.line_a_iter.next().or_else(|| self.line_b_iter.next())
                        {
                            break Some(Pixel(point, color));
                        }
                    }
                    self.state = IterState::Done;
                }
                IterState::Done => {
                    break None;
                }
            }
        }
    }
}

impl<'a, C: 'a> Drawable<C> for &Styled<Sector, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    fn draw<D: DrawTarget<Color = C>>(self, display: &mut D) -> Result<(), D::Error> {
        display.draw_iter(self)
    }
}

impl<'a, C> IntoIterator for &'a Styled<Sector, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = StyledSectorIterator<C>;

    fn into_iter(self) -> Self::IntoIter {
        StyledSectorIterator::new(self)
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

    #[test]
    fn stroke_width_doesnt_affect_fill() -> Result<(), core::convert::Infallible> {
        let mut expected = MockDisplay::new();
        let mut style = PrimitiveStyle::with_fill(BinaryColor::On);
        Sector::new(Point::new(5, 5), 4, 30.0.deg(), 120.0.deg())
            .into_styled(style)
            .draw(&mut expected)?;

        let mut with_stroke_width = MockDisplay::new();
        style.stroke_width = 1;
        Sector::new(Point::new(5, 5), 4, 30.0.deg(), 120.0.deg())
            .into_styled(style)
            .draw(&mut with_stroke_width)?;

        assert_eq!(expected, with_stroke_width);

        Ok(())
    }

    // Check the rendering of a simple sector
    #[test]
    fn tiny_sector() -> Result<(), core::convert::Infallible> {
        let mut display = MockDisplay::new();
        display.set_allow_overdraw(true);

        Sector::new(Point::zero(), 9, 30.0.deg(), 120.0.deg())
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut display)?;

        #[rustfmt::skip]
        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "  #####  ",
                " ##   ## ",
                " #     # ",
                "  ## ##  ",
                "    #    ",
            ])
        );

        Ok(())
    }

    // Check the rendering of a filled sector with negative sweep
    #[test]
    fn tiny_sector_filled() -> Result<(), core::convert::Infallible> {
        let mut display = MockDisplay::new();

        Sector::new(Point::zero(), 7, -30.0.deg(), -300.0.deg())
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut display)?;

        #[rustfmt::skip]
        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "  ###  ",
                " ##### ",
                "#####  ",
                "####   ",
                "#####  ",
                " ##### ",
                "  ###  ",
            ])
        );

        Ok(())
    }

    #[test]
    fn negative_dimensions() {
        let sector = Sector::new(Point::new(-15, -15), 10, 0.0.deg(), 90.0.deg());

        assert_eq!(
            sector.bounding_box(),
            Rectangle::new(Point::new(-15, -15), Size::new(10, 10))
        );
    }

    #[test]
    fn dimensions() {
        let sector = Sector::new(Point::new(5, 15), 10, 0.0.deg(), 90.0.deg());

        assert_eq!(
            sector.bounding_box(),
            Rectangle::new(Point::new(5, 15), Size::new(10, 10))
        );
    }

    #[test]
    fn transparent_border() {
        let sector: Styled<Sector, PrimitiveStyle<BinaryColor>> =
            Sector::new(Point::new(-5, -5), 21, 0.0.deg(), 90.0.deg())
                .into_styled(PrimitiveStyle::with_fill(BinaryColor::On));

        assert!(sector.into_iter().count() > 0);
    }

    #[test]
    fn it_handles_negative_coordinates() {
        let positive = Sector::new(Point::new(10, 10), 5, 0.0.deg(), 90.0.deg())
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .into_iter();

        let negative = Sector::new(Point::new(-10, -10), 5, 0.0.deg(), 90.0.deg())
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .into_iter();

        assert!(negative.into_iter().eq(positive
            .into_iter()
            .map(|Pixel(p, c)| Pixel(p - Point::new(20, 20), c))));
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
    fn points_iter() {
        let sector = Sector::with_center(Point::new(10, 10), 5, 0.0.deg(), 90.0.deg());

        let styled_points = sector
            .clone()
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .into_iter()
            .map(|Pixel(p, _)| p);

        assert!(sector.points().eq(styled_points));
    }

    #[test]
    fn plane_sector_iter() {
        let sector = Sector::new(Point::zero(), 3, 0.0.deg(), 90.0.deg());

        let mut iter = PlaneSectorIterator::new(
            &sector,
            sector.center(),
            sector.angle_start,
            sector.angle_sweep,
        );
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
        let sector = Sector::new(Point::zero(), 3, 0.0.deg(), 90.0.deg());

        let mut iter = DistanceIterator::new(
            sector.center_2x(),
            PlaneSectorIterator::new(
                &sector,
                sector.center(),
                sector.angle_start,
                sector.angle_sweep,
            ),
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
    fn contains() {
        let sector = Sector::new(Point::zero(), 10, 0.0.deg(), 90.0.deg());

        let contained_points = Rectangle::new(Point::new(-10, -10), Size::new(30, 30))
            .points()
            .filter(|p| sector.contains(*p));

        assert!(contained_points.eq(sector.points()));
    }

    #[test]
    fn stroke_alignment() {
        const CENTER: Point = Point::new(15, 15);
        const SIZE: u32 = 10;

        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 3);

        let mut display_center = MockDisplay::new();
        display_center.set_allow_overdraw(true);
        Sector::with_center(CENTER, SIZE, 0.0.deg(), 90.0.deg())
            .into_styled(style)
            .draw(&mut display_center)
            .unwrap();

        let mut display_inside = MockDisplay::new();
        display_inside.set_allow_overdraw(true);
        Sector::with_center(CENTER, SIZE + 2, 0.0.deg(), 90.0.deg())
            .into_styled(
                PrimitiveStyleBuilder::from(&style)
                    .stroke_alignment(StrokeAlignment::Inside)
                    .build(),
            )
            .draw(&mut display_inside)
            .unwrap();

        let mut display_outside = MockDisplay::new();
        display_outside.set_allow_overdraw(true);
        Sector::with_center(CENTER, SIZE - 4, 0.0.deg(), 90.0.deg())
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
