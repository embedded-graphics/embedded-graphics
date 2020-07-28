use crate::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point},
    pixel_iterator::PixelIteratorExt,
    primitives::Rectangle,
    transform::Transform,
    Pixel,
};

/// Cropped draw target.
#[derive(Debug)]
pub struct CroppedDrawTarget<'a, T>
where
    T: DrawTarget,
{
    target: &'a mut T,
    area: Rectangle,
}

impl<'a, T> CroppedDrawTarget<'a, T>
where
    T: DrawTarget,
{
    pub(super) fn new(target: &'a mut T, area: Rectangle) -> Self {
        Self { target, area }
    }
}

impl<T> DrawTarget for CroppedDrawTarget<'_, T>
where
    T: DrawTarget,
{
    type Color = T::Color;
    type Error = T::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.target
            .draw_iter(pixels.into_iter().translate(self.area.top_left))
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        let area = area.translate(self.area.top_left);
        self.target.fill_contiguous(&area, colors)
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        let area = area.translate(self.area.top_left);
        self.target.fill_solid(&area, color)
    }

    // TODO: how should clear be handled?
    // fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
    //     self.target.clear(color)
    // }
}

impl<T> Dimensions for CroppedDrawTarget<'_, T>
where
    T: DrawTarget,
{
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(Point::zero(), self.area.size)
    }
}
