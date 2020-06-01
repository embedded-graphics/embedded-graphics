//! The circle primitive

use crate::{
    drawable::{Drawable, Pixel},
    geometry::{Dimensions, Point, Size},
    pixelcolor::PixelColor,
    primitives::{ContainsPoint, Primitive, Rectangle, Styled},
    style::PrimitiveStyle,
    transform::Transform,
    DrawTarget,
};

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
///     primitives::Circle,
///     style::{PrimitiveStyle, PrimitiveStyleBuilder},
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
        let top_left = center - Size::new(diameter, diameter).center_offset();

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
    fn center_2x(&self) -> Point {
        // The radius scaled up by a factor of 2 is equal to the diameter
        let radius = self.diameter.saturating_sub(1);

        self.top_left * 2 + Size::new(radius, radius)
    }

    pub(crate) fn expand(&self, offset: u32) -> Self {
        let diameter = self.diameter.saturating_add(2 * offset);

        Self::with_center(self.center(), diameter)
    }

    pub(crate) fn shrink(&self, offset: u32) -> Self {
        let diameter = self.diameter.saturating_sub(2 * offset);

        Self::with_center(self.center(), diameter)
    }
}

impl Primitive for Circle {
    type PointsIter = Points;

    fn points(&self) -> Self::PointsIter {
        Points::new(self)
    }
}

impl ContainsPoint for Circle {
    fn contains(&self, point: Point) -> bool {
        let delta = self.center_2x() - point * 2;
        let distance = delta.length_squared() as u32;

        let threshold = diameter_to_threshold(self.diameter);

        distance < threshold
    }
}

impl Dimensions for Circle {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(self.top_left, Size::new(self.diameter, self.diameter))
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

/// Iterator over all points inside the circle.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Points {
    iter: DistanceIterator,
    threshold: u32,
}

impl Points {
    fn new(circle: &Circle) -> Self {
        let threshold = diameter_to_threshold(circle.diameter);

        Self {
            iter: DistanceIterator::new(&circle),
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

/// Pixel iterator for each pixel in the circle border
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct StyledCircleIterator<C>
where
    C: PixelColor,
{
    iter: DistanceIterator,

    outer_threshold: u32,
    outer_color: Option<C>,

    inner_threshold: u32,
    inner_color: Option<C>,
}

impl<C> StyledCircleIterator<C>
where
    C: PixelColor,
{
    fn new(styled: &Styled<Circle, PrimitiveStyle<C>>) -> Self {
        let Styled { primitive, style } = styled;

        let stroke_area = primitive.expand(style.outside_stroke_width());
        let fill_area = primitive.shrink(style.inside_stroke_width());

        let inner_threshold = diameter_to_threshold(fill_area.diameter);
        let outer_threshold = diameter_to_threshold(stroke_area.diameter);

        let iter = if !styled.style.is_transparent() {
            DistanceIterator::new(&stroke_area)
        } else {
            DistanceIterator::empty()
        };

        Self {
            iter,
            outer_threshold,
            outer_color: styled.style.stroke_color,
            inner_threshold,
            inner_color: styled.style.fill_color,
        }
    }
}

impl<C> Iterator for StyledCircleIterator<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        for (point, distance) in &mut self.iter {
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
        }

        None
    }
}

impl<'a, C: 'a> Drawable<C> for &Styled<Circle, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        display.draw_circle(self)
    }
}

pub(in crate::primitives) fn diameter_to_threshold(diameter: u32) -> u32 {
    if diameter <= 4 {
        diameter.pow(2) - diameter / 2
    } else {
        diameter.pow(2)
    }
}

impl<'a, C> IntoIterator for &'a Styled<Circle, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = StyledCircleIterator<C>;

    fn into_iter(self) -> Self::IntoIter {
        StyledCircleIterator::new(self)
    }
}

/// Iterator that returns the squared distance to the center for all points in the bounding box.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct DistanceIterator {
    center: Point,
    points: super::rectangle::Points,
}

