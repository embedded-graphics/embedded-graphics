//! Common types.

use core::marker::PhantomData;

use crate::{
    geometry::{Point, Size},
    pixelcolor::{raw::RawData, StorablePixelColor},
};

pub use embedded_graphics_core::common::{ColorType, GetPixel, OutOfBoundsError, SetPixel};

/// Buffer error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub enum BufferError {
    /// Truncated data.
    TruncatedData {
        /// The expected buffer size in bytes.
        expected_buffer_size: usize,
    },
    /// Invalid stride.
    InvalidStride {
        /// The minimum stride in pixels.
        minimum_stride: usize,
    },
}

/// Buffer dimensions.
#[derive(Eq, Clone, Copy, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub(crate) struct BufferDimensions<C, A = Horizontal> {
    size: Size,
    stride: usize,
    color_type: PhantomData<C>,
    pixel_arrangement: PhantomData<A>,
}

pub(crate) const fn pixels_to_bytes<C: StorablePixelColor>(pixels: usize) -> usize {
    if C::Raw::BITS_PER_PIXEL < 8 {
        (pixels * C::Raw::BITS_PER_PIXEL + 7) / 8
    } else {
        pixels * (C::Raw::BITS_PER_PIXEL / 8)
    }
}

pub(crate) const fn bytes_to_pixels<C: StorablePixelColor>(bytes: usize) -> usize {
    if C::Raw::BITS_PER_PIXEL >= 8 {
        bytes / (C::Raw::BITS_PER_PIXEL / 8)
    } else {
        bytes * (8 / C::Raw::BITS_PER_PIXEL)
    }
}

impl<C: StorablePixelColor, A: PixelArrangement> BufferDimensions<C, A> {
    pub const fn with_stride_unchecked(size: Size, stride: usize) -> Self {
        Self {
            size,
            stride,
            color_type: PhantomData,
            pixel_arrangement: PhantomData,
        }
    }

    /// Creates new buffer dimensions without checking.
    pub const fn new_unchecked(size: Size) -> Self {
        let mut self_ = Self::with_stride_unchecked(size, 0);
        self_.stride = default_stride::<C, A>(size);

        self_
    }

    /// Creates new buffer dimensions with the default stride.
    ///
    /// Returns an error if the buffer is too small or the stride is invalid.
    pub const fn new(buffer: &[u8], size: Size) -> Result<Self, BufferError> {
        let self_ = Self::new_unchecked(size);

        match self_.check(buffer) {
            Ok(_) => Ok(self_),
            Err(e) => Err(e),
        }
    }

    /// Creates buffer dimensions with a custom stride.
    pub const fn with_stride(
        buffer: &[u8],
        size: Size,
        stride: usize,
    ) -> Result<Self, BufferError> {
        let self_ = Self::with_stride_unchecked(size, stride);

        match self_.check(buffer) {
            Ok(_) => Ok(self_),
            Err(e) => Err(e),
        }
    }

    /// Returns the size.
    pub const fn size(&self) -> Size {
        self.size
    }

    /// Returns the stride in pixels.
    pub const fn stride(&self) -> usize {
        self.stride
    }

    /// Checks that the buffer length and stride is valid.
    pub const fn check(&self, buffer: &[u8]) -> Result<(), BufferError> {
        let main_size = A::ARRANGEMENT.size_to_main(self.size);
        let cross_size = A::ARRANGEMENT.size_to_cross(self.size);

        if self.stride < main_size as usize {
            return Err(BufferError::InvalidStride {
                minimum_stride: main_size as usize,
            });
        }

        let expected_pixels = self.stride * cross_size as usize;
        let expected_bytes = pixels_to_bytes::<C>(expected_pixels);

        if buffer.len() < expected_bytes {
            return Err(BufferError::TruncatedData {
                expected_buffer_size: expected_bytes,
            });
        }

        Ok(())
    }

