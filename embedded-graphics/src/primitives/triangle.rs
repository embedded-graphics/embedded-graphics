//! The triangle primitive.

use super::super::drawable::*;
use super::super::transform::*;
use super::line::{LineIterator, Line};
use crate::coord::{Coord, ToUnsigned};
use crate::pixelcolor::PixelColor;
use crate::primitives::Primitive;
use crate::style::Style;
use crate::style::WithStyle;
use crate::unsignedcoord::UnsignedCoord;

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

fn sort_two(p1: Coord, p2: Coord) -> (Coord, Coord) {
    if p1[1] < p2[1] || (p1[1] == p2[1] && p1[0] < p2[0]) {
        (p1, p2)
    } else {
        (p1, p2)
    }
}

fn sort_y(p1: Coord, p2: Coord, p3: Coord) -> (Coord, Coord, Coord) {
    let (y1, y2) = sort_two(p1, p2);
    let (y1, y3) = sort_two(p3, y1);
    let (y2, y3) = sort_two(y3, y2);

    (y1, y2, y3)
}

impl<'a, C> IntoIterator for &'a Triangle<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = TriangleIterator<C>;

    fn into_iter(self) -> Self::IntoIter {
        let (v1, v2, v3) = sort_y(self.p1, self.p2, self.p3);

        TriangleIterator {
            line_a: Line::new(v1, v2).into_iter(),
            line_b: Line::new(v1, v3).into_iter(),
            line_c: Line::new(v2, v3).into_iter(),
            y: v1[1],
            x: v1[0],
            xmin: v1[0],
            xmax: v1[0],
            style: self.style,
        }
    }
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
    y: i32,
    x: i32,
    xmin: i32,
    xmax: i32,
    style: Style<C>,

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
        let border_width = self.style.stroke_width as i32;

        if self.xmin <= self.x && self.xmin + border_width >= self.x 
        && self.x <= self.xmax && self.x <= self.xmax - border_width
        && self.style.stroke_color.is_some() {
            Some(Pixel(Coord::new(self.x as i32, self.y as i32), 
                       self.style.stroke_color.expect("Expected color")))
        } else {
            None
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
            ..self.clone()
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
        let tri: Triangle<TestPixelColor> = Triangle::new(Coord::new(5, 10), Coord::new(15, 20), Coord::new(5, 20));
        let moved = tri.translate(Coord::new(-10, -10));

        assert_eq!(tri.p1, Coord::new(5, 10));
        assert_eq!(tri.p2, Coord::new(15, 20));
        assert_eq!(tri.p3, Coord::new(5, 20));
        assert_eq!(tri.size(), UnsignedCoord::new(10, 10));

        assert_eq!(moved.p1, Coord::new(-5, 0));
        assert_eq!(moved.p2, Coord::new(5, 10));
        assert_eq!(moved.p3, Coord::new(5, 10));
        assert_eq!(moved.size(), UnsignedCoord::new(10, 10));
    }

    #[test]
    fn it_can_be_translated() {
        let tri: Triangle<TestPixelColor> = Triangle::new(Coord::new(5, 10), Coord::new(15, 20), Coord::new(10, 15));
        let moved = tri.translate(Coord::new(10, 10));

        assert_eq!(moved.p1, Coord::new(15, 20));
        assert_eq!(moved.p2, Coord::new(25, 30));
        assert_eq!(moved.p3, Coord::new(20, 25));
    }

    #[test]
    fn it_draws_unfilled_tri() {
        let mut tri: TriangleIterator<TestPixelColor> = Triangle::new(Coord::new(2, 2), Coord::new(4, 4), Coord::new(2, 4))
            .with_style(Style::with_stroke(1u8.into()))
            .into_iter();

        assert_eq!(tri.next(), Some(Pixel(UnsignedCoord::new(2, 2), 1.into())));
        assert_eq!(tri.next(), Some(Pixel(UnsignedCoord::new(3, 2), 1.into())));
        assert_eq!(tri.next(), Some(Pixel(UnsignedCoord::new(4, 2), 1.into())));

        assert_eq!(tri.next(), Some(Pixel(UnsignedCoord::new(2, 3), 1.into())));
        assert_eq!(tri.next(), Some(Pixel(UnsignedCoord::new(4, 3), 1.into())));

        assert_eq!(tri.next(), Some(Pixel(UnsignedCoord::new(2, 4), 1.into())));
        assert_eq!(tri.next(), Some(Pixel(UnsignedCoord::new(3, 4), 1.into())));
        assert_eq!(tri.next(), Some(Pixel(UnsignedCoord::new(4, 4), 1.into())));
    }

    #[test]
    fn it_can_be_negative() {
        let mut rect: TriangleIterator<TestPixelColor> =
            Triangle::new(Coord::new(-2, -2), Coord::new(2, 2), Coord::new(-2, 2))
                .with_style(Style::with_stroke(1u8.into()))
                .into_iter();

        // TODO: Macro
        // Only the bottom right corner of the rect should be visible
        assert_eq!(rect.next(), Some(Pixel(UnsignedCoord::new(2, 0), 1.into())));
        assert_eq!(rect.next(), Some(Pixel(UnsignedCoord::new(2, 1), 1.into())));
        assert_eq!(rect.next(), Some(Pixel(UnsignedCoord::new(0, 2), 1.into())));
        assert_eq!(rect.next(), Some(Pixel(UnsignedCoord::new(1, 2), 1.into())));
        assert_eq!(rect.next(), Some(Pixel(UnsignedCoord::new(2, 2), 1.into())));
    }
}
