//! The circle primitive

use crate::{
    drawable::{Drawable, Pixel},
    geometry::{Dimensions, Point, Size},
    pixelcolor::PixelColor,
    primitives::{Primitive, Styled},
    style::PrimitiveStyle,
    transform::Transform,
    DrawTarget,
};

/// Circle primitive
///
/// # Examples
///
/// The [macro examples](../../macro.egcircle.html) make for more concise code.
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
/// # let mut display = MockDisplay::default();
///
/// // Circle with 1 pixel wide white stroke with top-left point at (10, 20) with a diameter of 30
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
/// Circle::new(Point::new(50, 20), 30)
///     .into_styled(style)
///     .draw(&mut display)?;
///
/// // Circle with blue fill and no stroke with a translation applied
/// Circle::new(Point::new(10, 20), 30)
///     .translate(Point::new(65, 35))
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
    pub const fn with_center(center: Point, diameter: u32) -> Self {
        let top_left = Point::new(
            center.x - diameter as i32 / 2,
            center.y - diameter as i32 / 2,
        );
        Circle { top_left, diameter }
    }

    /// Return the center point of the circle
    pub fn center(&self) -> Point {
        self.top_left + Size::new(self.diameter / 2, self.diameter / 2)
    }
}

impl Primitive for Circle {}

impl Dimensions for Circle {
    fn top_left(&self) -> Point {
        self.top_left
    }

    fn bottom_right(&self) -> Point {
        self.top_left() + self.size()
    }

    fn size(&self) -> Size {
        if self.diameter < 1 {
            Size::new(0, 0)
        } else {
            Size::new(self.diameter - 1, self.diameter - 1)
        }
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

/// Pixel iterator for each pixel in the circle border
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct StyledCircleIterator<C: PixelColor> {
    top_left: Point,
    diameter: u32,
    style: PrimitiveStyle<C>,
    p: Point,
    c: Point,
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
            let len = (self.c.x - 2 * self.p.x).pow(2) + (self.c.y - 2 * self.p.y).pow(2);

            let color = if len < self.inner_threshold {
                self.style.fill_color
            } else if len < self.outer_threshold {
                // Use fill_color if no stroke_color was set
                self.style.stroke_color.or(self.style.fill_color)
            } else {
                None
            };
            let item = color.map(|c| Pixel(self.p, c));

            self.p.x += 1;

            if self.p.x > self.top_left.x + self.diameter as i32 {
                self.p.x = self.top_left.x;
                self.p.y += 1;
            }

            if self.p.y > self.top_left.y + self.diameter as i32 {
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
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        display.draw_circle(self)
    }
}

fn diameter_to_threshold(diameter: i32) -> i32 {
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
        let center = Point::new(2 * self.primitive.top_left.x, 2 * self.primitive.top_left.y)
            + self.primitive.size();

        let inner_diameter = self.primitive.diameter as i32 - 2 * self.style.stroke_width_i32();
        let outer_diameter = self.primitive.diameter as i32;

        let inner_threshold = diameter_to_threshold(core::cmp::max(inner_diameter, 0));
        let outer_threshold = diameter_to_threshold(outer_diameter);

        StyledCircleIterator {
            top_left: self.primitive.top_left,
            diameter: self.primitive.diameter,
            style: self.style,
            p: self.primitive.top_left,
            c: center,
            outer_threshold,
            inner_threshold,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{mock_display::MockDisplay, pixelcolor::BinaryColor, style::PrimitiveStyleBuilder};

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
        let circle = Circle::new(Point::new(-15, -15), 11);

        assert_eq!(circle.top_left(), Point::new(-15, -15));
        assert_eq!(circle.bottom_right(), Point::new(-5, -5));
        assert_eq!(circle.size(), Size::new(10, 10));
    }

    #[test]
    fn dimensions() {
        let circle = Circle::new(Point::new(5, 15), 11);

        assert_eq!(circle.top_left(), Point::new(5, 15));
        assert_eq!(circle.bottom_right(), Point::new(15, 25));
        assert_eq!(circle.size(), Size::new(10, 10));
    }

    #[test]
    fn large_diameter() {
        let circle = Circle::new(Point::new(-5, -5), 21);

        assert_eq!(circle.top_left(), Point::new(-5, -5));
        assert_eq!(circle.bottom_right(), Point::new(15, 15));
        assert_eq!(circle.size(), Size::new(20, 20));
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
        let circle = Circle::with_center(Point::new(10, 10), 5);

        assert_eq!(circle.top_left(), Point::new(8, 8));
        assert_eq!(circle.center(), Point::new(10, 10));
        assert_eq!(circle.bottom_right(), Point::new(12, 12));
    }
}
