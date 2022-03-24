use embedded_graphics_core::pixelcolor::{
    Bgr555, Bgr565, Bgr888, BinaryColor, Gray2, Gray4, Gray8, GrayColor, Rgb555, Rgb565, Rgb888,
    RgbColor, WebColors,
};

/// Mapping between `char`s and colors.
///
/// See the [module-level documentation](super) for a table of implemented mappings.
pub trait ColorMapping: Into<Rgb888> {
    /// Color used to display `None` values when `EG_FANCY_PANIC` is enabled.
    ///
    /// This color must be set to a color that isn't available in normal patterns to make it
    /// distinguishable in the output. For non grayscale colors the default value should be used.
    const NONE_COLOR: Rgb888 = Rgb888::new(128, 128, 128);

    /// Converts a char into a color of type `C`.
    fn char_to_color(c: char) -> Self;

    /// Converts a color of type `C` into a char.
    fn color_to_char(color: Self) -> char;
}

impl ColorMapping for BinaryColor {
    fn char_to_color(c: char) -> Self {
        match c {
            '.' => BinaryColor::Off,
            '#' => BinaryColor::On,
            _ => panic!("Invalid char in pattern: '{}'", c),
        }
    }

    fn color_to_char(color: Self) -> char {
        match color {
            BinaryColor::Off => '.',
            BinaryColor::On => '#',
        }
    }
}

macro_rules! impl_gray_color_mapping {
    ($type:ident, $radix:expr) => {
        impl ColorMapping for $type {
            const NONE_COLOR: Rgb888 = Rgb888::CSS_STEEL_BLUE;

            fn char_to_color(c: char) -> Self {
                if let Some(digit) = c.to_digit($radix) {
                    Self::new(digit as u8)
                } else {
                    panic!("invalid char in pattern: '{}'", c)
                }
            }

            fn color_to_char(color: Self) -> char {
                core::char::from_digit(color.luma() as u32, $radix)
                    .unwrap()
                    .to_ascii_uppercase()
            }
        }
    };
}

impl_gray_color_mapping!(Gray2, 4);
impl_gray_color_mapping!(Gray4, 16);

impl ColorMapping for Gray8 {
    const NONE_COLOR: Rgb888 = Rgb888::CSS_STEEL_BLUE;

    fn char_to_color(c: char) -> Self {
        if let Some(digit) = c.to_digit(16) {
            Self::new(digit as u8 * 0x11)
        } else {
            panic!("invalid char in pattern: '{}'", c);
        }
    }

    fn color_to_char(color: Self) -> char {
        let luma = color.luma();
        let lower = luma & 0xF;
        let upper = luma >> 4;

        if lower != upper {
            '?'
        } else {
            core::char::from_digit(lower as u32, 16)
                .unwrap()
                .to_ascii_uppercase()
        }
    }
}

macro_rules! impl_rgb_color_mapping {
    ($type:ident) => {
        impl ColorMapping for $type {
            fn char_to_color(c: char) -> Self {
                match c {
                    'K' => Self::BLACK,
                    'R' => Self::RED,
                    'G' => Self::GREEN,
                    'B' => Self::BLUE,
                    'Y' => Self::YELLOW,
                    'M' => Self::MAGENTA,
                    'C' => Self::CYAN,
                    'W' => Self::WHITE,
                    _ => panic!("Invalid char in pattern: '{}'", c),
                }
            }

            fn color_to_char(color: Self) -> char {
                match color {
                    Self::BLACK => 'K',
                    Self::RED => 'R',
                    Self::GREEN => 'G',
                    Self::BLUE => 'B',
                    Self::YELLOW => 'Y',
                    Self::MAGENTA => 'M',
                    Self::CYAN => 'C',
                    Self::WHITE => 'W',
                    _ => '?',
                }
            }
        }
    };
}

impl_rgb_color_mapping!(Rgb555);
impl_rgb_color_mapping!(Bgr555);
impl_rgb_color_mapping!(Rgb565);
impl_rgb_color_mapping!(Bgr565);
impl_rgb_color_mapping!(Rgb888);
impl_rgb_color_mapping!(Bgr888);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gray2_mapping() {
        for luma in 0..4 {
            let color = Gray2::new(luma);

            assert_eq!(color, Gray2::char_to_color(Gray2::color_to_char(color)));
        }
    }

    #[test]
    fn gray4_mapping() {
        for luma in 0..16 {
            let color = Gray4::new(luma);

            assert_eq!(color, Gray4::char_to_color(Gray4::color_to_char(color)));
        }
    }

    #[test]
    fn gray8_mapping() {
        for luma in 0..16 {
            let color = Gray8::new(luma * 0x11);

            assert_eq!(color, Gray8::char_to_color(Gray8::color_to_char(color)));
        }
    }

    #[test]
    #[should_panic(expected = "invalid char in pattern: '4'")]
    fn invalid_gray2_char_4() {
        Gray2::char_to_color('4');
    }

    #[test]
    #[should_panic(expected = "invalid char in pattern: 'A'")]
    fn invalid_gray2_char_a() {
        Gray2::char_to_color('A');
    }

    #[test]
    #[should_panic(expected = "invalid char in pattern: 'G'")]
    fn invalid_gray4_char_g() {
        Gray2::char_to_color('G');
    }

    #[test]
    #[should_panic(expected = "invalid char in pattern: 'G'")]
    fn invalid_gray8_char_g() {
        Gray8::char_to_color('G');
    }
}
