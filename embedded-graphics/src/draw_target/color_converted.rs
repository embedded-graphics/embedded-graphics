use crate::{
    draw_target::DrawTarget, geometry::Dimensions, pixelcolor::PixelColor, primitives::Rectangle,
    Pixel,
};
use core::marker::PhantomData;

/// Color conversion draw target.
///
/// Created by calling [`color_converted`] on any [`DrawTarget`].
/// See the [`color_converted`] method documentation for more information.
///
/// [`DrawTarget`]: trait.DrawTarget.html
/// [`color_converted`]: trait.DrawTargetExt.html#tymethod.color_converted
#[derive(Debug)]
pub struct ColorConverted<'a, T, C> {
    /// The parent draw target.
    parent: &'a mut T,

    /// The input color type.
    color_type: PhantomData<C>,
}

impl<'a, T, C> ColorConverted<'a, T, C>
where
    T: DrawTarget,
    C: PixelColor + Into<T::Color>,
{
    pub(super) fn new(parent: &'a mut T) -> Self {
        Self {
            parent,
            color_type: PhantomData,
        }
    }
}

impl<T, C> DrawTarget for ColorConverted<'_, T, C>
where
    T: DrawTarget,
    C: PixelColor + Into<T::Color>,
{
    type Color = C;
    type Error = T::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.parent
            .draw_iter(pixels.into_iter().map(|Pixel(p, c)| Pixel(p, c.into())))
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        self.parent
            .fill_contiguous(area, colors.into_iter().map(|c| c.into()))
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        self.parent.fill_solid(area, color.into())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.parent.clear(color.into())
    }
}

impl<T, C> Dimensions for ColorConverted<'_, T, C>
where
    T: DrawTarget,
{
    fn bounding_box(&self) -> Rectangle {
        self.parent.bounding_box()
    }
}
