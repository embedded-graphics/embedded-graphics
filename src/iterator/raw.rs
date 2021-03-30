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
//! use embedded_graphics::{iterator::raw::RawDataSlice, pixelcolor::raw::{RawU16, BigEndian}};
//!
//! let data = [0xAA, 0xBB, 0x12, 0x34];
//!
//! // The data type and byte order needs to be specified explicitly to set the data format.
//! let slice = RawDataSlice::<RawU16, BigEndian>::new(&data);
//!
//! let mut iter = slice.into_iter();
//! assert_eq!(iter.next(), Some(RawU16::new(0xAABB)));
//! assert_eq!(iter.next(), Some(RawU16::new(0x1234)));
//! assert_eq!(iter.next(), None);
//! ```
//!
//! [`ImageRaw`]: ../../image/struct.ImageRaw.html
//! [`RawDataSlice`]: struct.RawDataSlice.html

use core::{marker::PhantomData, slice};

use byteorder::{ByteOrder, BE, LE};

use crate::pixelcolor::raw::{
    BigEndian, LittleEndian, RawData, RawU1, RawU16, RawU2, RawU24, RawU32, RawU4, RawU8,
};

/// Raw data slice.
///
/// This type is a wrapper around a byte array to specify the stored data format.
///
/// See the [module-level documentation] for more information.
///
/// [module-level documentation]: index.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawDataSlice<'a, R, BO> {
    data: &'a [u8],
    raw_type: PhantomData<R>,
    byte_order: PhantomData<BO>,
}

impl<'a, R, BO> RawDataSlice<'a, R, BO> {
    /// Creates a new raw data slice.
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            raw_type: PhantomData,
            byte_order: PhantomData,
        }
    }
}

macro_rules! impl_bits_iterator {
    ($type:ident, $bit_index_bits:expr) => {
        impl<'a, BO> IntoIterator for RawDataSlice<'a, $type, BO> {
            type Item = $type;
            type IntoIter = BitsIterator<'a, $type>;

            fn into_iter(self) -> Self::IntoIter {
                BitsIterator::new(self.data)
            }
        }

        impl<'a> Iterator for BitsIterator<'a, $type> {
            type Item = $type;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                self.data.get(self.index >> $bit_index_bits).map(|byte| {
                    // Number of bits the value needs to be shifted to the right for the first pixel.
                    let first_pixel_shift = 8 - $type::BITS_PER_PIXEL;

                    // Index to one of the pixels inside this byte.
                    let sub_index = self.index & (1 << $bit_index_bits) - 1;

                    // Number of bits the value needs to be shifted.
                    let shift = first_pixel_shift - (sub_index << 3 - $bit_index_bits);

                    self.index += 1;

                    $type::new(*byte >> shift)
                })
            }

            #[inline]
            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                self.index = self.index.saturating_add(n);
                self.next()
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                let size =
                    (self.data.len() * (8 / $type::BITS_PER_PIXEL)).saturating_sub(self.index);

                (size, Some(size))
            }
        }
    };
}

impl_bits_iterator!(RawU1, 3);
impl_bits_iterator!(RawU2, 2);
impl_bits_iterator!(RawU4, 1);

impl<'a, BO> IntoIterator for RawDataSlice<'a, RawU8, BO> {
    type Item = RawU8;
    type IntoIter = ByteIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ByteIterator::new(self.data)
    }
}

/// Iterator for raw data slices with less than 8 BPP.
///
/// See the [module-level documentation] for more information.
///
/// [module-level documentation]: index.html
#[derive(Debug)]
pub struct BitsIterator<'a, R> {
    data: &'a [u8],
    index: usize,
    raw_type: PhantomData<R>,
}

impl<'a, R: RawData> BitsIterator<'a, R> {
    fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            index: 0,
            raw_type: PhantomData,
        }
    }
}

/// Iterator for raw data slices with 8 BPP.
///
/// See the [module-level documentation] for more information.
///
/// [module-level documentation]: index.html
#[derive(Debug)]
pub struct ByteIterator<'a> {
    data: slice::Iter<'a, u8>,
}

impl<'a> ByteIterator<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data: data.iter() }
    }
}

impl<'a> Iterator for ByteIterator<'a> {
    type Item = RawU8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.data.next().copied().map(RawU8::new)
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.data.nth(n).copied().map(RawU8::new)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.data.size_hint()
    }
}

/// Iterator for raw data slices more than 8 BPP.
///
/// See the [module-level documentation] for more information.
///
/// [module-level documentation]: index.html
#[derive(Debug)]
pub struct BytesIterator<'a, R, BO> {
    // MSRV: replace by ArrayChunks when the feature is stabilized
    data: slice::ChunksExact<'a, u8>,
    raw_type: PhantomData<R>,
    byte_order: PhantomData<BO>,
}

impl<'a, R: RawData, BO> BytesIterator<'a, R, BO> {
    fn new(data: &'a [u8]) -> Self {
        Self {
            data: data.chunks_exact(R::BITS_PER_PIXEL / 8),
            raw_type: PhantomData,
            byte_order: PhantomData,
        }
    }
}

