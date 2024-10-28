use super::{
    DataOrder, OutOfBoundsError, RawData, RawU1, RawU16, RawU2, RawU24, RawU32, RawU4, RawU8,
};

pub(crate) trait LoadStore<O: DataOrder>: Sized {
    fn load(buffer: &[u8], index: usize) -> Option<Self>;
    fn store(self, buffer: &mut [u8], index: usize) -> Result<(), OutOfBoundsError>;
}

#[inline]
fn bit_position<R: RawData, O: DataOrder>(index: usize) -> (usize, usize) {
    let pixels_per_byte = 8 / R::BITS_PER_PIXEL;

    let byte_index = index / pixels_per_byte;
    let bit_index = if O::IS_ALTERNATE_ORDER {
        index % pixels_per_byte
    } else {
        (pixels_per_byte - 1) - (index % pixels_per_byte)
    } * R::BITS_PER_PIXEL;

    (byte_index, bit_index)
}

macro_rules! impl_load_store_bits {
    ($raw_type:ty) => {
        impl<O: DataOrder> LoadStore<O> for $raw_type {
            #[inline]
            fn load(buffer: &[u8], index: usize) -> Option<Self> {
                let (byte_index, bit_index) = bit_position::<Self, O>(index);

                buffer
                    .get(byte_index)
                    .map(|byte| byte >> bit_index)
                    .map(Into::into)
            }

            #[inline]
            fn store(self, buffer: &mut [u8], index: usize) -> Result<(), OutOfBoundsError> {
                let (byte_index, bit_index) = bit_position::<Self, O>(index);

                buffer
                    .get_mut(byte_index)
                    .ok_or(OutOfBoundsError)
                    .map(|byte| {
                        *byte =
                            (*byte & !(Self::MASK << bit_index)) | (self.into_inner() << bit_index);
                    })
            }
        }
    };
}

impl_load_store_bits!(RawU1);
impl_load_store_bits!(RawU2);
impl_load_store_bits!(RawU4);

impl<O: DataOrder> LoadStore<O> for RawU8 {
    fn load(buffer: &[u8], index: usize) -> Option<Self> {
        buffer.get(index).copied().map(Self::new)
    }

    fn store(self, buffer: &mut [u8], index: usize) -> Result<(), OutOfBoundsError> {
        buffer
            .get_mut(index)
            .ok_or(OutOfBoundsError)
            .map(|byte| *byte = self.0)
    }
}

impl<O: DataOrder> LoadStore<O> for RawU16 {
    fn load(buffer: &[u8], index: usize) -> Option<Self> {
        buffer
            .get(index * 2..)
            .and_then(|buffer| buffer.get(0..2))
            .map(|slice| {
                let bytes = slice.try_into().unwrap();

                let value = if O::IS_ALTERNATE_ORDER {
                    u16::from_be_bytes(bytes)
                } else {
                    u16::from_le_bytes(bytes)
                };

                Self::new(value)
            })
    }

    fn store(self, buffer: &mut [u8], index: usize) -> Result<(), OutOfBoundsError> {
        let bytes = self.into_inner().to_le_bytes();

        buffer
            .get_mut(index * 2..)
            .and_then(|buffer| buffer.get_mut(0..2))
            .ok_or(OutOfBoundsError)
            .map(|buffer| buffer.copy_from_slice(&bytes))
    }
}

impl<O: DataOrder> LoadStore<O> for RawU24 {
    fn load(buffer: &[u8], index: usize) -> Option<Self> {
        buffer
            .get(index * 3..)
            .and_then(|buffer| buffer.get(0..3))
            .map(|slice| {
                let bytes: [_; 3] = slice.try_into().unwrap();
                let mut bytes_extended = [0u8; 4];

                let value = if O::IS_ALTERNATE_ORDER {
                    bytes_extended[1..4].copy_from_slice(&bytes);
                    u32::from_be_bytes(bytes_extended)
                } else {
                    bytes_extended[0..3].copy_from_slice(&bytes);
                    u32::from_le_bytes(bytes_extended)
                };

                // The value is already masked by only copying 3 bytes.
                Self::new_unmasked(value)
            })
    }

    fn store(self, buffer: &mut [u8], index: usize) -> Result<(), OutOfBoundsError> {
        let bytes = self.into_inner().to_le_bytes();

        buffer
            .get_mut(index * 3..)
            .and_then(|buffer| buffer.get_mut(0..3))
            .ok_or(OutOfBoundsError)
            .map(|buffer| buffer.copy_from_slice(&bytes[0..3]))
    }
}

impl<O: DataOrder> LoadStore<O> for RawU32 {
    fn load(buffer: &[u8], index: usize) -> Option<Self> {
        buffer
            .get(index * 4..)
            .and_then(|buffer| buffer.get(0..4))
            .map(|slice| {
                let bytes = slice.try_into().unwrap();

                let value = if O::IS_ALTERNATE_ORDER {
                    u32::from_be_bytes(bytes)
                } else {
                    u32::from_le_bytes(bytes)
                };

                Self::new(value)
            })
    }

    fn store(self, buffer: &mut [u8], index: usize) -> Result<(), OutOfBoundsError> {
        let bytes = self.into_inner().to_le_bytes();

        buffer
            .get_mut(index * 4..)
            .and_then(|buffer| buffer.get_mut(0..4))
            .ok_or(OutOfBoundsError)
            .map(|buffer| buffer.copy_from_slice(&bytes))
    }
}
