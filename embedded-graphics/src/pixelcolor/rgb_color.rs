use crate::pixelcolor::{
    raw::{RawData, RawU16, RawU24},
    PixelColor,
};
use core::fmt;

/// RGB color.
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

/// Macro to implement a RgbColor type with the given channel bit positions.
macro_rules! impl_rgb_color {
    (
        $type:ident,
        $data_type:ty,
        ($r_bits:expr, $g_bits:expr, $b_bits:expr),
        ($r_pos:expr, $g_pos:expr, $b_pos:expr),
        $type_str:expr
    ) => {
        #[doc = $type_str]
        #[doc = "color."]
        #[doc = ""]
        #[doc = "Use the methods provided by the [`RgbColor`] trait to access"]
        #[doc = "individual color channels and predefined color constants."]
        #[doc = ""]
        #[doc = "See the [module-level documentation] for more information about"]
        #[doc = "conversion between this type and raw data."]
        #[doc = ""]
        #[doc = "[`RgbColor`]: trait.RgbColor.html"]
        #[doc = "[module-level documentation]: index.html"]
        #[derive(Clone, Copy, PartialEq, Eq)]
        pub struct $type(<$data_type as RawData>::Storage);

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
            #[doc = "Creates a new"]
            #[doc = $type_str]
            #[doc = "color.\n"]
            #[doc = "Too large channel values will be limited by setting the"]
            #[doc = "unused most significant bits to zero."]
            pub const fn new(r: u8, g: u8, b: u8) -> Self {
                type Storage = <$data_type as RawData>::Storage;

                Self(
                    ((r & Self::MAX_R) as Storage) << $r_pos
                        | ((g & Self::MAX_G) as Storage) << $g_pos
                        | ((b & Self::MAX_B) as Storage) << $b_pos
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

        impl PixelColor for $type {
            type Raw = $data_type;
        }

        impl From<$data_type> for $type {
            fn from(data: $data_type) -> Self {
                type Storage = <$data_type as RawData>::Storage;

                let data = data.into_inner();

                const MASK: Storage =
                    ($type::MAX_R as Storage) << $r_pos
                    | ($type::MAX_G as Storage) << $g_pos
                    | ($type::MAX_B as Storage) << $b_pos;

                Self(data & MASK)
            }
        }

        impl From<$type> for $data_type {
            fn from(color: $type) -> Self {
                Self::new(color.0)
            }
        }
    };

    // Recursive macro to stringify the type.
    (
        $type:ident,
        $data_type:ty,
        ($r_bits:expr, $g_bits:expr, $b_bits:expr),
        ($r_pos:expr, $g_pos:expr, $b_pos:expr)
    ) => {
        impl_rgb_color!(
            $type,
            $data_type,
            ($r_bits, $g_bits, $b_bits),
            ($r_pos, $g_pos, $b_pos),
            stringify!($type)
        );
    };
}

/// Helper macro to calculate bit positions for RGB and BGR colors
macro_rules! rgb_color {
    (
        $type:ident, $data_type:ty,
        Rgb = ($r_bits:expr, $g_bits:expr, $b_bits:expr)
    ) => {
        impl_rgb_color!(
            $type,
            $data_type,
            ($r_bits, $g_bits, $b_bits),
            ($g_bits + $b_bits, $b_bits, 0)
        );
    };

    (
        $type:ident, $data_type:ty,
        Bgr = ($r_bits:expr, $g_bits:expr, $b_bits:expr)
    ) => {
        impl_rgb_color!(
            $type,
            $data_type,
            ($r_bits, $g_bits, $b_bits),
            (0, $r_bits, $r_bits + $g_bits)
        );
    };
}

rgb_color!(Rgb555, RawU16, Rgb = (5, 5, 5));
rgb_color!(Bgr555, RawU16, Bgr = (5, 5, 5));
rgb_color!(Rgb565, RawU16, Rgb = (5, 6, 5));
rgb_color!(Bgr565, RawU16, Bgr = (5, 6, 5));

rgb_color!(Rgb888, RawU24, Rgb = (8, 8, 8));
rgb_color!(Bgr888, RawU24, Bgr = (8, 8, 8));

#[cfg(test)]
mod tests {
    use super::*;

    /// Convert color to integer and back again to test bit positions
    fn test_bpp16<C>(color: C, value: u16)
    where
        C: RgbColor + From<RawU16> + Into<RawU16> + core::fmt::Debug,
    {
        let value = RawU16::new(value);

        assert_eq!(color.into(), value);
        assert_eq!(C::from(value), color);
    }

    /// Convert color to integer and back again to test bit positions
    fn test_bpp24<C>(color: C, value: u32)
    where
        C: RgbColor + From<RawU24> + Into<RawU24> + core::fmt::Debug,
    {
        let value = RawU24::new(value);

        assert_eq!(color.into(), value);
        assert_eq!(C::from(value), color);
    }

    #[test]
    pub fn bit_positions_rgb555() {
        test_bpp16(Rgb555::new(0b10001, 0, 0), 0b10001 << 5 + 5);
        test_bpp16(Rgb555::new(0, 0b10001, 0), 0b10001 << 5);
        test_bpp16(Rgb555::new(0, 0, 0b10001), 0b10001 << 0);
    }

    #[test]
    pub fn bit_positions_bgr555() {
        test_bpp16(Bgr555::new(0b10001, 0, 0), 0b10001 << 0);
        test_bpp16(Bgr555::new(0, 0b10001, 0), 0b10001 << 5);
        test_bpp16(Bgr555::new(0, 0, 0b10001), 0b10001 << 5 + 5);
    }

    #[test]
    pub fn bit_positions_rgb565() {
        test_bpp16(Rgb565::new(0b10001, 0, 0), 0b10001 << 5 + 6);
        test_bpp16(Rgb565::new(0, 0b100001, 0), 0b100001 << 5);
        test_bpp16(Rgb565::new(0, 0, 0b10001), 0b10001 << 0);
    }

    #[test]
    pub fn bit_positions_bgr565() {
        test_bpp16(Bgr565::new(0b10001, 0, 0), 0b10001 << 0);
        test_bpp16(Bgr565::new(0, 0b100001, 0), 0b100001 << 5);
        test_bpp16(Bgr565::new(0, 0, 0b10001), 0b10001 << 5 + 6);
    }

    #[test]
    pub fn bit_positions_rgb888() {
        test_bpp24(Rgb888::new(0b10000001, 0, 0), 0b10000001 << 8 + 8);
        test_bpp24(Rgb888::new(0, 0b10000001, 0), 0b10000001 << 8);
        test_bpp24(Rgb888::new(0, 0, 0b10000001), 0b10000001 << 0);
    }

    #[test]
    pub fn bit_positions_bgr888() {
        test_bpp24(Bgr888::new(0b10000001, 0, 0), 0b10000001 << 0);
        test_bpp24(Bgr888::new(0, 0b10000001, 0), 0b10000001 << 8);
        test_bpp24(Bgr888::new(0, 0, 0b10000001), 0b10000001 << 8 + 8);
    }
}
