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
///
/// # Conversion between BinaryColor and bool
///
/// ```
/// use embedded_graphics::pixelcolor::BinaryColor;
///
/// // A BinaryColor can be converted to a bool by using the is_on and is_off methods.
/// let color = BinaryColor::On;
/// assert!(color.is_on());
/// assert!(color.invert().is_off());
///
/// // BinaryColor implements From<bool>.
/// let bool_value = true;
/// let color: BinaryColor = bool_value.into();
/// assert_eq!(color, BinaryColor::On);
///
/// // this is equivalent to:
/// let bool_value = true;
/// let color = if bool_value {
///     BinaryColor::On
/// } else {
///     BinaryColor::Off
/// };
/// assert_eq!(color, BinaryColor::On);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub enum BinaryColor {
    /// Inactive pixel.
    Off,

    /// Active pixel.
    On,
}

impl Default for BinaryColor {
    fn default() -> Self {
        Self::Off
    }
}

impl BinaryColor {
    /// Inverts the color.
    ///
    /// # Examples
    ///
    /// ```
    /// use embedded_graphics::pixelcolor::BinaryColor;
    ///
    /// assert_eq!(BinaryColor::Off.invert(), BinaryColor::On);
    /// assert_eq!(BinaryColor::On.invert(), BinaryColor::Off);
    /// ```
    #[inline]
    pub const fn invert(self) -> Self {
        match self {
            BinaryColor::On => BinaryColor::Off,
            BinaryColor::Off => BinaryColor::On,
        }
    }

    /// Returns `true` if this color is `On`.
    ///
    /// # Examples
    ///
    /// ```
    /// use embedded_graphics::pixelcolor::BinaryColor;
    ///
    /// assert!(BinaryColor::On.is_on());
    /// ```
    #[inline]
    pub const fn is_on(self) -> bool {
        matches!(self, BinaryColor::On)
    }

    /// Returns `true` if this color is `Off`.
    ///
    /// # Examples
    ///
    /// ```
    /// use embedded_graphics::pixelcolor::BinaryColor;
    ///
    /// assert!(BinaryColor::Off.is_off());
    /// ```
    #[inline]
    pub const fn is_off(self) -> bool {
        matches!(self, BinaryColor::Off)
    }

    /// Maps active and inactive colors to a different type.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use embedded_graphics::pixelcolor::{BinaryColor, Rgb565};
    /// let color = BinaryColor::On;
    /// assert_eq!(color.map_color(Rgb565::RED, Rgb565::GREEN), Rgb565::GREEN)
    /// ```
    pub(crate) fn map_color<T>(self, value_off: T, value_on: T) -> T {
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

impl From<bool> for BinaryColor {
    fn from(value: bool) -> Self {
        if value {
            BinaryColor::On
        } else {
            BinaryColor::Off
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelcolor::{IntoStorage, Rgb565, RgbColor};

    #[test]
    fn default_color_is_off() {
        assert_eq!(BinaryColor::default(), BinaryColor::Off);
    }

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

    #[test]
    fn from_bool() {
        assert_eq!(BinaryColor::from(false), BinaryColor::Off);
        assert_eq!(BinaryColor::from(true), BinaryColor::On);
    }

    #[test]
    fn is_on_off() {
        assert!(BinaryColor::Off.is_off());
        assert!(!BinaryColor::On.is_off());

        assert!(!BinaryColor::Off.is_on());
        assert!(BinaryColor::On.is_on());
    }

    #[test]
    fn into_storage() {
        assert_eq!(BinaryColor::Off.into_storage(), 0u8);
        assert_eq!(BinaryColor::On.into_storage(), 1u8);
    }
}
