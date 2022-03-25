use crate::pixelcolor::{
    raw::{RawU1, RawU16, RawU2, RawU24, RawU32, RawU4, RawU8},
    PixelColor,
};

/// Trait to convert colors into a byte array.
///
/// See the [module-level documentation](super#converting-colors-to-raw-data) for an example.
pub trait ToBytes {
    /// Return type of methods in this trait.
    type Bytes;

    /// Converts a color into a byte array with big endian byte order.
    fn to_be_bytes(self) -> Self::Bytes;

    /// Converts a color into a byte array with little endian byte order.
    fn to_le_bytes(self) -> Self::Bytes;

    /// Converts a color into a byte array with native byte order.
    fn to_ne_bytes(self) -> Self::Bytes;
}

macro_rules! impl_to_bytes {
    ($type:ty, $bytes_type:ty) => {
        impl ToBytes for $type {
            type Bytes = $bytes_type;

            fn to_be_bytes(self) -> Self::Bytes {
                self.0.to_be_bytes()
            }

            fn to_le_bytes(self) -> Self::Bytes {
                self.0.to_le_bytes()
            }

            fn to_ne_bytes(self) -> Self::Bytes {
                self.0.to_ne_bytes()
            }
        }
    };
}

impl_to_bytes!(RawU1, [u8; 1]);
impl_to_bytes!(RawU2, [u8; 1]);
impl_to_bytes!(RawU4, [u8; 1]);
impl_to_bytes!(RawU8, [u8; 1]);
impl_to_bytes!(RawU16, [u8; 2]);
impl_to_bytes!(RawU32, [u8; 4]);

impl ToBytes for RawU24 {
    type Bytes = [u8; 3];

    fn to_be_bytes(self) -> Self::Bytes {
        let mut ret = [0; 3];

        ret.copy_from_slice(&self.0.to_be_bytes()[1..4]);

        ret
    }

    fn to_le_bytes(self) -> Self::Bytes {
        let mut ret = [0; 3];

        ret.copy_from_slice(&self.0.to_le_bytes()[0..3]);

        ret
    }

    #[cfg(target_endian = "big")]
    fn to_ne_bytes(self) -> Self::Bytes {
        self.to_be_bytes()
    }

    #[cfg(target_endian = "little")]
    fn to_ne_bytes(self) -> Self::Bytes {
        self.to_le_bytes()
    }
}

impl ToBytes for () {
    type Bytes = [u8; 0];

    fn to_be_bytes(self) -> Self::Bytes {
        []
    }

    fn to_le_bytes(self) -> Self::Bytes {
        []
    }

    fn to_ne_bytes(self) -> Self::Bytes {
        []
    }
}

