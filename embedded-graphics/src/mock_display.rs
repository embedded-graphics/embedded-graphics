use crate::drawable::{Dimensions, Pixel};
use crate::prelude::*;
use crate::{Drawing, SizedDrawing};

/// Mock display for use in tests and some doc examples. Do not use directly!
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Display(pub [[u8; 24]; 16]);

impl Default for Display {
    fn default() -> Self {
        Display([[0; 24]; 16])
    }
}

impl Drawing<u8> for Display {
    fn draw<T>(&mut self, item_pixels: T)
    where
        T: IntoIterator<Item = Pixel<u8>>,
    {
        for Pixel(coord, color) in item_pixels {
            if coord[0] >= 24 || coord[1] >= 16 {
                continue;
            }
            self.0[coord[1] as usize][coord[0] as usize] = color;
        }
    }
}

impl SizedDrawing<u8> for Display where {
    fn draw_sized<T>(&mut self, item: T)
    where
        T: IntoIterator<Item = Pixel<u8>> + Dimensions,
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
