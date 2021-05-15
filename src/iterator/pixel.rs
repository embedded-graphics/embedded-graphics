//! Pixel iterator.

use crate::{geometry::Point, pixelcolor::PixelColor, Pixel};

/// Translated pixel iterator.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct Translated<I> {
    iter: I,
    offset: Point,
}

impl<I, C> Translated<I>
where
    I: Iterator<Item = Pixel<C>>,
    C: PixelColor,
{
    pub(super) const fn new(iter: I, offset: Point) -> Self {
        Self { iter, offset }
    }
}

impl<I, C> Iterator for Translated<I>
where
    I: Iterator<Item = Pixel<C>>,
    C: PixelColor,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|Pixel(p, c)| Pixel(p + self.offset, c))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{iterator::PixelIteratorExt, pixelcolor::BinaryColor};

    #[test]
    fn translate() {
        let pixels = [
            Pixel(Point::new(1, 2), BinaryColor::On),
            Pixel(Point::new(3, 4), BinaryColor::On),
            Pixel(Point::new(5, 6), BinaryColor::On),
        ];
        let pixels = pixels.iter().copied();

        let expected = [
            Pixel(Point::new(1 + 4, 2 + 5), BinaryColor::On),
            Pixel(Point::new(3 + 4, 4 + 5), BinaryColor::On),
            Pixel(Point::new(5 + 4, 6 + 5), BinaryColor::On),
        ];
        let expected = expected.iter().copied();

        assert!(pixels.translated(Point::new(4, 5)).eq(expected));
    }
}
