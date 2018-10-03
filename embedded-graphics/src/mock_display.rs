use drawable::Pixel;
use unsignedcoord::UnsignedCoord;
use Drawing;

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
        T: Iterator<Item = Pixel<u8>>,
    {
        for Pixel(UnsignedCoord(x, y), color) in item_pixels {
            if x >= 24 || y >= 16 {
                continue;
            }
            self.0[y as usize][x as usize] = color;
        }
    }
}
