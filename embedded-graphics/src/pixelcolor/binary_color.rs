use crate::pixelcolor::{
    raw::{RawData, RawU1},
    PixelColor,
};

/// Binary color.
///
/// `BinaryColor` is used for displays and images with two possible color states.
///
/// The interpretation of active and inactive states can be different for
/// different types of displays. A `BinaryColor::On` might represent a black
/// pixel on an LCD and a white pixel on an OLED display.
///
/// To simplify the conversion from `BinaryColor` to RGB and grayscale color
/// types the default conversions assume that `BinaryColor::Off` is black and
/// `BinaryColor::On` is white.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryColor {
    /// Inactive pixel.
    Off,

    /// Active pixel.
    On,
}

impl BinaryColor {
    /// Inverts the color.
    pub fn invert(&self) -> Self {
        match self {
            BinaryColor::On => BinaryColor::Off,
            BinaryColor::Off => BinaryColor::On,
        }
    }

    /// Maps active and inactive colors to a different type.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use embedded_graphics::prelude::*;
    /// use embedded_graphics::pixelcolor::{BinaryColor, Rgb565};
    /// let color = BinaryColor::On;
    /// assert_eq!(color.map_color(Rgb565::RED, Rgb565::GREEN), Rgb565::GREEN)
    /// ```
    pub(crate) fn map_color<T>(&self, value_off: T, value_on: T) -> T {
        match self {
            BinaryColor::On => value_on,
            BinaryColor::Off => value_off,
        }
    }
}

impl PixelColor for BinaryColor {
    type Raw = RawU1;
}

impl From<RawU1> for BinaryColor {
    fn from(data: RawU1) -> Self {
        if data.into_inner() != 0 {
            BinaryColor::On
        } else {
            BinaryColor::Off
        }
    }
}

impl From<BinaryColor> for RawU1 {
    fn from(color: BinaryColor) -> Self {
        RawU1::new(color.map_color(0, 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelcolor::{Rgb565, RgbColor};

    #[test]
    fn invert_binary_color() {
        assert_eq!(BinaryColor::Off.invert(), BinaryColor::On);
        assert_eq!(BinaryColor::On.invert(), BinaryColor::Off);
    }

    #[test]
    fn map_binary_color() {
        assert_eq!(
            BinaryColor::Off.map_color(Rgb565::RED, Rgb565::GREEN),
            Rgb565::RED
        );
        assert_eq!(
            BinaryColor::On.map_color(Rgb565::RED, Rgb565::GREEN),
            Rgb565::GREEN
        );
    }

    #[test]
    fn from_data() {
        assert_eq!(BinaryColor::from(RawU1::new(0)), BinaryColor::Off);
        assert_eq!(BinaryColor::from(RawU1::new(1)), BinaryColor::On);
    }

    #[test]
    fn into_data() {
        assert_eq!(RawU1::from(BinaryColor::Off).into_inner(), 0);
        assert_eq!(RawU1::from(BinaryColor::On).into_inner(), 1);
    }
}
