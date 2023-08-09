//! Byte and bit order marker types.
//!
//! The types in this crate are used to specify the byte or bit order in images and framebuffers.

use crate::pixelcolor::raw::{private::Sealed, RawU1, RawU16, RawU2, RawU24, RawU32, RawU4, RawU8};

/// Lsb0 bit order.
///
/// The bits of the first pixel in a byte are aligned to the right (the last significant bits).
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub enum Lsb0 {}
impl Sealed for Lsb0 {}

/// Msb0 bit order.
///
/// The bits of the first pixel in a byte are aligned to the left (the last most significant bits).
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub enum Msb0 {}
impl Sealed for Msb0 {}

/// Little endian byte order.
///
/// The bytes of a pixel are stored in the big endian byte order.
///
/// # Layout example
///
/// This example shows how an pixel array, which contains the pixels `Rgb888::new(10, 20, 30)` and
/// `Rgb888::new(40, 50, 60)`,  would be stored using the big endian byte order:
///
/// | Byte  |  0 |  1 |  2 |  3 |  4 |  5 |…
/// |-------|----|----|----|----|----|----|--
/// | Value | 10 | 20 | 30 | 40 | 50 | 60 |…
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub enum BigEndian {}
impl Sealed for BigEndian {}

/// Little endian byte order.
///
/// The bytes of a pixel are stored in the little endian byte order.
///
/// # Layout example
///
/// This example shows how an pixel array, which contains the pixels `Rgb888::new(10, 20, 30)` and
/// `Rgb888::new(40, 50, 60)`,  would be stored using the little endian byte order:
///
/// | Byte  |  0 |  1 |  2 |  3 |  4 |  5 |…
/// |-------|----|----|----|----|----|----|--
/// | Value | 30 | 20 | 10 | 60 | 50 | 40 |…
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub enum LittleEndian {}
impl Sealed for LittleEndian {}

/// Data order.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub enum DataOrderEnum {
    /// Default data order.
    Default,
    /// Alternate data order.
    Alternate,
}

/// Data order.
pub trait DataOrder<R>: Sealed + Copy {
    /// Data order.
    const ORDER: DataOrderEnum;
}

macro_rules! impl_data_order {
    (bits $raw_type:ty) => {
        impl DataOrder<$raw_type> for Msb0 {
            const ORDER: DataOrderEnum = DataOrderEnum::Default;
        }

        impl DataOrder<$raw_type> for Lsb0 {
            const ORDER: DataOrderEnum = DataOrderEnum::Alternate;
        }
    };

    (bytes $raw_type:ty) => {
        impl DataOrder<$raw_type> for LittleEndian {
            const ORDER: DataOrderEnum = DataOrderEnum::Default;
        }

        impl DataOrder<$raw_type> for BigEndian {
            const ORDER: DataOrderEnum = DataOrderEnum::Alternate;
        }
    };
}

impl_data_order!(bits RawU1);
impl_data_order!(bits RawU2);
impl_data_order!(bits RawU4);
impl_data_order!(bytes RawU8);
impl_data_order!(bytes RawU16);
impl_data_order!(bytes RawU24);
impl_data_order!(bytes RawU32);
