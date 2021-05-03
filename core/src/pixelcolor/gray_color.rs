use crate::pixelcolor::{
    raw::{RawData, RawU2, RawU4, RawU8},
    PixelColor,
};

/// Grayscale color.
pub trait GrayColor: PixelColor {
    /// Returns the luma channel value.
    fn luma(&self) -> u8;

    /// Black color (0% luma).
    const BLACK: Self;

    /// White color (100% luma).
    const WHITE: Self;
}

macro_rules! gray_color {
    ($type:ident, $raw_type:ident, $bpp_str:expr) => {
        #[doc = $bpp_str]
        #[doc = "grayscale color."]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
        pub struct $type($raw_type);

        impl $type {
            pub(crate) const GRAY_50: Self = Self::new(0x80 >> (8 - $raw_type::BITS_PER_PIXEL));

            /// Creates a new grayscale color.
            ///
            /// Too large luma values are masked to the valid range by setting
            /// the upper bits to `0`.
            pub const fn new(luma: u8) -> Self {
                Self($raw_type::new(luma))
            }
        }

        impl PixelColor for $type {
            type Raw = $raw_type;
        }

        impl GrayColor for $type {
            fn luma(&self) -> u8 {
                self.0.into_inner()
            }

            const BLACK: Self = Self::new(0);
            const WHITE: Self = Self::new(255);
        }

        impl From<$raw_type> for $type {
            fn from(data: $raw_type) -> Self {
                Self(data)
            }
        }

        impl From<$type> for $raw_type {
            fn from(color: $type) -> Self {
                color.0
            }
        }
    };
}

gray_color!(Gray2, RawU2, "2 bit");
gray_color!(Gray4, RawU4, "4 bit");
gray_color!(Gray8, RawU8, "8 bit");

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelcolor::IntoStorage;

    #[test]
    pub fn new_masks_luma() {
        assert_eq!(Gray2::new(255).luma(), 0x3);
        assert_eq!(Gray4::new(255).luma(), 0xF);
        assert_eq!(Gray8::new(255).luma(), 0xFF);
    }

    #[test]
    pub fn color_constants() {
        assert_eq!(Gray2::BLACK.luma(), 0);
        assert_eq!(Gray4::BLACK.luma(), 0);
        assert_eq!(Gray8::BLACK.luma(), 0);

        assert_eq!(Gray2::GRAY_50.luma(), 0x2);
        assert_eq!(Gray4::GRAY_50.luma(), 0x8);
        assert_eq!(Gray8::GRAY_50.luma(), 0x80);

        assert_eq!(Gray2::WHITE.luma(), 0x3);
        assert_eq!(Gray4::WHITE.luma(), 0xF);
        assert_eq!(Gray8::WHITE.luma(), 0xFF);
    }

    #[test]
    pub fn from_data() {
        assert_eq!(Gray2::from(RawU2::new(0x2)), Gray2::new(0x2));
        assert_eq!(Gray4::from(RawU4::new(0x9)), Gray4::new(0x9));
        assert_eq!(Gray8::from(RawU8::new(0x81)), Gray8::new(0x81));
    }

    #[test]
    pub fn into_data() {
        assert_eq!(RawU2::from(Gray2::new(0x1)), RawU2::new(0x1));
        assert_eq!(RawU4::from(Gray4::new(0x6)), RawU4::new(0x6));
        assert_eq!(RawU8::from(Gray8::new(0x7E)), RawU8::new(0x7E));
    }

    #[test]
    fn convert_to_raw() {
        let color = Gray8::new(0xAA);

        assert_eq!(color.into_storage(), 0xAAu8);
    }
}
