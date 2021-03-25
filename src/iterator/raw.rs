//! Raw data iterator.

use crate::pixelcolor::raw::{
    BigEndian, LittleEndian, RawData, RawU1, RawU16, RawU2, RawU24, RawU32, RawU4, RawU8,
};
use byteorder::{ByteOrder, BE, LE};
use core::marker::PhantomData;

/// Iterator over a slice of raw pixel data.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct RawDataIter<'a, R, BO> {
    /// Pixel data.
    data: &'a [u8],

    /// Index into `data` for next read.
    byte_position: usize,

    /// Remaining bits in the current byte (only used for bpp < 8).
    bits_left: u8,

    raw_type: PhantomData<R>,
    byte_order: PhantomData<BO>,
}

impl<'a, R, BO> RawDataIter<'a, R, BO> {
    /// Creates a new raw data iterator.
    pub const fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            byte_position: 0,
            bits_left: 8,
            raw_type: PhantomData,
            byte_order: PhantomData,
        }
    }

    /// Sets the read position.
    pub fn set_byte_position(&mut self, position: usize) {
        self.byte_position = position;
        self.bits_left = 8;
    }

    /// Align the read position to the next whole byte.
    ///
    /// If the read position is already at the beginning of a byte this is a noop.
    pub fn align(&mut self) {
        if self.bits_left == 8 {
            return;
        }

        self.byte_position = self.data.len().min(self.byte_position + 1);
        self.bits_left = 8;
    }

    /// Returns the next `bit_count` bits.
    fn next_bits(&mut self, bit_count: u8) -> Option<u8> {
        if self.bits_left == 0 || self.byte_position >= self.data.len() {
            return None;
        }

        let current_byte = self.data[self.byte_position];
        let ret = current_byte >> (self.bits_left - bit_count);

        self.bits_left -= bit_count;

        if self.bits_left == 0 && self.byte_position < self.data.len() {
            self.byte_position += 1;
            self.bits_left = 8;
        }

        Some(ret)
    }

    /// Returns the a slice of the next `byte_count` bytes.
    fn next_bytes(&mut self, byte_count: usize) -> Option<&[u8]> {
        if (self.data.len() - self.byte_position) >= byte_count {
            let ret = &self.data[self.byte_position..];

            self.byte_position += byte_count;

            Some(ret)
        } else {
            None
        }
    }
}

impl<'a, R> Iterator for RawDataIter<'a, R, LittleEndian>
where
    R: RawData + RawDataIterNext<LittleEndian>,
{
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        R::next(self)
    }
}

impl<'a, R> Iterator for RawDataIter<'a, R, BigEndian>
where
    R: RawData + RawDataIterNext<BigEndian>,
{
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        R::next(self)
    }
}

/// Helper trait to implement the `next` method.
pub trait RawDataIterNext<BO>: Sized {
    /// Advances the iterator and returns the next raw value.
    fn next<'a>(iter: &mut RawDataIter<'a, Self, BO>) -> Option<Self>;
}

macro_rules! impl_next_for_bits {
    ($raw_type:ident, $bit_count:expr) => {
        impl<BO> RawDataIterNext<BO> for $raw_type {
            fn next<'a>(iter: &mut RawDataIter<'a, $raw_type, BO>) -> Option<$raw_type> {
                iter.next_bits($bit_count).map($raw_type::new)
            }
        }
    };
}

impl_next_for_bits!(RawU1, 1);
impl_next_for_bits!(RawU2, 2);
impl_next_for_bits!(RawU4, 4);

impl<BO> RawDataIterNext<BO> for RawU8 {
    fn next<'a>(iter: &mut RawDataIter<'a, RawU8, BO>) -> Option<RawU8> {
        iter.next_bytes(1).map(|data| RawU8::new(data[0]))
    }
}

macro_rules! impl_next_for_bytes {
    ($raw_type:ident, $byte_count:expr, $endian:ident, $read_function:path) => {
        impl RawDataIterNext<$endian> for $raw_type {
            fn next<'a>(iter: &mut RawDataIter<'a, $raw_type, $endian>) -> Option<$raw_type> {
                iter.next_bytes($byte_count)
                    .map(|data| $raw_type::new($read_function(data)))
            }
        }
    };
    ($raw_type:ident, $byte_count:expr, $read_function:ident) => {
        impl_next_for_bytes!($raw_type, $byte_count, BigEndian, BE::$read_function);
        impl_next_for_bytes!($raw_type, $byte_count, LittleEndian, LE::$read_function);
    };
}

impl_next_for_bytes!(RawU16, 2, read_u16);
impl_next_for_bytes!(RawU24, 3, read_u24);
impl_next_for_bytes!(RawU32, 4, read_u32);

