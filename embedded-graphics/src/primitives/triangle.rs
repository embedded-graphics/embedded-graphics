//! The triangle primitive.

use super::super::drawable::*;
use super::super::transform::*;
use crate::coord::{Coord, ToUnsigned};
use crate::pixelcolor::PixelColor;
use crate::primitives::line::{Line, LineIterator};
use crate::primitives::Primitive;
use crate::style::Style;
use crate::style::WithStyle;
use crate::unsignedcoord::{ToSigned, UnsignedCoord};

// TODO: Impl Default so people can leave the color bit out
/// Triangle primitive
#[derive(Debug, Clone, Copy)]
pub struct Triangle<C: PixelColor> {
    /// first triangle point
    pub p1: Coord,
    /// second triangle point
    pub p2: Coord,
    /// third triangle point
    pub p3: Coord,

    /// Object style
    pub style: Style<C>,
}

impl<C> Primitive for Triangle<C> where C: PixelColor {}

impl<C> Dimensions for Triangle<C>
where
    C: PixelColor,
{
    fn top_left(&self) -> Coord {
        let x = if self.p1[0] <= self.p2[0] && self.p1[0] <= self.p3[0] {
            self.p1[0]
        } else if self.p2[0] <= self.p1[0] && self.p2[0] <= self.p3[0] {
            self.p2[0]
        } else {
            self.p3[0]
        };
        let y = if self.p1[1] <= self.p2[1] && self.p1[1] <= self.p3[1] {
            self.p1[1]
        } else if self.p2[1] <= self.p1[1] && self.p2[1] <= self.p3[1] {
            self.p2[1]
        } else {
            self.p3[1]
        };
        Coord::new(x, y)
    }

    fn bottom_right(&self) -> Coord {
        let x = if self.p1[0] >= self.p2[0] && self.p1[0] >= self.p3[0] {
            self.p1[0]
        } else if self.p2[0] >= self.p1[0] && self.p2[0] >= self.p3[0] {
            self.p2[0]
        } else {
            self.p3[0]
        };
        let y = if self.p1[1] >= self.p2[1] && self.p1[1] >= self.p3[1] {
            self.p1[1]
        } else if self.p2[1] >= self.p1[1] && self.p2[1] >= self.p3[1] {
            self.p2[1]
        } else {
            self.p3[1]
        };
        Coord::new(x, y)
    }

    fn size(&self) -> UnsignedCoord {
        (self.bottom_right() - self.top_left()).abs().to_unsigned()
    }
}

impl<C> Triangle<C>
where
    C: PixelColor,
{
    /// Create a new triangle with a given style
    pub fn new(p1: Coord, p2: Coord, p3: Coord) -> Self {
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
    fn with_style(mut self, style: Style<C>) -> Self {
        self.style = style;

        self
    }

    fn with_stroke(mut self, color: Option<C>) -> Self {
        self.style.stroke_color = color;

        self
    }

    fn with_stroke_width(mut self, width: u8) -> Self {
        self.style.stroke_width = width;

        self
    }

    fn with_fill(mut self, color: Option<C>) -> Self {
        self.style.fill_color = color;

        self
    }
}

fn sort_two_yx(p1: Coord, p2: Coord) -> (Coord, Coord) {
    if p1[1] < p2[1] || (p1[1] == p2[1] && p1[0] < p2[0]) {
        (p1, p2)
    } else {
        (p2, p1)
    }
}

fn sort_yx(p1: Coord, p2: Coord, p3: Coord) -> (Coord, Coord, Coord) {
    let (y1, y2) = sort_two_yx(p1, p2);
    let (y1, y3) = sort_two_yx(p3, y1);
    let (y2, y3) = sort_two_yx(y3, y2);

    (y1, y2, y3)
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
            .with_stroke(self.style.stroke_color.or(self.style.fill_color))
            .into_iter();
        let mut line_b = Line::new(v1, v3)
            .with_stroke(self.style.stroke_color.or(self.style.fill_color))
            .into_iter();
        let mut line_c = Line::new(v2, v3)
            .with_stroke(self.style.stroke_color.or(self.style.fill_color))
            .into_iter();
        let next_ac = line_a
            .next()
            .or_else(|| line_c.next())
            .map(|p| p.0.to_signed());
        let next_b = line_b.next().map(|p| p.0.to_signed());

        TriangleIterator {
            line_a,
            line_b,
            line_c,
            cur_ac: None,
            cur_b: None,
            next_ac,
            next_b,
            x: 0,
            min_y: v1[1],
            max_y: v3[1],
            style: self.style,
        }
    }
}

