//! Pixel color trait

/// Pixel color
pub trait PixelColor: Clone + Copy + From<u8> {}

impl PixelColor for u8 {}
impl PixelColor for u16 {}
impl PixelColor for u32 {}

/// Pixel wrapper around `u8` type
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct PixelColorU8(pub u8);

impl From<u8> for PixelColorU8 {
    fn from(other: u8) -> Self {
        PixelColorU8(other)
    }
}

impl PixelColorU8 {
    pub fn into_inner(self) -> u8 {
        self.0
    }
}

impl PixelColor for PixelColorU8 {}

/// Pixel wrapper around `u16` type
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct PixelColorU16(pub u16);

impl From<u8> for PixelColorU16 {
    fn from(other: u8) -> Self {
        PixelColorU16(other as u16)
    }
}

impl From<u16> for PixelColorU16 {
    fn from(other: u16) -> Self {
        PixelColorU16(other)
    }
}

impl PixelColorU16 {
    pub fn into_inner(self) -> u16 {
        self.0
    }
}

impl PixelColor for PixelColorU16 {}

/// Pixel wrapper around `u32` type
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct PixelColorU32(pub u32);

impl From<u8> for PixelColorU32 {
    fn from(other: u8) -> Self {
        PixelColorU32(other as u32)
    }
}

impl From<u32> for PixelColorU32 {
    fn from(other: u32) -> Self {
        PixelColorU32(other)
    }
}

impl PixelColorU32 {
    pub fn into_inner(self) -> u32 {
        self.0
    }
}

impl PixelColor for PixelColorU32 {}