macro_rules! impl_bytes_iterator {
    ($type:ident, $byte_order:ident, $read_function:path) => {
        impl<'a> Iterator for BytesIterator<'a, $type, $byte_order> {
            type Item = $type;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                self.data.next().map(|raw| $read_function(raw).into())
            }

            #[inline]
            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                self.data.nth(n).map(|raw| $read_function(raw).into())
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                self.data.size_hint()
            }
        }

        impl<'a> IntoIterator for RawDataSlice<'a, $type, $byte_order> {
            type Item = $type;
            type IntoIter = BytesIterator<'a, $type, $byte_order>;

            fn into_iter(self) -> Self::IntoIter {
                BytesIterator::new(self.data)
            }
        }
    };

    ($type:ident, $read_function:ident) => {
        impl_bytes_iterator!($type, LittleEndian, LE::$read_function);
        impl_bytes_iterator!($type, BigEndian, BE::$read_function);
    };
}

impl_bytes_iterator!(RawU16, read_u16);
impl_bytes_iterator!(RawU24, read_u24);
impl_bytes_iterator!(RawU32, read_u32);

#[cfg(test)]
mod tests {
    use super::*;

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

        let iter = RawDataSlice::<RawU1, LittleEndian>::new(BITS_DATA).into_iter();
        assert!(iter.eq(expected));
    }

    #[test]
    fn raw_u2() {
        let expected = [0, 1, 0, 2, 1, 0, 2, 0, 1, 1, 2, 2, 0, 0, 3, 3]
            .iter()
            .copied()
            .map(RawU2::new);

        let iter = RawDataSlice::<RawU2, LittleEndian>::new(BITS_DATA).into_iter();
        assert!(iter.eq(expected));
    }

    #[test]
    fn raw_u4() {
        let expected = [0x1, 0x2, 0x4, 0x8, 0x5, 0xA, 0x0, 0xF]
            .iter()
            .copied()
            .map(RawU4::new);

        let iter = RawDataSlice::<RawU4, LittleEndian>::new(BITS_DATA).into_iter();
        assert!(iter.eq(expected));
    }

    #[test]
    fn raw_u8() {
        let expected = BYTES_DATA_1.iter().map(|&v| RawU8::new(v));

        let iter = RawDataSlice::<RawU8, LittleEndian>::new(BYTES_DATA_1).into_iter();
        assert!(iter.eq(expected));
    }

    #[test]
    fn raw_u16_le() {
        let expected = [0x2010, 0x4030, 0x6050].iter().copied().map(RawU16::new);

        let iter = RawDataSlice::<RawU16, LittleEndian>::new(BYTES_DATA_1).into_iter();
        assert!(iter.eq(expected));
    }

    #[test]
    fn raw_u16_be() {
        let expected = [0x1020, 0x3040, 0x5060].iter().copied().map(RawU16::new);

        let iter = RawDataSlice::<RawU16, BigEndian>::new(BYTES_DATA_1).into_iter();
        assert!(iter.eq(expected));
    }

    #[test]
    fn raw_u16_excess_bytes_are_ignored() {
        let iter = RawDataSlice::<RawU16, LittleEndian>::new(&[0; 3]).into_iter();
        assert_eq!(iter.count(), 1);
    }

    #[test]
    fn raw_u24_le() {
        let expected = [0x302010, 0x605040].iter().copied().map(RawU24::new);

        let iter = RawDataSlice::<RawU24, LittleEndian>::new(BYTES_DATA_1).into_iter();
        assert!(iter.into_iter().eq(expected));
    }

    #[test]
    fn raw_u24_be() {
        let expected = [0x102030, 0x405060].iter().copied().map(RawU24::new);

        let iter = RawDataSlice::<RawU24, BigEndian>::new(BYTES_DATA_1).into_iter();
        assert!(iter.into_iter().eq(expected));
    }

    #[test]
    fn raw_u24_excess_bytes_are_ignored() {
        let iter = RawDataSlice::<RawU24, LittleEndian>::new(&[0; 7]).into_iter();
        assert_eq!(iter.count(), 2);
    }

    #[test]
    fn raw_u32_le() {
        let expected = [0x40302010, 0x80706050].iter().copied().map(RawU32::new);

        let iter = RawDataSlice::<RawU32, LittleEndian>::new(BYTES_DATA_2).into_iter();
        assert!(iter.into_iter().eq(expected));
    }

    #[test]
    fn raw_u32_be() {
        let expected = [0x10203040, 0x50607080].iter().copied().map(RawU32::new);

        let iter = RawDataSlice::<RawU32, BigEndian>::new(BYTES_DATA_2).into_iter();
        assert!(iter.into_iter().eq(expected));
    }

    #[test]
    fn raw_u32_excess_bytes_are_ignored() {
        let iter = RawDataSlice::<RawU32, LittleEndian>::new(&[0; 13]).into_iter();
        assert_eq!(iter.count(), 3);
    }
}
