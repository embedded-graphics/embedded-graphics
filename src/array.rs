//! Pixel arrays and slices.
//!
//! Pixel arrays and slices can be used to store pixel data as a tightly packed sequence of bytes.
//! For color types with less then 8 bits per pixel multiple pixels will be stored in a single byte.
//!
//! Most applications won't use the types in this module directly and instead use higher level
//! objects, like [framebuffers](crate::framebuffer) or [raw images](crate::image::ImageRaw).

use core::marker::PhantomData;

use crate::{
    common::{bytes_to_pixels, ColorType, OutOfBoundsError},
    pixelcolor::{
        raw::{order::DataOrder, RawData},
        StorablePixelColor,
    },
};

/// Common functions for pixel slices and arrays.
pub trait PixelData: ColorType
where
    Self::Color: StorablePixelColor,
{
    /// The data order.
    type Order: DataOrder<<Self::Color as StorablePixelColor>::Raw>;

    /// Returns the raw pixel data.
    fn data(&self) -> &[u8];

    /// Returns the length in pixels.
    fn len(&self) -> usize {
        bytes_to_pixels::<Self::Color>(self.data().len())
    }

    /// Returns the color at a given index.
    ///
    /// `None` will be returned if the index is `>= len()`.
    #[inline]
    fn get(&self, index: usize) -> Option<Self::Color> {
        <Self::Color as StorablePixelColor>::Raw::load::<Self::Order>(self.data(), index)
            .map(Into::into)
    }
}

/// Common functions for mutable pixel slices and arrays.
pub trait PixelDataMut: PixelData
where
    Self::Color: StorablePixelColor,
{
    /// Returns a mutable slice of raw pixel data.
    fn data_mut(&mut self) -> &mut [u8];

    /// Sets the color at a given index.
    ///
    /// Trying to set out of bounds pixels is a noop. Use [`try_set`](Self::try_set) to detect
    /// invalid index errors.
    #[inline]
    fn set(&mut self, index: usize, color: Self::Color) {
        self.try_set(index, color).ok();
    }

    /// Sets the color at a given index.
    ///
    /// Returns an error if the index is out of bounds.
    #[inline]
    fn try_set(&mut self, index: usize, color: Self::Color) -> Result<(), OutOfBoundsError> {
        let raw: <Self::Color as StorablePixelColor>::Raw = color.into();
        raw.store::<Self::Order>(self.data_mut(), index)
    }
}

/// Pixel array.
#[derive(Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct PixelArray<C: StorablePixelColor, O: DataOrder<C::Raw>, const N: usize> {
    data: [u8; N],
    color_type: PhantomData<C>,
    order: PhantomData<O>,
}

impl<C: StorablePixelColor, O: DataOrder<C::Raw>, const N: usize> PixelArray<C, O, N> {
    /// Creates a new pixel array.
    pub const fn new() -> Self {
        Self {
            data: [0u8; N],
            color_type: PhantomData,
            order: PhantomData,
        }
    }

    /// Creates a new pixel from raw data.
    pub const fn with_data(data: [u8; N]) -> Self {
        Self {
            data,
            color_type: PhantomData,
            order: PhantomData,
        }
    }

    /// Returns a slice for this array.
    #[inline]
    pub const fn as_slice(&self) -> PixelSlice<'_, C, O> {
        PixelSlice::new(&self.data)
    }

    /// Returns a mutable slice for this array.
    #[inline]
    pub fn as_mut_slice(&mut self) -> PixelMutSlice<'_, C, O> {
        PixelMutSlice::new(self.data_mut())
    }
}

impl<C: StorablePixelColor, O: DataOrder<C::Raw>, const N: usize> ColorType
    for PixelArray<C, O, N>
{
    type Color = C;
}

impl<C: StorablePixelColor, O: DataOrder<C::Raw>, const N: usize> PixelData
    for PixelArray<C, O, N>
{
    type Order = O;

    #[inline]
    fn data(&self) -> &[u8] {
        &self.data
    }
}

impl<C: StorablePixelColor, O: DataOrder<C::Raw>, const N: usize> PixelDataMut
    for PixelArray<C, O, N>
{
    #[inline]
    fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}

/// Pixel slice.
#[derive(Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct PixelSlice<'a, C, O> {
    data: &'a [u8],
    color_type: PhantomData<C>,
    order: PhantomData<O>,
}

