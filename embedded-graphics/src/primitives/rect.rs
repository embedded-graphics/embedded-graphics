//! The rectangle primitive. Also good for drawing squares.

use super::super::drawable::*;
use super::super::transform::*;
use crate::coord::{Coord, ToUnsigned};
use crate::pixelcolor::PixelColor;
use crate::primitives::Primitive;
use crate::style::Style;
use crate::style::WithStyle;
use crate::unsignedcoord::UnsignedCoord;

// TODO: Impl Default so people can leave the color bit out
/// Rectangle primitive
#[derive(Debug, Clone, Copy)]
pub struct Rect<C: PixelColor> {
    /// Top left point of the rect
    pub top_left: Coord,

    /// Bottom right point of the rect
    pub bottom_right: Coord,

    /// Object style
    pub style: Style<C>,
}

impl<C> Primitive for Rect<C> where C: PixelColor {}

impl<C> Dimensions for Rect<C>
where
    C: PixelColor,
{
    fn top_left(&self) -> Coord {
        self.top_left
    }

    fn bottom_right(&self) -> Coord {
        self.bottom_right
    }

    fn size(&self) -> UnsignedCoord {
        (self.bottom_right - self.top_left).abs().to_unsigned()
    }
}

impl<C> Rect<C>
where
    C: PixelColor,
{
    /// Create a new rectangle from the top left point to the bottom right point with a given style
    pub fn new(top_left: Coord, bottom_right: Coord) -> Self {
        Rect {
            top_left,
            bottom_right,
            style: Style::default(),
        }
    }
}

impl<C> WithStyle<C> for Rect<C>
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

impl<'a, C> IntoIterator for &'a Rect<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = RectIterator<C>;

    fn into_iter(self) -> Self::IntoIter {
        RectIterator {
            top_left: self.top_left,
            bottom_right: self.bottom_right,
            style: self.style,
            x: self.top_left[0],
            y: self.top_left[1],
        }
    }
}

/// Pixel iterator for each pixel in the rect border
#[derive(Debug, Clone, Copy)]
pub struct RectIterator<C: PixelColor>
where
    C: PixelColor,
{
    top_left: Coord,
    bottom_right: Coord,
    style: Style<C>,
    x: i32,
    y: i32,
}

impl<C> Iterator for RectIterator<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        // If entire object is off the top left of the screen or has no border or fill colour,
        // don't render anything
        if (self.top_left[0] < 0 || self.top_left[1] < 0)
            && (self.bottom_right[0] < 0 || self.bottom_right[1] < 0)
            || (self.style.stroke_color.is_none() && self.style.fill_color.is_none())
        {
            return None;
        }

        let pixel = loop {
            let mut out = None;

            // Finished, i.e. we're below the rect
            if self.y > self.bottom_right[1] {
                break None;
            }

            let border_width = self.style.stroke_width as i32;
            let tl = self.top_left;
            let br = self.bottom_right;

            if self.x >= 0 && self.y >= 0 {
                // Border
                if (
                    // Top border
                    (self.y >= tl[1] && self.y < tl[1] + border_width)
                // Bottom border
                || (self.y <= br[1] && self.y > br[1] - border_width)
                // Left border
                || (self.x >= tl[0] && self.x < tl[0] + border_width)
                // Right border
                || (self.x <= br[0] && self.x > br[0] - border_width)
                ) && self.style.stroke_color.is_some()
                {
                    out = Some((
                        self.x,
                        self.y,
                        self.style.stroke_color.expect("Expected stroke"),
                    ));
                }
                // Fill
                else if let Some(fill) = self.style.fill_color {
                    out = Some((self.x, self.y, fill));
                }
            }

            self.x += 1;

            // Reached end of row? Jump down one line
            if self.x > self.bottom_right[0] {
                self.x = self.top_left[0];
                self.y += 1;
            }

            if out.is_some() {
                break out;
            }
        };

        pixel.map(|(x, y, c)| Pixel(Coord::new(x, y).to_unsigned(), c))
    }
}

impl<C> Drawable for Rect<C> where C: PixelColor {}

