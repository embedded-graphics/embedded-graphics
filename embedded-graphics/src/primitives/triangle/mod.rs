//! The triangle primitive.

mod points_iterator;
mod scanline_iterator;
mod styled_iterator;

use crate::{
    drawable::{Drawable, Pixel},
    geometry::{Dimensions, Point},
    pixelcolor::PixelColor,
    primitives::{ContainsPoint, Primitive, Rectangle},
    style::{PrimitiveStyle, Styled},
    transform::Transform,
    DrawTarget,
};
use core::{
    borrow::Borrow,
    cmp::{max, min},
};
pub use points_iterator::Points;
pub use styled_iterator::StyledTriangleIterator;

/// Triangle primitive
///
/// # Examples
///
/// ## Create some triangles with different styles
///
/// ```rust
/// use embedded_graphics::{
///     pixelcolor::Rgb565, prelude::*, primitives::Triangle, style::PrimitiveStyle,
/// };
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::default();
/// # display.set_allow_overdraw(true);
///
/// // Triangle with red 1 px wide stroke
/// Triangle::new(Point::new(40, 20), Point::new(50, 25), Point::new(60, 60))
///     .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 1))
///     .draw(&mut display)?;
///
/// // Triangle with translation applied
/// Triangle::new(Point::new(40, 20), Point::new(50, 25), Point::new(60, 60))
///     .translate(Point::new(-10, -20))
///     .into_styled(PrimitiveStyle::with_stroke(Rgb565::GREEN, 1))
///     .draw(&mut display)?;
/// # Ok::<(), core::convert::Infallible>(())
/// ```
///
/// ## Create a triangle from an array of points
///
/// ```rust
/// use embedded_graphics::{geometry::Point, primitives::Triangle};
///
/// let p1 = Point::new(5, 10);
/// let p2 = Point::new(15, 25);
/// let p3 = Point::new(5, 25);
///
/// // Owned
/// let tri = Triangle::from_points([p1, p2, p3]);
///
/// // Or borrowed
/// let tri_ref = Triangle::from_points(&[p1, p2, p3]);
/// #
/// # assert_eq!(tri, Triangle::new(p1, p2, p3));
/// # assert_eq!(tri_ref, Triangle::new(p1, p2, p3));
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Triangle {
    /// First point of the triangle
    pub p1: Point,

    /// Second point of the triangle
    pub p2: Point,

    /// Third point of the triangle
    pub p3: Point,
}

impl Primitive for Triangle {
    type PointsIter = Points;

    fn points(&self) -> Self::PointsIter {
        Points::new(self)
    }
}

impl ContainsPoint for Triangle {
    fn contains(&self, point: Point) -> bool {
        // Skip expensive calculations below if point is outside the bounding box
        if !self.bounding_box().contains(point) {
            return false;
        }

        // This is inefficient and should be replaced by a better algorithm to
        // determine if point is inside the triangle
        self.points().any(|p| p == point)
    }
}

impl Dimensions for Triangle {
    fn bounding_box(&self) -> Rectangle {
        let x_min = min(min(self.p1.x, self.p2.x), self.p3.x);
        let y_min = min(min(self.p1.y, self.p2.y), self.p3.y);

        let x_max = max(max(self.p1.x, self.p2.x), self.p3.x);
        let y_max = max(max(self.p1.y, self.p2.y), self.p3.y);

        Rectangle::with_corners(Point::new(x_min, y_min), Point::new(x_max, y_max))
    }
}

impl Triangle {
    /// Create a new triangle with a given style
    pub const fn new(p1: Point, p2: Point, p3: Point) -> Self {
        Triangle { p1, p2, p3 }
    }

    /// Creates a new triangle from an array of points.
    ///
    /// This supports both [`Point`]s, as well as anything that implements `Into<Point>` like
    /// `(i32, i32)`.
    ///
    /// [`Point`]: ../../geometry/struct.Point.html
    pub fn from_points<P, I>(points: P) -> Self
    where
        I: Into<Point> + Copy,
        P: Borrow<[I; 3]>,
    {
        let points = points.borrow();

        Triangle {
            p1: points[0].into(),
            p2: points[1].into(),
            p3: points[2].into(),
        }
    }
}

