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

// TODO: Implement as part of a new PR for sparse pixel iterators
// ///  TODO: Doc
// pub trait IntoSparsePixels<C>
// where
//     C: PixelColor,
// {
//     ///  TODO: Doc
//     type Iter: Iterator<Item = Option<C>> + Dimensions;

//     ///  TODO: Doc
//     fn into_sparse_pixels(self) -> Self::Iter;
// }
