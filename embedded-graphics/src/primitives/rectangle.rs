//! The rectangle primitive. Also good for drawing squares.

use super::super::drawable::{Drawable, Pixel};
use super::super::transform::Transform;
use crate::geometry::{Dimensions, Point, Size};
use crate::pixelcolor::PixelColor;
use crate::primitives::Primitive;
use crate::style::Style;
use crate::style::WithStyle;
use crate::DrawTarget;

/// Rectangle primitive
///
/// # Examples
///
/// The [macro examples](../../macro.egrectangle.html) make for more concise code.
///
/// ## Create some rectangles with different styles
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::primitives::Rectangle;
/// use embedded_graphics::pixelcolor::Rgb565;
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::default();
///
/// // Default rect from (10, 20) to (30, 40)
/// Rectangle::new(Point::new(10, 20), Point::new(30, 40)).draw(&mut display);
///
/// // Rectangle with styled stroke and fill from (50, 20) to (60, 35)
/// Rectangle::new(Point::new(50, 20), Point::new(60, 35))
///     .stroke_color(Some(Rgb565::RED))
///     .stroke_width(3)
///     .fill_color(Some(Rgb565::GREEN))
///     .draw(&mut display);
///
/// // Rectangle with translation applied
/// Rectangle::new(Point::new(50, 20), Point::new(60, 35))
///     .translate(Point::new(65, 35))
///     .draw(&mut display);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Rectangle<C: PixelColor> {
    /// Top left point of the rect
    pub top_left: Point,

    /// Bottom right point of the rect
    pub bottom_right: Point,

    /// Object style
    pub style: Style<C>,
}

impl<C> Primitive for Rectangle<C> where C: PixelColor {}

impl<C> Dimensions for Rectangle<C>
where
    C: PixelColor,
{
    fn top_left(&self) -> Point {
        self.top_left
    }

    fn bottom_right(&self) -> Point {
        self.bottom_right
    }

    fn size(&self) -> Size {
        Size::from_bounding_box(self.top_left, self.bottom_right)
    }
}

impl<C> Rectangle<C>
where
    C: PixelColor,
{
    /// Create a new rectangle from the top left point to the bottom right point with a given style
    pub fn new(top_left: Point, bottom_right: Point) -> Self {
        Rectangle {
            top_left,
            bottom_right,
            style: Style::default(),
        }
    }
}

impl<C> WithStyle<C> for Rectangle<C>
where
    C: PixelColor,
{
    fn style(mut self, style: Style<C>) -> Self {
        self.style = style;

        self
    }

    fn stroke_color(mut self, color: Option<C>) -> Self {
        self.style.stroke_color = color;

        self
    }

    fn stroke_width(mut self, width: u8) -> Self {
        self.style.stroke_width = width;

        self
    }

    fn fill_color(mut self, color: Option<C>) -> Self {
        self.style.fill_color = color;

        self
    }
}

impl<C> IntoIterator for Rectangle<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = RectangleIterator<C>;

    fn into_iter(self) -> Self::IntoIter {
        (&self).into_iter()
    }
}

impl<'a, C> IntoIterator for &'a Rectangle<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = RectangleIterator<C>;

    fn into_iter(self) -> Self::IntoIter {
        RectangleIterator {
            top_left: self.top_left,
            bottom_right: self.bottom_right,
            style: self.style,
            p: self.top_left,
        }
    }
}

impl<'a, C> IntoIterator for &'a mut Rectangle<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = RectangleIterator<C>;

    fn into_iter(self) -> Self::IntoIter {
        RectangleIterator {
            top_left: self.top_left,
            bottom_right: self.bottom_right,
            style: self.style,
            p: self.top_left,
        }
    }
}

/// Pixel iterator for each pixel in the rect border
#[derive(Debug, Clone, Copy)]
pub struct RectangleIterator<C: PixelColor>
where
    C: PixelColor,
{
    top_left: Point,
    bottom_right: Point,
    style: Style<C>,
    p: Point,
}

impl<C> Iterator for RectangleIterator<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        // Don't render anything if the rectangle has no border or fill color.
        if self.style.stroke_color.is_none() && self.style.fill_color.is_none() {
            return None;
        }

        loop {
            let mut out = None;

            // Finished, i.e. we're below the rect
            if self.p.y > self.bottom_right.y {
                break None;
            }

            let border_width = i32::from(self.style.stroke_width);
            let tl = self.top_left;
            let br = self.bottom_right;

            // Border
            if (
                // Top border
                (self.p.y >= tl.y && self.p.y < tl.y + border_width)
            // Bottom border
            || (self.p.y <= br.y && self.p.y > br.y - border_width)
            // Left border
            || (self.p.x >= tl.x && self.p.x < tl.x + border_width)
            // Right border
            || (self.p.x <= br.x && self.p.x > br.x - border_width)
            ) && self.style.stroke_color.is_some()
            {
                out = Some(Pixel(
                    self.p,
                    self.style.stroke_color.expect("Expected stroke"),
                ));
            }
            // Fill
            else if let Some(fill) = self.style.fill_color {
                out = Some(Pixel(self.p, fill));
            }

            self.p.x += 1;

            // Reached end of row? Jump down one line
            if self.p.x > self.bottom_right.x {
                self.p.x = self.top_left.x;
                self.p.y += 1;
            }

            if out.is_some() {
                break out;
            }
        }
    }
}

