//! `Drawable` trait and helpers
use crate::geometry::Point;
use crate::pixelcolor::PixelColor;
use crate::DrawTarget;

/// A single pixel
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Pixel<C: PixelColor>(pub Point, pub C);

/// Marks an object as "drawable". Must be implemented for all graphics objects
pub trait Drawable<C>
where
    C: PixelColor,
    for<'a> &'a mut Self: IntoIterator<Item = Pixel<C>>,
{
    /// Draw the graphics object. Override this method with primitive drawing methods as
    /// applicable.
    fn draw<T: DrawTarget<C>>(&mut self, display: &mut T) {
        display.draw_iter(self);
    }
}

impl<C, T> Drawable<C> for T
where
    C: PixelColor,
    T: Iterator<Item=Pixel<C>>,
    // for<'a> &'a T: Iterator<Item=Pixel<C>>,
{
}

// /// Draw container
// #[derive(Debug)]
// pub struct DrawContainer<C, T>
// where
//     C: PixelColor,
//     T: Iterator<Item=Pixel<C>>
// {
//     inner: T,
// }

// impl<C, T> DrawContainer<C, T>
// where
//     C: PixelColor,
//     T: Iterator<Item=Pixel<C>>
// {
//     fn new(inner: T) -> Self {
//         DrawContainer { inner }
//     }
// }

// impl<C, T> IntoIterator for &DrawContainer<C, T>
// where
//     C: PixelColor,
//     T: Iterator<Item=Pixel<C>>
// {
//     type Item = Pixel<C>;
//     type IntoIterator = Iterator<Item=Pixel<C>>;

//     fn into_iter(self) -> Iterator<Item=Self::Item> {

//     }
// }

// impl<C, T> Drawable<C> for DrawContainer<C, T>
// where
//     C: PixelColor,
//     T: Iterator<Item=Pixel<C>>
// {
// }
