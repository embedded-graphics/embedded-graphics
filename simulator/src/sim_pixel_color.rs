use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub struct SimPixelColor(pub u8, pub u8, pub u8);

impl PixelColor for SimPixelColor {}

impl From<u8> for SimPixelColor {
    fn from(other: u8) -> Self {
        SimPixelColor(other, other, other)
    }
}

// Danger: Chops off upper bits
impl From<u16> for SimPixelColor {
    fn from(other: u16) -> Self {
        SimPixelColor(other as u8, other as u8, other as u8)
    }
}

// Danger: Chops off upper byte
impl From<u32> for SimPixelColor {
    fn from(other: u32) -> Self {
        let [_, r, g, b] = other.to_be_bytes();

        SimPixelColor(r, g, b)
    }
}

impl From<Rgb565> for SimPixelColor {
    fn from(other: Rgb565) -> Self {
        SimPixelColor(other.r(), other.g(), other.b())
    }
}
