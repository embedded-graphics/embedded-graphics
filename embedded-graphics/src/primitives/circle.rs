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

    fn with_stroke_width(mut self, width: u8) -> Self {
        self.style.stroke_width = width;

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
            x: -(self.radius as i32),
            y: -(self.radius as i32),
            i: 0,
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
    x: i32,
    y: i32,
    i: u32,
}

impl<C> Iterator for CircleIterator<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    // http://www.sunshine2k.de/coding/java/Bresenham/RasterisingLinesCircles.pdf listing 5
    // https://stackoverflow.com/questions/1201200/fast-algorithm-for-drawing-filled-circles
    fn next(&mut self) -> Option<Self::Item> {
        // If border colour is `None`, treat it as transparent and exit early
        if self.style.stroke_color.is_none() {
            return None;
        }

        let item = loop {
            let cx = self.center[0];
            let cy = self.center[1];

            // Subtract 1 (the border width) from the radius
            let radius = self.radius as i32 - self.style.stroke_width as i32;
            let outer_radius = self.radius as i32;

            let r2 = radius * radius;
            let outer_r2 = outer_radius * outer_radius;
            let outer_diameter = self.radius * 2;
            let area = (self.radius * 2) * (self.radius * 2);

            let tx = (self.i / outer_diameter) as i32 - radius;
            let ty = (self.i % outer_diameter) as i32 - radius;
            let tx_sq = tx * tx;
            let ty_sq = ty * ty;

            let is_border = tx_sq + ty_sq > r2 - radius && tx_sq + ty_sq < outer_r2 + radius;

            let is_fill = tx_sq + ty_sq < r2 + radius;

            let item = if is_border && self.style.stroke_color.is_some() {
                Some((
                    cx + tx,
                    cy + ty,
                    self.style.stroke_color.expect("Border color not defined"),
                ))
            } else if is_fill && self.style.fill_color.is_some() {
                Some((
                    cx + tx,
                    cy + ty,
                    self.style.fill_color.expect("Fill color not defined"),
                ))
            } else {
                None
            };

            self.i += 1;

            if self.i > area {
                break None;
            }

            if let Some(i) = item {
                if i.0 >= 0 && i.1 >= 0 {
                    break item;
                }
            }
        };

        item.map(|(x, y, c)| Pixel(Coord::new(x, y).to_unsigned(), c))
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
