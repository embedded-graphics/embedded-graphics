//! RGB565 pixel type

use crate::pixelcolor::PixelColor;

const RED_MASK: u16 = 0b11111_000000_00000;
const GREEN_MASK: u16 = 0b00000_111111_00000;
const BLUE_MASK: u16 = 0b00000_000000_11111;

/// A pixel type defining the commonly used RGB565 format
///
/// Pixel values are stored in a single `u16` segmented as follows:
///
/// ```ignore
/// 15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
///  r  r  r  r  r  g  g  g  g  g  g  b  b  b  b  b
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgb565Pixel(pub u16);

impl PixelColor for Rgb565Pixel {}

/// Convert from an 8 bit greyscale colour into a 16 bit greyscale representation
///
/// The 5 (or 6 for the green channel) most significant bits are taken from the input and assigned
/// to the three colour channels see <http://www.barth-dev.de/online/rgb565-color-picker/> for a
/// more in depth explanation.
impl From<u8> for Rgb565Pixel {
    fn from(other: u8) -> Self {
        Self::from((other, other, other))
    }
}

/// Take a tuple of 8 bit `(red, green, blue)` color values and convert them to a single 16 bit
/// color
///
/// The 2 or 3 (for the green channel) least significant bits are discarded
impl From<(u8, u8, u8)> for Rgb565Pixel {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        let value =
        // Red
        (((r as u16) << 8) & RED_MASK)
        // Green
        | (((g as u16) << 3) & GREEN_MASK)
        // Blue
        | ((b as u16) >> 3 & BLUE_MASK);

        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u8() {
        assert_eq!(Rgb565Pixel::from(0u8), Rgb565Pixel(0));
        assert_eq!(Rgb565Pixel::from(0xffu8), Rgb565Pixel(0xffff));
        assert_eq!(
            Rgb565Pixel::from(0b0000_1111u8),
            Rgb565Pixel(0b00001_000011_00001)
        );
        assert_eq!(
            Rgb565Pixel::from(0b1010_1010u8),
            Rgb565Pixel(0b10101_101010_10101)
        );
    }

    #[test]
    fn from_tuple() {
        assert_eq!(Rgb565Pixel::from((0xff, 0xff, 0xff)), Rgb565Pixel(0xffff));
        assert_eq!(
            Rgb565Pixel::from((0xff, 0x0f, 0b0101_0101)),
            Rgb565Pixel(0b11111_000011_01010)
        );
    }
}
