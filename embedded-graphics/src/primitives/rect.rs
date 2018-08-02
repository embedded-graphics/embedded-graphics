//! The rectangle primitive. Also good for drawing squares.

use super::super::drawable::*;
use super::super::transform::*;
use coord::{Coord, ToUnsigned};
use pixelcolor::PixelColor;

// TODO: Impl Default so people can leave the color bit out
/// Rectangle primitive
#[derive(Debug, Clone, Copy)]
pub struct Rect<C: PixelColor> {
    /// Top left point of the rect
    pub top_left: Coord,

    /// Bottom right point of the rect
    pub bottom_right: Coord,

    /// Border color
    pub color: C,
}

impl<C> Rect<C>
where
    C: PixelColor,
{
    /// Create a new rectangle from the top left point to the bottom right point with a given border
    /// color
    pub fn new(top_left: Coord, bottom_right: Coord, color: C) -> Self {
        Rect {
            top_left,
            bottom_right,
            color,
        }
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
            color: self.color,
            x: self.top_left[0],
            y: self.top_left[1],
            screen_size: (self.bottom_right - self.top_left).abs(),
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
    color: C,
    x: i32,
    y: i32,
    screen_size: Coord,
}

impl<C> Iterator for RectIterator<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        // Entire object is off the top left of the screen, so render nothing
        if (self.top_left[0] < 0 || self.top_left[1] < 0)
            && (self.bottom_right[0] < 0 || self.bottom_right[1] < 0)
        {
            return None;
        }

        let coord = loop {
            // If we're below the rect, it's completely rendered and we're done
            if self.y > self.bottom_right[1] {
                return None;
            }

            let coord = Coord::new(self.x, self.y);

            // Step across 1 if rendering top/bottom lines
            if self.y == self.top_left[1] || self.y == self.bottom_right[1] {
                self.x += 1;
            }
            // Skip across rect empty space if rendering left/right lines
            else {
                self.x += self.screen_size[0];
            }

            // Reached end of row? Jump down one line
            if self.x > self.bottom_right[0] {
                self.x = self.top_left[0];
                self.y += 1;
            }

            if coord[0] >= 0 && coord[1] >= 0 {
                break coord;
            }
        };

        Some(Pixel(coord.to_unsigned(), self.color))
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
    /// # use embedded_graphics::transform::Transform;
    /// # use embedded_graphics::coord::Coord;
    ///
    /// let rect = Rect::new(Coord::new(5, 10), Coord::new(15, 20), 1);
    /// let moved = rect.translate(Coord::new(10, 10));
    ///
    /// assert_eq!(moved.top_left, Coord::new(15, 20));
    /// assert_eq!(moved.bottom_right, Coord::new(25, 30));
    /// ```
    fn translate(self, by: Coord) -> Self {
        Self {
            top_left: self.top_left + by,
            bottom_right: self.bottom_right + by,
            ..self
        }
    }

    /// Translate the rect from its current position to a new position by (x, y) pixels.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Rect;
    /// # use embedded_graphics::transform::Transform;
    /// # use embedded_graphics::coord::Coord;
    ///
    /// let mut rect = Rect::new(Coord::new(5, 10), Coord::new(15, 20), 1);
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
    use unsignedcoord::UnsignedCoord;

    #[test]
    fn it_can_be_translated() {
        let rect = Rect::new(Coord::new(5, 10), Coord::new(15, 20), 1);
        let moved = rect.translate(Coord::new(10, 10));

        assert_eq!(moved.top_left, Coord::new(15, 20));
        assert_eq!(moved.bottom_right, Coord::new(25, 30));
    }

    #[test]
    fn it_draws_unfilled_rect() {
        let mut rect = Rect::new(Coord::new(2, 2), Coord::new(4, 4), 1).into_iter();

        assert_eq!(rect.next(), Some((UnsignedCoord::new(2, 2), 1)));
        assert_eq!(rect.next(), Some((UnsignedCoord::new(3, 2), 1)));
        assert_eq!(rect.next(), Some((UnsignedCoord::new(4, 2), 1)));

        assert_eq!(rect.next(), Some((UnsignedCoord::new(2, 3), 1)));
        assert_eq!(rect.next(), Some((UnsignedCoord::new(4, 3), 1)));

        assert_eq!(rect.next(), Some((UnsignedCoord::new(2, 4), 1)));
        assert_eq!(rect.next(), Some((UnsignedCoord::new(3, 4), 1)));
        assert_eq!(rect.next(), Some((UnsignedCoord::new(4, 4), 1)));
    }

    #[test]
    fn it_can_be_negative() {
        let mut rect = Rect::new(Coord::new(-2, -2), Coord::new(2, 2), 1).into_iter();

        // TODO: Macro
        // Only the bottom right corner of the rect should be visible
        assert_eq!(rect.next(), Some((UnsignedCoord::new(2, 0), 1)));
        assert_eq!(rect.next(), Some((UnsignedCoord::new(2, 1), 1)));
        assert_eq!(rect.next(), Some((UnsignedCoord::new(0, 2), 1)));
        assert_eq!(rect.next(), Some((UnsignedCoord::new(1, 2), 1)));
        assert_eq!(rect.next(), Some((UnsignedCoord::new(2, 2), 1)));
    }
}
