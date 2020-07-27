use crate::{
    draw_target::DrawTarget,
    geometry::Size,
    primitives::{ContainsPoint, Primitive, Rectangle},
    Pixel,
};

/// Clipped draw target.
#[derive(Debug)]
pub struct ClippedDrawTarget<'a, T>
where
    T: DrawTarget,
{
    target: &'a mut T,
    clip_area: Rectangle,
}

impl<'a, T> ClippedDrawTarget<'a, T>
where
    T: DrawTarget,
{
    pub(super) fn new(target: &'a mut T, clip_area: Rectangle) -> Self {
        Self { target, clip_area }
    }
}

impl<T> DrawTarget for ClippedDrawTarget<'_, T>
where
    T: DrawTarget,
{
    type Color = T::Color;
    type Error = T::Error;

    fn size(&self) -> Size {
        self.clip_area.size
    }

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let clip_area = self.clip_area;

        let pixels = pixels
            .into_iter()
            .filter(|Pixel(p, _)| clip_area.contains(*p));

        self.target.draw_iter(pixels)
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        //TODO: this method should use `target.fill_contiguous` and not `target.draw_iter`

        let pixels = area.points().zip(colors).map(|(p, c)| Pixel(p, c));

        self.draw_iter(pixels)
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        let area = area.intersection(&self.clip_area);

        self.target.fill_solid(&area, color)
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.target.fill_solid(&self.clip_area, color)
    }
}
