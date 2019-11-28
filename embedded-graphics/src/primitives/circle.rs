//! The circle primitive

use super::super::drawable::{Drawable, Pixel};
use super::super::transform::Transform;
use crate::geometry::{Dimensions, Point, Size};
use crate::pixelcolor::PixelColor;
use crate::primitives::Primitive;
use crate::primitives::Styled;
use crate::style::PrimitiveStyle;
use crate::DrawTarget;

/// Circle primitive
///
/// # Examples
///
/// The [macro examples](../../macro.egcircle.html) make for more concise code.
///
/// ## Create some circles with different styles
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::primitives::Circle;
/// use embedded_graphics::pixelcolor::Rgb565;
/// use embedded_graphics::style::PrimitiveStyle;
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::default();
///
/// // Circle with 1 pixel wide white stroke centered around (10, 20) with a radius of 30
/// Circle::new(Point::new(10, 20), 30)
///     .into_styled(PrimitiveStyle::with_stroke(Rgb565::WHITE, 1))
///     .draw(&mut display);
///
/// // Circle with styled stroke and fill centered around (50, 20) with a radius of 30
/// let style = PrimitiveStyle {
///     stroke_color: Some(Rgb565::RED),
///     stroke_width: 3,
///     fill_color: Some(Rgb565::GREEN),
/// };
///
/// Circle::new(Point::new(50, 20), 30)
///     .into_styled(style)
///     .draw(&mut display);
///
/// // Circle with blue fill and no stroke with a translation applied
/// Circle::new(Point::new(10, 20), 30)
///     .translate(Point::new(65, 35))
///     .into_styled(PrimitiveStyle::with_fill(Rgb565::BLUE))
///     .draw(&mut display);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Circle {
    /// Center point of circle
    pub center: Point,

    /// Radius of the circle
    pub radius: u32,
}

impl Circle {
    /// Create a new circle centered around a given point with a specific radius
    pub const fn new(center: Point, radius: u32) -> Self {
        Circle { center, radius }
    }
}

impl Primitive for Circle {}

impl Dimensions for Circle {
    fn top_left(&self) -> Point {
        let radius_coord = Point::new(self.radius as i32, self.radius as i32);

        self.center - radius_coord
    }

    fn bottom_right(&self) -> Point {
        self.top_left() + self.size()
    }

    fn size(&self) -> Size {
        Size::new(self.radius * 2, self.radius * 2)
    }
}

impl Transform for Circle {
    /// Translate the circle center from its current position to a new position by (x, y) pixels,
    /// returning a new `Circle`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Circle;
    /// # use embedded_graphics::prelude::*;
    /// let circle = Circle::new(Point::new(5, 10), 10);
    /// let moved = circle.translate(Point::new(10, 10));
    ///
    /// assert_eq!(moved.center, Point::new(15, 20));
    /// ```
    fn translate(&self, by: Point) -> Self {
        Self {
            center: self.center + by,
            ..*self
        }
    }

    /// Translate the circle center from its current position to a new position by (x, y) pixels.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Circle;
    /// # use embedded_graphics::prelude::*;
    /// let mut circle = Circle::new(Point::new(5, 10), 10);
    /// circle.translate_mut(Point::new(10, 10));
    ///
    /// assert_eq!(circle.center, Point::new(15, 20));
    /// ```
    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.center += by;

        self
    }
}

/// Pixel iterator for each pixel in the circle border
#[derive(Debug, Copy, Clone)]
pub struct StyledCircleIterator<C: PixelColor> {
    center: Point,
    radius: u32,
    style: PrimitiveStyle<C>,
    p: Point,
    outer_threshold: i32,
    inner_threshold: i32,
}

impl<C> Iterator for StyledCircleIterator<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    // https://stackoverflow.com/a/1237519/383609
    // https://stackoverflow.com/questions/1201200/fast-algorithm-for-drawing-filled-circles#comment80182898_1237519
    fn next(&mut self) -> Option<Self::Item> {
        // If the fill and stroke colors are `None`, treat entire object as transparent and exit
        // early.
        if self.style.stroke_color.is_none() && self.style.fill_color.is_none() {
            return None;
        }

        loop {
            let t = self.p;
            let len = t.x.pow(2) + t.y.pow(2);

            let is_fill = len <= self.inner_threshold;
            let is_stroke = len <= self.outer_threshold && !is_fill;

            let item = if is_stroke && self.style.stroke_color.is_some() {
                Some(Pixel(self.center + t, self.style.stroke_color.unwrap()))
            } else if is_fill && self.style.fill_color.is_some() {
                Some(Pixel(self.center + t, self.style.fill_color.unwrap()))
            } else {
                None
            };

            self.p.x += 1;

            if self.p.x > self.radius as i32 {
                self.p.x = -(self.radius as i32);
                self.p.y += 1;
            }

            if self.p.y > self.radius as i32 {
                break None;
            }

            if item.is_some() {
                break item;
            }
        }
    }
}

