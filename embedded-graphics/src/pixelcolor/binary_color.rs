use crate::pixelcolor::PixelColor;

/// Binary color
///
/// `BinaryColor` is used for displays and images with two possible states.
/// The interpretation of active and inactive states can vary for different uses and isn't specified.
/// `BinaryColor::On` might represent a black pixel on a LCD panel and a white pixel on an OLED display.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryColor {
    /// An inactive pixel.
    Off,

    /// An active pixel.
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

    /// Maps active and inactive colors to a different `PixelColor` type.
    ///
    /// # Examples
    ///
    /// ```
    /// use embedded_graphics::prelude::*;
    /// use embedded_graphics::pixelcolor::{BinaryColor, Rgb565};
    /// let color = BinaryColor::On;
    /// assert_eq!(color.map_color(Rgb565::RED, Rgb565::GREEN), Rgb565::GREEN)
    /// ```
    pub fn map_color<T: PixelColor>(&self, color_off: T, color_on: T) -> T {
        match self {
            BinaryColor::On => color_on,
            BinaryColor::Off => color_off,
        }
    }
}

impl PixelColor for BinaryColor {
    const DEFAULT_BG: BinaryColor = BinaryColor::Off;
    const DEFAULT_FG: BinaryColor = BinaryColor::On;
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
}
