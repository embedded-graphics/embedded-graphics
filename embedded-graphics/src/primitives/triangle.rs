//! The triangle primitive.

use super::super::drawable::{Drawable, Pixel};
use super::super::transform::Transform;
use crate::geometry::{Dimensions, Point, Size};
use crate::pixelcolor::PixelColor;
use crate::primitives::line::{Line, LineIterator};
use crate::primitives::Primitive;
use crate::style::Style;
use crate::style::WithStyle;

/// Triangle primitive
///
/// # Examples
///
/// The [macro examples](../../macro.egtriangle.html) make for more concise code.
///
/// ## Create some triangles with different styles
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::primitives::Triangle;
/// use embedded_graphics::pixelcolor::Rgb565;
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::default();
///
/// // Default triangle with no styling
/// let t1 = Triangle::new(Point::new(10, 20), Point::new(30, 40), Point::new(50, 60));
///
/// // Triangle with styled stroke from (50, 20) to (60, 35)
/// let t2 = Triangle::new(Point::new(50, 20), Point::new(60, 35), Point::new(70, 80))
///     .stroke(Some(Rgb565::RED));
///
/// // Triangle with translation applied
/// let t3 = Triangle::new(Point::new(50, 20), Point::new(60, 35), Point::new(70, 80))
///     .translate(Point::new(65, 35));
///
/// display.draw(t1);
/// display.draw(t2);
/// display.draw(t3);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Triangle<C: PixelColor> {
    /// First point of the triangle
    pub p1: Point,

    /// Second point of the triangle
    pub p2: Point,

    /// Third point of the triangle
    pub p3: Point,

    /// Object style
    pub style: Style<C>,
}

impl<C> Primitive for Triangle<C> where C: PixelColor {}

impl<C> Dimensions for Triangle<C>
where
    C: PixelColor,
{
    fn top_left(&self) -> Point {
        let &x = [self.p1.x, self.p2.x, self.p3.x].iter().min().unwrap();
        let &y = [self.p1.y, self.p2.y, self.p3.y].iter().min().unwrap();

        Point::new(x, y)
    }

    fn bottom_right(&self) -> Point {
        let &x = [self.p1.x, self.p2.x, self.p3.x].iter().max().unwrap();
        let &y = [self.p1.y, self.p2.y, self.p3.y].iter().max().unwrap();

        Point::new(x, y)
    }

    fn size(&self) -> Size {
        Size::from_bounding_box(self.top_left(), self.bottom_right())
    }
}

impl<C> Triangle<C>
where
    C: PixelColor,
{
    /// Create a new triangle with a given style
    pub fn new(p1: Point, p2: Point, p3: Point) -> Self {
        Triangle {
            p1,
            p2,
            p3,
            style: Style::default(),
        }
    }
}

impl<C> WithStyle<C> for Triangle<C>
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

impl<C> IntoIterator for Triangle<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = TriangleIterator<C>;

    fn into_iter(self) -> Self::IntoIter {
        (&self).into_iter()
    }
}

impl<'a, C> IntoIterator for &'a Triangle<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = TriangleIterator<C>;

    fn into_iter(self) -> Self::IntoIter {
        let (v1, v2, v3) = sort_yx(self.p1, self.p2, self.p3);

        let mut line_a = Line::new(v1, v2)
            .stroke(self.style.stroke_color.or(self.style.fill_color))
            .into_iter();
        let mut line_b = Line::new(v1, v3)
            .stroke(self.style.stroke_color.or(self.style.fill_color))
            .into_iter();
        let mut line_c = Line::new(v2, v3)
            .stroke(self.style.stroke_color.or(self.style.fill_color))
            .into_iter();
        let next_ac = line_a.next().or_else(|| line_c.next()).map(|p| p.0);
        let next_b = line_b.next().map(|p| p.0);

        TriangleIterator {
            line_a,
            line_b,
            line_c,
            cur_ac: None,
            cur_b: None,
            next_ac,
            next_b,
            x: 0,
            min_y: v1.y,
            max_y: v3.y,
            style: self.style,
        }
    }
}

enum IterState {
    Border(Point),
    LeftRight(Point, Point),
    None,
}

