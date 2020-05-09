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
            self.bottom_right().map_or(false, |bottom_right| {
                point.x <= bottom_right.x && point.y <= bottom_right.y
            })
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

    /// Creates a new rectangle from the center point and the size.
    ///
    /// For rectangles with even width and/or height the top left corner doesn't
    /// align with the pixel grid. Because of this the coordinates of the top left
    /// corner will be rounded up to the nearest integer coordinate.
    pub fn with_center(center: Point, size: Size) -> Self {
        Rectangle {
            top_left: center - size.center_offset(),
            size,
        }
    }

    /// Returns the center of this rectangle.
    ///
    /// For rectangles with even width and/or height the returned value is rounded down
    /// to the nearest integer coordinate.
    pub fn center(&self) -> Point {
        self.top_left + self.size.center_offset()
    }

    pub(crate) fn expand(&self, offset: u32) -> Self {
        let size = self.size.saturating_add(Size::new(offset * 2, offset * 2));

        Self::with_center(self.center(), size)
    }

    pub(crate) fn shrink(&self, offset: u32) -> Self {
        let size = self.size.saturating_sub(Size::new(offset * 2, offset * 2));

        Self::with_center(self.center(), size)
    }

    /// Returns the bottom right corner of this rectangle.
    ///
    /// Because the smallest rectangle that can be represented by its corners
    /// has a size of 1 x 1 pixels, this function returns `None` if the width or
    /// height of the rectangle is zero.
    pub fn bottom_right(&self) -> Option<Point> {
        if self.size.width > 0 && self.size.height > 0 {
            Some(self.top_left + self.size - Point::new(1, 1))
        } else {
            None
        }
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
        StyledRectangleIterator::new(self)
    }
}

/// Iterator over all points inside the rectangle.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Points {
    left: i32,
    bottom_right: Point,
    current_point: Point,
}

impl Points {
    fn new(rectangle: &Rectangle) -> Self {
        // This doesn't use rectangle.bottom_right() to intentionally set bottom_right
        // to an coordinate outside the rectangle if the width or height is zero, which
        // stops the iterator.
        let bottom_right = rectangle.top_left + rectangle.size - Point::new(1, 1);

        Self {
            left: rectangle.top_left.x,
            bottom_right,
            current_point: rectangle.top_left,
        }
    }

    pub(in crate::primitives) const fn empty() -> Self {
        Self {
            left: 0,
            bottom_right: Point::new(-1, -1),
            current_point: Point::zero(),
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
            self.current_point.x = self.left;
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
    iter: Points,

    stroke_color: Option<C>,

    fill_area: Rectangle,
    fill_color: Option<C>,
}

impl<C> StyledRectangleIterator<C>
where
    C: PixelColor,
{
    pub(in crate::primitives) fn new(styled: &Styled<Rectangle, PrimitiveStyle<C>>) -> Self {
        let Styled { style, primitive } = styled;

        let iter = if !style.is_transparent() {
            let stroke_area = primitive.expand(style.outside_stroke_width());
            stroke_area.points()
        } else {
            Points::empty()
        };

        let fill_area = primitive.shrink(style.inside_stroke_width());

        Self {
            iter,
            stroke_color: style.stroke_color,
            fill_area,
            fill_color: style.fill_color,
        }
    }
}

impl<C> Iterator for StyledRectangleIterator<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        for point in &mut self.iter {
            let color = if self.fill_area.contains(point) {
                self.fill_color
            } else {
                self.stroke_color
            };

            if let Some(color) = color {
                return Some(Pixel(point, color));
            }
        }

        None
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
    use crate::{
        mock_display::MockDisplay,
        pixelcolor::{BinaryColor, Rgb565, RgbColor},
        style::{PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment},
    };

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
    fn points_iter_zero_size() {
        let rectangle = Rectangle::new(Point::new(1, 2), Size::zero());

        let mut points = rectangle.points();
        assert_eq!(points.next(), None);
    }

    #[test]
    fn points_iter_empty() {
        let mut points = Points::empty();
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

    #[test]
    fn bottom_right() {
        let zero = Rectangle::new(Point::new(10, 20), Size::zero());
        assert_eq!(zero.bottom_right(), None);

        let odd = Rectangle::new(Point::new(10, 20), Size::new(5, 7));
        assert_eq!(odd.bottom_right(), Some(Point::new(14, 26)));

        let even = Rectangle::new(Point::new(20, 30), Size::new(4, 8));
        assert_eq!(even.bottom_right(), Some(Point::new(23, 37)));
    }

    #[test]
    fn stroke_alignment() {
        const TOP_LEFT: Point = Point::new(5, 6);
        const SIZE: Size = Size::new(10, 5);

        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 3);

        let mut display_center = MockDisplay::new();
        Rectangle::new(TOP_LEFT, SIZE)
            .into_styled(style)
            .draw(&mut display_center)
            .unwrap();

        let mut display_inside = MockDisplay::new();
        Rectangle::new(TOP_LEFT - Point::new(1, 1), SIZE + Size::new(2, 2))
            .into_styled(
                PrimitiveStyleBuilder::from(&style)
                    .stroke_alignment(StrokeAlignment::Inside)
                    .build(),
            )
            .draw(&mut display_inside)
            .unwrap();

        let mut display_outside = MockDisplay::new();
        Rectangle::new(TOP_LEFT + Point::new(2, 2), SIZE - Size::new(4, 4))
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
