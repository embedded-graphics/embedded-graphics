//! The circle primitive

use super::super::drawable::*;
use super::super::transform::*;
use coord::Coord;

// TODO: Impl Default so people can leave the color bit out
/// Circle primitive
#[derive(Debug, Copy, Clone)]
pub struct Circle {
    /// Center point of circle
    pub center: Coord,

    /// Radius of the circle
    pub radius: u32,

    /// Line colour of circle
    pub color: Color,
}

impl Circle {
    /// Create a new circle with center point, radius and border color
    pub fn new(center: Coord, radius: u32, color: u8) -> Self {
        Circle {
            center,
            radius,
            color,
        }
    }
}

impl<'a> IntoIterator for &'a Circle {
    type Item = Pixel;
    type IntoIter = CircleIterator;

    fn into_iter(self) -> Self::IntoIter {
        CircleIterator {
            center: self.center,
            radius: self.radius,
            color: self.color,

            octant: 0,
            idx: 0,
            x: 0,
            y: self.radius,
            d: 1 - self.radius as i32,
        }
    }
}

/// Pixel iterator for each pixel in the circle border
#[derive(Debug, Copy, Clone)]
pub struct CircleIterator {
    center: Coord,
    radius: u32,
    color: Color,

    octant: u32,
    idx: u32,
    x: u32,
    y: u32,
    d: i32,
}

impl Iterator for CircleIterator {
    type Item = Pixel;

    // http://www.sunshine2k.de/coding/java/Bresenham/RasterisingLinesCircles.pdf listing 5
    fn next(&mut self) -> Option<Self::Item> {
        if self.x > self.y {
            return None;
        }

        let mx = self.center[0];
        let my = self.center[1];

        if self.octant > 7 {
            self.octant = 0;

            self.x += 1;

            if self.d < 0 {
                self.d += 2 * self.x as i32 + 3;
            } else {
                self.d += 2 * (self.x as i32 - self.y as i32) + 5;
                self.y -= 1;
            }
        }

        let item = match self.octant {
            0 => Some(Coord::new(mx + self.x, my + self.y)),
            1 => Some(Coord::new(mx + self.x, my - self.y)),
            2 => Some(Coord::new(mx - self.x, my + self.y)),
            3 => Some(Coord::new(mx - self.x, my - self.y)),
            4 => Some(Coord::new(mx + self.y, my + self.x)),
            5 => Some(Coord::new(mx + self.y, my - self.x)),
            6 => Some(Coord::new(mx - self.y, my + self.x)),
            7 => Some(Coord::new(mx - self.y, my - self.x)),
            _ => None,
        };

        self.octant += 1;

        if item.is_none() {
            None
        } else {
            Some((item.unwrap(), self.color))
        }
    }
}

impl Drawable for Circle {}

impl Transform for Circle {
    /// Translate the circle center from its current position to a new position by (x, y) pixels,
    /// returning a new `Circle`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Circle;
    /// # use embedded_graphics::transform::Transform;
    /// # use embedded_graphics::coord::Coord;
    ///
    /// let circle = Circle::new(Coord::new(5, 10), 10, 1);
    /// let moved = circle.translate(Coord::new(10, 10));
    ///
    /// assert_eq!(moved.center, Coord::new(15, 20));
    /// ```
    fn translate(&self, by: Coord) -> Self {
        Self {
            center: self.center + by,
            ..*self
        }
    }

    /// Translate the circle center from its current position to a new position by (x, y) pixels.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Circle;
    /// # use embedded_graphics::transform::Transform;
    /// # use embedded_graphics::coord::Coord;
    ///
    /// let mut circle = Circle::new(Coord::new(5, 10), 10, 1);
    /// circle.translate_mut(Coord::new(10, 10));
    ///
    /// assert_eq!(circle.center, Coord::new(15, 20));
    /// ```
    fn translate_mut(&mut self, by: Coord) -> &mut Self {
        self.center += by;

        self
    }
}