impl Transform for Triangle {
    /// Translate the triangle from its current position to a new position by (x, y) pixels,
    /// returning a new `Triangle`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Triangle;
    /// # use embedded_graphics::prelude::*;
    /// let tri = Triangle::new(Point::new(5, 10), Point::new(15, 20), Point::new(8, 15));
    /// let moved = tri.translate(Point::new(10, 10));
    ///
    /// assert_eq!(moved.p1, Point::new(15, 20));
    /// assert_eq!(moved.p2, Point::new(25, 30));
    /// assert_eq!(moved.p3, Point::new(18, 25));
    /// ```
    fn translate(&self, by: Point) -> Self {
        Self {
            p1: self.p1 + by,
            p2: self.p2 + by,
            p3: self.p3 + by,
        }
    }

    /// Translate the triangle from its current position to a new position by (x, y) pixels.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Triangle;
    /// # use embedded_graphics::prelude::*;
    /// let mut tri = Triangle::new(Point::new(5, 10), Point::new(15, 20), Point::new(10, 15));
    /// tri.translate_mut(Point::new(10, 10));
    ///
    /// assert_eq!(tri.p1, Point::new(15, 20));
    /// assert_eq!(tri.p2, Point::new(25, 30));
    /// assert_eq!(tri.p3, Point::new(20, 25));
    /// ```
    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.p1 += by;
        self.p2 += by;
        self.p3 += by;

        self
    }
}

fn sort_two_yx(p1: Point, p2: Point) -> (Point, Point) {
    if p1.y < p2.y || (p1.y == p2.y && p1.x < p2.x) {
        (p1, p2)
    } else {
        (p2, p1)
    }
}

fn sort_yx(p1: Point, p2: Point, p3: Point) -> (Point, Point, Point) {
    let (y1, y2) = sort_two_yx(p1, p2);
    let (y1, y3) = sort_two_yx(p3, y1);
    let (y2, y3) = sort_two_yx(y3, y2);

    (y1, y2, y3)
}

impl<C> IntoIterator for &Styled<Triangle, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = StyledTriangleIterator<C>;

    fn into_iter(self) -> Self::IntoIter {
        StyledTriangleIterator::new(self)
    }
}

enum IterState {
    Border(Point),
    LeftRight(Point, Point),
    None,
}

