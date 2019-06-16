//! RGB565 pixel type

use crate::pixelcolor::PixelColor;

const RED_MASK: u16 = 0b11111_000000_00000;
const GREEN_MASK: u16 = 0b00000_111111_00000;
const BLUE_MASK: u16 = 0b00000_000000_11111;

/// A pixel type defining the commonly used RGB565 format
///
/// Pixel values are stored in a single `u16` segmented as follows:
///
/// ```text
/// 15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
///  r  r  r  r  r  g  g  g  g  g  g  b  b  b  b  b
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgb565(pub u16);

impl Rgb565 {
    /// Get the red component as a `u8`
    ///
    /// The least significant 3 bits will always be `0`
    pub fn r(&self) -> u8 {
        ((self.0 & RED_MASK) >> 8) as u8
    }

    /// Get the green component as a `u8`
    ///
    /// The least significant 2 bits will always be `0`
    pub fn g(&self) -> u8 {
        ((self.0 & GREEN_MASK) >> 3) as u8
    }

    /// Get the blue component as a `u8`
    ///
    /// The least significant 3 bits will always be `0`
    pub fn b(&self) -> u8 {
        ((self.0 & BLUE_MASK) << 3) as u8
    }
}

impl PixelColor for Rgb565 {}

/// Convert from an 8 bit greyscale colour into a 16 bit greyscale representation
///
/// The 5 (or 6 for the green channel) most significant bits are taken from the input and assigned
/// to the three colour channels see <http://www.barth-dev.de/online/rgb565-color-picker/> for a
/// more in depth explanation.
impl From<u8> for Rgb565 {
    fn from(other: u8) -> Self {
        Self::from((other, other, other))
    }
}

impl From<u16> for Rgb565 {
    fn from(other: u16) -> Self {
        Self(other)
    }
}

/// Take a tuple of 8 bit `(red, green, blue)` color values and convert them to a single 16 bit
/// color
///
/// The 2 or 3 (for the green channel) least significant bits are discarded
impl From<(u8, u8, u8)> for Rgb565 {
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

/// Build a `u16` from two `u8`s
///
/// Uses [`u16::from_be_bytes`] internally. If the input bytes are little endian, use [`u16::from_le_bytes`] directly:
///
/// ```rust
/// use embedded_graphics::pixelcolor::Rgb565;
///
/// let pixel = Rgb565(u16::from_le_bytes([0xab, 0xcd]));
///
/// assert_eq!(pixel, Rgb565(0xCDAB));
/// ```
///
/// [`u16::from_le_bytes`]: https://doc.rust-lang.org/stable/std/primitive.u16.html#method.from_le_bytes
/// [`u16::from_be_bytes`]: https://doc.rust-lang.org/stable/std/primitive.u16.html#method.from_be_bytes
impl From<[u8; 2]> for Rgb565 {
    fn from(other: [u8; 2]) -> Self {
        Self(u16::from_be_bytes(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u8() {
        assert_eq!(Rgb565::from(0u8), Rgb565(0));
        assert_eq!(Rgb565::from(0xffu8), Rgb565(0xffff));
        assert_eq!(Rgb565::from(0b0000_1111u8), Rgb565(0b00001_000011_00001));
        assert_eq!(Rgb565::from(0b1010_1010u8), Rgb565(0b10101_101010_10101));
    }

    #[test]
    fn from_u16() {
        assert_eq!(Rgb565::from(0xffu16), Rgb565(0x00ff));
        assert_eq!(Rgb565::from(0xffffu16), Rgb565(0xffff));
        assert_eq!(Rgb565::from(0xababu16), Rgb565(0xabab));
    }

    #[test]
    fn from_tuple() {
        assert_eq!(Rgb565::from((0xff, 0xff, 0xff)), Rgb565(0xffff));
        assert_eq!(
            Rgb565::from((0xff, 0x0f, 0b0101_0101)),
            Rgb565(0b11111_000011_01010)
        );
    }

    #[test]
    fn from_be_bytes() {
        assert_eq!(Rgb565::from([0xff, 0x00]), Rgb565(0xff00));
        assert_eq!(Rgb565::from([0xab, 0xcd]), Rgb565(0xabcd));
    }

    #[test]
    fn accessors() {
        let p = Rgb565(0b11001_010101_10010);

        assert_eq!(p.r(), 0b11001_000);
        assert_eq!(p.g(), 0b010101_00);
        assert_eq!(p.b(), 0b10010_000);
    }
}
