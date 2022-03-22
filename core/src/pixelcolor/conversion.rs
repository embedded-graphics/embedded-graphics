use crate::pixelcolor::{binary_color::*, gray_color::*, rgb_color::*};

/// Convert color channel values from one bit depth to another.
///
/// Fixed point implementation of the conversion formula:
/// `out = round(in * from_max / to_max)`
const fn convert_channel<const FROM_MAX: u8, const TO_MAX: u8>(value: u8) -> u8 {
    if TO_MAX != FROM_MAX {
        const SHIFT: usize = 24;
        const CONST_0_5: u32 = 1 << (SHIFT - 1);

        // `value * from_max / to_max` scaled by `1 << SHIFT`.
        let result = value as u32 * (((TO_MAX as u32) << SHIFT) / FROM_MAX as u32);

        // Scale the result back down into an u8.
        ((result + CONST_0_5) >> SHIFT) as u8
    } else {
        value
    }
}

/// Calculates the luma value based on ITU-R BT.601.
fn luma(color: Rgb888) -> u8 {
    let r = u16::from(color.r());
    let g = u16::from(color.g());
    let b = u16::from(color.b());

    // Original formula: 0.299 * R + 0.587 * G + 0.144 * B
    ((r * 77 + g * 150 + b * 29 + 128) / 256) as u8
}

/// Macro to implement conversion between RGB color types.
macro_rules! impl_rgb_conversion {
    ($from_type:ident => $($to_type:ident),+) => {
        $(impl From<$from_type> for $to_type {
            fn from(other: $from_type) -> Self {
                Self::new(
                    convert_channel::<{$from_type::MAX_R}, {$to_type::MAX_R}>(other.r()),
                    convert_channel::<{$from_type::MAX_G}, {$to_type::MAX_G}>(other.g()),
                    convert_channel::<{$from_type::MAX_B}, {$to_type::MAX_B}>(other.b()),
                )
            }
        })*

        impl $from_type {
            pub(crate) const fn with_rgb888(r: u8, g: u8, b: u8) -> Self {
                Self::new(
                    convert_channel::<{Rgb888::MAX_R}, {$from_type::MAX_R}>(r),
                    convert_channel::<{Rgb888::MAX_G}, {$from_type::MAX_G}>(g),
                    convert_channel::<{Rgb888::MAX_B}, {$from_type::MAX_B}>(b),
                )
            }
        }
    };
}

impl_rgb_conversion!(Rgb555 => Bgr555, Rgb565, Bgr565, Rgb666, Bgr666, Rgb888, Bgr888);
impl_rgb_conversion!(Bgr555 => Rgb555, Rgb565, Bgr565, Rgb666, Bgr666, Rgb888, Bgr888);
impl_rgb_conversion!(Rgb565 => Rgb555, Bgr555, Bgr565, Rgb666, Bgr666, Rgb888, Bgr888);
impl_rgb_conversion!(Bgr565 => Rgb555, Bgr555, Rgb565, Rgb666, Bgr666, Rgb888, Bgr888);
impl_rgb_conversion!(Rgb666 => Rgb555, Bgr555, Rgb565, Bgr666, Bgr565, Bgr888, Rgb888);
impl_rgb_conversion!(Bgr666 => Rgb555, Bgr555, Rgb565, Rgb666, Bgr565, Bgr888, Rgb888);
impl_rgb_conversion!(Rgb888 => Rgb555, Bgr555, Rgb565, Rgb666, Bgr666, Bgr565, Bgr888);
impl_rgb_conversion!(Bgr888 => Rgb555, Bgr555, Rgb565, Rgb666, Bgr666, Bgr565, Rgb888);

/// Macro to implement conversion between grayscale color types.
macro_rules! impl_gray_conversion {
    ($from_type:ident => $($to_type:ident),+) => {
        $(impl From<$from_type> for $to_type {
            fn from(other: $from_type) -> Self {
                Self::new(convert_channel::<{$from_type::MAX_LUMA}, {$to_type::MAX_LUMA}>(other.luma()))
            }
        })*
    };
}

impl_gray_conversion!(Gray2 => Gray4, Gray8);
impl_gray_conversion!(Gray4 => Gray2, Gray8);
impl_gray_conversion!(Gray8 => Gray2, Gray4);

