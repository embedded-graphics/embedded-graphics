//! `Drawable` trait and helpers
use crate::geometry::Point;
use crate::pixelcolor::PixelColor;
use crate::DrawTarget;

/// A single pixel
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Pixel<C: PixelColor>(pub Point, pub C);

/// Marks an object as "drawable". Must be implemented for all graphics objects
pub trait Drawable<'a, C>
where
    C: PixelColor + 'a,
    Self: 'a,
    &'a mut Self: IntoIterator<Item = Pixel<C>>,
{
    /// Draw the graphics object. Override this method with primitive drawing methods as
    /// applicable.
    fn draw<T: DrawTarget<C>>(&'a mut self, display: &mut T) {
        display.draw_iter(self);
    }
}

impl<'a, C, T> Drawable<'a, C> for T
where
    C: PixelColor + 'a,
    T: 'a + Iterator<Item = Pixel<C>>,
{
}
