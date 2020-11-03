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

/// Dynamic BMP image.
///
/// `DynamicBmp` is used to draw images that don't have a known color type at compile time,
/// for example user supplied images. If the color type is known at compile time consider using
/// [`Bmp`] for improved performance.
///
/// `DynamicBmp` works for all embedded-graphics draw targets that use a color type that implements
/// `From` for `Rgb555, `Rgb565`, `Rgb888` and `Gray8`, like every `Rgb...` and `Bgr...` type
/// included in embedded-graphics.
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
    /// Creates a bitmap object from a byte slice.
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
                    // According to the GDI docs the default 16 bpp color format is Rgb555 if no
                    // color masks are defined:
                    // https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-bitmapinfoheader
                    ColorType::Rgb555
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

    /// Returns a reference to the raw BMP image.
    ///
    /// The [`RawBmp`] instance can be used to access lower level information about the BMP file.
    ///
    /// [`RawBmp`]: struct.RawBmp.html
    pub fn as_raw(&self) -> &RawBmp<'a> {
        &self.raw_bmp
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
