use crate::{
    draw_target::DrawTarget, drawable::Pixel, geometry::Dimensions, iterator::contiguous::Crop,
    rectangle::Rectangle,
};

/// Clipped draw target.
///
/// Created by calling [`clipped`] on any [`DrawTarget`].
/// See the [`clipped`] method documentation for more.
///
/// [`DrawTarget`]: trait.DrawTarget.html
/// [`clipped`]: trait.DrawTargetExt.html#tymethod.clipped
#[derive(Debug)]
pub struct Clipped<'a, T>
where
    T: DrawTarget,
{
    parent: &'a mut T,
    clip_area: Rectangle,
}

impl<'a, T> Clipped<'a, T>
where
    T: DrawTarget,
{
    pub(super) fn new(parent: &'a mut T, clip_area: &Rectangle) -> Self {
        let clip_area = clip_area.intersection(&parent.bounding_box());

        Self { parent, clip_area }
    }
}

impl<T> DrawTarget for Clipped<'_, T>
where
    T: DrawTarget,
{
    type Color = T::Color;
    type Error = T::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let clip_area = self.clip_area;

        let pixels = pixels
            .into_iter()
            .filter(|Pixel(p, _)| clip_area.contains(*p));

        self.parent.draw_iter(pixels)
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        let intersection = self.bounding_box().intersection(area);

        if &intersection == area {
            self.parent.fill_contiguous(area, colors)
        } else {
            let crop_area = intersection.translated(-area.top_left);
            let cropped = Crop::new(colors.into_iter(), area.size, &crop_area);
            self.parent.fill_contiguous(&intersection, cropped)
        }
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        let area = area.intersection(&self.clip_area);

        self.parent.fill_solid(&area, color)
    }
}

impl<T> Dimensions for Clipped<'_, T>
where
    T: DrawTarget,
{
    fn bounding_box(&self) -> Rectangle {
        self.clip_area
    }
}
