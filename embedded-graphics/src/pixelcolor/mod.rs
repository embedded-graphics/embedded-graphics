//! Pixel color

use core::fmt;

/// Pixel color trait
pub trait PixelColor: Clone + Copy + PartialEq + fmt::Debug {
    /// Default background color
    const DEFAULT_BG: Self;

    /// Default foreground color
    const DEFAULT_FG: Self;
}

/// from slice trait
pub trait FromSlice {
    /// create color from big endian data
    fn from_be_slice(data: &[u8]) -> Self;

    /// create color from little endian data
    fn from_le_slice(data: &[u8]) -> Self;
}

/// RGB color
pub trait RgbColor {
    /// red channel value
    fn r(&self) -> u8;

    /// green channel value
    fn g(&self) -> u8;

    /// blue channel value
    fn b(&self) -> u8;

    /// maximum value in red channel
    const MAX_R: u8;

    /// maximum value in red channel
    const MAX_G: u8;

    /// maximum value in red channel
    const MAX_B: u8;

    /// black
    const BLACK: Self;

    /// red
    const RED: Self;

    /// green
    const GREEN: Self;

    /// blue
    const BLUE: Self;

    /// yellow
    const YELLOW: Self;

    /// magenta
    const MAGENTA: Self;

    /// cyan
    const CYAN: Self;

    /// white
    const WHITE: Self;
}

macro_rules! rgb_color {
    ($name: ident, $type: ident, $r_mask: expr, $g_mask: expr, $b_mask: expr, $r_pos: expr, $g_pos: expr, $b_pos: expr) => {
        /// Color
        #[derive(Clone, Copy, PartialEq)]
        pub struct $name($type);

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "{}(r: {}, g: {}, b: {})",
                    stringify!($name),
                    self.r(),
                    self.g(),
                    self.b()
                )
            }
        }
        impl $name {
            /// New
            pub const fn new(r: u8, g: u8, b: u8) -> Self {
                #![allow(trivial_numeric_casts)]

                Self(
                    ((r as $type & $r_mask) << $r_pos)
                        | ((g as $type & $g_mask) << $g_pos)
                        | ((b as $type & $b_mask) << $b_pos),
                )
            }
        }

        impl PixelColor for $name {
            const DEFAULT_BG: Self = Self::BLACK;
            const DEFAULT_FG: Self = Self::WHITE;
        }

        impl RgbColor for $name {
            fn r(&self) -> u8 {
                #![allow(trivial_numeric_casts)]

                ((self.0 >> $r_pos) & $r_mask) as u8
            }

            fn g(&self) -> u8 {
                #![allow(trivial_numeric_casts)]

                ((self.0 >> $g_pos) & $g_mask) as u8
            }

            fn b(&self) -> u8 {
                #![allow(trivial_numeric_casts)]

                ((self.0 >> $b_pos) & $b_mask) as u8
            }

            const MAX_R: u8 = $r_mask;
            const MAX_G: u8 = $g_mask;
            const MAX_B: u8 = $b_mask;

            const BLACK: Self = Self::new(0, 0, 0);
            const RED: Self = Self::new($r_mask, 0, 0);
            const GREEN: Self = Self::new(0, $g_mask, 0);
            const BLUE: Self = Self::new(0, 0, $b_mask);
            const YELLOW: Self = Self::new($r_mask, $g_mask, 0);
            const MAGENTA: Self = Self::new($r_mask, 0, $b_mask);
            const CYAN: Self = Self::new(0, $g_mask, $b_mask);
            const WHITE: Self = Self::new($r_mask, $g_mask, $b_mask);
        }

        impl From<$name> for $type {
            fn from(color: $name) -> Self {
                color.0
            }
        }
    };
}

macro_rules! from_slice_u16 {
    ($type: ident) => {
        impl FromSlice for $type {
            fn from_le_slice(data: &[u8]) -> Self {
                let mut value = data[0] as u16;
                value += (data[1] as u16) << 8;

                Self(value)
            }

            fn from_be_slice(data: &[u8]) -> Self {
                let mut value = data[1] as u16;
                value += (data[0] as u16) << 8;

                Self(value)
            }
        }
    };
}