/// Macro to implement conversions between grayscale and RGB color types.
macro_rules! impl_rgb_to_and_from_gray {
    ($($gray_type:ident),+ => $rgb_type:ident) => {
        $(impl From<$gray_type> for $rgb_type {
            fn from(other: $gray_type) -> Self {
                Self::new(
                    convert_channel::<{$gray_type::MAX_LUMA}, {$rgb_type::MAX_R}>(other.luma()),
                    convert_channel::<{$gray_type::MAX_LUMA}, {$rgb_type::MAX_G}>(other.luma()),
                    convert_channel::<{$gray_type::MAX_LUMA}, {$rgb_type::MAX_B}>(other.luma()),
                )
            }
        })+

        $(impl From<$rgb_type> for $gray_type {
            fn from(other: $rgb_type) -> Self {
                let intensity = luma(Rgb888::from(other));
                Gray8::new(intensity).into()
            }
        })+
    };

    ($($gray_type:ident),+ => $rgb_type:ident, $($rest:ident),+) => {
        impl_rgb_to_and_from_gray!($($gray_type),+ => $rgb_type);
        impl_rgb_to_and_from_gray!($($gray_type),+ => $($rest),*);
    }
}

impl_rgb_to_and_from_gray!(Gray2, Gray4, Gray8 => Rgb555, Bgr555, Rgb565, Bgr565, Rgb666, Bgr666, Rgb888, Bgr888);

/// Macro to implement conversion from `BinaryColor` to RGB and grayscale types.
macro_rules! impl_from_binary {
    ($($type:ident),*) => {
        $(impl From<BinaryColor> for $type {
            fn from(color: BinaryColor) -> Self {
                color.map_color(Self::BLACK, Self::WHITE)
            }
        })*
    };
}

impl_from_binary!(
    Rgb555, Bgr555, Rgb565, Bgr565, Rgb666, Bgr666, Rgb888, Bgr888, Gray2, Gray4, Gray8
);

/// Macro to implement conversion from grayscale types to `BinaryColor`.
macro_rules! impl_gray_to_binary {
    ($($type:ident),* ) => {
        $(impl From<$type> for BinaryColor {
            fn from(color: $type) -> Self {
                (color.luma() >= $type::GRAY_50.luma()).into()
            }
        })*
    };
}

impl_gray_to_binary!(Gray2, Gray4, Gray8);

/// Macro to implement conversion from RGB types to `BinaryColor`.
macro_rules! impl_rgb_to_binary {
    ($($type:ident),*) => {
        $(impl From<$type> for BinaryColor {
            fn from(color: $type) -> Self {
                (luma(Rgb888::from(color)) >= 128).into()
            }
        })*
    };
}

impl_rgb_to_binary!(Rgb555, Bgr555, Rgb565, Bgr565, Rgb666, Bgr666, Rgb888, Bgr888);

#[cfg(test)]
mod tests {
    use core::fmt::Debug;

    use super::*;

    #[test]
    fn convert_rgb565_to_rgb888_and_back() {
        for r in 0..=Rgb565::MAX_R {
            let c = Rgb565::new(r, 0, 0);
            let c2 = Rgb888::from(c);
            let c3 = Rgb565::from(c2);

            assert_eq!(c, c3);
        }

        for g in 0..=Rgb565::MAX_G {
            let c = Rgb565::new(0, g, 0);
            let c2 = Rgb888::from(c);
            let c3 = Rgb565::from(c2);

            assert_eq!(c, c3);
        }

        for b in 0..=Rgb565::MAX_B {
            let c = Rgb565::new(0, 0, b);
            let c2 = Rgb888::from(c);
            let c3 = Rgb565::from(c2);

            assert_eq!(c, c3);
        }
    }

    /// Calls the given function with every combination of two sets of types.
    ///
    /// If only one set of types is given the same types will be used for both sets.
    macro_rules! type_matrix {
        ($function:ident; $from_type:ident => $to_type:ident) => {
            $function::<$from_type, $to_type>();
        };

        ($function:ident; $from_type:ident => $to_type:ident, $($to_types:ident),+ ) => {
            type_matrix!($function; $from_type => $to_type);
            type_matrix!($function; $from_type => $($to_types),*);
        };

        ($function:ident; $from_type:ident, $($from_types:ident),+ => $($to_types:ident),+ ) => {
            type_matrix!($function; $from_type => $($to_types),*);
            type_matrix!($function; $($from_types),* => $($to_types),*);
        };

        ($function:ident; $($types:ident),+) => {
            type_matrix!($function; $($types),* => $($types),*);
        };
    }