impl<'a, C: 'a> Drawable<'a, C> for &Rectangle<C>
where
    C: PixelColor,
{
    fn draw<T: DrawTarget<C>>(self, display: &mut T) {
        display.draw_rectangle(self);
    }
}

impl<C> Transform for Rectangle<C>
where
    C: PixelColor,
{
    /// Translate the rect from its current position to a new position by (x, y) pixels, returning
    /// a new `Rectangle`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Rectangle;
    /// # use embedded_graphics::prelude::*;
    /// # use embedded_graphics::pixelcolor::Rgb565;
    /// #
    /// # let style = Style::stroke_color(Rgb565::RED);
    /// #
    /// let rect = Rectangle::new(Point::new(5, 10), Point::new(15, 20))
    /// #    .style(style);
    /// let moved = rect.translate(Point::new(10, 10));
    ///
    /// assert_eq!(moved.top_left, Point::new(15, 20));
    /// assert_eq!(moved.bottom_right, Point::new(25, 30));
    /// ```
    fn translate(&self, by: Point) -> Self {
        Self {
            top_left: self.top_left + by,
            bottom_right: self.bottom_right + by,
            ..*self
        }
    }

    /// Translate the rect from its current position to a new position by (x, y) pixels.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Rectangle;
    /// # use embedded_graphics::prelude::*;
    /// # use embedded_graphics::pixelcolor::Rgb565;
    /// #
    /// # let style = Style::stroke_color(Rgb565::RED);
    /// #
    /// let mut rect = Rectangle::new(Point::new(5, 10), Point::new(15, 20))
    /// #    .style(style);
    /// rect.translate_mut(Point::new(10, 10));
    ///
    /// assert_eq!(rect.top_left, Point::new(15, 20));
    /// assert_eq!(rect.bottom_right, Point::new(25, 30));
    /// ```
    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.top_left += by;
        self.bottom_right += by;

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelcolor::BinaryColor;
    use crate::pixelcolor::{Rgb565, RgbColor};

    #[test]
    fn dimensions() {
        let rect: Rectangle<BinaryColor> = Rectangle::new(Point::new(5, 10), Point::new(15, 30));
        let moved = rect.translate(Point::new(-10, -20));

        assert_eq!(rect.top_left(), Point::new(5, 10));
        assert_eq!(rect.bottom_right(), Point::new(15, 30));
        assert_eq!(rect.size(), Size::new(10, 20));

        assert_eq!(moved.top_left(), Point::new(-5, -10));
        assert_eq!(moved.bottom_right(), Point::new(5, 10));
        assert_eq!(moved.size(), Size::new(10, 20));
    }

    #[test]
    fn it_can_be_translated() {
        let rect: Rectangle<BinaryColor> = Rectangle::new(Point::new(5, 10), Point::new(15, 20));
        let moved = rect.translate(Point::new(10, 15));

        assert_eq!(moved.top_left, Point::new(15, 25));
        assert_eq!(moved.bottom_right, Point::new(25, 35));
    }

    #[test]
    fn it_draws_unfilled_rect() {
        let mut rect: RectangleIterator<Rgb565> =
            Rectangle::new(Point::new(2, 2), Point::new(4, 4))
                .style(Style::stroke_color(Rgb565::RED))
                .into_iter();

        assert_eq!(rect.next(), Some(Pixel(Point::new(2, 2), Rgb565::RED)));
        assert_eq!(rect.next(), Some(Pixel(Point::new(3, 2), Rgb565::RED)));
        assert_eq!(rect.next(), Some(Pixel(Point::new(4, 2), Rgb565::RED)));

        assert_eq!(rect.next(), Some(Pixel(Point::new(2, 3), Rgb565::RED)));
        assert_eq!(rect.next(), Some(Pixel(Point::new(4, 3), Rgb565::RED)));

        assert_eq!(rect.next(), Some(Pixel(Point::new(2, 4), Rgb565::RED)));
        assert_eq!(rect.next(), Some(Pixel(Point::new(3, 4), Rgb565::RED)));
        assert_eq!(rect.next(), Some(Pixel(Point::new(4, 4), Rgb565::RED)));
    }

    #[test]
    fn it_can_be_negative() {
        let negative: RectangleIterator<Rgb565> =
            Rectangle::new(Point::new(-2, -2), Point::new(2, 2))
                .fill_color(Some(Rgb565::GREEN))
                .into_iter();

        let positive: RectangleIterator<Rgb565> =
            Rectangle::new(Point::new(2, 2), Point::new(6, 6))
                .fill_color(Some(Rgb565::GREEN))
                .into_iter();

        assert!(negative.eq(positive.map(|Pixel(p, c)| Pixel(p - Point::new(4, 4), c))));
    }
}