impl<C> ToBytes for C
where
    C: PixelColor + Into<<C as PixelColor>::Raw>,
{
    type Bytes = <<C as PixelColor>::Raw as ToBytes>::Bytes;

    fn to_le_bytes(self) -> Self::Bytes {
        self.into().to_le_bytes()
    }

    fn to_be_bytes(self) -> Self::Bytes {
        self.into().to_be_bytes()
    }

    fn to_ne_bytes(self) -> Self::Bytes {
        self.into().to_ne_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelcolor::{
        Bgr565, Bgr666, Bgr888, BinaryColor, Gray2, Gray4, Gray8, Rgb565, Rgb666, Rgb888,
    };

    fn assert_all_orders<T>(value: T, bytes: T::Bytes)
    where
        T: ToBytes + Copy,
        T::Bytes: PartialEq + core::fmt::Debug,
    {
        assert_eq!(value.to_le_bytes(), bytes);
        assert_eq!(value.to_be_bytes(), bytes);
        assert_eq!(value.to_ne_bytes(), bytes);
    }

    #[test]
    fn bpp1() {
        assert_all_orders(BinaryColor::Off, [0]);
        assert_all_orders(BinaryColor::On, [1]);
    }

    #[test]
    fn bpp2() {
        assert_all_orders(Gray2::new(0), [0]);
        assert_all_orders(Gray2::new(3), [3]);
    }

    #[test]
    fn bpp4() {
        assert_all_orders(Gray4::new(0), [0]);
        assert_all_orders(Gray4::new(15), [15]);
    }

    #[test]
    fn bpp8() {
        assert_all_orders(Gray8::new(0), [0]);
        assert_all_orders(Gray8::new(255), [255]);
    }

    #[test]
    fn bpp16_rgb_be() {
        assert_eq!(
            Rgb565::new(255, 0, 0).to_be_bytes(),
            [0b11111_000, 0b000_00000]
        );
        assert_eq!(
            Rgb565::new(0, 255, 0).to_be_bytes(),
            [0b00000_111, 0b111_00000]
        );
        assert_eq!(
            Rgb565::new(0, 0, 255).to_be_bytes(),
            [0b00000_000, 0b000_11111]
        );
    }

    #[test]
    fn bpp16_rgb_le() {
        assert_eq!(
            Rgb565::new(255, 0, 0).to_le_bytes(),
            [0b000_00000, 0b11111_000]
        );
        assert_eq!(
            Rgb565::new(0, 255, 0).to_le_bytes(),
            [0b111_00000, 0b00000_111]
        );
        assert_eq!(
            Rgb565::new(0, 0, 255).to_le_bytes(),
            [0b000_11111, 0b00000_000]
        );
    }

    #[test]
    fn bpp16_bgr_be() {
        assert_eq!(
            Bgr565::new(255, 0, 0).to_be_bytes(),
            [0b00000_000, 0b000_11111]
        );
        assert_eq!(
            Bgr565::new(0, 255, 0).to_be_bytes(),
            [0b00000_111, 0b111_00000]
        );
        assert_eq!(
            Bgr565::new(0, 0, 255).to_be_bytes(),
            [0b11111_000, 0b000_00000]
        );
    }

    #[test]
    fn bpp16_bgr_le() {
        assert_eq!(
            Bgr565::new(255, 0, 0).to_le_bytes(),
            [0b000_11111, 0b00000_000]
        );
        assert_eq!(
            Bgr565::new(0, 255, 0).to_le_bytes(),
            [0b111_00000, 0b00000_111]
        );
        assert_eq!(
            Bgr565::new(0, 0, 255).to_le_bytes(),
            [0b000_00000, 0b11111_000]
        );
    }

    #[test]
    fn bpp18_bgr_be() {
        assert_eq!(
            Bgr666::new(0xFF, 0x00, 0x00).to_be_bytes(),
            [0b000000000, 0b000000000, 0b00_111111]
        );
        assert_eq!(
            Bgr666::new(0x0, 0xFF, 0x00).to_be_bytes(),
            [0b000000000, 0b0000_1111, 0b11_000000]
        );
        assert_eq!(
            Bgr666::new(0x00, 0x00, 0xFF).to_be_bytes(),
            [0b0000000_11, 0b1111_0000, 0b000000000]
        );
    }

    #[test]
    fn bpp18_bgr_le() {
        assert_eq!(
            Bgr666::new(0xFF, 0x00, 0x00).to_le_bytes(),
            [0b00_111111, 0b000000000, 0b000000000]
        );
        assert_eq!(
            Bgr666::new(0x0, 0xFF, 0x00).to_le_bytes(),
            [0b11_000000, 0b0000_1111, 0b000000000]
        );
        assert_eq!(
            Bgr666::new(0x00, 0x00, 0xFF).to_le_bytes(),
            [0b000000000, 0b1111_0000, 0b0000000_11]
        );
    }

    #[test]
    fn bpp18_rgb_be() {
        assert_eq!(
            Rgb666::new(0xFF, 0x00, 0x00).to_be_bytes(),
            [0b0000000_11, 0b1111_0000, 0b000000000]
        );
        assert_eq!(
            Rgb666::new(0x0, 0xFF, 0x00).to_be_bytes(),
            [0b000000000, 0b0000_1111, 0b11_000000]
        );
        assert_eq!(
            Rgb666::new(0x00, 0x00, 0xFF).to_be_bytes(),
            [0b000000000, 0b000000000, 0b00_111111]
        );
    }

    #[test]
    fn bpp18_rgb_le() {
        assert_eq!(
            Rgb666::new(0xFF, 0x00, 0x00).to_le_bytes(),
            [0b000000000, 0b1111_0000, 0b0000000_11]
        );
        assert_eq!(
            Rgb666::new(0x0, 0xFF, 0x00).to_le_bytes(),
            [0b11_000000, 0b0000_1111, 0b000000000]
        );
        assert_eq!(
            Rgb666::new(0x00, 0x00, 0xFF).to_le_bytes(),
            [0b00_111111, 0b000000000, 0b000000000]
        );
    }

    #[test]
    fn bpp24_rgb_be() {
        assert_eq!(
            Rgb888::new(0xFF, 0x00, 0x00).to_be_bytes(),
            [0xFF, 0x00, 0x00]
        );
        assert_eq!(
            Rgb888::new(0x00, 0xFF, 0x00).to_be_bytes(),
            [0x00, 0xFF, 0x00]
        );
        assert_eq!(
            Rgb888::new(0x00, 0x00, 0xFF).to_be_bytes(),
            [0x00, 0x00, 0xFF]
        );
    }

    #[test]
    fn bpp24_rgb_le() {
        assert_eq!(
            Rgb888::new(0xFF, 0x00, 0x00).to_le_bytes(),
            [0x00, 0x00, 0xFF]
        );
        assert_eq!(
            Rgb888::new(0x00, 0xFF, 0x00).to_le_bytes(),
            [0x00, 0xFF, 0x00]
        );
        assert_eq!(
            Rgb888::new(0x00, 0x00, 0xFF).to_le_bytes(),
            [0xFF, 0x00, 0x00]
        );
    }

    #[test]
    fn bpp24_bgr_be() {
        assert_eq!(
            Bgr888::new(0xFF, 0x00, 0x00).to_be_bytes(),
            [0x00, 0x00, 0xFF]
        );
        assert_eq!(
            Bgr888::new(0x00, 0xFF, 0x00).to_be_bytes(),
            [0x00, 0xFF, 0x00]
        );
        assert_eq!(
            Bgr888::new(0x00, 0x00, 0xFF).to_be_bytes(),
            [0xFF, 0x00, 0x00]
        );
    }

    #[test]
    fn bpp24_bgr_le() {
        assert_eq!(
            Bgr888::new(0xFF, 0x00, 0x00).to_le_bytes(),
            [0xFF, 0x00, 0x00]
        );
        assert_eq!(
            Bgr888::new(0x00, 0xFF, 0x00).to_le_bytes(),
            [0x00, 0xFF, 0x00]
        );
        assert_eq!(
            Bgr888::new(0x00, 0x00, 0xFF).to_le_bytes(),
            [0x00, 0x00, 0xFF]
        );
    }

    #[test]
    fn bpp32_be() {
        // This test uses `RawU32` instead of a color, because no color included
        // in this crate uses 32 bpp.
        assert_eq!(
            RawU32::new(0x11223344).to_be_bytes(),
            [0x11, 0x22, 0x33, 0x44]
        );
    }

    #[test]
    fn bpp32_le() {
        // This test uses `RawU32` instead of a color, because no color included
        // in this crate uses 32 bpp.
        assert_eq!(
            RawU32::new(0x11223344).to_le_bytes(),
            [0x44, 0x33, 0x22, 0x11]
        );
    }

    #[test]
    fn native_byte_ordering() {
        #[cfg(target_endian = "big")]
        {
            assert_eq!(RawU1::new(0x1).to_ne_bytes(), RawU1::new(0x1).to_be_bytes());
            assert_eq!(RawU2::new(0x1).to_ne_bytes(), RawU2::new(0x1).to_be_bytes());
            assert_eq!(RawU4::new(0x1).to_ne_bytes(), RawU4::new(0x1).to_be_bytes());
            assert_eq!(
                RawU8::new(0x12).to_ne_bytes(),
                RawU8::new(0x12).to_be_bytes()
            );
            assert_eq!(
                RawU16::new(0x1234).to_ne_bytes(),
                RawU16::new(0x1234).to_be_bytes()
            );
            assert_eq!(
                RawU24::new(0x123456).to_ne_bytes(),
                RawU24::new(0x123456).to_be_bytes()
            );
            assert_eq!(
                RawU32::new(0x12345678).to_ne_bytes(),
                RawU32::new(0x12345678).to_be_bytes()
            );
        }

        #[cfg(target_endian = "little")]
        {
            assert_eq!(RawU1::new(0x1).to_ne_bytes(), RawU1::new(0x1).to_le_bytes());
            assert_eq!(RawU2::new(0x1).to_ne_bytes(), RawU2::new(0x1).to_le_bytes());
            assert_eq!(RawU4::new(0x1).to_ne_bytes(), RawU4::new(0x1).to_le_bytes());
            assert_eq!(
                RawU8::new(0x12).to_ne_bytes(),
                RawU8::new(0x12).to_le_bytes()
            );
            assert_eq!(
                RawU16::new(0x1234).to_ne_bytes(),
                RawU16::new(0x1234).to_le_bytes()
            );
            assert_eq!(
                RawU24::new(0x123456).to_ne_bytes(),
                RawU24::new(0x123456).to_le_bytes()
            );
            assert_eq!(
                RawU32::new(0x12345678).to_ne_bytes(),
                RawU32::new(0x12345678).to_le_bytes()
            );
        }
    }
}
