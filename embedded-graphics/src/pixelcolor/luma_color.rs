use crate::pixelcolor::{FromRawData, PixelColor};

/// 8 bit luminance color stored in a `u8`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Y8(u8);

impl Y8 {
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
    pub const BLACK: Y8 = Self::new(0);

    /// White color (Y = 100%)
    pub const WHITE: Y8 = Self::new(255);
}

impl PixelColor for Y8 {
    const DEFAULT_BG: Self = Y8::BLACK;
    const DEFAULT_FG: Self = Y8::WHITE;
}

impl From<Y8> for u8 {
    fn from(color: Y8) -> Self {
        color.0
    }
}

impl From<u8> for Y8 {
    fn from(value: u8) -> Self {
        Y8::new(value)
    }
}

impl FromRawData for Y8 {
    fn from_raw_data(value: u32) -> Self {
        Y8::new(value as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn luma() {
        assert_eq!(Y8::from(0u8), Y8::BLACK);
        assert_eq!(Y8::from(255u8), Y8::WHITE);

        assert_eq!(u8::from(Y8::BLACK), 0);
        assert_eq!(u8::from(Y8::WHITE), 255);
    }
}