/// Pixel iterator for each pixel in the triangle border
#[derive(Debug, Clone, Copy)]
pub struct TriangleIterator<C: PixelColor>
where
    C: PixelColor,
{
    line_a: LineIterator<C>,
    line_b: LineIterator<C>,
    line_c: LineIterator<C>,
    cur_ac: Option<Point>,
    cur_b: Option<Point>,
    next_ac: Option<Point>,
    next_b: Option<Point>,
    x: i32,
    max_y: i32,
    min_y: i32,
    style: Style<C>,
}

impl<C> TriangleIterator<C>
where
    C: PixelColor,
{
    fn update_ac(&mut self) -> IterState {
        if let Some(ac) = self.next_ac {
            self.cur_ac = Some(ac);
            self.next_ac = self
                .line_a
                .next()
                .or_else(|| self.line_c.next())
                .map(|p| p.0);
            self.x = 0;
            IterState::Border(ac)
        } else {
            IterState::None
        }
    }

    fn update_b(&mut self) -> IterState {
        if let Some(b) = self.next_b {
            self.cur_b = Some(b);
            self.next_b = self.line_b.next().map(|p| p.0);
            self.x = 0;
            IterState::Border(b)
        } else {
            IterState::None
        }
    }

    fn points(&mut self) -> IterState {
        match (self.cur_ac, self.cur_b) {
            // Point of ac line or b line is missing
            (None, _) => self.update_ac(),
            (_, None) => self.update_b(),
            // Both points are present
            (Some(ac), Some(b)) => {
                match (self.next_ac, self.next_b) {
                    (Some(n_ac), Some(n_b)) => {
                        // If y component differs, take new points from edge until both side have
                        // the same y
                        if n_ac.y < n_b.y {
                            self.update_ac()
                        } else if n_ac.y > n_b.y {
                            self.update_b()
                        } else {
                            let (l, r) = sort_two_yx(ac, b);
                            IterState::LeftRight(l, r)
                        }
                    }
                    (None, Some(_)) => self.update_b(),
                    (Some(_), None) => self.update_ac(),
                    (None, None) => {
                        let (l, r) = sort_two_yx(ac, b);
                        IterState::LeftRight(l, r)
                    }
                }
            }
        }
    }
}

impl<C> Iterator for TriangleIterator<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.style.stroke_color.is_none() && self.style.fill_color.is_none() {
            return None;
        }

        loop {
            match self.points() {
                IterState::Border(point) => {
                    // Draw edges of the triangle
                    if let Some(color) = self.style.stroke_color.or_else(|| self.style.fill_color) {
                        if point.x >= 0 && point.y >= 0 {
                            return Some(Pixel(point, color));
                        }
                    }
                }
                IterState::LeftRight(l, r) => {
                    // Fill the space between the left and right points
                    if let Some(color) = self.style.fill_color {
                        if l.x >= 0 && l.y >= 0 && r.x >= 0 && r.y >= 0 && l.x + self.x < r.x {
                            let point = Point::new(l.x + self.x, l.y);
                            self.x += 1;
                            return Some(Pixel(point, color));
                        } else if l.x + self.x >= r.x {
                            // We reached the right edge, move on to next row
                            self.cur_ac = None;
                            self.cur_b = None;
                        }
                    } else {
                        // We don't want to fill the triangle
                        self.cur_ac = None;
                        self.cur_b = None;
                    }
                }
                IterState::None => return None,
            }
        }
    }
}

impl<C> Drawable for Triangle<C> where C: PixelColor {}

