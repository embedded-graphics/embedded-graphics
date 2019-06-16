use crate::drawable::{Dimensions, Pixel};
use crate::prelude::*;
use crate::{Drawing, SizedDrawing};

/// Mock display for use in tests and some doc examples. Do not use directly!
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct MockDisplay<P>(pub [[P; 24]; 16]);

impl MockDisplay<u8> {
    pub fn new(bytes: [[u8; 24]; 16]) -> Self {
        Self(bytes)
    }
}

impl MockDisplay<u16> {
    pub fn new(bytes: [[u16; 24]; 16]) -> Self {
        Self(bytes)
    }
}

impl MockDisplay<u32> {
    pub fn new(bytes: [[u32; 24]; 16]) -> Self {
        Self(bytes)
    }
}

impl Default for MockDisplay<u8> {
    fn default() -> Self {
        MockDisplay::<u8>::new([[0; 24]; 16])
    }
}

impl Default for MockDisplay<u16> {
    fn default() -> Self {
        MockDisplay::<u16>::new([[0u16; 24]; 16])
    }
}

impl Default for MockDisplay<u32> {
    fn default() -> Self {
        MockDisplay::<u32>::new([[0u32; 24]; 16])
    }
}

impl<P> Drawing<P> for MockDisplay<P>
where
    P: PixelColor,
{
    fn draw<T>(&mut self, item_pixels: T)
    where
        T: IntoIterator<Item = Pixel<P>>,
    {
        for Pixel(coord, color) in item_pixels {
            if coord[0] >= 24 || coord[1] >= 16 {
                continue;
            }
            self.0[coord[1] as usize][coord[0] as usize] = color;
        }
    }
}

impl<P> SizedDrawing<P> for MockDisplay<P>
where
    P: PixelColor,
{
    fn draw_sized<T>(&mut self, item: T)
    where
        T: IntoIterator<Item = Pixel<P>> + Dimensions,
    {
        // Use `top_left()`, `size()`, etc methods defined on Dimensions to set draw area here

        let offs = item.top_left().to_unsigned();

        for Pixel(coord, color) in item {
            // Undo any translations applied to this object
            let coord_untransformed = coord - offs;

            self.0[coord_untransformed[1] as usize][coord_untransformed[0] as usize] = color;
        }
    }
}

pub type Display = MockDisplay<u8>;
pub type Display16Bpp = MockDisplay<u16>;
pub type Display32Bpp = MockDisplay<u32>;