impl DistanceIterator {
    fn new(circle: &Circle) -> Self {
        Self {
            center: circle.center_2x(),
            points: circle.bounding_box().points(),
        }
    }

    fn empty() -> Self {
        Self {
            center: Point::zero(),
            points: Rectangle::new(Point::zero(), Size::zero()).points(),
        }
    }
}

impl Iterator for DistanceIterator {
    type Item = (Point, u32);

    fn next(&mut self) -> Option<Self::Item> {
        self.points.next().map(|p| {
            let delta = self.center - p * 2;
            let distance = delta.length_squared() as u32;

            (p, distance)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        style::{PrimitiveStyleBuilder, StrokeAlignment},
    };

    #[test]
    fn stroke_width_doesnt_affect_fill() -> Result<(), core::convert::Infallible> {
        let mut expected = MockDisplay::new();
        let mut style = PrimitiveStyle::with_fill(BinaryColor::On);
        Circle::new(Point::new(5, 5), 4)
            .into_styled(style)
            .draw(&mut expected)?;

        let mut with_stroke_width = MockDisplay::new();
        style.stroke_width = 1;
        Circle::new(Point::new(5, 5), 4)
            .into_styled(style)
            .draw(&mut with_stroke_width)?;

        assert_eq!(expected, with_stroke_width);

        Ok(())
    }

    // Check that tiny circles render as a "+" shape with a hole in the center
    #[test]
    fn tiny_circle() -> Result<(), core::convert::Infallible> {
        let mut display = MockDisplay::new();

        Circle::new(Point::new(0, 0), 3)
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut display)?;

        #[rustfmt::skip]
        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                " # ",
                "# #",
                " # "
            ])
        );

        Ok(())
    }

    // Check that tiny filled circle render as a "+" shape with NO hole in the center
    #[test]
    fn tiny_circle_filled() -> Result<(), core::convert::Infallible> {
        let mut display = MockDisplay::new();

        Circle::new(Point::new(0, 0), 3)
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut display)?;

        #[rustfmt::skip]
        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                " # ",
                "###",
                " # "
            ])
        );

        Ok(())
    }

    /// Test for issue #143
    #[test]
    fn issue_143_stroke_and_fill() {
        for size in 0..10 {
            let circle_no_stroke: Styled<Circle, PrimitiveStyle<BinaryColor>> =
                Circle::new(Point::new(10, 16), size)
                    .into_styled(PrimitiveStyle::with_fill(BinaryColor::On));

            let style = PrimitiveStyleBuilder::new()
                .fill_color(BinaryColor::On)
                .stroke_color(BinaryColor::On)
                .stroke_width(1)
                .build();
            let circle_stroke: Styled<Circle, PrimitiveStyle<BinaryColor>> =
                Circle::new(Point::new(10, 16), size).into_styled(style);

            assert_eq!(
                circle_stroke.bounding_box(),
                circle_no_stroke.bounding_box(),
                "Filled and unfilled circle bounding boxes are unequal for radius {}",
                size
            );
            assert!(
                circle_no_stroke.into_iter().eq(circle_stroke.into_iter()),
                "Filled and unfilled circle iters are unequal for radius {}",
                size
            );
        }
    }

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
    fn transparent_border() {
        let circle: Styled<Circle, PrimitiveStyle<BinaryColor>> =
            Circle::new(Point::new(-5, -5), 21)
                .into_styled(PrimitiveStyle::with_fill(BinaryColor::On));

        assert!(circle.into_iter().count() > 0);
    }

    #[test]
    fn it_handles_negative_coordinates() {
        let positive = Circle::new(Point::new(10, 10), 5)
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .into_iter();

        let negative = Circle::new(Point::new(-10, -10), 5)
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .into_iter();

        assert!(negative.into_iter().eq(positive
            .into_iter()
            .map(|Pixel(p, c)| Pixel(p - Point::new(20, 20), c))));
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
    fn points_iter() {
        let circle = Circle::with_center(Point::new(10, 10), 5);

        let styled_points = circle
            .clone()
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .into_iter()
            .map(|Pixel(p, _)| p);

        assert!(circle.points().eq(styled_points));
    }

    #[test]
    fn distance_iter() {
        let circle = Circle::new(Point::zero(), 3);

        let mut iter = DistanceIterator::new(&circle);
        assert_eq!(iter.next(), Some((Point::new(0, 0), 8)));
        assert_eq!(iter.next(), Some((Point::new(1, 0), 4)));
        assert_eq!(iter.next(), Some((Point::new(2, 0), 8)));
        assert_eq!(iter.next(), Some((Point::new(0, 1), 4)));
        assert_eq!(iter.next(), Some((Point::new(1, 1), 0)));
        assert_eq!(iter.next(), Some((Point::new(2, 1), 4)));
        assert_eq!(iter.next(), Some((Point::new(0, 2), 8)));
        assert_eq!(iter.next(), Some((Point::new(1, 2), 4)));
        assert_eq!(iter.next(), Some((Point::new(2, 2), 8)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn distance_iter_empty() {
        let mut iter = DistanceIterator::empty();
        assert_eq!(iter.next(), None);
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
    fn stroke_alignment() {
        const CENTER: Point = Point::new(15, 15);
        const SIZE: u32 = 10;

        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 3);

        let mut display_center = MockDisplay::new();
        Circle::with_center(CENTER, SIZE)
            .into_styled(style)
            .draw(&mut display_center)
            .unwrap();

        let mut display_inside = MockDisplay::new();
        Circle::with_center(CENTER, SIZE + 2)
            .into_styled(
                PrimitiveStyleBuilder::from(&style)
                    .stroke_alignment(StrokeAlignment::Inside)
                    .build(),
            )
            .draw(&mut display_inside)
            .unwrap();

        let mut display_outside = MockDisplay::new();
        Circle::with_center(CENTER, SIZE - 4)
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

    fn test_circle(diameter: u32, pattern: &[&str]) {
        let mut display = MockDisplay::new();

        Circle::new(Point::new(0, 0), diameter)
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut display)
            .unwrap();

        assert_eq!(display, MockDisplay::from_pattern(pattern));
    }

    #[test]
    fn circle_1() {
        #[rustfmt::skip]
        test_circle(1, &[
            "#",
        ],);
    }

    #[test]
    fn circle_2() {
        #[rustfmt::skip]
        test_circle(2, &[
            "##",
            "##",
        ],);
    }

    #[test]
    fn circle_3() {
        #[rustfmt::skip]
        test_circle(3, &[
            " # ",
            "###",
            " # ",
        ],);
    }

    #[test]
    fn circle_4() {
        #[rustfmt::skip]
        test_circle(4, &[
            " ## ",
            "####",
            "####",
            " ## ",
        ],);
    }

    #[test]
    fn circle_5() {
        #[rustfmt::skip]
        test_circle(5, &[
            " ### ",
            "#####",
            "#####",
            "#####",
            " ### ",
        ],);
    }

    #[test]
    fn circle_6() {
        #[rustfmt::skip]
        test_circle(6, &[
            " #### ",
            "######",
            "######",
            "######",
            "######",
            " #### ",
        ],);
    }

    #[test]
    fn circle_7() {
        #[rustfmt::skip]
        test_circle(7, &[
            "  ###  ",
            " ##### ",
            "#######",
            "#######",
            "#######",
            " ##### ",
            "  ###  ",
        ],);
    }

    #[test]
    fn circle_8() {
        #[rustfmt::skip]
        test_circle(8, &[
            "  ####  ",
            " ###### ",
            "########",
            "########",
            "########",
            "########",
            " ###### ",
            "  ####  ",
        ],);
    }

    #[test]
    fn circle_9() {
        #[rustfmt::skip]
        test_circle(9, &[
            "  #####  ",
            " ####### ",
            "#########",
            "#########",
            "#########",
            "#########",
            "#########",
            " ####### ",
            "  #####  ",
        ],);
    }
}
