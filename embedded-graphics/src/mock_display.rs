use crate::drawable::Pixel;
use crate::Drawing;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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
