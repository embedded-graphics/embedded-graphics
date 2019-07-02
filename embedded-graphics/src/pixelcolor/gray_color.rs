use crate::pixelcolor::{BinaryColor, FromRawData, PixelColor};

/// 8 bit grayscale color stored in a `u8`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Gray8(u8);

impl Gray8 {
    /// Creates a new color.
    pub const fn new(y: u8) -> Self {
        Self(y)
    }

    /// Returns the luminance channel value.
    pub fn y(&self) -> u8 {
        self.0
    }

    /// The maximum value in luminance channel.
    pub const MAX_Y: u8 = 255;

    /// Black color (Y = 0%)
    pub const BLACK: Self = Self::new(0);

    /// White color (Y = 100%)
    pub const WHITE: Self = Self::new(255);
}

impl PixelColor for Gray8 {
    const DEFAULT_BG: Self = Self::BLACK;
    const DEFAULT_FG: Self = Self::WHITE;
}

impl From<Gray8> for u8 {
    fn from(color: Gray8) -> Self {
        color.0
    }
}

impl From<u8> for Gray8 {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl FromRawData for Gray8 {
    fn from_raw_data(value: u32) -> Self {
        Self::new(value as u8)
    }
}

impl From<BinaryColor> for Gray8 {
    fn from(color: BinaryColor) -> Self {
        color.map_color(Self::BLACK, Self::WHITE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn luma() {
        assert_eq!(Gray8::from(0u8), Gray8::BLACK);
        assert_eq!(Gray8::from(255u8), Gray8::WHITE);

        assert_eq!(u8::from(Gray8::BLACK), 0);
        assert_eq!(u8::from(Gray8::WHITE), 255);
    }

    #[test]
    pub fn conversion_from_binary_color() {
        assert_eq!(Gray8::from(BinaryColor::Off), Gray8::BLACK);
        assert_eq!(Gray8::from(BinaryColor::On), Gray8::WHITE);
    }
}
