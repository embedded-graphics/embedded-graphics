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
//! use embedded_graphics::{iterator::raw::RawDataSlice, pixelcolor::raw::{RawU16, storage::BigEndian}};
//!
//! let data = [0xAA, 0xBB, 0x12, 0x34];
//!
//! // The data type and byte order needs to be specified explicitly to set the data format.
//! let slice = RawDataSlice::<BigEndian<RawU16>>::new(&data);
//!
//! let mut iter = slice.into_iter();
//! assert_eq!(iter.next(), Some(RawU16::new(0xAABB)));
//! assert_eq!(iter.next(), Some(RawU16::new(0x1234)));
//! assert_eq!(iter.next(), None);
//! ```
//!
//! [`ImageRaw`]: super::super::image::ImageRaw

use core::{marker::PhantomData, slice};

use byteorder::{ByteOrder, BE, LE};
use embedded_graphics_core::pixelcolor::raw::storage::RawStorage;

use crate::pixelcolor::raw::{
    storage::{BigEndian, LittleEndian, Lsb0, Msb0},
    RawData, RawU1, RawU16, RawU2, RawU24, RawU32, RawU4, RawU8,
};

/// Raw data slice.
///
/// This type is a wrapper around a byte array to specify the stored data format.
///
/// See the [module-level documentation] for more information.
///
/// [module-level documentation]: self
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct RawDataSlice<'a, RS> {
    data: &'a [u8],
    raw_storage_type: PhantomData<RS>,
}

impl<'a, RS> RawDataSlice<'a, RS> {
    /// Creates a new raw data slice.
    pub const fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            raw_storage_type: PhantomData,
        }
    }
}

macro_rules! impl_bits_iterator {
    ($storage_type:ty, $raw_type:ty, $bit_index_bits:expr, $is_msb:expr) => {
        impl<'a> IntoIterator for RawDataSlice<'a, $storage_type> {
            type Item = $raw_type;
            type IntoIter = BitsIterator<'a, $storage_type>;

            fn into_iter(self) -> Self::IntoIter {
                BitsIterator::new(self.data)
            }
        }

        impl<'a> Iterator for BitsIterator<'a, $storage_type> {
            type Item = $raw_type;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                self.data.get(self.index >> $bit_index_bits).map(|byte| {
                    // Index to one of the pixels inside this byte.
                    let sub_index = self.index & (1 << $bit_index_bits) - 1;

                    self.index += 1;

                    let shift = if $is_msb {
                        let first_pixel_shift = 8 - <$raw_type>::BITS_PER_PIXEL;

                        first_pixel_shift - (sub_index << 3 - $bit_index_bits)
                    } else {
                        sub_index << 3 - $bit_index_bits
                    };

                    <$raw_type>::new(*byte >> shift)
                })
            }

            #[inline]
            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                self.index = self.index.saturating_add(n);
                self.next()
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                let size = (self.data.len() * (8 / <$raw_type>::BITS_PER_PIXEL))
                    .saturating_sub(self.index);

                (size, Some(size))
            }
        }
    };

    ($raw_type:ty, $bit_index_bits:expr) => {
        impl_bits_iterator!($raw_type, $raw_type, $bit_index_bits, true);
        impl_bits_iterator!(Msb0<$raw_type>, $raw_type, $bit_index_bits, true);
        impl_bits_iterator!(Lsb0<$raw_type>, $raw_type, $bit_index_bits, false);
    };
}

impl_bits_iterator!(RawU1, 3);
impl_bits_iterator!(RawU2, 2);
impl_bits_iterator!(RawU4, 1);

impl<'a> IntoIterator for RawDataSlice<'a, RawU8> {
    type Item = RawU8;
    type IntoIter = ByteIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ByteIterator::new(self.data)
    }
}

impl<'a> IntoIterator for RawDataSlice<'a, LittleEndian<RawU8>> {
    type Item = RawU8;
    type IntoIter = ByteIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ByteIterator::new(self.data)
    }
}

impl<'a> IntoIterator for RawDataSlice<'a, BigEndian<RawU8>> {
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
/// [module-level documentation]: self
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct BitsIterator<'a, RS> {
    data: &'a [u8],
    index: usize,
    raw_storage_type: PhantomData<RS>,
}

impl<'a, RS> BitsIterator<'a, RS> {
    const fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            index: 0,
            raw_storage_type: PhantomData,
        }
    }
}

/// Iterator for raw data slices with 8 BPP.
///
/// See the [module-level documentation] for more information.
///
/// [module-level documentation]: self
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
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
/// [module-level documentation]: self
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct BytesIterator<'a, RS> {
    // MSRV: replace by ArrayChunks when the feature is stabilized
    data: slice::ChunksExact<'a, u8>,
    raw_storage_type: PhantomData<RS>,
}

impl<'a, RS: RawStorage> BytesIterator<'a, RS> {
    fn new(data: &'a [u8]) -> Self {
        Self {
            data: data.chunks_exact(RS::Raw::BITS_PER_PIXEL / 8),
            raw_storage_type: PhantomData,
        }
    }
}

