use crate::BmpRaw;
use core::marker::PhantomData;
use embedded_graphics::{
    pixelcolor::{raw::RawData, PixelColor},
    prelude::*,
};

/// A BMP-format bitmap for use with embedded-graphics
#[derive(Debug)]
pub struct Bmp<'a, C> {
    bmp: BmpRaw<'a>,
    color_type: PhantomData<C>,
}

impl<'a, C> Bmp<'a, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    /// Create a bitmap object from a byte array
    ///
    /// This method keeps a slice of the original input and does not dynamically allocate memory.
    /// The input data must live for as long as this BMP instance does.
    pub fn from_slice(data: &'a [u8]) -> Result<Self, ()> {
        Ok(Self {
            bmp: BmpRaw::from_slice(data)?,
            color_type: PhantomData,
        })
    }
}

impl<C> ImageDrawable for Bmp<'_, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    type Color = C;

    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        target.fill_contiguous(
            &self.bounding_box(),
            self.bmp
                .into_iter()
                .map(|p| C::Raw::from_u32(p.color).into()),
        )
    }
}

impl<C> OriginDimensions for Bmp<'_, C>
where
    C: PixelColor,
{
    fn size(&self) -> Size {
        Size::new(self.bmp.width(), self.bmp.height())
    }
}
