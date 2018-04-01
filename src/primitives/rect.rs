//! The rectangle primitive. Also good for drawing squares.

use super::super::drawable::*;
use super::super::transform::*;

// TODO: Impl Default so people can leave the color bit out
/// Rectangle primitive
#[derive(Debug, Copy, Clone)]
pub struct Rect {
    /// Top left point of the rect
    pub top_left: Coord,

    /// Bottom right point of the rect
    pub bottom_right: Coord,

    /// Border color
    pub color: Color,
}

impl Rect {
    /// Create a new rectangle from the top left point to the bottom right point with a given border
    /// color
    pub fn new(top_left: Coord, bottom_right: Coord, color: u8) -> Self {
        Rect {
            top_left,
            bottom_right,
            color,
        }
    }
}

impl<'a> IntoIterator for &'a Rect {
    type Item = Pixel;
    type IntoIter = RectIterator;

    fn into_iter(self) -> Self::IntoIter {
        RectIterator {
            top_left: self.top_left,
            bottom_right: self.bottom_right,
            color: self.color,
            x: self.top_left.0,
            y: self.top_left.1,
        }
    }
}

/// Pixel iterator for each pixel in the rect border
#[derive(Debug, Copy, Clone)]
pub struct RectIterator {
    top_left: Coord,
    bottom_right: Coord,
    color: Color,
    x: u32,
    y: u32,
}

impl Iterator for RectIterator {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y > self.bottom_right.1 {
            return None;
        }

        let coord = (self.x, self.y);

        // Step across 1 if rendering top/bottom lines
        if self.y == self.top_left.1 || self.y == self.bottom_right.1 {
            self.x += 1;
        }
        // Skip across rect empty space if rendering left/right lines
        else {
            self.x += self.bottom_right.0 - self.top_left.0;
        }

        // Reached end of row? Jump down one line
        if self.x > self.bottom_right.0 {
            self.x = self.top_left.0;
            self.y += 1;
        }

        Some((coord, self.color))
    }
}

impl Drawable for Rect {}

impl Transform for Rect {
    /// Translate the rect from its current position to a new position by (x, y) pixels, returning
    /// a new `Rect`.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Rect;
    /// # use embedded_graphics::transform::Transform;
    ///
    /// let rect = Rect::new((5, 10), (15, 20), 1);
    /// let moved = rect.translate((10, 10));
    ///
    /// assert_eq!(moved.top_left, (15, 20));
    /// assert_eq!(moved.bottom_right, (25, 30));
    /// ```
    fn translate(&self, by: Coord) -> Self {
        Self {
            top_left: (self.top_left.0 + by.0, self.top_left.1 + by.1),
            bottom_right: (self.bottom_right.0 + by.0, self.bottom_right.1 + by.1),
            ..*self
        }
    }
}
