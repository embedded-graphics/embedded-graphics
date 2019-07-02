use crate::pixelcolor::{FromRawData, PixelColor};
use core::fmt;

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

impl<C> PixelColor for C where C: RgbColor {}

/// Macro to implement a RgbColor type with the given channel bit positions.
macro_rules! impl_rgb_color {
    (
        $type:ident,
        $base_type:ident,
        ($r_bits:expr, $g_bits:expr, $b_bits:expr),
        ($r_pos:expr, $g_pos:expr, $b_pos:expr),
        $doc:expr
    ) => {
        #[doc = $doc]
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
            /// Create new color.
            pub const fn new(r: u8, g: u8, b: u8) -> Self {
                #![allow(trivial_numeric_casts)]

                Self(
                    ((r & Self::MAX_R) as $base_type) << $r_pos
                        | ((g & Self::MAX_G) as $base_type) << $g_pos
                        | ((b & Self::MAX_B) as $base_type) << $b_pos,
                )
            }
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

        impl From<$base_type> for $type {
            fn from(value: $base_type) -> Self {
                const MASK: $base_type = ($type::MAX_R as $base_type) << $r_pos
                    | ($type::MAX_G as $base_type) << $g_pos
                    | ($type::MAX_B as $base_type) << $b_pos;

                Self(value & MASK)
            }
        }

        impl FromRawData for $type {
            fn from_raw_data(value: u32) -> Self {
                #[allow(trivial_numeric_casts)]
                (value as $base_type).into()
            }
        }
    };

    // Recursive macro to build the documentation string.
    (
        $type:ident,
        $base_type:ident,
        ($r_bits:expr, $g_bits:expr, $b_bits:expr),
        ($r_pos:expr, $g_pos:expr, $b_pos:expr)
    ) => {
        impl_rgb_color!(
            $type,
            $base_type,
            ($r_bits, $g_bits, $b_bits),
            ($r_pos, $g_pos, $b_pos),
            concat!(
                stringify!($type),
                " color stored in a `",
                stringify!($base_type),
                "`"
            )
        );
    };
}

/// Helper macro to calculate bit posisions for RGB and BGR colors
macro_rules! rgb_color {
    (
        $type:ident : $base_type:ident,
        Rgb = ($r_bits:expr, $g_bits:expr, $b_bits:expr)
    ) => {
        impl_rgb_color!(
            $type,
            $base_type,
            ($r_bits, $g_bits, $b_bits),
            ($g_bits + $b_bits, $b_bits, 0)
        );
    };

    (
        $type:ident : $base_type:ident,
        Bgr = ($r_bits:expr, $g_bits:expr, $b_bits:expr)
    ) => {
        impl_rgb_color!(
            $type,
            $base_type,
            ($r_bits, $g_bits, $b_bits),
            (0, $r_bits, $r_bits + $g_bits)
        );
    };
}

rgb_color!(Rgb555: u16, Rgb = (5, 5, 5));
rgb_color!(Bgr555: u16, Bgr = (5, 5, 5));
rgb_color!(Rgb565: u16, Rgb = (5, 6, 5));
rgb_color!(Bgr565: u16, Bgr = (5, 6, 5));

rgb_color!(Rgb888: u32, Rgb = (8, 8, 8));
rgb_color!(Bgr888: u32, Bgr = (8, 8, 8));

#[cfg(test)]
mod tests {
    use super::*;

    /// Convert color to integer and back again to test bit positions
    fn test_bits<C, T>(color: C, value: T)
    where
        T: PartialEq + fmt::Debug,
        C: PixelColor + From<T> + Into<T>,
    {
        assert_eq!(color.into(), value);
        assert_eq!(C::from(value), color);
    }

    #[test]
    pub fn bit_positions_rgb555() {
        test_bits(Rgb555::new(0b10001, 0, 0), 0b10001 << 5 + 5);
        test_bits(Rgb555::new(0, 0b10001, 0), 0b10001 << 5);
        test_bits(Rgb555::new(0, 0, 0b10001), 0b10001 << 0);
    }

    #[test]
    pub fn bit_positions_bgr555() {
        test_bits(Bgr555::new(0b10001, 0, 0), 0b10001 << 0);
        test_bits(Bgr555::new(0, 0b10001, 0), 0b10001 << 5);
        test_bits(Bgr555::new(0, 0, 0b10001), 0b10001 << 5 + 5);
    }

    #[test]
    pub fn bit_positions_rgb565() {
        test_bits(Rgb565::new(0b10001, 0, 0), 0b10001 << 5 + 6);
        test_bits(Rgb565::new(0, 0b100001, 0), 0b100001 << 5);
        test_bits(Rgb565::new(0, 0, 0b10001), 0b10001 << 0);
    }

    #[test]
    pub fn bit_positions_bgr565() {
        test_bits(Bgr565::new(0b10001, 0, 0), 0b10001 << 0);
        test_bits(Bgr565::new(0, 0b100001, 0), 0b100001 << 5);
        test_bits(Bgr565::new(0, 0, 0b10001), 0b10001 << 5 + 6);
    }

    #[test]
    pub fn bit_positions_rgb888() {
        test_bits(Rgb888::new(0b10000001, 0, 0), 0b10000001 << 8 + 8);
        test_bits(Rgb888::new(0, 0b10000001, 0), 0b10000001 << 8);
        test_bits(Rgb888::new(0, 0, 0b10000001), 0b10000001 << 0);
    }

    #[test]
    pub fn bit_positions_bgr888() {
        test_bits(Bgr888::new(0b10000001, 0, 0), 0b10000001 << 0);
        test_bits(Bgr888::new(0, 0b10000001, 0), 0b10000001 << 8);
        test_bits(Bgr888::new(0, 0, 0b10000001), 0b10000001 << 8 + 8);
    }

    #[test]
    pub fn ignore_unused_bits() {
        let c1: Rgb888 = 0xFF000000.into();
        let c2: Rgb888 = 0.into();

        assert_eq!(c1, c2);
        assert_eq!(c1, Rgb888::BLACK);
    }
}
