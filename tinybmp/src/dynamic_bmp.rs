use core::marker::PhantomData;
use embedded_graphics::{
    pixelcolor::{Gray8, PixelColor, Rgb555, Rgb565, Rgb888},
    prelude::*,
};

use crate::{
    header::{Bpp, ChannelMasks},
    raw_bmp::RawBmp,
    ParseError,
};

/// TODO: docs
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct DynamicBmp<'a, C> {
    raw_bmp: RawBmp<'a>,
    color_type: ColorType,
    target_color_type: PhantomData<C>,
}

impl<'a, C> DynamicBmp<'a, C>
where
    C: PixelColor + From<Rgb555> + From<Rgb565> + From<Rgb888> + From<Gray8>,
{
    /// TODO: docs
    pub fn from_slice(bytes: &'a [u8]) -> Result<Self, ParseError> {
        let raw_bmp = RawBmp::from_slice(bytes)?;

        let color_type = match raw_bmp.color_bpp() {
            Bpp::Bits1 => return Err(ParseError::UnsupportedDynamicBmpFormat),
            Bpp::Bits8 => ColorType::Gray8,
            Bpp::Bits16 => {
                if let Some(masks) = raw_bmp.header().channel_masks {
                    match masks {
                        ChannelMasks::RGB555 => ColorType::Rgb555,
                        ChannelMasks::RGB565 => ColorType::Rgb565,
                        _ => return Err(ParseError::UnsupportedDynamicBmpFormat),
                    }
                } else {
                    // TODO: should we assume Rgb555 or Rgb565 if no color masks are present?
                    ColorType::Rgb565
                }
            }
            Bpp::Bits24 => ColorType::Rgb888,
            Bpp::Bits32 => {
                if let Some(masks) = raw_bmp.header().channel_masks {
                    if masks == ChannelMasks::RGB888 {
                        ColorType::Rgb888
                    } else {
                        return Err(ParseError::UnsupportedDynamicBmpFormat);
                    }
                } else {
                    ColorType::Rgb888
                }
            }
        };

        Ok(Self {
            raw_bmp,
            color_type,
            target_color_type: PhantomData,
        })
    }
}

impl<C> ImageDrawable for DynamicBmp<'_, C>
where
    C: PixelColor + From<Rgb555> + From<Rgb565> + From<Rgb888> + From<Gray8>,
{
    type Color = C;

    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        match self.color_type {
            ColorType::Rgb555 => self.raw_bmp.draw(&mut target.color_converted::<Rgb555>()),
            ColorType::Rgb565 => self.raw_bmp.draw(&mut target.color_converted::<Rgb565>()),
            ColorType::Rgb888 => self.raw_bmp.draw(&mut target.color_converted::<Rgb888>()),
            ColorType::Gray8 => self.raw_bmp.draw(&mut target.color_converted::<Gray8>()),
        }
    }
}

impl<C> OriginDimensions for DynamicBmp<'_, C>
where
    C: PixelColor,
{
    fn size(&self) -> Size {
        self.raw_bmp.size()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum ColorType {
    Rgb555,
    Rgb565,
    Rgb888,
    Gray8,
}
