use crate::{
    common::OutOfBoundsError,
    pixelcolor::raw::{
        order::{DataOrder, DataOrderEnum},
        RawData, RawDataBits, RawU16, RawU24, RawU32, RawU8,
    },
};

pub trait LoadStore
where
    Self: Sized,
{
    fn store<O: DataOrder<Self>>(
        value: Self,
        buffer: &mut [u8],
        index: usize,
    ) -> Result<(), OutOfBoundsError>;
    fn load<O: DataOrder<Self>>(buffer: &[u8], index: usize) -> Option<Self>;
}

#[inline]
fn bit_position<O: DataOrder<R>, R: RawDataBits>(index: usize) -> (usize, usize) {
    let pixels_per_byte = 8 / R::BITS_PER_PIXEL;

    let byte_index = index / pixels_per_byte;
    let bit_index = match O::ORDER {
        DataOrderEnum::Default => (pixels_per_byte - 1) - (index % pixels_per_byte),
        DataOrderEnum::Alternate => index % pixels_per_byte,
    } * R::BITS_PER_PIXEL;

    (byte_index, bit_index)
}

impl<R: RawDataBits> LoadStore for R {
    #[inline]
    fn store<O: DataOrder<Self>>(
        value: Self,
        buffer: &mut [u8],
        index: usize,
    ) -> Result<(), OutOfBoundsError> {
        let (byte_index, bit_index) = bit_position::<O, R>(index);

        buffer
            .get_mut(byte_index)
            .ok_or(OutOfBoundsError)
            .map(|byte| {
                *byte = (*byte & !(R::MASK << bit_index)) | (value.into_inner() << bit_index);
            })
    }

    #[inline]
    fn load<O: DataOrder<Self>>(buffer: &[u8], index: usize) -> Option<Self> {
        let (byte_index, bit_index) = bit_position::<O, R>(index);

        buffer
            .get(byte_index)
            .map(|byte| byte >> bit_index)
            .map(Into::into)
    }
}

impl LoadStore for RawU8 {
    fn store<O: DataOrder<Self>>(
        value: Self,
        buffer: &mut [u8],
        index: usize,
    ) -> Result<(), OutOfBoundsError> {
        buffer
            .get_mut(index)
            .ok_or(OutOfBoundsError)
            .map(|byte| *byte = value.0)
    }

    fn load<O: DataOrder<Self>>(buffer: &[u8], index: usize) -> Option<Self> {
        buffer.get(index).copied().map(RawU8::new)
    }
}

impl LoadStore for RawU16 {
    fn store<O: DataOrder<Self>>(
        value: Self,
        buffer: &mut [u8],
        index: usize,
    ) -> Result<(), OutOfBoundsError> {
        let bytes = match O::ORDER {
            DataOrderEnum::Default => value.into_inner().to_le_bytes(),
            DataOrderEnum::Alternate => value.into_inner().to_be_bytes(),
        };

        buffer
            .get_mut(index * 2..)
            .and_then(|buffer| buffer.get_mut(0..2))
            .ok_or(OutOfBoundsError)
            .map(|buffer| buffer.copy_from_slice(&bytes))
    }

    fn load<O: DataOrder<Self>>(buffer: &[u8], index: usize) -> Option<Self> {
        buffer
            .get(index * 2..)
            .and_then(|buffer| buffer.get(0..2))
            .map(|slice| {
                let bytes = slice.try_into().unwrap();

                let value = match O::ORDER {
                    DataOrderEnum::Default => u16::from_le_bytes(bytes),
                    DataOrderEnum::Alternate => u16::from_be_bytes(bytes),
                };

                RawU16::new(value)
            })
    }
}

impl LoadStore for RawU24 {
    fn store<O: DataOrder<Self>>(
        value: Self,
        buffer: &mut [u8],
        index: usize,
    ) -> Result<(), OutOfBoundsError> {
        let bytes = match O::ORDER {
            DataOrderEnum::Default => value.into_inner().to_le_bytes(),
            DataOrderEnum::Alternate => value.into_inner().to_be_bytes(),
        };

        buffer
            .get_mut(index * 3..)
            .and_then(|buffer| buffer.get_mut(0..3))
            .ok_or(OutOfBoundsError)
            .map(|buffer| {
                buffer.copy_from_slice({
                    match O::ORDER {
                        DataOrderEnum::Default => &bytes[0..3],
                        DataOrderEnum::Alternate => &bytes[1..4],
                    }
                })
            })
    }

    fn load<O: DataOrder<Self>>(buffer: &[u8], index: usize) -> Option<Self> {
        buffer
            .get(index * 3..)
            .and_then(|buffer| buffer.get(0..3))
            .map(|slice| {
                let bytes: [_; 3] = slice.try_into().unwrap();
                let mut bytes_extended = [0u8; 4];

                let value = match O::ORDER {
                    DataOrderEnum::Default => {
                        bytes_extended[0..3].copy_from_slice(&bytes);
                        u32::from_le_bytes(bytes_extended)
                    }
                    DataOrderEnum::Alternate => {
                        bytes_extended[1..4].copy_from_slice(&bytes);
                        u32::from_be_bytes(bytes_extended)
                    }
                };

                // The value is already masked by only copying 3 bytes.
                RawU24::new_unmasked(value)
            })
    }
}

impl LoadStore for RawU32 {
    fn store<O: DataOrder<Self>>(
        value: Self,
        buffer: &mut [u8],
        index: usize,
    ) -> Result<(), OutOfBoundsError> {
        let bytes = match O::ORDER {
            DataOrderEnum::Default => value.into_inner().to_le_bytes(),
            DataOrderEnum::Alternate => value.into_inner().to_be_bytes(),
        };

        buffer
            .get_mut(index * 4..)
            .and_then(|buffer| buffer.get_mut(0..4))
            .ok_or(OutOfBoundsError)
            .map(|buffer| buffer.copy_from_slice(&bytes))
    }

    fn load<O: DataOrder<Self>>(buffer: &[u8], index: usize) -> Option<Self> {
        buffer
            .get(index * 4..)
            .and_then(|buffer| buffer.get(0..4))
            .map(|slice| {
                let bytes = slice.try_into().unwrap();

                let value = match O::ORDER {
                    DataOrderEnum::Default => u32::from_le_bytes(bytes),
                    DataOrderEnum::Alternate => u32::from_be_bytes(bytes),
                };

                RawU32::new(value)
            })
    }
}
