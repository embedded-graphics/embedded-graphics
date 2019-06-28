//! Pixel color

use byteorder::{BigEndian, ByteOrder, LittleEndian};
use core::fmt;

/// Pixel color trait
pub trait PixelColor: Clone + Copy + PartialEq + fmt::Debug {
    /// Default background color
    const DEFAULT_BG: Self;

    /// Default foreground color
    const DEFAULT_FG: Self;
}

/// Convert raw data to color structs.
pub trait FromSlice {
    /// Convert big endian data to color.
    fn from_be_slice(data: &[u8]) -> Self;

    /// Convert little endian data to color.
    fn from_le_slice(data: &[u8]) -> Self;
}

/// RGB color
pub trait RgbColor: PixelColor {
    /// Returns the red channel value.
    fn r(&self) -> u8;

    /// Returns the green channel value.
    fn g(&self) -> u8;

    /// Returns the blue channel value.
    fn b(&self) -> u8;

    /// The maximum value in the red channel.
    const MAX_R: u8;

    /// The maximum value in the green channel.
    const MAX_G: u8;

    /// The maximum value in the blue channel.
    const MAX_B: u8;

    /// Black color (R: 0%, G: 0%, B: 0%)
    const BLACK: Self;

    /// Red color (R: 100%, G: 0%, B: 0%)
    const RED: Self;

    /// Green color (R: 0%, G: 100%, B: 0%)
    const GREEN: Self;

    /// Blue color (R: 0%, G: 0%, B: 100%)
    const BLUE: Self;

    /// Yellow color (R: 100%, G: 100%, B: 0%)
    const YELLOW: Self;

    /// Magenta color (R: 100%, G: 0%, B: 100%)
    const MAGENTA: Self;

    /// Cyan color (R: 0%, G: 100%, B: 100%)
    const CYAN: Self;

    /// White color (R: 100%, G: 100%, B: 100%)
    const WHITE: Self;
}

macro_rules! impl_rgb_color {
    ($type: ident, $type_string: expr, $base_type: ident, $base_type_string: expr, ($r_bits: expr, $g_bits: expr, $b_bits: expr), ($r_pos: expr, $g_pos: expr, $b_pos: expr)) => {
        #[doc=$type_string]
        #[doc = " color stored in a `"]
        #[doc=$base_type_string]
        #[doc = "`"]
        #[derive(Clone, Copy, PartialEq)]
        pub struct $type($base_type);

        impl fmt::Debug for $type {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "{}(r: {}, g: {}, b: {})",
                    stringify!($type),
                    self.r(),
                    self.g(),
                    self.b()
                )
            }
        }

        impl $type
        where
            Self: RgbColor,
        {
            /// New
            pub const fn new(r: u8, g: u8, b: u8) -> Self {
                #![allow(trivial_numeric_casts)]

                Self(
                    ((r & Self::MAX_R) as $base_type) << $r_pos
                        | ((g & Self::MAX_G) as $base_type) << $g_pos
                        | ((b & Self::MAX_B) as $base_type) << $b_pos,
                )
            }
        }

        impl PixelColor for $type {
            const DEFAULT_BG: Self = Self::BLACK;
            const DEFAULT_FG: Self = Self::WHITE;
        }

        impl RgbColor for $type {
            fn r(&self) -> u8 {
                #![allow(trivial_numeric_casts)]

                (self.0 >> $r_pos) as u8 & Self::MAX_R
            }

            fn g(&self) -> u8 {
                #![allow(trivial_numeric_casts)]

                (self.0 >> $g_pos) as u8 & Self::MAX_G
            }

            fn b(&self) -> u8 {
                #![allow(trivial_numeric_casts)]

                (self.0 >> $b_pos) as u8 & Self::MAX_B
            }

            const MAX_R: u8 = ((1usize << $r_bits) - 1) as u8;
            const MAX_G: u8 = ((1usize << $g_bits) - 1) as u8;
            const MAX_B: u8 = ((1usize << $b_bits) - 1) as u8;

            const BLACK: Self = Self::new(0, 0, 0);
            const RED: Self = Self::new(Self::MAX_R, 0, 0);
            const GREEN: Self = Self::new(0, Self::MAX_G, 0);
            const BLUE: Self = Self::new(0, 0, Self::MAX_B);
            const YELLOW: Self = Self::new(Self::MAX_R, Self::MAX_G, 0);
            const MAGENTA: Self = Self::new(Self::MAX_R, 0, Self::MAX_B);
            const CYAN: Self = Self::new(0, Self::MAX_G, Self::MAX_B);
            const WHITE: Self = Self::new(Self::MAX_R, Self::MAX_G, Self::MAX_B);
        }

        impl From<$type> for $base_type {
            fn from(color: $type) -> Self {
                color.0
            }
        }
    };

    ($type: ident, $base_type: ident, ($r_bits: expr, $g_bits: expr, $b_bits: expr), ($r_pos: expr, $g_pos: expr, $b_pos: expr)) => {
        impl_rgb_color!(
            $type,
            stringify!($type),
            $base_type,
            stringify!($base_type),
            ($r_bits, $g_bits, $b_bits),
            ($r_pos, $g_pos, $b_pos)
        );
    };
}

