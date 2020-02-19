//! Raw color types.
//!
//! This module contains structs to represent the raw data used to store color
//! information. Colors that implement the [`PixelColor`] trait can use the
//! associated [`Raw`] type to define their raw data representation.
//!
//! Specifying a [`Raw`] type for a [`PixelColor`] is required to use that color
//! with the [`Image`] struct.
//!
//! # Implementing PixelColor with Raw support
//!
//! This example shows how to implement a new [`PixelColor`] that can be used
//! with images.
//!
//! The RGBI color type uses 4 bits per pixel, one for each color channel and
//! an additional intensity bit.
//!
//! ```rust
//! use embedded_graphics::{image::ImageRaw, pixelcolor::raw::RawU4, prelude::*};
//!
//! /// RGBI color
//! #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
//! pub struct RGBI(RawU4);
//!
//! impl RGBI {
//!     /// Creates a RGBI color.
//!     pub fn new(red: bool, green: bool, blue: bool, intensity: bool) -> Self {
//!         let mut value = 0;
//!
//!         if red {
//!             value |= 0b0100;
//!         }
//!         if green {
//!             value |= 0b0010;
//!         }
//!         if blue {
//!             value |= 0b0001;
//!         }
//!         if intensity {
//!             value |= 0b1000;
//!         }
//!
//!         Self(RawU4::new(value))
//!     }
//! }
//!
//! /// Implement `PixelColor` to associate a raw data type with the `RGBI` struct.
//! impl PixelColor for RGBI {
//!     type Raw = RawU4;
//!
//!     fn into_raw(&self) -> <Self::Raw as RawData>::Storage {
//!         self.0.into_inner()
//!     }
//! }
//!
//! /// `From<RawU4>` is used by `Image` to construct RGBI colors.
//! impl From<RawU4> for RGBI {
//!     fn from(data: RawU4) -> Self {
//!         Self(data)
//!     }
//! }
//!
//! /// Raw image data with 2 pixels per byte.
//! #[rustfmt::skip]
//! const IMAGE_DATA: &[u8] = &[
//!     0b0001_0010,
//!     0b0100_1111,
//! ];
//!
//! fn main() {
//!     // Create new image with RGBI colors.
//!     let image: ImageRaw<RGBI> = ImageRaw::new(IMAGE_DATA, 2, 2);
//!
//!     // In a real application the image could now be drawn to a display:
//!     // display.draw(&image);
//! #
//! #   // Convert image to iterator of colors to make testing easier.
//! #   let mut iter = image.into_iter().map(|Pixel(_, color)| color);
//! #
//! #   assert_eq!(iter.next(), Some(RGBI::new(false, false, true, false)));
//! #   assert_eq!(iter.next(), Some(RGBI::new(false, true, false, false)));
//! #   assert_eq!(iter.next(), Some(RGBI::new(true, false, false, false)));
//! #   assert_eq!(iter.next(), Some(RGBI::new(true, true, true, true)));
//! #   assert_eq!(iter.next(), None);
//! }
//! ```
//!
//! [`PixelColor`]: ../trait.PixelColor.html
//! [`Raw`]: ../trait.PixelColor.html#associatedtype.Raw
//! [`Image`]: ../../image/struct.Image.html

mod iter;
pub(crate) use iter::{RawDataIter, RawDataIterNext};

/// Trait implemented by all `RawUx` types.
pub trait RawData:
    Sized
    + private::Sealed
    + RawDataIterNext<LittleEndian>
    + RawDataIterNext<BigEndian>
    + From<<Self as RawData>::Storage>
{
    /// Storage type.
    ///
    /// A primitive unsigned integer storage type that contains at least `BITS_PER_PIXEL` bits.
    type Storage;

    /// Bits per pixel.
    const BITS_PER_PIXEL: usize;

    /// Converts this raw data into the storage type.
    ///
    /// If the primitive integer types used as the storage type contains more bits
    /// than used by this type the unused most significant bits are set to `0`.
    fn into_inner(self) -> Self::Storage;

    /// Converts a `u32` into a `RawData` type.
    ///
    /// This method can be used to generically construct all `RawData` types from
    /// the same integer type. If the width of the `RawData` type is less than
    /// 32 bits only the least significant bits are used.
    fn from_u32(value: u32) -> Self;
}

