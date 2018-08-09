//! The circle primitive

use super::super::drawable::*;
use super::super::transform::*;
use coord::{Coord, ToUnsigned};
use pixelcolor::PixelColor;
use style::Style;
use style::WithStyle;

// TODO: Impl Default so people can leave the color bit out
/// Circle primitive
#[derive(Debug, Copy, Clone)]
pub struct Circle<C: PixelColor> {
    /// Center point of circle
    pub center: Coord,

    /// Radius of the circle
    pub radius: u32,

    /// Style of the circle
    pub style: Style<C>,
}

impl<C> Circle<C>
where
    C: PixelColor,
{
    /// Create a new circle with center point, radius and border color
    pub fn new(center: Coord, radius: u32) -> Self {
        Circle {
            center,
            radius,
            style: Style::default(),
        }
    }
}

impl<C> WithStyle<C> for Circle<C>
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

    fn with_fill(mut self, color: Option<C>) -> Self {
        self.style.fill_color = color;

        self
    }
}

impl<'a, C> IntoIterator for &'a Circle<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = CircleIterator<C>;

    fn into_iter(self) -> Self::IntoIter {
        CircleIterator {
            center: self.center,
            radius: self.radius,
            style: self.style,

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
pub struct CircleIterator<C: PixelColor> {
    center: Coord,
    radius: u32,
    style: Style<C>,

    octant: u32,
    idx: u32,
    x: u32,
    y: u32,
    d: i32,
}

impl<C> Iterator for CircleIterator<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    // http://www.sunshine2k.de/coding/java/Bresenham/RasterisingLinesCircles.pdf listing 5
    fn next(&mut self) -> Option<Self::Item> {
        // If border colour is `None`, treat it as transparent and exit early
        if self.style.stroke_color.is_none() {
            return None;
        }

        let item = loop {
            if self.x > self.y {
                break None;
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
                0 => Some((mx + self.x as i32, my + self.y as i32)),
                1 => Some((mx + self.x as i32, my - self.y as i32)),
                2 => Some((mx - self.x as i32, my + self.y as i32)),
                3 => Some((mx - self.x as i32, my - self.y as i32)),
                4 => Some((mx + self.y as i32, my + self.x as i32)),
                5 => Some((mx + self.y as i32, my - self.x as i32)),
                6 => Some((mx - self.y as i32, my + self.x as i32)),
                7 => Some((mx - self.y as i32, my - self.x as i32)),
                _ => None,
            };

            self.octant += 1;

            if let Some(i) = item {
                if i.0 > 0 && i.1 > 0 {
                    break item;
                }
            }
        };

        item.map(|(x, y)| {
            Pixel(
                Coord::new(x, y).to_unsigned(),
                self.style.stroke_color.expect("No stroke color given"),
            )
        })
    }
}

impl<C> Drawable for Circle<C> where C: PixelColor {}

impl<C> Transform for Circle<C>
where
    C: PixelColor,
{
    /// Translate the circle center from its current position to a new position by (x, y) pixels,
    /// returning a new `Circle`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Circle;
    /// # use embedded_graphics::dev::TestPixelColor;
    /// # use embedded_graphics::prelude::*;
    /// #
    /// # let style: Style<TestPixelColor> = Style::with_stroke(TestPixelColor(1));
    /// #
    /// let circle = Circle::new(Coord::new(5, 10), 10)
    /// #    .with_style(style);
    /// let moved = circle.translate(Coord::new(10, 10));
    ///
    /// assert_eq!(moved.center, Coord::new(15, 20));
    /// ```
    fn translate(&self, by: Coord) -> Self {
        Self {
            center: self.center + by,
            ..self.clone()
        }
    }

    /// Translate the circle center from its current position to a new position by (x, y) pixels.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Circle;
    /// # use embedded_graphics::dev::TestPixelColor;
    /// # use embedded_graphics::prelude::*;
    /// #
    /// # let style: Style<TestPixelColor> = Style::with_stroke(TestPixelColor(1));
    /// #
    /// let mut circle = Circle::new(Coord::new(5, 10), 10)
    /// #    .with_style(style);
    /// circle.translate_mut(Coord::new(10, 10));
    ///
    /// assert_eq!(circle.center, Coord::new(15, 20));
    /// ```
    fn translate_mut(&mut self, by: Coord) -> &mut Self {
        self.center += by;

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dev::TestPixelColor;

    #[test]
    fn it_handles_offscreen_coords() {
        let mut circ: CircleIterator<TestPixelColor> = Circle::new(Coord::new(-10, -10), 5)
            .with_style(Style::with_stroke(1u8.into()))
            .into_iter();

        assert_eq!(circ.next(), None);
    }

    #[test]
    fn it_handles_partially_on_screen_coords() {
        let mut circ: CircleIterator<TestPixelColor> = Circle::new(Coord::new(-5, -5), 30)
            .with_style(Style::with_stroke(1u8.into()))
            .into_iter();

        assert!(circ.next().is_some());
    }
}
