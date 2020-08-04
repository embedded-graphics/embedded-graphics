use crate::{
    draw_target::DrawTarget, geometry::OriginDimensions, image::ImageDrawable,
    pixelcolor::PixelColor,
};
use core::marker::PhantomData;

/// TODO: Docs
#[derive(Debug)]
pub struct ImageFile<T, C>
where
    C: PixelColor,
{
    image_data: T,

    color_type: PhantomData<C>,
}

impl<'a, T, C> ImageFile<T, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    T: ImageData<'a>,
{
    /// TODO: Docs
    pub fn from_slice(data: &'a [u8]) -> Result<Self, T::Error> {
        Ok(Self {
            image_data: T::from_slice(data)?,
            color_type: PhantomData,
        })
    }
}

impl<T, C> ImageDrawable<C> for ImageFile<T, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    T: ImageDrawable<C>,
{
    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        self.image_data.draw(target)
    }

    fn draw_sub_image<D>(
        &self,
        target: &mut D,
        area: &crate::primitives::Rectangle,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        self.image_data.draw_sub_image(target, area)
    }
}

impl<T, C> OriginDimensions for ImageFile<T, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    T: ImageDrawable<C>,
{
    fn size(&self) -> crate::prelude::Size {
        self.image_data.size()
    }
}

/// TODO: Docs
pub trait ImageData<'a>: Sized {
    /// TODO: Docs
    type Error;

    /// TODO: Docs
    fn from_slice(data: &'a [u8]) -> Result<Self, Self::Error>;
}
