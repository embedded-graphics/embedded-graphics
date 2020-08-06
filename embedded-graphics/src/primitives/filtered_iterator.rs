use crate::{
    pixelcolor::PixelColor,
    primitives::{
        rectangle::{self, Rectangle},
        Primitive,
    },
    Pixel,
};
use core::iter::Zip;

/// TODO: Doc
#[derive(Debug, Clone)]
pub struct FilteredIterator<C, I>
where
    C: PixelColor,
    I: Iterator<Item = Option<C>>,
{
    iter: Zip<rectangle::Points, I>,
}

impl<C, I> FilteredIterator<C, I>
where
    C: PixelColor,
    I: Iterator<Item = Option<C>>,
{
    /// TODO: Doc
    pub fn new(sparse_iter: I, bounding_box: Rectangle) -> Self {
        let iter = bounding_box.points().zip(sparse_iter);

        Self { iter }
    }
}

impl<C, I> Iterator for FilteredIterator<C, I>
where
    C: PixelColor,
    I: Iterator<Item = Option<C>>,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .find_map(|(point, color)| color.map(|c| Pixel(point, c)))
    }
}