impl<'a, C: StorablePixelColor, O: DataOrder<C::Raw>> PixelSlice<'a, C, O> {
    /// Creates a new pixel slice.
    pub const fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            color_type: PhantomData,
            order: PhantomData,
        }
    }
}

impl<'a, C: StorablePixelColor, O: DataOrder<C::Raw>> ColorType for PixelSlice<'a, C, O> {
    type Color = C;
}

impl<'a, C: StorablePixelColor, O: DataOrder<C::Raw>> PixelData for PixelSlice<'a, C, O> {
    type Order = O;

    #[inline]
    fn data(&self) -> &[u8] {
        &self.data
    }
}

/// Mutable pixel slice.
#[derive(PartialEq, PartialOrd, Ord, Eq, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct PixelMutSlice<'a, C, O> {
    data: &'a mut [u8],
    color_type: PhantomData<C>,
    order: PhantomData<O>,
}

impl<'a, C: StorablePixelColor, O: DataOrder<C::Raw>> PixelMutSlice<'a, C, O> {
    /// Creates a new mutable pixel slice.
    pub fn new(data: &'a mut [u8]) -> Self {
        Self {
            data,
            color_type: PhantomData,
            order: PhantomData,
        }
    }
}

impl<'a, C: StorablePixelColor, O: DataOrder<C::Raw>> ColorType for PixelMutSlice<'a, C, O> {
    type Color = C;
}

impl<'a, C: StorablePixelColor, O: DataOrder<C::Raw>> PixelData for PixelMutSlice<'a, C, O> {
    type Order = O;

    #[inline]
    fn data(&self) -> &[u8] {
        &self.data
    }
}

