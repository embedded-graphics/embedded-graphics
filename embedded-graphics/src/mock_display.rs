use crate::drawable::{Dimensions, Pixel};
use crate::prelude::*;
use crate::{Drawing, SizedDrawing};

/// Mock display for use in tests and some doc examples. Do not use directly!
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct MockDisplay<C>(pub [[C; 24]; 16]);

impl<C> MockDisplay<C> {
    pub fn new(bytes: [[C; 24]; 16]) -> Self {
        Self(bytes)
    }
}

impl<C> Default for MockDisplay<C>
where
    C: PixelColor,
{
    fn default() -> Self {
        MockDisplay::new([[C::DEFAULT_BG; 24]; 16])
    }
}

impl<C> Drawing<C> for MockDisplay<C>
where
    C: PixelColor,
{
    fn draw<T>(&mut self, item_pixels: T)
    where
        T: IntoIterator<Item = Pixel<C>>,
    {
        for Pixel(coord, color) in item_pixels {
            if coord[0] >= 24 || coord[1] >= 16 {
                continue;
            }
            self.0[coord[1] as usize][coord[0] as usize] = color;
        }
    }
}

impl<C> SizedDrawing<C> for MockDisplay<C>
where
    C: PixelColor,
{
    fn draw_sized<T>(&mut self, item: T)
    where
        T: IntoIterator<Item = Pixel<C>> + Dimensions,
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
