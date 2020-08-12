use crate::pixelcolor::{
    raw::{RawU1, RawU16, RawU2, RawU24, RawU32, RawU4, RawU8},
    PixelColor,
};

/// Trait to convert colors into a byte array.
///
/// See the [module-level documentation] for an example.
///
/// [module-level documentation]: index.html#converting-colors-to-raw-data
pub trait ToBytes {
    /// Return type of `to_bytes`.
    type Bytes;

    /// Converts a color into a byte array.
    ///
    /// The resulting array will always use big endian byte ordering regardless
    /// of the host endianness.
    fn to_bytes(self) -> Self::Bytes;
}

macro_rules! impl_to_bytes {
    ($type:ty) => {
        impl ToBytes for $type {
            type Bytes = [u8; 1];

            fn to_bytes(self) -> Self::Bytes {
                [self.0]
            }
        }
    };
}

impl_to_bytes!(RawU1);
impl_to_bytes!(RawU2);
impl_to_bytes!(RawU4);
impl_to_bytes!(RawU8);

impl ToBytes for RawU16 {
    type Bytes = [u8; 2];

    fn to_bytes(self) -> Self::Bytes {
        self.0.to_be_bytes()
    }
}

impl ToBytes for RawU24 {
    type Bytes = [u8; 3];

    fn to_bytes(self) -> Self::Bytes {
        let mut ret = [0; 3];

        ret.copy_from_slice(&self.0.to_be_bytes()[1..4]);

        ret
    }
}

impl ToBytes for RawU32 {
    type Bytes = [u8; 4];

    fn to_bytes(self) -> Self::Bytes {
        self.0.to_be_bytes()
    }
}

impl ToBytes for () {
    type Bytes = [u8; 0];

    fn to_bytes(self) -> Self::Bytes {
        []
    }
}

impl<C> ToBytes for C
where
    C: PixelColor + Into<<C as PixelColor>::Raw>,
{
    type Bytes = <<C as PixelColor>::Raw as ToBytes>::Bytes;

    fn to_bytes(self) -> Self::Bytes {
        self.into().to_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelcolor::{Bgr565, Bgr888, BinaryColor, Gray2, Gray4, Gray8, Rgb565, Rgb888};

    #[test]
    fn bpp1() {
        assert_eq!(BinaryColor::Off.to_bytes(), [0]);
        assert_eq!(BinaryColor::On.to_bytes(), [1]);
    }

    #[test]
    fn bpp2() {
        assert_eq!(Gray2::new(0).to_bytes(), [0]);
        assert_eq!(Gray2::new(3).to_bytes(), [3]);
    }

    #[test]
    fn bpp4() {
        assert_eq!(Gray4::new(0).to_bytes(), [0]);
        assert_eq!(Gray4::new(15).to_bytes(), [15]);
    }

    #[test]
    fn bpp8() {
        assert_eq!(Gray8::new(0).to_bytes(), [0]);
        assert_eq!(Gray8::new(255).to_bytes(), [255]);
    }

    #[test]
    fn bpp16_rgb() {
        assert_eq!(
            Rgb565::new(255, 0, 0).to_bytes(),
            [0b11111_000, 0b000_00000]
        );
        assert_eq!(
            Rgb565::new(0, 255, 0).to_bytes(),
            [0b00000_111, 0b111_00000]
        );
        assert_eq!(
            Rgb565::new(0, 0, 255).to_bytes(),
            [0b00000_000, 0b000_11111]
        );
    }

    #[test]
    fn bpp16_bgr() {
        assert_eq!(
            Bgr565::new(255, 0, 0).to_bytes(),
            [0b00000_000, 0b000_11111]
        );
        assert_eq!(
            Bgr565::new(0, 255, 0).to_bytes(),
            [0b00000_111, 0b111_00000]
        );
        assert_eq!(
            Bgr565::new(0, 0, 255).to_bytes(),
            [0b11111_000, 0b000_00000]
        );
    }

    #[test]
    fn bpp24_rgb() {
        assert_eq!(Rgb888::new(0xFF, 0x00, 0x00).to_bytes(), [0xFF, 0x00, 0x00]);
        assert_eq!(Rgb888::new(0x00, 0xFF, 0x00).to_bytes(), [0x00, 0xFF, 0x00]);
        assert_eq!(Rgb888::new(0x00, 0x00, 0xFF).to_bytes(), [0x00, 0x00, 0xFF]);
    }

    #[test]
    fn bpp24_bgr() {
        assert_eq!(Bgr888::new(0xFF, 0x00, 0x00).to_bytes(), [0x00, 0x00, 0xFF]);
        assert_eq!(Bgr888::new(0x00, 0xFF, 0x00).to_bytes(), [0x00, 0xFF, 0x00]);
        assert_eq!(Bgr888::new(0x00, 0x00, 0xFF).to_bytes(), [0xFF, 0x00, 0x00]);
    }

    #[test]
    fn bpp32() {
        // This test uses `RawU32` instead of a color, because no color included
        // in this crate uses 32 bpp.
        assert_eq!(RawU32::new(0x11223344).to_bytes(), [0x11, 0x22, 0x33, 0x44]);
    }
}
