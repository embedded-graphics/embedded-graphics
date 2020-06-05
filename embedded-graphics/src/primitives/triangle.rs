//! The triangle primitive.

use crate::{
    drawable::{Drawable, Pixel},
    geometry::{Dimensions, Point},
    pixelcolor::PixelColor,
    primitives::{
        line::{self, Line},
        ContainsPoint, Primitive, Rectangle,
    },
    style::{PrimitiveStyle, Styled},
    transform::Transform,
    DrawTarget,
};
use core::{
    borrow::Borrow,
    cmp::{max, min},
};

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

/// Iterator over all points inside the triangle.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Points {
    line_a: line::Points,
    line_b: line::Points,
    line_c: line::Points,
    cur_ac: Option<Point>,
    cur_b: Option<Point>,
    next_ac: Option<Point>,
    next_b: Option<Point>,
    x: i32,
    max_y: i32,
    min_y: i32,
}

impl Points {
    fn new(triangle: &Triangle) -> Self {
        let (v1, v2, v3) = sort_yx(triangle.p1, triangle.p2, triangle.p3);

        let mut line_a = Line::new(v1, v2).points();
        let mut line_b = Line::new(v1, v3).points();
        let mut line_c = Line::new(v2, v3).points();

        let next_ac = line_a.next().or_else(|| line_c.next());
        let next_b = line_b.next();

        Self {
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
        }
    }

    fn update_ac(&mut self) -> IterState {
        if let Some(ac) = self.next_ac {
            self.cur_ac = Some(ac);
            self.next_ac = self.line_a.next().or_else(|| self.line_c.next());
            self.x = 0;
            IterState::Border(ac)
        } else {
            IterState::None
        }
    }

    fn update_b(&mut self) -> IterState {
        if let Some(b) = self.next_b {
            self.cur_b = Some(b);
            self.next_b = self.line_b.next();
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
                            let (l, r) = sort_two_yx(n_ac, n_b);
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

impl Iterator for Points {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.points() {
                IterState::Border(point) => {
                    // Draw edges of the triangle
                    self.x += 1;
                    return Some(point);
                }
                IterState::LeftRight(l, r) => {
                    // Fill the space between the left and right points
                    if l.x + self.x < r.x {
                        let point = Point::new(l.x + self.x, l.y);
                        self.x += 1;
                        return Some(point);
                    } else if l.x + self.x >= r.x {
                        // We reached the right edge, move on to next row
                        self.cur_ac = None;
                        self.cur_b = None;
                    }
                }
                IterState::None => return None,
            }
        }
    }
}

/// Pixel iterator for each pixel in the triangle border
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct StyledTriangleIterator<C: PixelColor>
where
    C: PixelColor,
{
    line_a: line::Points,
    line_b: line::Points,
    line_c: line::Points,
    cur_ac: Option<Point>,
    cur_b: Option<Point>,
    next_ac: Option<Point>,
    next_b: Option<Point>,
    x: i32,
    max_y: i32,
    min_y: i32,
    style: PrimitiveStyle<C>,
}

impl<C> StyledTriangleIterator<C>
where
    C: PixelColor,
{
    fn new(styled: &Styled<Triangle, PrimitiveStyle<C>>) -> Self {
        let (v1, v2, v3) = sort_yx(
            styled.primitive.p1,
            styled.primitive.p2,
            styled.primitive.p3,
        );

        let mut line_a = Line::new(v1, v2).points();
        let mut line_b = Line::new(v1, v3).points();
        let mut line_c = Line::new(v2, v3).points();

        let next_ac = line_a.next().or_else(|| line_c.next());
        let next_b = line_b.next();

        StyledTriangleIterator {
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
            style: styled.style,
        }
    }
    fn update_ac(&mut self) -> IterState {
        if let Some(ac) = self.next_ac {
            self.cur_ac = Some(ac);
            self.next_ac = self.line_a.next().or_else(|| self.line_c.next());
            self.x = 0;
            IterState::Border(ac)
        } else {
            IterState::None
        }
    }

    fn update_b(&mut self) -> IterState {
        if let Some(b) = self.next_b {
            self.cur_b = Some(b);
            self.next_b = self.line_b.next();
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
                            let (l, r) = sort_two_yx(n_ac, n_b);
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

impl<C> Iterator for StyledTriangleIterator<C>
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
                    if self.style.stroke_width > 0 {
                        if let Some(stroke_color) = self.style.stroke_color {
                            self.x += 1;
                            return Some(Pixel(point, stroke_color));
                        }
                    } else if let Some(fill_color) = self.style.fill_color {
                        self.x += 1;
                        return Some(Pixel(point, fill_color));
                    }
                }
                IterState::LeftRight(l, r) => {
                    // Fill the space between the left and right points
                    if let Some(color) = self.style.fill_color {
                        if l.x + self.x < r.x {
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
    use crate::{
        geometry::Size,
        mock_display::MockDisplay,
        pixelcolor::{BinaryColor, Rgb888, RgbColor},
        style::PrimitiveStyleBuilder,
    };

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
    fn unfilled_no_stroke_width_no_triangle() {
        let mut tri = Triangle::new(Point::new(2, 2), Point::new(4, 2), Point::new(2, 4))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 0))
            .into_iter();

        assert_eq!(tri.next(), None);
    }

    #[test]
    fn stroke_fill_colors() {
        let mut display: MockDisplay<Rgb888> = MockDisplay::new();
        display.set_allow_overdraw(true);

        Triangle::new(Point::new(2, 2), Point::new(8, 2), Point::new(2, 8))
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .stroke_width(1)
                    .stroke_color(Rgb888::RED)
                    .fill_color(Rgb888::GREEN)
                    .build(),
            )
            .draw(&mut display)
            .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "          ",
                "          ",
                "  RRRRRRR ",
                "  RGGGGR  ",
                "  RGGGR   ",
                "  RGGR    ",
                "  RGR     ",
                "  RR      ",
                "  R       ",
            ])
        );
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
    fn points_iter() {
        let triangle = Triangle::new(Point::new(5, 10), Point::new(15, 20), Point::new(10, 15));

        let styled_points = triangle
            .clone()
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .into_iter()
            .map(|Pixel(p, _)| p);

        assert!(triangle.points().eq(styled_points));
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
    fn off_screen_still_draws_points() {
        let off_screen = Triangle::new(Point::new(10, 10), Point::new(20, 20), Point::new(30, -30));
        let on_screen = off_screen.translate(Point::new(0, 35));

        assert!(off_screen
            .points()
            .eq(on_screen.points().map(|p| p - Point::new(0, 35))));
    }

    #[test]
    fn styled_off_screen_still_draws_points() {
        let off_screen = Triangle::new(Point::new(10, 10), Point::new(20, 20), Point::new(30, -30))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On));
        let on_screen = off_screen.translate(Point::new(0, 35));

        assert!(off_screen.into_iter().eq(on_screen
            .into_iter()
            .map(|Pixel(p, col)| Pixel(p - Point::new(0, 35), col))));
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
