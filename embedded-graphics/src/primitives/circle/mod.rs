//! The circle primitive

mod distance_iterator;
mod points_iterator;
mod styled_iterator;

use crate::{
    draw_target::DrawTarget,
    drawable::{Drawable, Pixel},
    geometry::{Dimensions, Point, Size},
    pixelcolor::PixelColor,
    primitives::{ContainsPoint, Primitive, Rectangle, Styled},
    style::PrimitiveStyle,
    transform::Transform,
};
pub use points_iterator::Points;
pub use styled_iterator::StyledCircleIterator;

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

impl<'a, C: 'a> Drawable<C> for &Styled<Circle, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    fn draw<D: DrawTarget<Color = C>>(self, display: &mut D) -> Result<(), D::Error> {
        display.draw_iter(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{mock_display::MockDisplay, pixelcolor::BinaryColor, prelude::*};

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

        assert!(negative.eq(positive.map(|Pixel(p, c)| Pixel(p - Point::new(20, 20), c))));
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
