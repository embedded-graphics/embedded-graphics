use core::marker::PhantomData;
use embedded_graphics::prelude::*;

use crate::RawPixels;

/// Iterator over individual TGA pixels.
///
/// See the [`pixels`] method for additional information.
///
/// [`pixels`]: struct.Tga.html#method.pixels
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Pixels<'a, 'b, C> {
    raw: RawPixels<'a, 'b>,
    color_type: PhantomData<C>,
}

impl<'a, 'b, C> Pixels<'a, 'b, C> {
    pub(crate) fn new(raw: RawPixels<'a, 'b>) -> Self {
        Self {
            raw,
            color_type: PhantomData,
        }
    }
}

impl<C> Iterator for Pixels<'_, '_, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        self.raw
            .next()
            .map(|p| Pixel(p.position, C::Raw::from_u32(p.color).into()))
    }
}