/// Dummy implementation to allow () as `PixelColor::Raw`.
impl<BO> RawDataIterNext<BO> for () {
    fn next<'a>(_iter: &mut RawDataIter<'a, (), BO>) -> Option<()> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BITS_DATA: &[u8] = &[0x12, 0x48, 0x5A, 0x0F];
    const BYTES_DATA_1: &[u8] = &[0x10, 0x20, 0x30, 0x40, 0x50, 0x60];
    const BYTES_DATA_2: &[u8] = &[0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80];

    #[test]
    fn align_advances_to_next_byte() {
        let data = &[0x80, 0x80];

        let mut iter: RawDataIter<RawU1, LittleEndian> = RawDataIter::new(data);
        assert_eq!(iter.next(), Some(RawU1::new(1)));
        assert_eq!(iter.next(), Some(RawU1::new(0)));

        let mut iter: RawDataIter<RawU1, LittleEndian> = RawDataIter::new(data);
        assert_eq!(iter.next(), Some(RawU1::new(1)));
        iter.align();
        assert_eq!(iter.next(), Some(RawU1::new(1)));
    }

    #[test]
    fn calling_align_again_is_a_noop() {
        let data = &[0x80, 0xFF, 0x00];

        let mut iter: RawDataIter<RawU1, LittleEndian> = RawDataIter::new(data);
        assert_eq!(iter.next(), Some(RawU1::new(1)));
        iter.align();
        iter.align();
        assert_eq!(iter.next(), Some(RawU1::new(1)));
    }

    #[test]
    fn set_byte_position_resets_bit_position() {
        let data = &[0x0F, 0x0F];

        let mut iter: RawDataIter<RawU4, LittleEndian> = RawDataIter::new(data);
        assert_eq!(iter.next(), Some(RawU4::new(0)));
        iter.set_byte_position(1);
        assert_eq!(iter.next(), Some(RawU4::new(0)));
    }

    #[test]
    fn raw_u1() {
        #[rustfmt::skip]
        let expected = [
            0, 0, 0, 1,
            0, 0, 1, 0,
            0, 1, 0, 0,
            1, 0, 0, 0,
            0, 1, 0, 1,
            1, 0, 1, 0,
            0, 0, 0, 0,
            1, 1, 1, 1,
        ]
        .iter()
        .copied()
        .map(RawU1::new);

        let data: RawDataIter<RawU1, LittleEndian> = RawDataIter::new(BITS_DATA);
        assert!(data.eq(expected));
    }

    #[test]
    fn raw_u2() {
        let expected = [0, 1, 0, 2, 1, 0, 2, 0, 1, 1, 2, 2, 0, 0, 3, 3]
            .iter()
            .copied()
            .map(RawU2::new);

        let iter: RawDataIter<RawU2, LittleEndian> = RawDataIter::new(BITS_DATA);
        assert!(iter.eq(expected));
    }

    #[test]
    fn raw_u4() {
        let expected = [0x1, 0x2, 0x4, 0x8, 0x5, 0xA, 0x0, 0xF]
            .iter()
            .copied()
            .map(RawU4::new);

        let iter: RawDataIter<RawU4, LittleEndian> = RawDataIter::new(BITS_DATA);
        assert!(iter.eq(expected));
    }

    #[test]
    fn raw_u8() {
        let expected = BYTES_DATA_1.iter().map(|&v| RawU8::new(v));

        let iter: RawDataIter<RawU8, LittleEndian> = RawDataIter::new(BYTES_DATA_1);
        assert!(iter.eq(expected));
    }

    #[test]
    fn raw_u16_le() {
        let expected = [0x2010, 0x4030, 0x6050].iter().copied().map(RawU16::new);

        let iter: RawDataIter<RawU16, LittleEndian> = RawDataIter::new(BYTES_DATA_1);
        assert!(iter.eq(expected));
    }

    #[test]
    fn raw_u16_be() {
        let expected = [0x1020, 0x3040, 0x5060].iter().copied().map(RawU16::new);

        let iter: RawDataIter<RawU16, BigEndian> = RawDataIter::new(BYTES_DATA_1);
        assert!(iter.eq(expected));
    }

    #[test]
    fn raw_u16_excess_bytes_are_ignored() {
        let iter: RawDataIter<RawU16, LittleEndian> = RawDataIter::new(&[0; 3]);
        assert_eq!(iter.count(), 1);
    }

    #[test]
    fn raw_u24_le() {
        let expected = [0x302010, 0x605040].iter().copied().map(RawU24::new);

        let iter: RawDataIter<RawU24, LittleEndian> = RawDataIter::new(BYTES_DATA_1);
        assert!(iter.into_iter().eq(expected));
    }

    #[test]
    fn raw_u24_be() {
        let expected = [0x102030, 0x405060].iter().copied().map(RawU24::new);

        let iter: RawDataIter<RawU24, BigEndian> = RawDataIter::new(BYTES_DATA_1);
        assert!(iter.into_iter().eq(expected));
    }

    #[test]
    fn raw_u24_excess_bytes_are_ignored() {
        let iter: RawDataIter<RawU24, LittleEndian> = RawDataIter::new(&[0; 7]);
        assert_eq!(iter.count(), 2);
    }

    #[test]
    fn raw_u32_le() {
        let expected = [0x40302010, 0x80706050].iter().copied().map(RawU32::new);

        let iter: RawDataIter<RawU32, LittleEndian> = RawDataIter::new(BYTES_DATA_2);
        assert!(iter.into_iter().eq(expected));
    }

    #[test]
    fn raw_u32_be() {
        let expected = [0x10203040, 0x50607080].iter().copied().map(RawU32::new);

        let iter: RawDataIter<RawU32, BigEndian> = RawDataIter::new(BYTES_DATA_2);
        assert!(iter.into_iter().eq(expected));
    }

    #[test]
    fn raw_u32_excess_bytes_are_ignored() {
        let iter: RawDataIter<RawU32, LittleEndian> = RawDataIter::new(&[0; 13]);
        assert_eq!(iter.count(), 3);
    }
}
