use crate::{
    draw_target::{DrawTarget, DrawTargetExt, Translated},
    drawable::Pixel,
    geometry::{Dimensions, Point, Size},
    rectangle::Rectangle,
};

/// Cropped draw target.
///
/// Created by calling [`cropped`] on any [`DrawTarget`].
/// See the [`cropped`] method documentation for more.
///
/// [`DrawTarget`]: trait.DrawTarget.html
/// [`cropped`]: trait.DrawTargetExt.html#tymethod.cropped
#[derive(Debug)]
pub struct Cropped<'a, T>
where
    T: DrawTarget,
{
    parent: Translated<'a, T>,
    size: Size,
}

impl<'a, T> Cropped<'a, T>
where
    T: DrawTarget,
{
    pub(super) fn new(parent: &'a mut T, area: &Rectangle) -> Self {
        let area = area.intersection(&parent.bounding_box());

        Self {
            parent: parent.translated(area.top_left),
            size: area.size,
        }
    }
}

impl<T> DrawTarget for Cropped<'_, T>
where
    T: DrawTarget,
{
    type Color = T::Color;
    type Error = T::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.parent.draw_iter(pixels)
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        self.parent.fill_contiguous(area, colors)
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        self.parent.fill_solid(area, color)
    }
}

impl<T> Dimensions for Cropped<'_, T>
where
    T: DrawTarget,
{
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(Point::zero(), self.size)
    }
}
