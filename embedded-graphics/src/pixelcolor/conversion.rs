use crate::pixelcolor::{binary_color::*, gray_color::*, rgb_color::*};

/// Convert color channel values from one bit depth to another.
pub const fn convert_channel(value: u8, from_max: u8, to_max: u8) -> u8 {
    ((value as u16 * to_max as u16 + from_max as u16 / 2) / from_max as u16) as u8
}

/// Macro to implement conversion between RGB color types.
macro_rules! impl_rgb_conversion {
    ($type:ident, ($($other_type:ident),+)) => {
        $(
            impl From<$other_type> for $type {
                fn from(other: $other_type) -> Self {
                    Self::new(
                        convert_channel(other.r(), $other_type::MAX_R, $type::MAX_R),
                        convert_channel(other.g(), $other_type::MAX_G, $type::MAX_G),
                        convert_channel(other.b(), $other_type::MAX_B, $type::MAX_B),
                    )
                }
            }
        )*

        impl $type {
            pub(crate) const fn with_rgb888(r: u8, g: u8, b: u8) -> Self {
                Self::new(
                    convert_channel(r, Rgb888::MAX_R, $type::MAX_R),
                    convert_channel(g, Rgb888::MAX_G, $type::MAX_G),
                    convert_channel(b, Rgb888::MAX_B, $type::MAX_B),
                )
            }
        }
    };
}

impl_rgb_conversion!(Rgb555, (Bgr555, Rgb565, Bgr565, Rgb888, Bgr888));
impl_rgb_conversion!(Bgr555, (Rgb555, Rgb565, Bgr565, Rgb888, Bgr888));
impl_rgb_conversion!(Rgb565, (Rgb555, Bgr555, Bgr565, Rgb888, Bgr888));
impl_rgb_conversion!(Bgr565, (Rgb555, Bgr555, Rgb565, Rgb888, Bgr888));
impl_rgb_conversion!(Rgb888, (Rgb555, Bgr555, Rgb565, Bgr565, Bgr888));
impl_rgb_conversion!(Bgr888, (Rgb555, Bgr555, Rgb565, Bgr565, Rgb888));

/// Macro to implement conversions from `GrayX` to RGB color types.
macro_rules! impl_from_gray {
    ($($gray_type:ident),+ => $rgb_type:ident) => {
        $(impl From<$gray_type> for $rgb_type {
            fn from(other: $gray_type) -> Self {
                Self::new(
                    convert_channel(other.luma(), <$gray_type>::WHITE.luma(), $rgb_type::MAX_R),
                    convert_channel(other.luma(), <$gray_type>::WHITE.luma(), $rgb_type::MAX_G),
                    convert_channel(other.luma(), <$gray_type>::WHITE.luma(), $rgb_type::MAX_B),
                )
            }
        })+
    };
    ($($gray_type:ident),+ => $rgb_type:ident, $($rest:ident),+) => {
        impl_from_gray!($($gray_type),+ => $rgb_type);
        impl_from_gray!($($gray_type),+ => $($rest),*);
    }
}

impl_from_gray!(Gray2, Gray4, Gray8 => Rgb555, Bgr555, Rgb565, Bgr565, Rgb888, Bgr888);

/// Macro to implement conversion from `BinaryColor` to RGB and grayscale types.
macro_rules! impl_from_binary {
    ($type:ident) => {
        // Convert BinaryColor::Off to black and BinaryColor::On to white
        impl From<BinaryColor> for $type {
            fn from(color: BinaryColor) -> Self {
                color.map_color(Self::BLACK, Self::WHITE)
            }
        }
    };
}

impl_from_binary!(Rgb555);
impl_from_binary!(Bgr555);
impl_from_binary!(Rgb565);
impl_from_binary!(Bgr565);
impl_from_binary!(Rgb888);
impl_from_binary!(Bgr888);
impl_from_binary!(Gray2);
impl_from_binary!(Gray4);
impl_from_binary!(Gray8);

#[cfg(test)]
mod tests {
    use super::*;

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

    macro_rules! test_rgb_conversions {
        ($from_type:ident, ($($to_type:ident),+)) => {
            $(
                assert_eq!($to_type::from($from_type::BLACK), $to_type::BLACK);
                assert_eq!($to_type::from($from_type::RED), $to_type::RED);
                assert_eq!($to_type::from($from_type::GREEN), $to_type::GREEN);
                assert_eq!($to_type::from($from_type::BLUE), $to_type::BLUE);
                assert_eq!($to_type::from($from_type::YELLOW), $to_type::YELLOW);
                assert_eq!($to_type::from($from_type::MAGENTA), $to_type::MAGENTA);
                assert_eq!($to_type::from($from_type::CYAN), $to_type::CYAN);
                assert_eq!($to_type::from($from_type::WHITE), $to_type::WHITE);
            )*
        };

        ($from_type:ident) => {
            test_rgb_conversions!($from_type, (Rgb555, Bgr555, Rgb565, Bgr565, Rgb888, Bgr888));
        };
    }

    #[test]
    fn rgb_color_conversions() {
        test_rgb_conversions!(Rgb555);
        test_rgb_conversions!(Bgr555);
        test_rgb_conversions!(Rgb565);
        test_rgb_conversions!(Bgr565);
        test_rgb_conversions!(Rgb888);
        test_rgb_conversions!(Bgr888);
    }

    macro_rules! test_rgb_from_gray {
        ($rgb_type:ident, $($gray_type:ident),+) => {
            $(
                assert_eq!($rgb_type::from(<$gray_type>::BLACK), $rgb_type::BLACK);
                assert_eq!($rgb_type::from(<$gray_type>::WHITE), $rgb_type::WHITE);
            )+
        };
        ($rgb_type:ident) => {
            test_rgb_from_gray!($rgb_type, Gray2, Gray4, Gray8);
        }
    }

    #[test]
    fn rgb_from_gray() {
        test_rgb_from_gray!(Rgb555);
        test_rgb_from_gray!(Bgr555);
        test_rgb_from_gray!(Rgb565);
        test_rgb_from_gray!(Bgr565);
        test_rgb_from_gray!(Rgb888);
        test_rgb_from_gray!(Bgr888);
    }

    macro_rules! test_from_binary {
        ($type:ident) => {
            assert_eq!($type::from(BinaryColor::Off), $type::BLACK);
            assert_eq!($type::from(BinaryColor::On), $type::WHITE);
        };
    }

    #[test]
    pub fn conversion_from_binary_color() {
        test_from_binary!(Rgb555);
        test_from_binary!(Bgr555);
        test_from_binary!(Rgb565);
        test_from_binary!(Bgr565);
        test_from_binary!(Rgb888);
        test_from_binary!(Bgr888);
        test_from_binary!(Gray2);
        test_from_binary!(Gray4);
        test_from_binary!(Gray8);
    }
}