/// Dummy implementation for `()`.
///
/// `()` can be used as [`PixelColor::Raw`] if raw data conversion isn't required.
///
/// [`PixelColor::Raw`]: ../trait.PixelColor.html#associatedtype.Raw
impl RawData for () {
    type Storage = ();

    const BITS_PER_PIXEL: usize = 0;

    fn into_inner(self) {}

    fn from_u32(_value: u32) {}
}

impl private::Sealed for () {}

macro_rules! impl_raw_data {
    ($type:ident : $storage_type:ident, $bpp:expr, $mask:expr, $bpp_str:expr, $doc:expr) => {
        #[doc = $bpp_str]
        #[doc = "per pixel raw data."]
        #[doc = ""]
        #[doc = $doc]
        #[doc = ""]
        #[doc = "See the [module-level documentation] for more information."]
        #[doc = ""]
        #[doc = "[module-level documentation]: index.html"]
        #[doc = "[`new`]: #method.new"]
        #[doc = "[`into_inner`]: trait.RawData.html#tymethod.into_inner"]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
        pub struct $type($storage_type);

        impl $type {
            /// Creates a new color from the least significant
            #[doc = $bpp_str]
            /// of value.
            pub const fn new(value: $storage_type) -> Self {
                $type(value & $mask)
            }
        }

        impl RawData for $type {
            type Storage = $storage_type;

            const BITS_PER_PIXEL: usize = $bpp;

            fn into_inner(self) -> Self::Storage {
                self.0
            }

            fn from_u32(value: u32) -> Self {
                #[allow(trivial_numeric_casts)]
                Self::new(value as $storage_type)
            }
        }

        impl From<$storage_type> for $type {
            fn from(value: $storage_type) -> Self {
                Self::new(value)
            }
        }

        impl private::Sealed for $type {}
    };
    ($type:ident : $storage_type:ident, $bpp:expr, $mask:expr, $bpp_str:expr) => {
        impl_raw_data!(
            $type: $storage_type,
            $bpp, $mask, $bpp_str,
            concat!(
                "`", stringify!($type), "` is internally stored in an `", stringify!($storage_type),
                "`. It can be constructed from an `", stringify!($storage_type), "` by using the ",
                "[`new`] method or by calling `",
                stringify!($type), "::from(", stringify!($storage_type),"_value)`. ",
                "To convert a `", stringify!($type), "` back into a `", stringify!($storage_type),
                "` the [`into_inner`] method can be used."
            )
        );
    };
}

impl_raw_data!(RawU1: u8, 1, 0x01, "1 bit");
impl_raw_data!(RawU2: u8, 2, 0x03, "2 bits");
impl_raw_data!(RawU4: u8, 4, 0x0F, "4 bits");
impl_raw_data!(RawU8: u8, 8, 0xFF, "8 bits");
impl_raw_data!(RawU16: u16, 16, 0xFFFF, "16 bits");
impl_raw_data!(RawU24: u32, 24, 0xFF_FFFF, "24 bits");
impl_raw_data!(RawU32: u32, 32, 0xFFFF_FFFF, "32 bits");

/// Raw data byte order.
pub trait ByteOrder: private::Sealed {}

/// Little endian byte order marker.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum LittleEndian {}

impl ByteOrder for LittleEndian {}
impl private::Sealed for LittleEndian {}

/// Big endian byte order marker.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum BigEndian {}

impl ByteOrder for BigEndian {}
impl private::Sealed for BigEndian {}

mod private {
    /// Sealed trait to prevent implementation of traits in other crates.
    pub trait Sealed {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upper_bits_are_masked() {
        assert_eq!(RawU1::new(u8::max_value()).0, 0x1);
        assert_eq!(RawU2::new(u8::max_value()).0, 0x3);
        assert_eq!(RawU4::new(u8::max_value()).0, 0xF);
        assert_eq!(RawU24::new(u32::max_value()).0, 0xFFFFFF);
    }
}
