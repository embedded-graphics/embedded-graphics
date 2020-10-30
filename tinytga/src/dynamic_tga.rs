use core::marker::PhantomData;
use embedded_graphics::{
    pixelcolor::{Gray8, Rgb555, Rgb888},
    prelude::*,
};

use crate::{parse_error::ParseError, raw_tga::RawTga, Bpp};

/// Dynamic TGA image.
///
/// `DynamicTga` can be used to draw images that don't have a known color type
/// at compile time, for example user supplied images. If the color type is
/// known at compile time consider using the [`Tga`] for improved performance.
///
/// `DynamicTga` works with all draw targets that use a color type that implements
/// `From` for `Gray8`, `Rgb555` and `Rgb888`.
///
/// [`Tga`]: struct.Tga.html
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct DynamicTga<'a, C> {
    raw: RawTga<'a>,
    color_type: ColorType,
    target_color_type: PhantomData<C>,
}

impl<'a, C> DynamicTga<'a, C>
where
    C: PixelColor + From<Gray8> + From<Rgb555> + From<Rgb888>,
{
    /// Parses a TGA image from a byte slice.
    pub fn from_slice(data: &'a [u8]) -> Result<Self, ParseError> {
        let raw = RawTga::from_slice(data)?;

        let color_type = match (raw.color_bpp(), raw.image_type().is_monochrome()) {
            (Bpp::Bits8, true) => ColorType::Gray8,
            (Bpp::Bits16, false) => ColorType::Rgb555,
            (Bpp::Bits24, false) => ColorType::Rgb888,
            _ => {
                return Err(ParseError::UnsupportedDynamicTgaType(
                    raw.image_type(),
                    raw.color_bpp(),
                ))
            }
        };

        Ok(Self {
            raw,
            color_type,
            target_color_type: PhantomData,
        })
    }

    /// Returns a reference to the raw TGA image.
    ///
    /// The [`RawTga`] object can be used to access lower level details about the TGA file.
    ///
    /// [`RawTga`]: struct.RawTga.html
    pub fn as_raw(&self) -> &RawTga<'a> {
        &self.raw
    }
}

impl<C> OriginDimensions for DynamicTga<'_, C> {
    fn size(&self) -> Size {
        self.raw.size()
    }
}

impl<C> ImageDrawable for DynamicTga<'_, C>
where
    C: PixelColor + From<Gray8> + From<Rgb555> + From<Rgb888>,
{
    type Color = C;

    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        match self.color_type {
            ColorType::Gray8 => self.raw.draw(&mut target.color_converted::<Gray8>()),
            ColorType::Rgb555 => self.raw.draw(&mut target.color_converted::<Rgb555>()),
            ColorType::Rgb888 => self.raw.draw(&mut target.color_converted::<Rgb888>()),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum ColorType {
    Gray8,
    Rgb555,
    Rgb888,
}
