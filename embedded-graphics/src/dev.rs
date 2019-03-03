//! Devlopment/test helpers

use crate::pixelcolor::PixelColor;

/// Fake pixel colour useful for testing
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct TestPixelColor(pub u8);

impl PixelColor for TestPixelColor {}

impl From<u8> for TestPixelColor {
    fn from(other: u8) -> Self {
        TestPixelColor(other)
    }
}