macro_rules! impl_rgb_bgr_pair {
    (($rgb_type: ident, $bgr_type: ident) : $base_type: ident, ($r_bits: expr, $g_bits: expr, $b_bits: expr)) => {
        impl_rgb_color!(
            $rgb_type,
            $base_type,
            ($r_bits, $g_bits, $b_bits),
            ($b_bits + $g_bits, $b_bits, 0)
        );

        impl_rgb_color!(
            $bgr_type,
            $base_type,
            ($r_bits, $g_bits, $b_bits),
            (0, $r_bits, $r_bits + $g_bits)
        );

        impl From<$rgb_type> for $bgr_type {
            fn from(c: $rgb_type) -> Self {
                Self::new(c.r(), c.g(), c.b())
            }
        }

        impl From<$bgr_type> for $rgb_type {
            fn from(c: $bgr_type) -> Self {
                Self::new(c.r(), c.g(), c.b())
            }
        }
    };
}

macro_rules! impl_from_slice_u16 {
    ($type: ident) => {
        impl FromSlice for $type {
            fn from_le_slice(data: &[u8]) -> Self {
                Self(LittleEndian::read_u16(data))
            }

            fn from_be_slice(data: &[u8]) -> Self {
                Self(BigEndian::read_u16(data))
            }
        }
    };
}

macro_rules! impl_from_slice_u32 {
    ($type: ident) => {
        impl FromSlice for $type {
            fn from_le_slice(data: &[u8]) -> Self {
                if data.len() == 3 {
                    Self(LittleEndian::read_u24(data))
                } else {
                    Self(LittleEndian::read_u32(data) & 0xFFFFFF)
                }
            }

            fn from_be_slice(data: &[u8]) -> Self {
                if data.len() == 3 {
                    Self(BigEndian::read_u24(data))
                } else {
                    Self(BigEndian::read_u32(data) & 0xFFFFFF)
                }
            }
        }
    };
}

impl_rgb_bgr_pair!((Rgb555, Bgr555): u16, (5, 5, 5));
impl_rgb_bgr_pair!((Rgb565, Bgr565): u16, (5, 6, 5));
impl_rgb_bgr_pair!((Rgb888, Bgr888): u32, (8, 8, 8));

impl_from_slice_u16!(Rgb555);
impl_from_slice_u16!(Bgr555);
impl_from_slice_u16!(Rgb565);
impl_from_slice_u16!(Bgr565);

impl_from_slice_u32!(Rgb888);
impl_from_slice_u32!(Bgr888);

macro_rules! convert_channel {
    ($value: expr, $from_max: expr, $to_max: expr) => {
        (($value as u16 * $to_max as u16 + $from_max as u16 / 2) / $from_max as u16) as u8
    };
}

