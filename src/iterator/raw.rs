//! Raw data iterator.
//!
//! Raw data iterators are used to implement rendering of custom image formats. Most users
//! won't need to use these types directly and should instead use [`ImageRaw`].
//!
//! The [`RawDataSlice`] is used to specify the raw data format for a byte slice. This slice can
//! than be converted into an optimized iterator for that data format by using `into_iter()`.
//!
//! # Examples
//!
//! ```
//! use embedded_graphics::{iterator::raw::RawDataSlice, pixelcolor::raw::{RawU16, BigEndianLsb0}};
//!
//! let data = [0xAA, 0xBB, 0x12, 0x34];
//!
//! // The data type and byte order needs to be specified explicitly to set the data format.
//! let slice = RawDataSlice::<RawU16, BigEndianLsb0>::new(&data);
//!
//! let mut iter = slice.into_iter();
//! assert_eq!(iter.next(), Some(RawU16::new(0xAABB)));
//! assert_eq!(iter.next(), Some(RawU16::new(0x1234)));
//! assert_eq!(iter.next(), None);
//! ```
//!
//! [`ImageRaw`]: super::super::image::ImageRaw

use core::marker::PhantomData;

use embedded_graphics_core::pixelcolor::raw::DataOrder;

use crate::pixelcolor::raw::RawData;

/// Raw data slice.
///
/// This type is a wrapper around a byte array to specify the stored data format.
///
/// See the [module-level documentation] for more information.
///
/// [module-level documentation]: self
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct RawDataSlice<'a, R, O> {
    data: &'a [u8],
    raw_type: PhantomData<R>,
    data_order: PhantomData<O>,
}

impl<'a, R, O> RawDataSlice<'a, R, O> {
    /// Creates a new raw data slice.
    pub const fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            raw_type: PhantomData,
            data_order: PhantomData,
        }
    }
}

impl<'a, R: RawData, O: DataOrder> IntoIterator for RawDataSlice<'a, R, O> {
    type Item = R;
    type IntoIter = RawDataIterator<'a, R, O>;

    fn into_iter(self) -> Self::IntoIter {
        RawDataIterator::new(self.data)
    }
}

/// TODO
#[derive(Debug)]
pub struct RawDataIterator<'a, R, O> {
    data: &'a [u8],
    index: usize,
    raw_type: PhantomData<R>,
    data_order: PhantomData<O>,
}

impl<'a, R, O> RawDataIterator<'a, R, O> {
    fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            index: 0,
            raw_type: PhantomData,
            data_order: PhantomData,
        }
    }
}

impl<R: RawData, O: DataOrder> Iterator for RawDataIterator<'_, R, O> {
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        R::load::<O>(self.data, self.index).map(|v| {
            self.index += 1;
            v
        })
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.index = self.index.saturating_add(n);
        self.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let pixels_total = if R::BITS_PER_PIXEL >= 8 {
            self.data.len() * (8 / R::BITS_PER_PIXEL)
        } else {
            self.data.len() * (R::BITS_PER_PIXEL / 8)
        };

        let size = pixels_total.saturating_sub(self.index);

        (size, Some(size))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelcolor::raw::*;

    const BITS_DATA: &[u8] = &[0x12, 0x48, 0x5A, 0x0F];
    const BYTES_DATA_1: &[u8] = &[0x10, 0x20, 0x30, 0x40, 0x50, 0x60];
    const BYTES_DATA_2: &[u8] = &[0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80];

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

        let iter = RawDataSlice::<RawU1, LittleEndianMsb0>::new(BITS_DATA).into_iter();
        assert!(iter.eq(expected));
    }

    #[test]
    fn raw_u2() {
        let expected = [0, 1, 0, 2, 1, 0, 2, 0, 1, 1, 2, 2, 0, 0, 3, 3]
            .iter()
            .copied()
            .map(RawU2::new);

        let iter = RawDataSlice::<RawU2, LittleEndianMsb0>::new(BITS_DATA).into_iter();
        assert!(iter.eq(expected));
    }

    #[test]
    fn raw_u4() {
        let expected = [0x1, 0x2, 0x4, 0x8, 0x5, 0xA, 0x0, 0xF]
            .iter()
            .copied()
            .map(RawU4::new);

        let iter = RawDataSlice::<RawU4, LittleEndianMsb0>::new(BITS_DATA).into_iter();
        assert!(iter.eq(expected));
    }

    #[test]
    fn raw_u8() {
        let expected = BYTES_DATA_1.iter().map(|&v| RawU8::new(v));

        let iter = RawDataSlice::<RawU8, LittleEndianMsb0>::new(BYTES_DATA_1).into_iter();
        assert!(iter.eq(expected));
    }

    #[test]
    fn raw_u16_le() {
        let expected = [0x2010, 0x4030, 0x6050].iter().copied().map(RawU16::new);

        let iter = RawDataSlice::<RawU16, LittleEndianMsb0>::new(BYTES_DATA_1).into_iter();
        assert!(iter.eq(expected));
    }

    #[test]
    fn raw_u16_be() {
        let expected = [0x1020, 0x3040, 0x5060].iter().copied().map(RawU16::new);

        let iter = RawDataSlice::<RawU16, BigEndianLsb0>::new(BYTES_DATA_1).into_iter();
        assert!(iter.eq(expected));
    }

    #[test]
    fn raw_u16_excess_bytes_are_ignored() {
        let iter = RawDataSlice::<RawU16, LittleEndianMsb0>::new(&[0; 3]).into_iter();
        assert_eq!(iter.count(), 1);
    }

    #[test]
    fn raw_u24_le() {
        let expected = [0x302010, 0x605040].iter().copied().map(RawU24::new);

        let iter = RawDataSlice::<RawU24, LittleEndianMsb0>::new(BYTES_DATA_1).into_iter();
        assert!(iter.into_iter().eq(expected));
    }

    #[test]
    fn raw_u24_be() {
        let expected = [0x102030, 0x405060].iter().copied().map(RawU24::new);

        let iter = RawDataSlice::<RawU24, BigEndianLsb0>::new(BYTES_DATA_1).into_iter();
        assert!(iter.into_iter().eq(expected));
    }

    #[test]
    fn raw_u24_excess_bytes_are_ignored() {
        let iter = RawDataSlice::<RawU24, LittleEndianMsb0>::new(&[0; 7]).into_iter();
        assert_eq!(iter.count(), 2);
    }

    #[test]
    fn raw_u32_le() {
        let expected = [0x40302010, 0x80706050].iter().copied().map(RawU32::new);

        let iter = RawDataSlice::<RawU32, LittleEndianMsb0>::new(BYTES_DATA_2).into_iter();
        assert!(iter.into_iter().eq(expected));
    }

    #[test]
    fn raw_u32_be() {
        let expected = [0x10203040, 0x50607080].iter().copied().map(RawU32::new);

        let iter = RawDataSlice::<RawU32, BigEndianLsb0>::new(BYTES_DATA_2).into_iter();
        assert!(iter.into_iter().eq(expected));
    }

    #[test]
    fn raw_u32_excess_bytes_are_ignored() {
        let iter = RawDataSlice::<RawU32, LittleEndianMsb0>::new(&[0; 13]).into_iter();
        assert_eq!(iter.count(), 3);
    }
}