macro_rules! impl_bytes_iterator {
    ($storage_type:ty, $raw_type:ty, $read_function:path) => {
        impl<'a> Iterator for BytesIterator<'a, $storage_type> {
            type Item = $raw_type;

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

        impl<'a> IntoIterator for RawDataSlice<'a, $storage_type> {
            type Item = $raw_type;
            type IntoIter = BytesIterator<'a, $storage_type>;

            fn into_iter(self) -> Self::IntoIter {
                BytesIterator::new(self.data)
            }
        }
    };

    ($type:ty, $read_function:ident) => {
        impl_bytes_iterator!($type, $type, LE::$read_function);
        impl_bytes_iterator!(LittleEndian<$type>, $type, LE::$read_function);
        impl_bytes_iterator!(BigEndian<$type>, $type, BE::$read_function);
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
    fn raw_u1_msb0() {
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

        let iter = RawDataSlice::<Msb0<RawU1>>::new(BITS_DATA).into_iter();
        assert!(iter.eq(expected));
    }

    #[test]
    fn raw_u1_lsb0() {
        #[rustfmt::skip]
        let expected = [
            0, 1, 0, 0,
            1, 0, 0, 0,
            0, 0, 0, 1,
            0, 0, 1, 0,
            0, 1, 0, 1,
            1, 0, 1, 0,
            1, 1, 1, 1,
            0, 0, 0, 0,
        ]
        .iter()
        .copied()
        .map(RawU1::new);

        let iter = RawDataSlice::<Lsb0<RawU1>>::new(BITS_DATA).into_iter();
        assert!(iter.eq(expected));
    }

    #[test]
    fn raw_u2_msb0() {
        let expected = [0, 1, 0, 2, 1, 0, 2, 0, 1, 1, 2, 2, 0, 0, 3, 3]
            .iter()
            .copied()
            .map(RawU2::new);

        let iter = RawDataSlice::<Msb0<RawU2>>::new(BITS_DATA).into_iter();
        assert!(iter.eq(expected));
    }

    #[test]
    fn raw_u2_lsb0() {
        let expected = [2, 0, 1, 0, 0, 2, 0, 1, 2, 2, 1, 1, 3, 3, 0, 0]
            .iter()
            .copied()
            .map(RawU2::new);

        let iter = RawDataSlice::<Lsb0<RawU2>>::new(BITS_DATA).into_iter();
        assert!(iter.eq(expected));
    }

    #[test]
    fn raw_u4_msb0() {
        let expected = [0x1, 0x2, 0x4, 0x8, 0x5, 0xA, 0x0, 0xF]
            .iter()
            .copied()
            .map(RawU4::new);

        let iter = RawDataSlice::<Msb0<RawU4>>::new(BITS_DATA).into_iter();
        assert!(iter.eq(expected));
    }

    #[test]
    fn raw_u4_lsb0() {
        let expected = [0x2, 0x1, 0x8, 0x4, 0xA, 0x5, 0xF, 0x0]
            .iter()
            .copied()
            .map(RawU4::new);

        let iter = RawDataSlice::<Lsb0<RawU4>>::new(BITS_DATA).into_iter();
        assert!(iter.eq(expected));
    }

    #[test]
    fn raw_u8() {
        let expected = BYTES_DATA_1.iter().map(|&v| RawU8::new(v));

        let iter = RawDataSlice::<RawU8>::new(BYTES_DATA_1).into_iter();
        assert!(iter.eq(expected));
    }

    #[test]
    fn raw_u16_le() {
        let expected = [0x2010, 0x4030, 0x6050].iter().copied().map(RawU16::new);

        let iter = RawDataSlice::<LittleEndian<RawU16>>::new(BYTES_DATA_1).into_iter();
        assert!(iter.eq(expected));
    }

    #[test]
    fn raw_u16_be() {
        let expected = [0x1020, 0x3040, 0x5060].iter().copied().map(RawU16::new);

        let iter = RawDataSlice::<BigEndian<RawU16>>::new(BYTES_DATA_1).into_iter();
        assert!(iter.eq(expected));
    }

    #[test]
    fn raw_u16_excess_bytes_are_ignored() {
        let iter = RawDataSlice::<LittleEndian<RawU16>>::new(&[0; 3]).into_iter();
        assert_eq!(iter.count(), 1);
    }

    #[test]
    fn raw_u24_le() {
        let expected = [0x302010, 0x605040].iter().copied().map(RawU24::new);

        let iter = RawDataSlice::<LittleEndian<RawU24>>::new(BYTES_DATA_1).into_iter();
        assert!(iter.into_iter().eq(expected));
    }

    #[test]
    fn raw_u24_be() {
        let expected = [0x102030, 0x405060].iter().copied().map(RawU24::new);

        let iter = RawDataSlice::<BigEndian<RawU24>>::new(BYTES_DATA_1).into_iter();
        assert!(iter.into_iter().eq(expected));
    }

    #[test]
    fn raw_u24_excess_bytes_are_ignored() {
        let iter = RawDataSlice::<LittleEndian<RawU24>>::new(&[0; 7]).into_iter();
        assert_eq!(iter.count(), 2);
    }

    #[test]
    fn raw_u32_le() {
        let expected = [0x40302010, 0x80706050].iter().copied().map(RawU32::new);

        let iter = RawDataSlice::<LittleEndian<RawU32>>::new(BYTES_DATA_2).into_iter();
        assert!(iter.into_iter().eq(expected));
    }

    #[test]
    fn raw_u32_be() {
        let expected = [0x10203040, 0x50607080].iter().copied().map(RawU32::new);

        let iter = RawDataSlice::<BigEndian<RawU32>>::new(BYTES_DATA_2).into_iter();
        assert!(iter.into_iter().eq(expected));
    }

    #[test]
    fn raw_u32_excess_bytes_are_ignored() {
        let iter = RawDataSlice::<LittleEndian<RawU32>>::new(&[0; 13]).into_iter();
        assert_eq!(iter.count(), 3);
    }
}
