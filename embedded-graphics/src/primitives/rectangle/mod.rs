//! The rectangle primitive. Also good for drawing squares.

mod points_iterator;
mod styled;

use crate::{
    geometry::{Dimensions, Point, Size},
    primitives::{ContainsPoint, Primitive},
    transform::Transform,
};
use core::cmp::min;
pub use points_iterator::Points;
pub use styled::StyledRectangleIterator;

/// Rectangle primitive
///
/// # Examples
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
/// // Rectangle with red 3 pixel wide stroke and green fill with the top left corner at (30, 20) and
/// // a size of (10, 15)
/// let style = PrimitiveStyleBuilder::new()
///     .stroke_color(Rgb565::RED)
///     .stroke_width(3)
///     .fill_color(Rgb565::GREEN)
///     .build();
///
/// Rectangle::new(Point::new(30, 20), Size::new(10, 15))
///     .into_styled(style)
///     .draw(&mut display)?;
///
/// // Rectangle with translation applied
/// Rectangle::new(Point::new(30, 20), Size::new(10, 15))
///     .translate(Point::new(-20, -10))
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

    /// Returns a new `Rectangle` containing the intersection of `self` and `other`.
    ///
    /// If no intersection is present, this method will return a zero sized rectangle.
    ///
    /// # Examples
    ///
    /// ## Intersection
    ///
    /// This example draws two rectangles to a mock display using the `.` character, along with
    /// their intersection shown with `#` characters.
    ///
    /// ```rust
    /// use embedded_graphics::{
    ///     mock_display::MockDisplay, pixelcolor::BinaryColor, prelude::*, primitives::Rectangle,
    ///     style::PrimitiveStyle,
    /// };
    ///
    /// let mut display = MockDisplay::new();
    /// # display.set_allow_overdraw(true);
    ///
    /// let rect1 = Rectangle::new(Point::zero(), Size::new(7, 8));
    /// let rect2 = Rectangle::new(Point::new(2, 3), Size::new(10, 7));
    ///
    /// let intersection = rect1.intersection(&rect2);
    ///
    /// rect1
    ///     .into_styled(PrimitiveStyle::with_stroke(BinaryColor::Off, 1))
    ///     .draw(&mut display)?;
    ///
    /// rect2
    ///     .into_styled(PrimitiveStyle::with_stroke(BinaryColor::Off, 1))
    ///     .draw(&mut display)?;
    ///
    /// intersection
    ///     .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
    ///     .draw(&mut display)?;
    ///
    /// assert_eq!(
    ///     display,
    ///     MockDisplay::from_pattern(&[
    ///         ".......     ",
    ///         ".     .     ",
    ///         ".     .     ",
    ///         ". #####.....",
    ///         ". #   #    .",
    ///         ". #   #    .",
    ///         ". #   #    .",
    ///         "..#####    .",
    ///         "  .        .",
    ///         "  ..........",
    ///     ])
    /// );
    /// # Ok::<(), core::convert::Infallible>(())
    /// ```
    ///
    /// ## No intersection
    ///
    /// This example creates two rectangles with no intersection between them. In this case,
    /// `intersection` returns a zero-sized rectangle.
    ///
    /// ```rust
    /// use embedded_graphics::{prelude::*, primitives::Rectangle, style::PrimitiveStyle};
    ///
    /// let rect1 = Rectangle::new(Point::zero(), Size::new(7, 8));
    /// let rect2 = Rectangle::new(Point::new(10, 15), Size::new(10, 7));
    ///
    /// let intersection = rect1.intersection(&rect2);
    ///
    /// assert_eq!(intersection.size, Size::zero());
    /// # Ok::<(), core::convert::Infallible>(())
    /// ```
    pub fn intersection(&self, other: &Rectangle) -> Rectangle {
        if let (Some(other_bottom_right), Some(self_bottom_right)) =
            (other.bottom_right(), self.bottom_right())
        {
            // Check for overlap
            if self.contains(other.top_left)
                || self.contains(other_bottom_right)
                || other.contains(self.top_left)
                || other.contains(self_bottom_right)
            {
                return Rectangle::with_corners(
                    self.top_left.component_max(other.top_left),
                    self_bottom_right.component_min(other_bottom_right),
                );
            }
        }

        // No overlap present
        Rectangle::new(Point::zero(), Size::zero())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        drawable::{Drawable, Pixel},
        geometry::{Dimensions, Point, Size},
        mock_display::MockDisplay,
        pixelcolor::{BinaryColor, Rgb565, RgbColor},
        primitives::{ContainsPoint, Primitive},
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
    fn stroke_iter_vs_draw() {
        const TOP_LEFT: Point = Point::new(5, 6);
        const SIZE: Size = Size::new(10, 5);

        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 3);

        let rectangle_center = Rectangle::new(TOP_LEFT, SIZE).into_styled(style);

        let mut drawn_center = MockDisplay::new();
        let mut iter_center = MockDisplay::new();
        rectangle_center.draw(&mut drawn_center).unwrap();
        rectangle_center.into_iter().draw(&mut iter_center).unwrap();
        assert_eq!(drawn_center, iter_center);

        let rectangle_inside = Rectangle::new(TOP_LEFT - Point::new(1, 1), SIZE + Size::new(2, 2))
            .into_styled(
                PrimitiveStyleBuilder::from(&style)
                    .stroke_alignment(StrokeAlignment::Inside)
                    .build(),
            );

        let mut drawn_inside = MockDisplay::new();
        let mut iter_inside = MockDisplay::new();
        rectangle_inside.draw(&mut drawn_inside).unwrap();
        rectangle_inside.into_iter().draw(&mut iter_inside).unwrap();
        assert_eq!(drawn_inside, iter_inside);

        let rectangle_outside = Rectangle::new(TOP_LEFT + Point::new(2, 2), SIZE - Size::new(4, 4))
            .into_styled(
                PrimitiveStyleBuilder::from(&style)
                    .stroke_alignment(StrokeAlignment::Outside)
                    .build(),
            );

        let mut drawn_outside = MockDisplay::new();
        let mut iter_outside = MockDisplay::new();
        rectangle_outside.draw(&mut drawn_outside).unwrap();
        rectangle_outside
            .into_iter()
            .draw(&mut iter_outside)
            .unwrap();
        assert_eq!(drawn_outside, iter_outside);
    }

    #[test]
    fn fill_iter_vs_draw() {
        const TOP_LEFT: Point = Point::new(5, 6);
        const SIZE: Size = Size::new(10, 5);

        let style = PrimitiveStyle::with_fill(BinaryColor::On);

        let rectangle = Rectangle::new(TOP_LEFT, SIZE).into_styled(style);

        let mut drawn = MockDisplay::new();
        let mut iter = MockDisplay::new();
        rectangle.draw(&mut drawn).unwrap();
        rectangle.into_iter().draw(&mut iter).unwrap();
        assert_eq!(drawn, iter);
    }

    #[test]
    fn rectangle_intersection() {
        let rect1 = Rectangle::new(Point::new_equal(10), Size::new(20, 30));
        let rect2 = Rectangle::new(Point::new_equal(25), Size::new(30, 40));

        assert_eq!(
            rect1.intersection(&rect2),
            Rectangle::new(Point::new_equal(25), Size::new(5, 15))
        );
    }

    #[test]
    fn rectangle_no_intersection() {
        let rect1 = Rectangle::new(Point::new_equal(10), Size::new(20, 30));
        let rect2 = Rectangle::new(Point::new_equal(35), Size::new(30, 40));

        assert_eq!(
            rect1.intersection(&rect2),
            Rectangle::new(Point::zero(), Size::zero())
        );
    }

    #[test]
    fn rectangle_complete_intersection() {
        let rect1 = Rectangle::new(Point::new_equal(10), Size::new(20, 30));
        let rect2 = rect1;

        assert_eq!(rect1.intersection(&rect2), rect1);
    }

    #[test]
    fn rectangle_contained_intersection() {
        let rect1 = Rectangle::with_corners(Point::new_equal(10), Point::new(20, 30));
        let rect2 = Rectangle::with_corners(Point::new_equal(5), Point::new(30, 40));

        assert_eq!(rect1.intersection(&rect2), rect1);
    }

    #[test]
    fn zero_sized_intersection() {
        let rect1 = Rectangle::new(Point::new(0, 0), Size::new(0, 0));
        let rect2 = Rectangle::new(Point::new(-10, -10), Size::new(20, 20));

        assert_eq!(rect1.intersection(&rect2).size, Size::zero());
    }

    /// Compare the output of the draw() call vs iterators across multiple styles and stroke
    /// alignments.
    fn compare_drawable_iter(rect: Rectangle) {
        let thin_stroke = PrimitiveStyle::with_stroke(Rgb565::RED, 1);
        let stroke = PrimitiveStyle::with_stroke(Rgb565::RED, 5);
        let stroke_fill = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb565::RED)
            .stroke_width(5)
            .fill_color(Rgb565::GREEN)
            .build();
        let fill = PrimitiveStyle::with_fill(Rgb565::BLUE);

        for (name, style) in [
            ("thin_stroke", thin_stroke),
            ("stroke", stroke),
            ("stroke_fill", stroke_fill),
            ("fill", fill),
        ]
        .iter()
        {
            for alignment in [
                StrokeAlignment::Center,
                StrokeAlignment::Inside,
                StrokeAlignment::Outside,
            ]
            .iter()
            {
                let style = PrimitiveStyleBuilder::from(style)
                    .stroke_alignment(*alignment)
                    .build();

                let mut display_drawable = MockDisplay::new();
                let mut display_iter = MockDisplay::new();

                // Calls draw() impl above using fill_solid()
                rect.into_styled(style).draw(&mut display_drawable).unwrap();

                // Calls draw_iter()
                rect.into_styled(style)
                    .into_iter()
                    .draw(&mut display_iter)
                    .unwrap();

                assert_eq!(
                    display_drawable, display_iter,
                    "{} x {} rectangle with style '{}' and alignment {:?} does not match iterator",
                    rect.size.width, rect.size.height, name, alignment
                );
            }
        }
    }

    #[test]
    fn drawable_vs_iterator() {
        compare_drawable_iter(Rectangle::new(Point::new(10, 20), Size::new(20, 30)))
    }

    #[test]
    fn drawable_vs_iterator_squares() {
        for i in 0..20 {
            compare_drawable_iter(Rectangle::new(Point::new(7, 7), Size::new_equal(i)))
        }
    }
}