impl<C> Transform for Triangle<C>
where
    C: PixelColor,
{
    /// Translate the triangle from its current position to a new position by (x, y) pixels,
    /// returning a new `Triangle`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Triangle;
    /// # use embedded_graphics::prelude::*;
    /// # use embedded_graphics::pixelcolor::BinaryColor;
    /// #
    /// # let style = Style::stroke(BinaryColor::On);
    /// #
    /// let tri = Triangle::new(Point::new(5, 10), Point::new(15, 20), Point::new(8, 15))
    /// #    .style(style);
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
            ..*self
        }
    }

    /// Translate the triangle from its current position to a new position by (x, y) pixels.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Triangle;
    /// # use embedded_graphics::prelude::*;
    /// # use embedded_graphics::pixelcolor::BinaryColor;
    /// #
    /// # let style = Style::stroke(BinaryColor::On);
    /// #
    /// let mut tri = Triangle::new(Point::new(5, 10), Point::new(15, 20), Point::new(10, 15))
    /// #    .style(style);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelcolor::BinaryColor;

    #[test]
    fn dimensions() {
        let tri: Triangle<BinaryColor> =
            Triangle::new(Point::new(5, 10), Point::new(15, 25), Point::new(5, 25));
        let moved = tri.translate(Point::new(-10, -11));

        assert_eq!(tri.p1, Point::new(5, 10));
        assert_eq!(tri.p2, Point::new(15, 25));
        assert_eq!(tri.p3, Point::new(5, 25));
        assert_eq!(tri.size(), Size::new(10, 15));

        assert_eq!(moved.p1, Point::new(-5, -1));
        assert_eq!(moved.p2, Point::new(5, 14));
        assert_eq!(moved.p3, Point::new(-5, 14));
        assert_eq!(moved.size(), Size::new(10, 15));
    }

    #[test]
    fn it_can_be_translated() {
        let tri: Triangle<BinaryColor> =
            Triangle::new(Point::new(5, 10), Point::new(15, 20), Point::new(10, 15));
        let moved = tri.translate(Point::new(5, 10));

        assert_eq!(moved.p1, Point::new(10, 20));
        assert_eq!(moved.p2, Point::new(20, 30));
        assert_eq!(moved.p3, Point::new(15, 25));
    }

    #[test]
    fn it_draws_unfilled_tri_line_y() {
        let mut tri: TriangleIterator<BinaryColor> =
            Triangle::new(Point::new(2, 2), Point::new(2, 4), Point::new(2, 4))
                .style(Style::stroke(BinaryColor::On))
                .into_iter();

        // Nodes are returned twice. first line a and b yield the same point.
        // After that line a ends where line c starts.
        assert_eq!(tri.next(), Some(Pixel(Point::new(2, 2), BinaryColor::On)));
        assert_eq!(tri.next(), Some(Pixel(Point::new(2, 2), BinaryColor::On)));
        assert_eq!(tri.next(), Some(Pixel(Point::new(2, 3), BinaryColor::On)));
        assert_eq!(tri.next(), Some(Pixel(Point::new(2, 3), BinaryColor::On)));
        assert_eq!(tri.next(), Some(Pixel(Point::new(2, 4), BinaryColor::On)));
        assert_eq!(tri.next(), Some(Pixel(Point::new(2, 4), BinaryColor::On)));
        assert_eq!(tri.next(), None);
    }

    #[test]
    fn it_draws_unfilled_tri_line_x() {
        let mut tri: TriangleIterator<BinaryColor> =
            Triangle::new(Point::new(2, 2), Point::new(4, 2), Point::new(4, 2))
                .style(Style::stroke(BinaryColor::On))
                .into_iter();

        assert_eq!(tri.next(), Some(Pixel(Point::new(2, 2), BinaryColor::On)));
        assert_eq!(tri.next(), Some(Pixel(Point::new(2, 2), BinaryColor::On)));
        assert_eq!(tri.next(), Some(Pixel(Point::new(3, 2), BinaryColor::On)));
        assert_eq!(tri.next(), Some(Pixel(Point::new(3, 2), BinaryColor::On)));
        assert_eq!(tri.next(), Some(Pixel(Point::new(4, 2), BinaryColor::On)));
        assert_eq!(tri.next(), Some(Pixel(Point::new(4, 2), BinaryColor::On)));
        assert_eq!(tri.next(), None);
    }

    #[test]
    #[ignore]
    fn it_can_be_negative() {
        let mut tri: TriangleIterator<BinaryColor> =
            Triangle::new(Point::new(-2, -2), Point::new(2, 0), Point::new(-2, 0))
                .style(Style::stroke(BinaryColor::On))
                .into_iter();

        // Only the bottom of the triangle should be visible
        assert_eq!(tri.next(), Some(Pixel(Point::new(0, 0), BinaryColor::On)));
        assert_eq!(tri.next(), Some(Pixel(Point::new(2, 0), BinaryColor::On)));
        assert_eq!(tri.next(), Some(Pixel(Point::new(1, 0), BinaryColor::On)));
        assert_eq!(tri.next(), Some(Pixel(Point::new(2, 0), BinaryColor::On)));
        assert_eq!(tri.next(), None);
    }
}