macro_rules! from_slice_u32 {
    ($type: ident) => {
        impl FromSlice for $type {
            fn from_le_slice(data: &[u8]) -> Self {
                if data.len() >= 4 {
                    let mut value = data[0] as u32;
                    value += (data[1] as u32) << 8;
                    value += (data[2] as u32) << 16;
                    value += (data[3] as u32) << 24;

                    Self(value & 0xFFFFFF)
                } else if data.len() == 3 {
                    let mut value = data[0] as u32;
                    value += (data[1] as u32) << 8;
                    value += (data[2] as u32) << 16;

                    Self(value)
                } else {
                    panic!("slice to short")
                }
            }

            fn from_be_slice(data: &[u8]) -> Self {
                if data.len() >= 4 {
                    let mut value = data[3] as u32;
                    value += (data[2] as u32) << 8;
                    value += (data[1] as u32) << 16;
                    value += (data[0] as u32) << 24;

                    Self(value & 0xFFFFFF)
                } else if data.len() == 3 {
                    let mut value = data[2] as u32;
                    value += (data[1] as u32) << 8;
                    value += (data[0] as u32) << 16;

                    Self(value)
                } else {
                    panic!("slice to short")
                }
            }
        }
    };
}

/// Rgb555
rgb_color!(Rgb555, u16, 0x1F, 0x1F, 0x1F, 10, 5, 0);
from_slice_u16!(Rgb555);

/// Bgr555
rgb_color!(Bgr555, u16, 0x1F, 0x1F, 0x1F, 10, 5, 0);
from_slice_u16!(Bgr555);

/// Rgb565
rgb_color!(Rgb565, u16, 0x1F, 0x3F, 0x1F, 11, 5, 0);
from_slice_u16!(Rgb565);

/// Bgr565
rgb_color!(Bgr565, u16, 0x1F, 0x3F, 0x1F, 0, 5, 11);
from_slice_u16!(Bgr565);

/// Rgb888
rgb_color!(Rgb888, u32, 0xFF, 0xFF, 0xFF, 16, 8, 0);
from_slice_u32!(Rgb888);

/// Bgr888
rgb_color!(Bgr888, u32, 0xFF, 0xFF, 0xFF, 0, 8, 16);
from_slice_u32!(Bgr888);

/// Binary color
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryColor {
    /// inactive pixel
    Off,

    /// active pixel
    On,
}

impl PixelColor for BinaryColor {
    const DEFAULT_BG: BinaryColor = BinaryColor::Off;
    const DEFAULT_FG: BinaryColor = BinaryColor::On;
}

impl From<Rgb565> for Rgb888 {
    fn from(color: Rgb565) -> Self {
        //TODO: add rounding
        let r = color.r() as u16 * 255 / 31;
        let g = color.g() as u16 * 255 / 63;
        let b = color.b() as u16 * 255 / 31;

        Self::new(r as u8, g as u8, b as u8)
    }
}

impl From<Rgb888> for Rgb565 {
    fn from(color: Rgb888) -> Self {
        //TODO: add rounding
        let r = color.r() >> 3;
        let g = color.g() >> 2;
        let b = color.b() >> 3;

        Self::new(r, g, b)
    }
}

//TODO: add more conversions

/// 8bit luma
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Y8(u8);

impl Y8 {
    /// new
    pub const fn new(value: u8) -> Self {
        Self(value)
    }

    /// luminance
    pub fn y(&self) -> u8 {
        self.0
    }

    /// black
    pub const BLACK: Y8 = Self::new(0);

    /// white
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let c = Rgb565::new(1, 2, 3);
        assert_eq!(c.r(), 1);
        assert_eq!(c.g(), 2);
        assert_eq!(c.b(), 3);

        let c = Bgr565::new(1, 2, 3);
        assert_eq!(c.r(), 1);
        assert_eq!(c.g(), 2);
        assert_eq!(c.b(), 3);
    }

    #[test]
    fn convert_rgb565_to_rgb888() {
        assert_eq!(Rgb888::from(Rgb565::BLACK), Rgb888::BLACK);
        assert_eq!(Rgb888::from(Rgb565::WHITE), Rgb888::WHITE);
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

    //TODO: add additional tests
}