macro_rules! impl_rgb_conversion {
    ($type: ident, ($($other_type: ident),+)) => {
        $(
            impl From<$other_type> for $type {
                fn from(other: $other_type) -> Self {
                    Self::new(
                        convert_channel!(other.r(), $other_type::MAX_R, $type::MAX_R),
                        convert_channel!(other.g(), $other_type::MAX_G, $type::MAX_G),
                        convert_channel!(other.b(), $other_type::MAX_B, $type::MAX_B),
                    )
                }
            }
        )*

        impl From<Y8> for $type {
            fn from(other: Y8) -> Self {
                Self::new(
                    convert_channel!(other.y(), Y8::MAX_Y, $type::MAX_R),
                    convert_channel!(other.y(), Y8::MAX_Y, $type::MAX_G),
                    convert_channel!(other.y(), Y8::MAX_Y, $type::MAX_B),
                )

            }
        }
    };
}

impl_rgb_conversion!(Rgb555, (Rgb565, Bgr565, Rgb888, Bgr888));
impl_rgb_conversion!(Bgr555, (Rgb565, Bgr565, Rgb888, Bgr888));
impl_rgb_conversion!(Rgb565, (Rgb555, Bgr555, Rgb888, Bgr888));
impl_rgb_conversion!(Bgr565, (Rgb555, Bgr555, Rgb888, Bgr888));
impl_rgb_conversion!(Rgb888, (Rgb555, Bgr555, Rgb565, Bgr565));
impl_rgb_conversion!(Bgr888, (Rgb555, Bgr555, Rgb565, Bgr565));

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

impl FromSlice for Y8 {
    fn from_be_slice(data: &[u8]) -> Self {
        Self(data[0])
    }

    fn from_le_slice(data: &[u8]) -> Self {
        Self(data[0])
    }
}

impl From<Y8> for u8 {
    fn from(color: Y8) -> Self {
        color.0
    }
}

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
    /// # use embedded_graphics::pixelcolor::{BinaryColor, RgbColor, Rgb565};
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

    macro_rules! test_rgb_conversions {
        ($type: ident, ($($other_type: ident),+)) => {
            $(
                assert_eq!($type::from($other_type::BLACK), $type::BLACK);
                assert_eq!($type::from($other_type::RED), $type::RED);
                assert_eq!($type::from($other_type::GREEN), $type::GREEN);
                assert_eq!($type::from($other_type::BLUE), $type::BLUE);
                assert_eq!($type::from($other_type::YELLOW), $type::YELLOW);
                assert_eq!($type::from($other_type::MAGENTA), $type::MAGENTA);
                assert_eq!($type::from($other_type::CYAN), $type::CYAN);
                assert_eq!($type::from($other_type::WHITE), $type::WHITE);
            )*

            assert_eq!($type::from(Y8::BLACK), $type::BLACK);
            assert_eq!($type::from(Y8::WHITE), $type::WHITE);
        }
    }

    #[test]
    fn rgb_color_constant_conversions() {
        test_rgb_conversions!(Rgb555, (Rgb555, Bgr555, Rgb565, Bgr565, Rgb888, Bgr888));
        test_rgb_conversions!(Bgr555, (Rgb555, Rgb555, Rgb565, Bgr565, Rgb888, Bgr888));
        test_rgb_conversions!(Rgb565, (Rgb555, Bgr555, Rgb565, Bgr565, Rgb888, Bgr888));
        test_rgb_conversions!(Bgr565, (Rgb555, Bgr555, Rgb565, Bgr565, Rgb888, Bgr888));
        test_rgb_conversions!(Rgb888, (Rgb555, Bgr555, Rgb565, Bgr565, Rgb888, Bgr888));
        test_rgb_conversions!(Bgr888, (Rgb555, Bgr555, Rgb565, Bgr565, Rgb888, Bgr888));
    }

    #[test]
    fn convert_rgb565_to_rgb888_and_back() {
        for r in 0..=63 {
            let c = Rgb565::new(r, 0, 0);
            let c2 = Rgb888::from(c);
            let c3 = Rgb565::from(c2);

            assert_eq!(c, c3);
        }

        for g in 0..=63 {
            let c = Rgb565::new(0, g, 0);
            let c2 = Rgb888::from(c);
            let c3 = Rgb565::from(c2);

            assert_eq!(c, c3);
        }

        for b in 0..=63 {
            let c = Rgb565::new(0, 0, b);
            let c2 = Rgb888::from(c);
            let c3 = Rgb565::from(c2);

            assert_eq!(c, c3);
        }
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
}
