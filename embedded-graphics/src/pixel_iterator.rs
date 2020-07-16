//! Pixel iterator

use crate::drawable::Pixel;
use crate::geometry::Dimensions;
use crate::{pixelcolor::PixelColor, DrawTarget};

/// Extension trait for pixel iterators.
pub trait PixelIteratorExt<C>
where
    C: PixelColor,
{
    /// Draws the pixel iterator to a draw target.
    fn draw<D>(self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>;
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
}

// /// Pixel iterator trait
// pub trait Pixels<'a, C>
// where
//     C: PixelColor,
// {
//     /// TODO: Doc
//     type Iter: Iterator<Item = Pixel<C>> + 'a;

//     /// TODO: Doc
//     fn pixels(&'a self) -> Self::Iter;
// }

// /// Sparse pixel iterator
// pub trait SparsePixels<'a, C>
// where
//     C: PixelColor,
// {
//     ///  TODO: Doc
//     type Iter: Iterator<Item = Option<C>> + Dimensions + 'a;

//     /// TODO: Doc
//     fn sparse_pixels(&'a self) -> Self::Iter;
// }

///  TODO: Doc
pub trait Pixels<C>
where
    C: PixelColor,
{
    ///  TODO: Doc
    type Iter: Iterator<Item = Pixel<C>>;

    ///  TODO: Doc
    fn pixels(self) -> Self::Iter;
}

///  TODO: Doc
pub trait IntoPixels<C>
where
    C: PixelColor,
{
    ///  TODO: Doc
    type Iter: Iterator<Item = Pixel<C>>;

    ///  TODO: Doc
    fn into_pixels(self) -> Self::Iter;
}

impl<'a, C, T> Pixels<C> for &'a T
where
    &'a T: IntoPixels<C>,
    C: PixelColor,
{
    type Iter = <Self as IntoPixels<C>>::Iter;

    fn pixels(self) -> Self::Iter {
        self.into_pixels()
    }
}