impl<'a, C: StorablePixelColor, O: DataOrder<C::Raw>> PixelDataMut for PixelMutSlice<'a, C, O> {
    #[inline]
    fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        common::tests::U32Color,
        pixelcolor::{
            raw::order::{BigEndian, LittleEndian, Lsb0, Msb0},
            BinaryColor, Gray2, Gray4, Gray8, Rgb565, Rgb888,
        },
    };

    // Static checks that the constructors are const.
    const _A: PixelArray<BinaryColor, Msb0, 4> = PixelArray::new();
    const _B: PixelArray<BinaryColor, Msb0, 4> = PixelArray::with_data([0; 4]);

    fn test_len<C: StorablePixelColor, O: DataOrder<C::Raw>, const N: usize>(expected_len: usize) {
        let mut array = PixelArray::<C, O, N>::new();
        assert_eq!(array.len(), expected_len);
        assert_eq!(array.as_slice().len(), expected_len);
        assert_eq!(array.as_mut_slice().len(), expected_len);
    }

    #[test]
    fn len() {
        test_len::<BinaryColor, Msb0, 1>(8);
        test_len::<Gray2, Lsb0, 1>(4);
        test_len::<Gray4, Msb0, 1>(2);
        test_len::<Gray8, LittleEndian, 1>(1);
        test_len::<Rgb565, BigEndian, 1>(0);
        test_len::<Rgb888, LittleEndian, 1>(0);

        test_len::<BinaryColor, Lsb0, 4>(32);
        test_len::<Gray2, Msb0, 4>(16);
        test_len::<Gray4, Lsb0, 4>(8);
        test_len::<Gray8, BigEndian, 4>(4);
        test_len::<Rgb565, LittleEndian, 4>(2);
        test_len::<Rgb888, BigEndian, 4>(1);
    }

    fn test_get<C: StorablePixelColor + core::fmt::Debug, O: DataOrder<C::Raw>, const N: usize>(
        data: [u8; N],
        expected_colors: &[C],
    ) {
        let mut array = PixelArray::<C, O, N>::with_data(data);
        for (index, expected) in expected_colors.iter().copied().enumerate() {
            assert_eq!(array.get(index), Some(expected), "{}", index);
        }
        assert_eq!(array.get(expected_colors.len()), None);

        let slice = array.as_slice();
        for (index, expected) in expected_colors.iter().copied().enumerate() {
            assert_eq!(slice.get(index), Some(expected), "{}", index);
        }
        assert_eq!(slice.get(expected_colors.len()), None);

        let mut_slice = array.as_mut_slice();
        for (index, expected) in expected_colors.iter().copied().enumerate() {
            assert_eq!(mut_slice.get(index), Some(expected), "{}", index);
        }
        assert_eq!(mut_slice.get(expected_colors.len()), None);
    }

    fn test_set<C: StorablePixelColor + core::fmt::Debug, O: DataOrder<C::Raw>, const N: usize>(
        colors: &[C],
        expected_data: [u8; N],
    ) {
        let mut array = PixelArray::<C, O, N>::new();
        for (index, color) in colors.iter().copied().enumerate() {
            assert!(array.try_set(index, color).is_ok(), "{}", index);
        }
        assert_eq!(
            array.try_set(colors.len(), colors[0]),
            Err(OutOfBoundsError)
        );
        assert_eq!(array.data(), &expected_data);

        let mut array = PixelArray::<C, O, N>::new();
        let mut mut_slice = array.as_mut_slice();
        for (index, color) in colors.iter().copied().enumerate() {
            assert!(mut_slice.try_set(index, color).is_ok(), "{}", index);
        }
        assert_eq!(
            mut_slice.try_set(colors.len(), colors[0]),
            Err(OutOfBoundsError)
        );
        assert_eq!(mut_slice.data(), &expected_data);
    }

    fn test_set_and_get<
        C: StorablePixelColor + core::fmt::Debug,
        O: DataOrder<C::Raw>,
        const N: usize,
    >(
        data: [u8; N],
        colors: &[C],
    ) {
        test_get::<C, O, N>(data, colors);
        test_set::<C, O, N>(colors, data);
    }

    #[test]
    fn test_1bpp() {
        use BinaryColor::{Off, On};

        test_set_and_get::<BinaryColor, Msb0, 1>(
            [0b00011011],
            &[Off, Off, Off, On, On, Off, On, On],
        );
        test_set_and_get::<BinaryColor, Lsb0, 2>(
            [0b00011011, 0b10000000],
            &[
                On, On, Off, On, On, Off, Off, Off, //
                Off, Off, Off, Off, Off, Off, Off, On, //
            ],
        );
    }

    #[test]
    fn test_2bpp() {
        let g = Gray2::new;

        test_set_and_get::<Gray2, Msb0, 1>([0b00_01_10_11], &[g(0b00), g(0b01), g(0b10), g(0b11)]);
        test_set_and_get::<Gray2, Lsb0, 2>(
            [0b00_01_10_11, 0b10_00_00_00],
            &[
                g(0b11),
                g(0b10),
                g(0b01),
                g(0b00),
                g(0b00),
                g(0b00),
                g(0b00),
                g(0b10),
            ],
        );
    }

    #[test]
    fn test_4bpp() {
        let g = Gray4::new;

        test_set_and_get::<Gray4, Msb0, 1>([0b00011011], &[g(0b0001), g(0b1011)]);
        test_set_and_get::<Gray4, Lsb0, 2>(
            [0b00011011, 0b10000000],
            &[g(0b1011), g(0b0001), g(0b0000), g(0b1000)],
        );
    }

    #[test]
    fn test_8bpp() {
        let g = Gray8::new;

        test_set_and_get::<Gray8, LittleEndian, 1>([0xAA], &[g(0xAA)]);
        test_set_and_get::<Gray8, BigEndian, 2>([0xAA, 0xBB], &[g(0xAA), g(0xBB)]);
    }

    #[test]
    fn test_16bpp() {
        let rgb = Rgb565::new;

        test_set_and_get::<Rgb565, LittleEndian, 2>([0b010_00011, 0b00001_000], &[rgb(1, 2, 3)]);
        test_set_and_get::<Rgb565, BigEndian, 4>(
            [0b00001_000, 0b010_00011, 0xFF, 0xFF],
            &[rgb(1, 2, 3), rgb(31, 63, 31)],
        );
    }

    #[test]
    fn test_24bpp() {
        let rgb = Rgb888::new;

        test_set_and_get::<Rgb888, LittleEndian, 3>([3, 2, 1], &[rgb(1, 2, 3)]);
        test_set_and_get::<Rgb888, BigEndian, 6>([1, 2, 3, 4, 5, 6], &[rgb(1, 2, 3), rgb(4, 5, 6)]);
    }

    #[test]
    fn test_32bpp() {
        test_set_and_get::<U32Color, LittleEndian, 4>([4, 3, 2, 1], &[U32Color(0x0001020304)]);
        test_set_and_get::<U32Color, BigEndian, 8>(
            [1, 2, 3, 4, 5, 6, 7, 8],
            &[U32Color(0x01020304), U32Color(0x05060708)],
        );
    }
}