enum IterState {
    Border(Coord),
    LeftRight(Coord, Coord),
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
    cur_ac: Option<Coord>,
    cur_b: Option<Coord>,
    next_ac: Option<Coord>,
    next_b: Option<Coord>,
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
                .map(|p| p.0.to_signed());
            self.x = 0;
            IterState::Border(ac)
        } else {
            IterState::None
        }
    }

    fn update_b(&mut self) -> IterState {
        if let Some(b) = self.next_b {
            self.cur_b = Some(b);
            self.next_b = self.line_b.next().map(|p| p.0.to_signed());
            self.x = 0;
            IterState::Border(b)
        } else {
            IterState::None
        }
    }

    fn points(&mut self) -> IterState {
        match (self.cur_ac, self.cur_b) {
            // point of ac line or b line is missing
            (None, _) => self.update_ac(),
            (_, None) => self.update_b(),
            // both points are present
            (Some(ac), Some(b)) => {
                match (self.next_ac, self.next_b) {
                    (Some(n_ac), Some(n_b)) => {
                        // if y component differs, take new points from edge until
                        // both side have the same y
                        if n_ac[1] < n_b[1] {
                            self.update_ac()
                        } else if n_ac[1] > n_b[1] {
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
                IterState::Border(coord) => {
                    // draw edges of the triangle
                    if let Some(color) = self.style.stroke_color.or_else(|| self.style.fill_color) {
                        if coord[0] >= 0 && coord[1] >= 0 {
                            return Some(Pixel(coord.to_unsigned(), color));
                        }
                    }
                }
                IterState::LeftRight(l, r) => {
                    // fill the space between the left and right points
                    if let Some(color) = self.style.fill_color {
                        if l[0] >= 0 && l[1] >= 0 && r[0] >= 0 && r[1] >= 0 && l[0] + self.x < r[0]
                        {
                            let coord = UnsignedCoord::new((l[0] + self.x) as u32, l[1] as u32);
                            self.x += 1;
                            return Some(Pixel(coord, color));
                        } else if l[0] + self.x >= r[0] {
                            // we reached the right edge, move on to next row
                            self.cur_ac = None;
                            self.cur_b = None;
                        }
                    } else {
                        // we don't want to fill the triangle
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
    /// Translate the triangle from its current position to a new position by (x, y) pixels, returning
    /// a new `Triangle`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Triangle;
    /// # use embedded_graphics::dev::TestPixelColor;
    /// # use embedded_graphics::prelude::*;
    /// #
    /// # let style: Style<TestPixelColor> = Style::with_stroke(TestPixelColor(1));
    /// #
    /// let tri = Triangle::new(Coord::new(5, 10), Coord::new(15, 20), Coord::new(8, 15))
    /// #    .with_style(style);
    /// let moved = tri.translate(Coord::new(10, 10));
    ///
    /// assert_eq!(moved.p1, Coord::new(15, 20));
    /// assert_eq!(moved.p2, Coord::new(25, 30));
    /// assert_eq!(moved.p3, Coord::new(18, 25));
    /// ```
    fn translate(&self, by: Coord) -> Self {
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
    /// # use embedded_graphics::dev::TestPixelColor;
    /// # use embedded_graphics::prelude::*;
    /// #
    /// # let style: Style<TestPixelColor> = Style::with_stroke(TestPixelColor(1));
    /// #
    /// let mut tri = Triangle::new(Coord::new(5, 10), Coord::new(15, 20), Coord::new(10, 15))
    /// #    .with_style(style);
    /// tri.translate_mut(Coord::new(10, 10));
    ///
    /// assert_eq!(tri.p1, Coord::new(15, 20));
    /// assert_eq!(tri.p2, Coord::new(25, 30));
    /// assert_eq!(tri.p3, Coord::new(20, 25));
    /// ```
    fn translate_mut(&mut self, by: Coord) -> &mut Self {
        self.p1 += by;
        self.p2 += by;
        self.p3 += by;

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dev::TestPixelColor;
    use crate::unsignedcoord::UnsignedCoord;

    #[test]
    fn dimensions() {
        let tri: Triangle<TestPixelColor> =
            Triangle::new(Coord::new(5, 10), Coord::new(15, 20), Coord::new(5, 20));
        let moved = tri.translate(Coord::new(-10, -10));

        assert_eq!(tri.p1, Coord::new(5, 10));
        assert_eq!(tri.p2, Coord::new(15, 20));
        assert_eq!(tri.p3, Coord::new(5, 20));
        assert_eq!(tri.size(), UnsignedCoord::new(10, 10));

        assert_eq!(moved.p1, Coord::new(-5, 0));
        assert_eq!(moved.p2, Coord::new(5, 10));
        assert_eq!(moved.p3, Coord::new(-5, 10));
        assert_eq!(moved.size(), UnsignedCoord::new(10, 10));
    }

    #[test]
    fn it_can_be_translated() {
        let tri: Triangle<TestPixelColor> =
            Triangle::new(Coord::new(5, 10), Coord::new(15, 20), Coord::new(10, 15));
        let moved = tri.translate(Coord::new(10, 10));

        assert_eq!(moved.p1, Coord::new(15, 20));
        assert_eq!(moved.p2, Coord::new(25, 30));
        assert_eq!(moved.p3, Coord::new(20, 25));
    }

    #[test]
    fn it_draws_unfilled_tri_line_y() {
        let mut tri: TriangleIterator<TestPixelColor> =
            Triangle::new(Coord::new(2, 2), Coord::new(2, 4), Coord::new(2, 4))
                .with_style(Style::with_stroke(1u8.into()))
                .into_iter();

        // Nodes are returned twice. first line a and b yield the same point.
        // After that line a ends where line c starts.
        assert_eq!(tri.next(), Some(Pixel(UnsignedCoord::new(2, 2), 1.into())));
        assert_eq!(tri.next(), Some(Pixel(UnsignedCoord::new(2, 2), 1.into())));
        assert_eq!(tri.next(), Some(Pixel(UnsignedCoord::new(2, 3), 1.into())));
        assert_eq!(tri.next(), Some(Pixel(UnsignedCoord::new(2, 3), 1.into())));
        assert_eq!(tri.next(), Some(Pixel(UnsignedCoord::new(2, 4), 1.into())));
        assert_eq!(tri.next(), Some(Pixel(UnsignedCoord::new(2, 4), 1.into())));
        assert_eq!(tri.next(), Some(Pixel(UnsignedCoord::new(2, 4), 1.into())));
        assert_eq!(tri.next(), None);
    }

    #[test]
    fn it_draws_unfilled_tri_line_x() {
        let mut tri: TriangleIterator<TestPixelColor> =
            Triangle::new(Coord::new(2, 2), Coord::new(4, 2), Coord::new(4, 2))
                .with_style(Style::with_stroke(1u8.into()))
                .into_iter();

        assert_eq!(tri.next(), Some(Pixel(UnsignedCoord::new(2, 2), 1.into())));
        assert_eq!(tri.next(), Some(Pixel(UnsignedCoord::new(2, 2), 1.into())));
        assert_eq!(tri.next(), Some(Pixel(UnsignedCoord::new(3, 2), 1.into())));
        assert_eq!(tri.next(), Some(Pixel(UnsignedCoord::new(3, 2), 1.into())));
        assert_eq!(tri.next(), Some(Pixel(UnsignedCoord::new(4, 2), 1.into())));
        assert_eq!(tri.next(), Some(Pixel(UnsignedCoord::new(4, 2), 1.into())));
        assert_eq!(tri.next(), Some(Pixel(UnsignedCoord::new(4, 2), 1.into())));
        assert_eq!(tri.next(), None);
    }

    #[test]
    fn it_can_be_negative() {
        let mut tri: TriangleIterator<TestPixelColor> =
            Triangle::new(Coord::new(-2, -2), Coord::new(2, 0), Coord::new(-2, 0))
                .with_style(Style::with_stroke(1u8.into()))
                .into_iter();

        // TODO: Macro
        // Only the bottom of the triangle should be visible
        assert_eq!(tri.next(), Some(Pixel(UnsignedCoord::new(0, 0), 1.into())));
        assert_eq!(tri.next(), Some(Pixel(UnsignedCoord::new(2, 0), 1.into())));
        assert_eq!(tri.next(), Some(Pixel(UnsignedCoord::new(1, 0), 1.into())));
        assert_eq!(tri.next(), Some(Pixel(UnsignedCoord::new(2, 0), 1.into())));
        assert_eq!(tri.next(), None);
    }
}