impl<C> Transform for Rect<C>
where
    C: PixelColor,
{
    /// Translate the rect from its current position to a new position by (x, y) pixels, returning
    /// a new `Rect`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Rect;
    /// # use embedded_graphics::dev::TestPixelColor;
    /// # use embedded_graphics::prelude::*;
    /// #
    /// # let style: Style<TestPixelColor> = Style::with_stroke(TestPixelColor(1));
    /// #
    /// let rect = Rect::new(Coord::new(5, 10), Coord::new(15, 20))
    /// #    .with_style(style);
    /// let moved = rect.translate(Coord::new(10, 10));
    ///
    /// assert_eq!(moved.top_left, Coord::new(15, 20));
    /// assert_eq!(moved.bottom_right, Coord::new(25, 30));
    /// ```
    fn translate(&self, by: Coord) -> Self {
        Self {
            top_left: self.top_left + by,
            bottom_right: self.bottom_right + by,
            ..self.clone()
        }
    }

    /// Translate the rect from its current position to a new position by (x, y) pixels.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Rect;
    /// # use embedded_graphics::dev::TestPixelColor;
    /// # use embedded_graphics::prelude::*;
    /// #
    /// # let style: Style<TestPixelColor> = Style::with_stroke(TestPixelColor(1));
    /// #
    /// let mut rect = Rect::new(Coord::new(5, 10), Coord::new(15, 20))
    /// #    .with_style(style);
    /// rect.translate_mut(Coord::new(10, 10));
    ///
    /// assert_eq!(rect.top_left, Coord::new(15, 20));
    /// assert_eq!(rect.bottom_right, Coord::new(25, 30));
    /// ```
    fn translate_mut(&mut self, by: Coord) -> &mut Self {
        self.top_left += by;
        self.bottom_right += by;

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
        let rect: Rect<TestPixelColor> = Rect::new(Coord::new(5, 10), Coord::new(15, 20));
        let moved = rect.translate(Coord::new(-10, -10));

        assert_eq!(rect.top_left(), Coord::new(5, 10));
        assert_eq!(rect.bottom_right(), Coord::new(15, 20));
        assert_eq!(rect.size(), UnsignedCoord::new(10, 10));

        assert_eq!(moved.top_left(), Coord::new(-5, 0));
        assert_eq!(moved.bottom_right(), Coord::new(5, 10));
        assert_eq!(moved.size(), UnsignedCoord::new(10, 10));
    }

    #[test]
    fn it_can_be_translated() {
        let rect: Rect<TestPixelColor> = Rect::new(Coord::new(5, 10), Coord::new(15, 20));
        let moved = rect.translate(Coord::new(10, 10));

        assert_eq!(moved.top_left, Coord::new(15, 20));
        assert_eq!(moved.bottom_right, Coord::new(25, 30));
    }

    #[test]
    fn it_draws_unfilled_rect() {
        let mut rect: RectIterator<TestPixelColor> = Rect::new(Coord::new(2, 2), Coord::new(4, 4))
            .with_style(Style::with_stroke(1u8.into()))
            .into_iter();

        assert_eq!(rect.next(), Some(Pixel(UnsignedCoord::new(2, 2), 1.into())));
        assert_eq!(rect.next(), Some(Pixel(UnsignedCoord::new(3, 2), 1.into())));
        assert_eq!(rect.next(), Some(Pixel(UnsignedCoord::new(4, 2), 1.into())));

        assert_eq!(rect.next(), Some(Pixel(UnsignedCoord::new(2, 3), 1.into())));
        assert_eq!(rect.next(), Some(Pixel(UnsignedCoord::new(4, 3), 1.into())));

        assert_eq!(rect.next(), Some(Pixel(UnsignedCoord::new(2, 4), 1.into())));
        assert_eq!(rect.next(), Some(Pixel(UnsignedCoord::new(3, 4), 1.into())));
        assert_eq!(rect.next(), Some(Pixel(UnsignedCoord::new(4, 4), 1.into())));
    }

    #[test]
    fn it_can_be_negative() {
        let mut rect: RectIterator<TestPixelColor> =
            Rect::new(Coord::new(-2, -2), Coord::new(2, 2))
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
