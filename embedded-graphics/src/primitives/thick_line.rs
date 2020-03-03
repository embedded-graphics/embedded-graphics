//! TODO: Docs

use crate::drawable::Pixel;
use crate::geometry::Point;
use crate::pixelcolor::PixelColor;
use crate::style::PrimitiveStyle;

/// TODO: Docs
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ThickLine<C: PixelColor> {
    start: Point,
    end: Point,
    style: PrimitiveStyle<C>,
}

impl<C> ThickLine<C>
where
    C: PixelColor,
{
    /// TODO: Docs
    pub fn new(start: Point, end: Point, style: PrimitiveStyle<C>) -> Self {
        Self { start, end, style }
    }
}

impl<C> IntoIterator for ThickLine<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = ThickLineIterator<C>;

    fn into_iter(self) -> Self::IntoIter {
        ThickLineIterator::new(&self, self.style)
    }
}

/// TODO: Docs
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ThickLineIterator<C: PixelColor> {
    error: i32,
    x: i32,
    y: i32,
    threshold: i32,
    e_diag: i32,
    e_square: i32,
    length: i32,
    style: PrimitiveStyle<C>,
    line: ThickLine<C>,
}

impl<C> ThickLineIterator<C>
where
    C: PixelColor,
{
    /// TODO: Docs
    pub fn new(line: &ThickLine<C>, style: PrimitiveStyle<C>) -> Self {
        let dx = line.end.x - line.start.x;
        let dy = line.end.y - line.start.y;

        Self {
            error: 0,
            x: line.start.x,
            y: line.start.y,
            line: line.clone(),
            threshold: dx - 2 * dy,
            e_diag: -2 * dx,
            e_square: 2 * dy,
            length: dx,
            style,
        }
    }
}

impl<C> Iterator for ThickLineIterator<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    // Octant 1 only
    fn next(&mut self) -> Option<Self::Item> {
        if self.x > self.line.end.x {
            None
        } else {
            let point = Point::new(self.x, self.y);

            if self.error > self.threshold {
                self.y += 1;

                self.error += self.e_diag
            }

            self.error += self.e_square;

            self.x += 1;

            Some(Pixel(point, self.style.stroke_color.unwrap()))
        }
    }
}
