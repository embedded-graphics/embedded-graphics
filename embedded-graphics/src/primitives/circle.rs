//! The circle primitive

use super::super::drawable::{Drawable, Pixel};
use super::super::transform::Transform;
use crate::geometry::{Dimensions, Point, Size};
use crate::pixelcolor::PixelColor;
use crate::primitives::Primitive;
use crate::style::Style;
use crate::style::WithStyle;

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
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::default();
///
/// // Default circle with only a stroke centered around (10, 20) with a radius of 30
/// let c1 = Circle::new(Point::new(10, 20), 30);
///
/// // Circle with styled stroke and fill centered around (50, 20) with a radius of 30
/// let c2 = Circle::new(Point::new(50, 20), 30)
///     .stroke(Some(Rgb565::RED))
///     .stroke_width(3)
///     .fill(Some(Rgb565::GREEN));
///
/// // Circle with no stroke and a translation applied
/// let c3 = Circle::new(Point::new(10, 20), 30)
///     .stroke(None)
///     .fill(Some(Rgb565::BLUE))
///     .translate(Point::new(65, 35));
///
/// display.draw(c1);
/// display.draw(c2);
/// display.draw(c3);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Circle<C: PixelColor> {
    /// Center point of circle
    pub center: Point,

    /// Radius of the circle
    pub radius: u32,

    /// Style of the circle
    pub style: Style<C>,
}

impl<C> Circle<C>
where
    C: PixelColor,
{
    /// Create a new circle centered around a given point with a specific radius
    pub fn new(center: Point, radius: u32) -> Self {
        Circle {
            center,
            radius,
            style: Style::default(),
        }
    }
}

impl<C> Primitive for Circle<C> where C: PixelColor {}

impl<C> Dimensions for Circle<C>
where
    C: PixelColor,
{
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

impl<C> WithStyle<C> for Circle<C>
where
    C: PixelColor,
{
    fn style(mut self, style: Style<C>) -> Self {
        self.style = style;

        self
    }

    fn stroke(mut self, color: Option<C>) -> Self {
        self.style.stroke_color = color;

        self
    }

    fn stroke_width(mut self, width: u8) -> Self {
        self.style.stroke_width = width;

        self
    }

    fn fill(mut self, color: Option<C>) -> Self {
        self.style.fill_color = color;

        self
    }
}

impl<C> IntoIterator for Circle<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = CircleIterator<C>;

    fn into_iter(self) -> Self::IntoIter {
        (&self).into_iter()
    }
}

impl<'a, C> IntoIterator for &'a Circle<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = CircleIterator<C>;

    fn into_iter(self) -> Self::IntoIter {
        CircleIterator {
            center: self.center,
            radius: self.radius,
            style: self.style,
            p: Point::new(-(self.radius as i32), -(self.radius as i32)),
        }
    }
}

/// Pixel iterator for each pixel in the circle border
#[derive(Debug, Copy, Clone)]
pub struct CircleIterator<C: PixelColor> {
    center: Point,
    radius: u32,
    style: Style<C>,
    p: Point,
}

impl<C> Iterator for CircleIterator<C>
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

        let radius = self.radius as i32 - i32::from(self.style.stroke_width) + 1;
        let outer_radius = self.radius as i32;

        let radius_sq = radius * radius;
        let outer_radius_sq = outer_radius * outer_radius;

        loop {
            let t = self.p;
            let len = t.x * t.x + t.y * t.y;

            let is_border = len > radius_sq - radius && len < outer_radius_sq + radius;

            // TODO: Should this be a <= or a <?
            let is_fill = len <= outer_radius_sq;

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

impl<C> Drawable for Circle<C> where C: PixelColor {}

impl<C> Transform for Circle<C>
where
    C: PixelColor,
{
    /// Translate the circle center from its current position to a new position by (x, y) pixels,
    /// returning a new `Circle`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Circle;
    /// # use embedded_graphics::prelude::*;
    /// # use embedded_graphics::pixelcolor::Rgb565;
    /// #
    /// # let style = Style::stroke(Rgb565::RED);
    /// #
    /// let circle = Circle::new(Point::new(5, 10), 10)
    /// #    .style(style);
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
    /// # use embedded_graphics::pixelcolor::Rgb565;
    /// #
    /// # let style = Style::stroke(Rgb565::RED);
    /// #
    /// let mut circle = Circle::new(Point::new(5, 10), 10)
    /// #    .style(style);
    /// circle.translate_mut(Point::new(10, 10));
    ///
    /// assert_eq!(circle.center, Point::new(15, 20));
    /// ```
    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.center += by;

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelcolor::BinaryColor;

    #[test]
    fn issue_143_stroke_and_fill() {
        let circle_no_stroke: Circle<BinaryColor> =
            Circle::new(Point::new(10, 16), 3).fill(Some(BinaryColor::On));
        let circle_stroke: Circle<BinaryColor> = Circle::new(Point::new(10, 16), 3)
            .fill(Some(BinaryColor::On))
            .stroke(Some(BinaryColor::On));

        assert_eq!(circle_stroke.size(), circle_no_stroke.size());
        assert_eq!(
            circle_stroke.into_iter().count(),
            circle_no_stroke.into_iter().count()
        );

        assert!(circle_no_stroke.into_iter().eq(circle_stroke.into_iter()));
    }

    #[test]
    fn negative_dimensions() {
        let circ: Circle<BinaryColor> = Circle::new(Point::new(-10, -10), 5);

        assert_eq!(circ.top_left(), Point::new(-15, -15));
        assert_eq!(circ.bottom_right(), Point::new(-5, -5));
        assert_eq!(circ.size(), Size::new(10, 10));
    }

    #[test]
    fn dimensions() {
        let circ: Circle<BinaryColor> = Circle::new(Point::new(10, 20), 5);

        assert_eq!(circ.top_left(), Point::new(5, 15));
        assert_eq!(circ.bottom_right(), Point::new(15, 25));
        assert_eq!(circ.size(), Size::new(10, 10));
    }

    #[test]
    fn large_radius() {
        let circ: Circle<BinaryColor> = Circle::new(Point::new(5, 5), 10);

        assert_eq!(circ.top_left(), Point::new(-5, -5));
        assert_eq!(circ.bottom_right(), Point::new(15, 15));
        assert_eq!(circ.size(), Size::new(20, 20));
    }

    #[test]
    fn transparent_border() {
        let circ: Circle<BinaryColor> = Circle::new(Point::new(5, 5), 10)
            .stroke(None)
            .fill(Some(BinaryColor::On));

        assert!(circ.into_iter().count() > 0);
    }

    #[test]
    fn it_handles_negative_coordinates() {
        let positive: CircleIterator<BinaryColor> = Circle::new(Point::new(10, 10), 5)
            .style(Style::stroke(BinaryColor::On))
            .into_iter();

        let negative: CircleIterator<BinaryColor> = Circle::new(Point::new(-10, -10), 5)
            .style(Style::stroke(BinaryColor::On))
            .into_iter();

        assert!(negative.into_iter().eq(positive
            .into_iter()
            .map(|Pixel(p, c)| Pixel(p - Point::new(20, 20), c))));
    }
}
