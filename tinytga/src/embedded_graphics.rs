use crate::{parse_error::ParseError, ImageOrigin, TgaRaw};
use core::marker::PhantomData;
use embedded_graphics::prelude::*;

/// TGA image for use with embedded-graphics.
///
/// # Performance
///
/// `tinytga` uses different code paths to draw images with different [`ImageOrigin`]s.
/// The performance difference between the origins will depend on the display driver, but using
/// images with the origin at the top left corner will generally result in the best performance.
///
/// [`ImageOrigin`]: enum.ImageOrigin.html
#[derive(Debug)]
pub struct Tga<'a, C> {
    tga: TgaRaw<'a>,
    color_type: PhantomData<C>,
}

impl<'a, C> Tga<'a, C> {
    /// Parse a TGA image from a byte slice
    pub fn from_slice(data: &'a [u8]) -> Result<Self, ParseError> {
        Ok(Self {
            tga: TgaRaw::from_slice(data)?,
            color_type: PhantomData,
        })
    }
}

impl<C> ImageDrawable for Tga<'_, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    type Color = C;

    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        // TGA files with the origin in the top left corner can be drawn using `fill_contiguous`.
        // All other origins are drawn by falling back to `draw_iter`.
        if self.tga.header.image_origin == ImageOrigin::TopLeft {
            target.fill_contiguous(
                &self.bounding_box(),
                self.tga
                    .into_iter()
                    .map(|p| C::Raw::from_u32(p.color).into()),
            )
        } else {
            target.draw_iter(self.tga.into_iter().map(|p| {
                Pixel(
                    Point::new(i32::from(p.x), i32::from(p.y)),
                    C::Raw::from_u32(p.color).into(),
                )
            }))
        }
    }
}

impl<C> OriginDimensions for Tga<'_, C> {
    fn size(&self) -> Size {
        Size::new(self.tga.width().into(), self.tga.height().into())
    }
}