    // TODO: ensure that size.width and size.height are small enough
    pub const fn index(&self, point: Point) -> Result<usize, OutOfBoundsError> {
        // point.x and point.y are implicitly also checked to be >= 0:
        // Casting a negative i32 value to u32 will result in a value >= 0x80000000, which is
        // larger thant the maximum allowed buffer size.
        if point.x as u32 >= self.size.width || point.y as u32 >= self.size.height {
            return Err(OutOfBoundsError);
        }

        let (main, cross) = A::ARRANGEMENT.point_to_main_cross(point);

        Ok(main as usize + cross as usize * self.stride)
    }
}

const fn default_stride<C: StorablePixelColor, A: PixelArrangement>(size: Size) -> usize {
    let mut stride = A::ARRANGEMENT.size_to_main(size) as usize;

    if C::Raw::BITS_PER_PIXEL < 8 {
        let pixels_per_bit = 8 / C::Raw::BITS_PER_PIXEL;
        stride = (stride + (pixels_per_bit - 1)) / pixels_per_bit * pixels_per_bit;
    }

    stride
}

/// Calculates the required buffer size.
///
/// This function is a workaround for current limitations in Rust const generics.  It can be used to
/// calculate the `N` parameter for [`ArrayFramebuffer`] based on the size and color type of the
/// framebuffer.
///
/// [`ArrayFramebuffer`]: crate::framebuffer::ArrayFramebuffer
pub const fn buffer_size<C: StorablePixelColor, A: PixelArrangement>(size: Size) -> usize {
    buffer_size_with_stride::<C, A>(size, default_stride::<C, A>(size))
}

/// Calculates the required buffer size with a custom stride.
pub const fn buffer_size_with_stride<C: StorablePixelColor, A: PixelArrangement>(
    size: Size,
    stride: usize,
) -> usize {
    let pixels = stride * A::ARRANGEMENT.size_to_cross(size) as usize;

    pixels_to_bytes::<C>(pixels)
}

/// Pixel arrangement.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
#[non_exhaustive]
pub enum PixelArrangementEnum {
    /// Horizontal arrangement.
    Horizontal,
    /// Vertical arrangement.
    Vertical,
}

impl PixelArrangementEnum {
    pub(crate) const fn size_to_main(self, size: Size) -> u32 {
        match self {
            PixelArrangementEnum::Horizontal => size.width,
            PixelArrangementEnum::Vertical => size.height,
        }
    }

    pub(crate) const fn size_to_cross(self, size: Size) -> u32 {
        match self {
            PixelArrangementEnum::Horizontal => size.height,
            PixelArrangementEnum::Vertical => size.width,
        }
    }

    pub(crate) const fn point_to_main_cross(self, point: Point) -> (i32, i32) {
        match self {
            PixelArrangementEnum::Horizontal => (point.x, point.y),
            PixelArrangementEnum::Vertical => (point.y, point.x),
        }
    }
}

/// Pixel arrangement.
pub trait PixelArrangement: private::Sealed + Copy {
    /// Pixel arrangement.
    const ARRANGEMENT: PixelArrangementEnum;
}

/// Horizontal pixel arrangement.
///
/// The pixel data is arranged in rows starting from the top left corner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub enum Horizontal {}

impl PixelArrangement for Horizontal {
    const ARRANGEMENT: PixelArrangementEnum = PixelArrangementEnum::Horizontal;
}
impl private::Sealed for Horizontal {}

/// Vertical pixel arrangement.
///
/// The pixel data is arranged in rows starting from the top left corner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub enum Vertical {}
impl PixelArrangement for Vertical {
    const ARRANGEMENT: PixelArrangementEnum = PixelArrangementEnum::Vertical;
}
impl private::Sealed for Vertical {}

mod private {
    pub trait Sealed {}
}
#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    use crate::pixelcolor::{raw::RawU32, PixelColor};

    // TODO: remove when a 32 BPP color type is added to e-g
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct U32Color(pub u32);

    impl PixelColor for U32Color {}

    impl StorablePixelColor for U32Color {
        type Raw = RawU32;
    }

    impl From<RawU32> for U32Color {
        fn from(raw: RawU32) -> Self {
            Self(raw.into_inner())
        }
    }

    impl From<U32Color> for RawU32 {
        fn from(color: U32Color) -> Self {
            Self::new(color.0)
        }
    }
}