impl<'a, C: 'a> Drawable<C> for &Styled<Circle, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    fn draw<T: DrawTarget<C>>(self, display: &mut T) {
        display.draw_circle(self);
    }
}

impl<'a, C> IntoIterator for &'a Styled<Circle, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = StyledCircleIterator<C>;

    fn into_iter(self) -> Self::IntoIter {
        let top_left = Point::new(
            -(self.primitive.radius as i32),
            -(self.primitive.radius as i32),
        );

        // A stroke width of zero renders a 1px stroke, so add 1 to inner radius to compensate
        let inner_radius = self.primitive.radius as i32 - self.style.stroke_width_i32() + 1;
        let outer_radius = self.primitive.radius as i32;

        let mut inner_threshold = inner_radius.pow(2) - inner_radius;
        let mut outer_threshold = outer_radius.pow(2) + outer_radius;

        // Special case for small circles. This kludge removes the top-left pixel and leaves the
        // circle as a `+` shape.
        if self.primitive.radius == 1 {
            inner_threshold -= 1;
            outer_threshold -= 1;
        }

        StyledCircleIterator {
            center: self.primitive.center,
            radius: self.primitive.radius,
            style: self.style,
            p: top_left,
            outer_threshold,
            inner_threshold,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock_display::MockDisplay;
    use crate::pixelcolor::BinaryColor;

    // Check that tiny circles render as a "+" shape with a hole in the center
    #[test]
    fn tiny_circle() {
        let mut display = MockDisplay::new();

        Circle::new(Point::new(1, 1), 1)
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut display);

        #[rustfmt::skip]
        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                " # ",
                "# #",
                " # "
            ])
        );
    }

    // Check that tiny filled circle render as a "+" shape with NO hole in the center
    #[test]
    fn tiny_circle_filled() {
        let mut display = MockDisplay::new();

        Circle::new(Point::new(1, 1), 1)
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut display);

        #[rustfmt::skip]
        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                " # ",
                "###",
                " # "
            ])
        );
    }

    /// Test for issue #143
    #[test]
    fn issue_143_stroke_and_fill() {
        for size in 0..10 {
            let circle_no_stroke: Styled<Circle, PrimitiveStyle<BinaryColor>> =
                Circle::new(Point::new(10, 16), size)
                    .into_styled(PrimitiveStyle::with_fill(BinaryColor::On));

            let style = PrimitiveStyle {
                fill_color: Some(BinaryColor::On),
                stroke_color: Some(BinaryColor::On),
                stroke_width: 1,
            };
            let circle_stroke: Styled<Circle, PrimitiveStyle<BinaryColor>> =
                Circle::new(Point::new(10, 16), size).into_styled(style);

            assert_eq!(
                circle_stroke.size(),
                circle_no_stroke.size(),
                "Filled and unfilled circle iters are unequal for radius {}",
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
        let circle = Circle::new(Point::new(-10, -10), 5);

        assert_eq!(circle.top_left(), Point::new(-15, -15));
        assert_eq!(circle.bottom_right(), Point::new(-5, -5));
        assert_eq!(circle.size(), Size::new(10, 10));
    }

    #[test]
    fn dimensions() {
        let circle = Circle::new(Point::new(10, 20), 5);

        assert_eq!(circle.top_left(), Point::new(5, 15));
        assert_eq!(circle.bottom_right(), Point::new(15, 25));
        assert_eq!(circle.size(), Size::new(10, 10));
    }

    #[test]
    fn large_radius() {
        let circle = Circle::new(Point::new(5, 5), 10);

        assert_eq!(circle.top_left(), Point::new(-5, -5));
        assert_eq!(circle.bottom_right(), Point::new(15, 15));
        assert_eq!(circle.size(), Size::new(20, 20));
    }

    #[test]
    fn transparent_border() {
        let circle: Styled<Circle, PrimitiveStyle<BinaryColor>> = Circle::new(Point::new(5, 5), 10)
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
}
