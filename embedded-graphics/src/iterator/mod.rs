//! Iterators.

use crate::{
    draw_target::DrawTarget, geometry::Point, pixelcolor::PixelColor, primitives::Rectangle, Pixel,
};

pub mod contiguous;
pub mod pixel;

/// Produce an iterator over all pixels in an object.
///
/// This trait is implemented for _references_ to all styled items in embedded-graphics, therefore
/// does not consume the original item.
pub trait IntoPixels {
    /// The type of color for each pixel produced by the iterator returned from [`into_pixels`].
    ///
    /// [`into_pixels`]: #tymethod.into_pixels
    type Color: PixelColor;

    /// The iterator produced when calling [`into_pixels`].
    ///
    /// [`into_pixels`]: #tymethod.into_pixels
    type Iter: Iterator<Item = Pixel<Self::Color>>;

    /// Create an iterator over all pixels in the object.
    ///
    /// The iterator may return pixels in any order, however it may be beneficial for performance
    /// reasons to return them starting at the top left corner in row-first order.
    fn into_pixels(self) -> Self::Iter;
}

/// Extension trait for contiguous iterators.
pub trait ContiguousIteratorExt
where
    Self: Iterator + Sized,
    <Self as Iterator>::Item: PixelColor,
{
    /// Converts a contiguous iterator into a pixel iterator.
    fn into_pixels(self, bounding_box: &Rectangle) -> contiguous::IntoPixels<Self>;
}

impl<I> ContiguousIteratorExt for I
where
    I: Iterator,
    I::Item: PixelColor,
{
    fn into_pixels(self, bounding_box: &Rectangle) -> contiguous::IntoPixels<Self> {
        contiguous::IntoPixels::new(self, bounding_box.clone())
    }
}

/// Extension trait for pixel iterators.
pub trait PixelIteratorExt<C>
where
    Self: Sized,
    C: PixelColor,
{
    /// Draws the pixel iterator to a draw target.
    fn draw<D>(self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>;

    /// Returns a translated version of the iterator.
    fn translate(self, offset: Point) -> pixel::Translate<Self>;
}

impl<I, C> PixelIteratorExt<C> for I
where
    C: PixelColor,
    I: Iterator<Item = Pixel<C>>,
{
    fn draw<D>(self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        target.draw_iter(self)
    }

    fn translate(self, offset: Point) -> pixel::Translate<Self> {
        pixel::Translate::new(self, offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{mock_display::MockDisplay, pixelcolor::BinaryColor};

    #[test]
    fn draw_pixel_iterator() {
        let pixels = [
            Pixel(Point::new(0, 0), BinaryColor::On),
            Pixel(Point::new(1, 0), BinaryColor::Off),
            Pixel(Point::new(2, 0), BinaryColor::On),
            Pixel(Point::new(2, 1), BinaryColor::Off),
        ];

        let mut display = MockDisplay::new();
        pixels.iter().copied().draw(&mut display).unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "#.#", //
                "  .", //
            ])
        );
    }
}