    #[test]
    fn rgb_to_rgb() {
        fn test_rgb_to_rgb<FromC: RgbColor + Debug, ToC: RgbColor + From<FromC> + Debug>() {
            assert_eq!(ToC::from(FromC::BLACK), ToC::BLACK);
            assert_eq!(ToC::from(FromC::RED), ToC::RED);
            assert_eq!(ToC::from(FromC::GREEN), ToC::GREEN);
            assert_eq!(ToC::from(FromC::BLUE), ToC::BLUE);
            assert_eq!(ToC::from(FromC::YELLOW), ToC::YELLOW);
            assert_eq!(ToC::from(FromC::MAGENTA), ToC::MAGENTA);
            assert_eq!(ToC::from(FromC::CYAN), ToC::CYAN);
            assert_eq!(ToC::from(FromC::WHITE), ToC::WHITE);
        }

        type_matrix!(test_rgb_to_rgb; Rgb555, Bgr555, Rgb565, Bgr565, Rgb666, Bgr666, Rgb888, Bgr888);
    }

    #[test]
    fn rgb_to_gray() {
        fn test_rgb_to_gray<FromC: RgbColor + Debug, ToC: GrayColor + From<FromC> + Debug>() {
            assert_eq!(ToC::from(FromC::BLACK), ToC::BLACK);
            assert_eq!(ToC::from(FromC::WHITE), ToC::WHITE);
        }

        type_matrix!(test_rgb_to_gray; Rgb555, Bgr555, Rgb565, Bgr565, Rgb666, Bgr666, Rgb888, Bgr888 => Gray2, Gray4, Gray8);
    }

    #[test]
    fn rgb_to_binary() {
        fn test_rgb_to_binary<FromC: RgbColor + Debug, ToC>()
        where
            BinaryColor: From<FromC>,
        {
            assert_eq!(BinaryColor::from(FromC::BLACK), BinaryColor::Off);
            assert_eq!(BinaryColor::from(FromC::WHITE), BinaryColor::On);
        }

        type_matrix!(test_rgb_to_binary; Rgb555, Bgr555, Rgb565, Bgr565, Rgb666, Bgr666, Rgb888, Bgr888 => BinaryColor);
    }

    #[test]
    fn gray_to_gray() {
        fn test_gray_to_gray<FromC: GrayColor + Debug, ToC: GrayColor + From<FromC> + Debug>() {
            assert_eq!(ToC::from(FromC::BLACK), ToC::BLACK);
            assert_eq!(ToC::from(FromC::WHITE), ToC::WHITE);
        }

        type_matrix!(test_gray_to_gray; Gray2, Gray4, Gray8);
    }

    #[test]
    fn gray_to_rgb() {
        fn test_gray_to_rgb<FromC: GrayColor + Debug, ToC: RgbColor + From<FromC> + Debug>() {
            assert_eq!(ToC::from(FromC::BLACK), ToC::BLACK);
            assert_eq!(ToC::from(FromC::WHITE), ToC::WHITE);
        }

        type_matrix!(test_gray_to_rgb; Gray2, Gray4, Gray8 => Rgb555, Bgr555, Rgb565, Bgr565, Rgb666, Bgr666, Rgb888, Bgr888);
    }

    #[test]
    fn gray_to_binary() {
        fn test_gray_to_binary<FromC: GrayColor + Debug, ToC>()
        where
            BinaryColor: From<FromC>,
        {
            assert_eq!(BinaryColor::from(FromC::BLACK), BinaryColor::Off);
            assert_eq!(BinaryColor::from(FromC::WHITE), BinaryColor::On);
        }

        type_matrix!(test_gray_to_binary; Gray2, Gray4, Gray8 => BinaryColor);
    }

    #[test]
    fn binary_to_rgb() {
        fn test_binary_to_rgb<FromC, ToC: RgbColor + From<BinaryColor> + Debug>() {
            assert_eq!(ToC::from(BinaryColor::Off), ToC::BLACK);
            assert_eq!(ToC::from(BinaryColor::On), ToC::WHITE);
        }

        type_matrix!(test_binary_to_rgb; BinaryColor => Rgb555, Bgr555, Rgb565, Bgr565, Rgb666, Bgr666, Rgb888, Bgr888);
    }

