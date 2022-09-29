//! Contiguous iterator.

use crate::{
    geometry::{Point, Size},
    pixelcolor::PixelColor,
    primitives::{rectangle, PointsIter, Rectangle},
    Pixel,
};
use core::iter::Zip;

/// Converts a contiguous iterator into a pixel iterator.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct IntoPixels<I>
where
    I: Iterator,
    I::Item: PixelColor,
{
    iter: Zip<rectangle::Points, I>,
}

impl<I> IntoPixels<I>
where
    I: Iterator,
    I::Item: PixelColor,
{
    pub(super) fn new(iter: I, bounding_box: Rectangle) -> Self {
        Self {
            iter: bounding_box.points().zip(iter),
        }
    }
}

impl<I> Iterator for IntoPixels<I>
where
    I: Iterator,
    I::Item: PixelColor,
{
    type Item = Pixel<I::Item>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(p, c)| Pixel(p, c))
    }
}

/// Crops a part of the underlying iterator.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub(crate) struct Cropped<I>
where
    I: Iterator,
    I::Item: PixelColor,
{
    iter: I,

    x: u32,
    y: u32,
    size: Size,
    row_skip: usize,
}

impl<I> Cropped<I>
where
    I: Iterator,
    I::Item: PixelColor,
{
    pub(crate) fn new(mut iter: I, size: Size, crop_area: &Rectangle) -> Self {
        let crop_area = Rectangle::new(Point::zero(), size).intersection(crop_area);

        let initial_skip =
            crop_area.top_left.y as usize * size.width as usize + crop_area.top_left.x as usize;

        if initial_skip > 0 {
            iter.nth(initial_skip - 1);
        }

        Self {
            iter,
            x: 0,
            y: 0,
            size: crop_area.size,
            row_skip: (size.width - crop_area.size.width) as usize,
        }
    }
}

impl<I> Iterator for Cropped<I>
where
    I: Iterator,
    I::Item: PixelColor,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.size.height || self.size.width == 0 {
            return None;
        }

        if self.x < self.size.width {
            self.x += 1;

            self.iter.next()
        } else {
            self.x = 1;
            self.y += 1;

            if self.y < self.size.height {
                self.iter.nth(self.row_skip)
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelcolor::Gray8;

    #[test]
    fn cropped() {
        let parent = (0..=255).map(Gray8::new);
        let parent_size = Size::new(16, 16);

        let crop_area = Rectangle::new(Point::new(2, 3), Size::new(4, 3));

        let mut cropped_iter = Cropped::new(parent, parent_size, &crop_area);

        let expected = &[
            50, 51, 52, 53, //
            66, 67, 68, 69, //
            82, 83, 84, 85, //
        ];

        for value in expected {
            assert_eq!(cropped_iter.next(), Some(Gray8::new(*value)));
        }
        assert_eq!(cropped_iter.next(), None);
    }

    #[test]
    fn cropped_empty() {
        let parent = (0..=255).map(Gray8::new);
        let parent_size = Size::new(16, 16);

        let crop_area = Rectangle::zero();

        let mut cropped_iter = Cropped::new(parent, parent_size, &crop_area);

        assert_eq!(cropped_iter.next(), None);
    }

    #[test]
    fn cropped_overlapping() {
        let parent = (0..=255).map(Gray8::new);
        let parent_size = Size::new(16, 16);

        let crop_area = Rectangle::new(Point::new(14, 10), Size::new(4, 4));

        let mut cropped_iter = Cropped::new(parent, parent_size, &crop_area);

        let expected = &[
            174, 175, //
            190, 191, //
            206, 207, //
            222, 223, //
        ];

        for value in expected {
            assert_eq!(cropped_iter.next(), Some(Gray8::new(*value)));
        }
        assert_eq!(cropped_iter.next(), None);
    }
}
