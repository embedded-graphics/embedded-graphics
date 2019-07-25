//! The circle primitive

use super::super::drawable::*;
use super::super::transform::*;
use crate::coord::{Coord, ToUnsigned};
use crate::pixelcolor::PixelColor;
use crate::primitives::Primitive;
use crate::style::Style;
use crate::style::WithStyle;
use crate::unsignedcoord::{ToSigned, UnsignedCoord};

/// Circle primitive
///
/// # Examples
///
/// The [macro examples](../../macro.egcircle.html) make for more concise code.
///
/// ## Create some circles with different styles
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::primitives::Circle;
/// use embedded_graphics::pixelcolor::Rgb565;
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::default();
///
/// // Default circle with only a stroke centered around (10, 20) with a radius of 30
/// let c1 = Circle::new(Coord::new(10, 20), 30);
///
/// // Circle with styled stroke and fill centered around (50, 20) with a radius of 30
/// let c2 = Circle::new(Coord::new(50, 20), 30)
///     .stroke(Some(Rgb565::RED))
///     .stroke_width(3)
///     .fill(Some(Rgb565::GREEN));
///
/// // Circle with no stroke and a translation applied
/// let c3 = Circle::new(Coord::new(10, 20), 30)
///     .stroke(None)
///     .fill(Some(Rgb565::BLUE))
///     .translate(Coord::new(65, 35));
///
/// display.draw(c1);
/// display.draw(c2);
/// display.draw(c3);
/// ```
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
    /// Create a new circle centered around a given point with a specific radius
    pub fn new(center: Coord, radius: u32) -> Self {
        Circle {
            center,
            radius,
            style: Style::default(),
        }
    }
}

impl<C> Primitive for Circle<C> where C: PixelColor {}

impl<C> Dimensions for Circle<C>
where
    C: PixelColor,
{
    fn top_left(&self) -> Coord {
        let radius_coord = Coord::new(self.radius as i32, self.radius as i32);

        self.center - radius_coord
    }

    fn bottom_right(&self) -> Coord {
        self.top_left() + self.size().to_signed() - Coord::new(1,1)
    }

    fn size(&self) -> UnsignedCoord {
        UnsignedCoord::new(self.radius * 2, self.radius * 2) + UnsignedCoord::new(1,1)
    }
}

impl<C> WithStyle<C> for Circle<C>
where
    C: PixelColor,
{
    fn style(mut self, style: Style<C>) -> Self {
        self.style = style;

        self
    }

    fn stroke(mut self, color: Option<C>) -> Self {
        self.style.stroke_color = color;

        self
    }

    fn stroke_width(mut self, width: u8) -> Self {
        self.style.stroke_width = width;

        self
    }

    fn fill(mut self, color: Option<C>) -> Self {
        self.style.fill_color = color;

        self
    }
}

impl<C> IntoIterator for Circle<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = CircleIterator<C>;

    fn into_iter(self) -> Self::IntoIter {
        (&self).into_iter()
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
            x: -(self.radius as i32),
            y: -(self.radius as i32),
        }
    }
}

/// Pixel iterator for each pixel in the circle border
#[derive(Debug, Copy, Clone)]
pub struct CircleIterator<C: PixelColor> {
    center: Coord,
    radius: u32,
    style: Style<C>,
    x: i32,
    y: i32,
}

impl<C> Iterator for CircleIterator<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    // https://stackoverflow.com/questions/1201200/fast-algorithm-for-drawing-filled-circles
    fn next(&mut self) -> Option<Self::Item> {
        // If border or stroke colour is `None`, treat entire object as transparent and exit early
        if self.style.stroke_color.is_none() && self.style.fill_color.is_none() {
            return None;
        }

        let cx = self.center[0];
        let cy = self.center[1];

        let radius = self.radius as i32 - self.style.stroke_width as i32 + 1;
        let outer_radius = self.radius as i32;

        let radius_sq = radius * radius;
        let outer_radius_sq = outer_radius * outer_radius;

        let item = loop {
            let tx = self.x;
            let ty = self.y;
            let len = tx * tx + ty * ty;

            let is_border = len > radius_sq - radius && len < outer_radius_sq + radius;

            // TODO: Should this be a <= or a <?
            let is_fill = len <= outer_radius_sq;

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

            self.x += 1;

            if self.x > self.radius as i32 {
                self.x = -(self.radius as i32);
                self.y += 1;
            }

            if self.y > self.radius as i32 {
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
    /// # use embedded_graphics::prelude::*;
    /// # use embedded_graphics::pixelcolor::Rgb565;
    /// #
    /// # let style = Style::stroke(Rgb565::RED);
    /// #
    /// let circle = Circle::new(Coord::new(5, 10), 10)
    /// #    .style(style);
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
    /// # use embedded_graphics::prelude::*;
    /// # use embedded_graphics::pixelcolor::Rgb565;
    /// #
    /// # let style = Style::stroke(Rgb565::RED);
    /// #
    /// let mut circle = Circle::new(Coord::new(5, 10), 10)
    /// #    .style(style);
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
    use crate::drawable::Dimensions;
    use crate::pixelcolor::BinaryColor;

    #[test]
    fn negative_dimensions() {
        let circ: Circle<BinaryColor> = Circle::new(Coord::new(-10, -10), 5);

        assert_eq!(circ.top_left(), Coord::new(-15, -15));
        assert_eq!(circ.bottom_right(), Coord::new(-5, -5));
        assert_eq!(circ.size(), UnsignedCoord::new(11, 11));
    }

    #[test]
    fn dimensions() {
        let circ: Circle<BinaryColor> = Circle::new(Coord::new(10, 20), 5);

        assert_eq!(circ.top_left(), Coord::new(5, 15));
        assert_eq!(circ.bottom_right(), Coord::new(15, 25));
        assert_eq!(circ.size(), UnsignedCoord::new(11, 11));
    }

    #[test]
    fn large_radius() {
        let circ: Circle<BinaryColor> = Circle::new(Coord::new(5, 5), 10);

        assert_eq!(circ.top_left(), Coord::new(-5, -5));
        assert_eq!(circ.bottom_right(), Coord::new(15, 15));
        assert_eq!(circ.size(), UnsignedCoord::new(21, 21));
    }

    #[test]
    fn transparent_border() {
        let circ: Circle<BinaryColor> = Circle::new(Coord::new(5, 5), 10)
            .stroke(None)
            .fill(Some(BinaryColor::On));

        assert!(circ.into_iter().count() > 0);
    }

    #[test]
    fn it_handles_offscreen_coords() {
        let mut circ: CircleIterator<BinaryColor> = Circle::new(Coord::new(-10, -10), 5)
            .style(Style::stroke(BinaryColor::On))
            .into_iter();

        assert_eq!(circ.next(), None);
    }

    #[test]
    fn it_handles_partially_on_screen_coords() {
        let mut circ: CircleIterator<BinaryColor> = Circle::new(Coord::new(-5, -5), 30)
            .style(Style::stroke(BinaryColor::On))
            .into_iter();

        assert!(circ.next().is_some());
    }
}
