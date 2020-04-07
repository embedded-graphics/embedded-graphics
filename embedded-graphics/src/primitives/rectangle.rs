//! The rectangle primitive. Also good for drawing squares.

use crate::{
    drawable::{Drawable, Pixel},
    geometry::{Dimensions, Point, Size},
    pixelcolor::PixelColor,
    primitives::{ContainsPoint, Primitive},
    style::{PrimitiveStyle, Styled},
    transform::Transform,
    DrawTarget,
};
use core::cmp::min;

/// Rectangle primitive
///
/// # Examples
///
/// The [macro examples](../../macro.egrectangle.html) make for more concise code.
///
/// ## Create some rectangles with different styles
///
/// ```rust
/// use embedded_graphics::{
///     pixelcolor::Rgb565, prelude::*, primitives::Rectangle, style::PrimitiveStyleBuilder,
/// };
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::default();
///
/// // Rectangle with red 3 pixel wide stroke and green fill from (50, 20) to (60, 35)
/// let style = PrimitiveStyleBuilder::new()
///     .stroke_color(Rgb565::RED)
///     .stroke_width(3)
///     .fill_color(Rgb565::GREEN)
///     .build();
///
/// Rectangle::new(Point::new(50, 20), Size::new(10, 15))
///     .into_styled(style)
///     .draw(&mut display)?;
///
/// // Rectangle with translation applied
/// Rectangle::new(Point::new(50, 20), Size::new(10, 15))
///     .translate(Point::new(65, 35))
///     .into_styled(style)
///     .draw(&mut display)?;
/// # Ok::<(), core::convert::Infallible>(())
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Rectangle {
    /// Top left point of the rectangle.
    pub top_left: Point,

    /// Size of the rectangle.
    pub size: Size,
}

impl Primitive for Rectangle {
    type PointsIter = Points;

    fn points(&self) -> Self::PointsIter {
        Points::new(self)
    }
}

impl ContainsPoint for Rectangle {
    fn contains(&self, point: Point) -> bool {
        if point.x >= self.top_left.x && point.y >= self.top_left.y {
            // FIXME: use Rectangle::bottom_right
            let delta = Size::from_bounding_box(self.top_left, point);

            delta.width <= self.size.width && delta.height <= self.size.height
        } else {
            false
        }
    }
}

impl Dimensions for Rectangle {
    fn bounding_box(&self) -> Rectangle {
        *self
    }
}

impl Rectangle {
    /// Creates a new rectangle from the top left point and the size.
    pub const fn new(top_left: Point, size: Size) -> Self {
        Rectangle { top_left, size }
    }

    /// Creates a new rectangle from two corners.
    pub fn with_corners(corner_1: Point, corner_2: Point) -> Self {
        let left = min(corner_1.x, corner_2.x);
        let top = min(corner_1.y, corner_2.y);

        Rectangle {
            top_left: Point::new(left, top),
            size: Size::from_bounding_box(corner_1, corner_2),
        }
    }

    /// Returns the center of this rectangle.
    ///
    /// For rectangles with even width and/or height the returned value is rounded down
    /// to the nearest integer pixel.
    pub fn center(&self) -> Point {
        let dx = self.size.width.saturating_sub(1) / 2;
        let dy = self.size.height.saturating_sub(1) / 2;

        self.top_left + Size::new(dx, dy)
    }
}

impl Transform for Rectangle {
    /// Translate the rect from its current position to a new position by (x, y) pixels, returning
    /// a new `Rectangle`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Rectangle;
    /// # use embedded_graphics::prelude::*;
    /// let rect = Rectangle::new(Point::new(5, 10), Size::new(10, 10));
    /// let moved = rect.translate(Point::new(10, 10));
    ///
    /// assert_eq!(moved.top_left, Point::new(15, 20));
    /// assert_eq!(moved.size, Size::new(10, 10));
    /// ```
    fn translate(&self, by: Point) -> Self {
        Self {
            top_left: self.top_left + by,
            ..*self
        }
    }

    /// Translate the rect from its current position to a new position by (x, y) pixels.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Rectangle;
    /// # use embedded_graphics::prelude::*;
    /// let mut rect = Rectangle::new(Point::new(5, 10), Size::new(10, 10));
    /// rect.translate_mut(Point::new(10, 10));
    ///
    /// assert_eq!(rect.top_left, Point::new(15, 20));
    /// assert_eq!(rect.size, Size::new(10, 10));
    /// ```
    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.top_left += by;

        self
    }
}

impl<C> IntoIterator for &Styled<Rectangle, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = StyledRectangleIterator<C>;

    fn into_iter(self) -> Self::IntoIter {
        StyledRectangleIterator {
            top_left: self.primitive.top_left,
            bottom_right: self.primitive.top_left + self.primitive.size - Point::new(1, 1),
            style: self.style,
            p: self.primitive.top_left,
        }
    }
}

