use super::gray_color::*;
use super::rgb_color::*;

/// Convert color channel values from one bitdepth to another.
const fn convert_channel(value: u8, from_max: u8, to_max: u8) -> u8 {
    ((value as u16 * to_max as u16 + from_max as u16 / 2) / from_max as u16) as u8
}

/// Macro to implement conversion between RGB color types.
macro_rules! impl_rgb_conversion {
    ($type: ident, ($($other_type: ident),+)) => {
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
    };
}

impl_rgb_conversion!(Rgb555, (Bgr555, Rgb565, Bgr565, Rgb888, Bgr888));
impl_rgb_conversion!(Bgr555, (Rgb555, Rgb565, Bgr565, Rgb888, Bgr888));
impl_rgb_conversion!(Rgb565, (Rgb555, Bgr555, Bgr565, Rgb888, Bgr888));
impl_rgb_conversion!(Bgr565, (Rgb555, Bgr555, Rgb565, Rgb888, Bgr888));
impl_rgb_conversion!(Rgb888, (Rgb555, Bgr555, Rgb565, Bgr565, Bgr888));
impl_rgb_conversion!(Bgr888, (Rgb555, Bgr555, Rgb565, Bgr565, Rgb888));

/// Macro to implement conversions between `Gray8` and RGB color types.
macro_rules! impl_gray8_conversion {
    ($type:ident) => {
        impl From<Gray8> for $type {
            fn from(other: Gray8) -> Self {
                Self::new(
                    convert_channel(other.y(), Gray8::MAX_Y, $type::MAX_R),
                    convert_channel(other.y(), Gray8::MAX_Y, $type::MAX_G),
                    convert_channel(other.y(), Gray8::MAX_Y, $type::MAX_B),
                )
            }
        }

        // Convert RGB colors to grayscale by calculating the HSI intensity.
        impl From<$type> for Gray8 {
            fn from(other: $type) -> Self {
                let other: Rgb888 = other.into();
                let sum: u16 = other.r() as u16 + other.g() as u16 + other.b() as u16;
                Gray8::new((sum / 3) as u8)
            }
        }
    };
}

impl_gray8_conversion!(Rgb555);
impl_gray8_conversion!(Bgr555);
impl_gray8_conversion!(Rgb565);
impl_gray8_conversion!(Bgr565);
impl_gray8_conversion!(Rgb888);
impl_gray8_conversion!(Bgr888);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_rgb_conversions {
        ($from_type: ident, ($($to_type: ident),+)) => {
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

        ($from_type: ident) => {
            test_rgb_conversions!($from_type, (Rgb555, Bgr555, Rgb565, Bgr565, Rgb888, Bgr888));
        };
    }

    #[test]
    fn rgb_color_constant_conversions() {
        test_rgb_conversions!(Rgb555);
        test_rgb_conversions!(Bgr555);
        test_rgb_conversions!(Rgb565);
        test_rgb_conversions!(Bgr565);
        test_rgb_conversions!(Rgb888);
        test_rgb_conversions!(Bgr888);
    }

    macro_rules! test_grayscale_conversions {
        ($type:ident) => {
            // convert Gray8 to RGB
            assert_eq!($type::from(Gray8::BLACK), $type::BLACK);
            assert_eq!($type::from(Gray8::WHITE), $type::WHITE);

            // convert RGB to Gray8
            assert_eq!(Gray8::from($type::BLACK), Gray8::BLACK);
            assert_eq!(Gray8::from($type::WHITE), Gray8::WHITE);
            assert_eq!(Gray8::from($type::RED), Gray8::new(255 / 3));
            assert_eq!(Gray8::from($type::YELLOW), Gray8::new(255 / 3 * 2));
        };
    }

    #[test]
    fn grayscale_conversions() {
        test_grayscale_conversions!(Rgb555);
        test_grayscale_conversions!(Bgr555);
        test_grayscale_conversions!(Rgb565);
        test_grayscale_conversions!(Bgr565);
        test_grayscale_conversions!(Rgb888);
        test_grayscale_conversions!(Bgr888);
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
}
