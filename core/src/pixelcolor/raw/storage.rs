//! TODO: docs

use crate::pixelcolor::{
    raw::{RawU1, RawU16, RawU2, RawU24, RawU32, RawU4, RawU8},
    PixelColor,
};

use super::RawData;

/// TODO: docs
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct LittleEndian<T>(T);

/// TODO: docs
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct BigEndian<T>(T);

/// TODO: docs
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct Lsb0<T>(T);

/// TODO: docs
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct Msb0<T>(T);

/// TODO: docs
pub trait RawStorage {
    /// TODO: docs
    type Raw: RawData;
}

macro_rules! impl_raw_storage {
    (bits: $raw_type:ty) => {
        impl RawStorage for $raw_type {
            type Raw = $raw_type;
        }

        impl RawStorage for Lsb0<$raw_type> {
            type Raw = $raw_type;
        }

        impl RawStorage for Msb0<$raw_type> {
            type Raw = $raw_type;
        }
    };

    (bytes: $raw_type:ty) => {
        impl RawStorage for $raw_type {
            type Raw = $raw_type;
        }

        impl RawStorage for LittleEndian<$raw_type> {
            type Raw = $raw_type;
        }

        impl RawStorage for BigEndian<$raw_type> {
            type Raw = $raw_type;
        }
    };
}

impl_raw_storage!(bits: RawU1);
impl_raw_storage!(bits: RawU2);
impl_raw_storage!(bits: RawU4);
impl_raw_storage!(bytes: RawU8);
impl_raw_storage!(bytes: RawU16);
impl_raw_storage!(bytes: RawU24);
impl_raw_storage!(bytes: RawU32);

/// TODO: docs
pub trait ColorStorage {
    /// TODO: docs
    type Color: PixelColor;
    /// TODO: docs
    type RawStorage: RawStorage<Raw = <Self::Color as PixelColor>::Raw>;
}

impl<C> ColorStorage for C
where
    C: PixelColor,
    C::Raw: RawStorage<Raw = C::Raw>,
{
    type Color = C;
    type RawStorage = C::Raw;
}

impl<C> ColorStorage for LittleEndian<C>
where
    C: PixelColor,
    LittleEndian<C::Raw>: RawStorage<Raw = C::Raw>,
{
    type Color = C;
    type RawStorage = LittleEndian<C::Raw>;
}

impl<C> ColorStorage for BigEndian<C>
where
    C: PixelColor,
    BigEndian<C::Raw>: RawStorage<Raw = C::Raw>,
{
    type Color = C;
    type RawStorage = BigEndian<C::Raw>;
}

impl<C> ColorStorage for Msb0<C>
where
    C: PixelColor,
    Msb0<C::Raw>: RawStorage<Raw = C::Raw>,
{
    type Color = C;
    type RawStorage = Msb0<C::Raw>;
}

impl<C> ColorStorage for Lsb0<C>
where
    C: PixelColor,
    Lsb0<C::Raw>: RawStorage<Raw = C::Raw>,
{
    type Color = C;
    type RawStorage = Lsb0<C::Raw>;
}