    #[test]
    fn binary_to_gray() {
        fn test_binary_to_gray<FromC, ToC: GrayColor + From<BinaryColor> + Debug>() {
            assert_eq!(ToC::from(BinaryColor::Off), ToC::BLACK);
            assert_eq!(ToC::from(BinaryColor::On), ToC::WHITE);
        }

        type_matrix!(test_binary_to_gray; BinaryColor => Gray2, Gray4, Gray8);
    }

    #[test]
    fn test_luma() {
        assert_eq!(luma(Rgb888::BLACK), 0);
        assert_eq!(luma(Rgb888::WHITE), 255);

        assert_eq!(
            luma(Rgb888::new(255, 255, 254)),
            255,
            "should be rounded upward"
        );
    }

    fn test_channel_conversion<const FROM_MAX: u8, const TO_MAX: u8>() {
        fn convert_channel_reference(value: u8, from_max: u8, to_max: u8) -> u8 {
            ((value as u16 * to_max as u16 + from_max as u16 / 2) / from_max as u16) as u8
        }

        for value in 0..FROM_MAX {
            assert_eq!(
                convert_channel::<FROM_MAX, TO_MAX>(value),
                convert_channel_reference(value, FROM_MAX, TO_MAX),
                "from_max: {}, to_max: {}, value: {}",
                FROM_MAX,
                TO_MAX,
                value,
            );
        }
    }

    const fn bits_to_max(bits: u8) -> u8 {
        0xFF >> (8 - bits)
    }

    #[test]
    fn channel_conversions_larger() {
        test_channel_conversion::<{ bits_to_max(2) }, { bits_to_max(4) }>();
        test_channel_conversion::<{ bits_to_max(2) }, { bits_to_max(5) }>();
        test_channel_conversion::<{ bits_to_max(2) }, { bits_to_max(6) }>();
        test_channel_conversion::<{ bits_to_max(2) }, { bits_to_max(8) }>();

        test_channel_conversion::<{ bits_to_max(4) }, { bits_to_max(5) }>();
        test_channel_conversion::<{ bits_to_max(4) }, { bits_to_max(6) }>();
        test_channel_conversion::<{ bits_to_max(4) }, { bits_to_max(8) }>();

        test_channel_conversion::<{ bits_to_max(5) }, { bits_to_max(6) }>();
        test_channel_conversion::<{ bits_to_max(5) }, { bits_to_max(8) }>();

        test_channel_conversion::<{ bits_to_max(6) }, { bits_to_max(8) }>();
    }

    #[test]
    fn channel_conversions_smaller() {
        test_channel_conversion::<{ bits_to_max(8) }, { bits_to_max(6) }>();
        test_channel_conversion::<{ bits_to_max(8) }, { bits_to_max(5) }>();
        test_channel_conversion::<{ bits_to_max(8) }, { bits_to_max(4) }>();
        test_channel_conversion::<{ bits_to_max(8) }, { bits_to_max(2) }>();

        test_channel_conversion::<{ bits_to_max(6) }, { bits_to_max(5) }>();
        test_channel_conversion::<{ bits_to_max(6) }, { bits_to_max(4) }>();
        test_channel_conversion::<{ bits_to_max(6) }, { bits_to_max(2) }>();

        test_channel_conversion::<{ bits_to_max(5) }, { bits_to_max(4) }>();
        test_channel_conversion::<{ bits_to_max(4) }, { bits_to_max(2) }>();

        test_channel_conversion::<{ bits_to_max(4) }, { bits_to_max(2) }>();
    }

    #[test]
    fn channel_conversions_identity() {
        test_channel_conversion::<{ bits_to_max(8) }, { bits_to_max(8) }>();
        test_channel_conversion::<{ bits_to_max(6) }, { bits_to_max(6) }>();
        test_channel_conversion::<{ bits_to_max(5) }, { bits_to_max(5) }>();
        test_channel_conversion::<{ bits_to_max(4) }, { bits_to_max(4) }>();
        test_channel_conversion::<{ bits_to_max(2) }, { bits_to_max(2) }>();
    }
}