/// Iterator over all points inside the rectangle.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Points {
    top_left: Point,
    bottom_right: Point,
    current_point: Point,
}

impl Points {
    fn new(rectangle: &Rectangle) -> Self {
        Self {
            top_left: rectangle.top_left,
            bottom_right: rectangle.top_left + rectangle.size - Point::new(1, 1),
            current_point: rectangle.top_left,
        }
    }
}

impl Iterator for Points {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        // Finished, i.e. we're below the rect
        if self.current_point.y > self.bottom_right.y {
            return None;
        }

        let ret = self.current_point;

        self.current_point.x += 1;

        // Reached end of row? Jump down one line
        if self.current_point.x > self.bottom_right.x {
            self.current_point.x = self.top_left.x;
            self.current_point.y += 1;
        }

        Some(ret)
    }
}

/// Pixel iterator for each pixel in the rect border
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct StyledRectangleIterator<C: PixelColor>
where
    C: PixelColor,
{
    top_left: Point,
    bottom_right: Point,
    style: PrimitiveStyle<C>,
    p: Point,
}

impl<C> Iterator for StyledRectangleIterator<C>
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

            let border_width = self.style.stroke_width_i32();
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

impl<C> Drawable<C> for &Styled<Rectangle, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        display.draw_rectangle(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelcolor::{Rgb565, RgbColor};

    #[test]
    fn dimensions() {
        let rect = Rectangle::new(Point::new(5, 10), Size::new(10, 20));
        let moved = rect.translate(Point::new(-10, -20));

        assert_eq!(
            rect.bounding_box(),
            Rectangle::new(Point::new(5, 10), Size::new(10, 20))
        );

        assert_eq!(
            moved.bounding_box(),
            Rectangle::new(Point::new(-5, -10), Size::new(10, 20))
        );
    }

    #[test]
    fn it_can_be_translated() {
        let rect = Rectangle::new(Point::new(5, 10), Size::new(10, 20));
        let moved = rect.translate(Point::new(10, 15));

        let bounding_box = moved.bounding_box();
        assert_eq!(bounding_box.top_left, Point::new(15, 25));
        assert_eq!(bounding_box.size, Size::new(10, 20));
    }

    #[test]
    fn it_draws_unfilled_rect() {
        let mut rect = Rectangle::new(Point::new(2, 2), Size::new(3, 3))
            .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 1))
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
        let negative = Rectangle::new(Point::new(-2, -2), Size::new(4, 4))
            .into_styled(PrimitiveStyle::with_fill(Rgb565::GREEN))
            .into_iter();

        let positive = Rectangle::new(Point::new(2, 2), Size::new(4, 4))
            .into_styled(PrimitiveStyle::with_fill(Rgb565::GREEN))
            .into_iter();

        assert!(negative.eq(positive.map(|Pixel(p, c)| Pixel(p - Point::new(4, 4), c))));
    }

    #[test]
    fn points_iter_matches_filled_styled() {
        let rectangle = Rectangle::new(Point::new(10, 10), Size::new(20, 30));

        let styled_points = rectangle
            .clone()
            .into_styled(PrimitiveStyle::with_fill(Rgb565::WHITE))
            .into_iter()
            .map(|Pixel(p, _)| p);

        assert!(rectangle.points().eq(styled_points));
    }

    #[test]
    fn points_iter() {
        let rectangle = Rectangle::new(Point::new(10, 20), Size::new(2, 3));

        let mut points = rectangle.points();
        assert_eq!(points.next(), Some(Point::new(10, 20)));
        assert_eq!(points.next(), Some(Point::new(11, 20)));
        assert_eq!(points.next(), Some(Point::new(10, 21)));
        assert_eq!(points.next(), Some(Point::new(11, 21)));
        assert_eq!(points.next(), Some(Point::new(10, 22)));
        assert_eq!(points.next(), Some(Point::new(11, 22)));
        assert_eq!(points.next(), None);
    }

    #[test]
    fn contains() {
        let outer = Rectangle::new(Point::zero(), Size::new(10, 10));
        let inner = Rectangle::new(Point::new(2, 4), Size::new(3, 5));

        for p in outer.points() {
            let expected = p.x >= 2 && p.x < 2 + 3 && p.y >= 4 && p.y < 4 + 5;

            assert_eq!(inner.contains(p), expected, "{:?}", p);
        }
    }

    #[test]
    fn center() {
        let odd = Rectangle::new(Point::new(10, 20), Size::new(5, 7));
        assert_eq!(odd.center(), Point::new(12, 23));

        let even = Rectangle::new(Point::new(20, 30), Size::new(4, 8));
        assert_eq!(even.center(), Point::new(21, 33));
    }
}
