use crate::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point},
    iterator::PixelIteratorExt,
    primitives::Rectangle,
    Pixel,
};

/// Translated draw target.
///
/// Created by calling [`translated`] on any [`DrawTarget`].
/// See the [`translated`] method documentation for more.
///
/// [`DrawTarget`]: trait.DrawTarget.html
/// [`translated`]: trait.DrawTargetExt.html#tymethod.translated
#[derive(Debug)]
pub struct Translated<'a, T>
where
    T: DrawTarget,
{
    parent: &'a mut T,
    offset: Point,
}

impl<'a, T> Translated<'a, T>
where
    T: DrawTarget,
{
    pub(super) fn new(parent: &'a mut T, offset: Point) -> Self {
        Self { parent, offset }
    }
}

impl<T> DrawTarget for Translated<'_, T>
where
    T: DrawTarget,
{
    type Color = T::Color;
    type Error = T::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.parent
            .draw_iter(pixels.into_iter().translate(self.offset))
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        let area = area.translated(self.offset);
        self.parent.fill_contiguous(&area, colors)
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        let area = area.translated(self.offset);
        self.parent.fill_solid(&area, color)
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.parent.clear(color)
    }
}

impl<T> Dimensions for Translated<'_, T>
where
    T: DrawTarget,
{
    fn bounding_box(&self) -> Rectangle {
        self.parent.bounding_box().translated(-self.offset)
    }
}