impl<'a, C: 'a> Drawable<C> for &Styled<Triangle, PrimitiveStyle<C>>
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
    use crate::{geometry::Size, mock_display::MockDisplay, pixelcolor::BinaryColor};

    #[test]
    fn dimensions() {
        let tri = Triangle::new(Point::new(5, 10), Point::new(15, 25), Point::new(5, 25));
        let moved = tri.translate(Point::new(-10, -11));

        assert_eq!(tri.p1, Point::new(5, 10));
        assert_eq!(tri.p2, Point::new(15, 25));
        assert_eq!(tri.p3, Point::new(5, 25));
        assert_eq!(tri.bounding_box().size, Size::new(11, 16));

        assert_eq!(moved.p1, Point::new(-5, -1));
        assert_eq!(moved.p2, Point::new(5, 14));
        assert_eq!(moved.p3, Point::new(-5, 14));
        assert_eq!(moved.bounding_box().size, Size::new(11, 16));
    }

    #[test]
    fn it_can_be_translated() {
        let tri = Triangle::new(Point::new(5, 10), Point::new(15, 20), Point::new(10, 15));
        let moved = tri.translate(Point::new(5, 10));

        assert_eq!(moved.p1, Point::new(10, 20));
        assert_eq!(moved.p2, Point::new(20, 30));
        assert_eq!(moved.p3, Point::new(15, 25));
    }

    #[test]
    fn it_draws_unfilled_tri_line_y() {
        let mut tri = Triangle::new(Point::new(2, 2), Point::new(2, 4), Point::new(2, 4))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .into_iter();

        // Nodes are returned twice. first line a and b yield the same point.
        // After that line a ends where line c starts.
        assert_eq!(tri.next(), Some(Pixel(Point::new(2, 2), BinaryColor::On)));
        assert_eq!(tri.next(), Some(Pixel(Point::new(2, 2), BinaryColor::On)));
        assert_eq!(tri.next(), Some(Pixel(Point::new(2, 3), BinaryColor::On)));
        assert_eq!(tri.next(), Some(Pixel(Point::new(2, 3), BinaryColor::On)));
        assert_eq!(tri.next(), Some(Pixel(Point::new(2, 4), BinaryColor::On)));
        assert_eq!(tri.next(), Some(Pixel(Point::new(2, 4), BinaryColor::On)));
        assert_eq!(tri.next(), Some(Pixel(Point::new(2, 4), BinaryColor::On)));
        assert_eq!(tri.next(), None);
    }

    #[test]
    fn it_draws_filled_strokeless_tri() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();
        display.set_allow_overdraw(true);

        Triangle::new(Point::new(2, 2), Point::new(2, 4), Point::new(4, 2))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut display)
            .unwrap();

        #[rustfmt::skip]
        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "     ",
                "     ",
                "  ###",
                "  ## ",
                "  #  ",
            ])
        );
    }

    #[test]
    fn it_draws_unfilled_tri_line_x() {
        let mut tri = Triangle::new(Point::new(2, 2), Point::new(4, 2), Point::new(4, 2))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .into_iter();

        assert_eq!(tri.next(), Some(Pixel(Point::new(2, 2), BinaryColor::On)));
        assert_eq!(tri.next(), Some(Pixel(Point::new(2, 2), BinaryColor::On)));
        assert_eq!(tri.next(), Some(Pixel(Point::new(3, 2), BinaryColor::On)));
        assert_eq!(tri.next(), Some(Pixel(Point::new(3, 2), BinaryColor::On)));
        assert_eq!(tri.next(), Some(Pixel(Point::new(4, 2), BinaryColor::On)));
        assert_eq!(tri.next(), Some(Pixel(Point::new(4, 2), BinaryColor::On)));
        assert_eq!(tri.next(), Some(Pixel(Point::new(4, 2), BinaryColor::On)));
        assert_eq!(tri.next(), None);
    }

    #[test]
    #[ignore]
    fn it_can_be_negative() {
        let mut tri = Triangle::new(Point::new(-2, -2), Point::new(2, 0), Point::new(-2, 0))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .into_iter();

        // Only the bottom of the triangle should be visible
        assert_eq!(tri.next(), Some(Pixel(Point::new(0, 0), BinaryColor::On)));
        assert_eq!(tri.next(), Some(Pixel(Point::new(2, 0), BinaryColor::On)));
        assert_eq!(tri.next(), Some(Pixel(Point::new(1, 0), BinaryColor::On)));
        assert_eq!(tri.next(), Some(Pixel(Point::new(2, 0), BinaryColor::On)));
        assert_eq!(tri.next(), None);
    }

    #[test]
    fn contains() {
        let triangle = Triangle::new(Point::new(5, 10), Point::new(15, 20), Point::new(10, 15));

        for point in Rectangle::new(Point::new(0, 5), Size::new(15, 25)).points() {
            let expected = triangle.points().any(|p| p == point);

            assert_eq!(triangle.contains(point), expected, "{:?}", point);
        }
    }

    #[test]
    fn issue_308_infinite() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();
        display.set_allow_overdraw(true);
        display.set_allow_out_of_bounds_drawing(true);

        Triangle::new(Point::new(10, 10), Point::new(20, 30), Point::new(30, -10))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut display)
            .unwrap();
    }

    #[test]
    fn off_screen() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();
        display.set_allow_overdraw(true);
        display.set_allow_out_of_bounds_drawing(true);

        Triangle::new(Point::new(5, 5), Point::new(10, 15), Point::new(15, -5))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut display)
            .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "          #####",
                "         ######",
                "        ###### ",
                "       ####### ",
                "      ######## ",
                "     ######### ",
                "     ########  ",
                "      #######  ",
                "      #######  ",
                "       ######  ",
                "       #####   ",
                "        ####   ",
                "        ####   ",
                "         ###   ",
                "         ##    ",
                "          #    ",
            ])
        );
    }
}
