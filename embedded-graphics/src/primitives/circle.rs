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
}

impl<C> Iterator for StyledCircleIterator<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    // https://stackoverflow.com/questions/1201200/fast-algorithm-for-drawing-filled-circles
    fn next(&mut self) -> Option<Self::Item> {
        // If border or stroke colour is `None`, treat entire object as transparent and exit early
        if self.style.stroke_color.is_none() && self.style.fill_color.is_none() {
            return None;
        }

        let radius = self.radius as i32 - self.style.stroke_width_i32() + 1;
        let outer_radius = self.radius as i32;

        let radius_sq = radius * radius;
        let outer_radius_sq = outer_radius * outer_radius;

        loop {
            let t = self.p;
            let len = t.x * t.x + t.y * t.y;

            let is_border = len > radius_sq - radius && len < outer_radius_sq + radius;

            let is_fill = len <= outer_radius_sq + 1;

            let item = if is_border && self.style.stroke_color.is_some() {
                Some(Pixel(
                    self.center + t,
                    self.style.stroke_color.expect("Border color not defined"),
                ))
            } else if is_fill && self.style.fill_color.is_some() {
                Some(Pixel(
                    self.center + t,
                    self.style.fill_color.expect("Fill color not defined"),
                ))
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
        StyledCircleIterator {
            center: self.primitive.center,
            radius: self.primitive.radius,
            style: self.style,
            p: top_left,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelcolor::BinaryColor;

    /// Test for issue #143
    #[test]
    fn issue_143_stroke_and_fill() {
        let circle_no_stroke: Styled<Circle, PrimitiveStyle<BinaryColor>> =
            Circle::new(Point::new(10, 16), 3)
                .into_styled(PrimitiveStyle::with_fill(BinaryColor::On));

        let style = PrimitiveStyle {
            fill_color: Some(BinaryColor::On),
            stroke_color: Some(BinaryColor::On),
            stroke_width: 1,
        };
        let circle_stroke: Styled<Circle, PrimitiveStyle<BinaryColor>> =
            Circle::new(Point::new(10, 16), 3).into_styled(style);

        assert_eq!(circle_stroke.size(), circle_no_stroke.size());
        assert!(circle_no_stroke.into_iter().eq(circle_stroke.into_iter()));
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
