//! The rectangle primitive. Also good for drawing squares.

use super::super::drawable::*;
use super::super::transform::*;

// TODO: Impl Default so people can leave the color bit out
/// Rectangle primitive
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy)]
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

        let coord = Coord::new(self.x, self.y);

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
    /// a new `Rect`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Rect;
    /// # use embedded_graphics::transform::Transform;
    /// # use embedded_graphics::drawable::Coord;
    ///
    /// let rect = Rect::new(Coord::new(5, 10), Coord::new(15, 20), 1);
    /// let moved = rect.translate(Coord::new(10, 10));
    ///
    /// assert_eq!(moved.top_left, Coord::new(15, 20));
    /// assert_eq!(moved.bottom_right, Coord::new(25, 30));
    /// ```
    fn translate(&self, by: Coord) -> Self {
        Self {
            top_left: Coord::new(self.top_left.0 + by.0, self.top_left.1 + by.1),
            bottom_right: Coord::new(self.bottom_right.0 + by.0, self.bottom_right.1 + by.1),
            ..*self
        }
    }

    /// Translate the rect from its current position to a new position by (x, y) pixels.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Rect;
    /// # use embedded_graphics::transform::Transform;
    /// # use embedded_graphics::drawable::Coord;
    ///
    /// let mut rect = Rect::new(Coord::new(5, 10), Coord::new(15, 20), 1);
    /// rect.translate_mut(Coord::new(10, 10));
    ///
    /// assert_eq!(rect.top_left, Coord::new(15, 20));
    /// assert_eq!(rect.bottom_right, Coord::new(25, 30));
    /// ```
    fn translate_mut(&mut self, by: Coord) -> &mut Self {
        self.top_left = Coord::new(self.top_left.0 + by.0, self.top_left.1 + by.1);
        self.bottom_right = Coord::new(self.bottom_right.0 + by.0, self.bottom_right.1 + by.1);

        self
    }
}
