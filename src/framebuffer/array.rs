use core::convert::Infallible;

use crate::{
    array::{PixelArray, PixelData, PixelDataMut},
    common::{
        buffer_size, buffer_size_with_stride, BufferDimensions, ColorType, GetPixel, Horizontal,
        OutOfBoundsError, PixelArrangement, SetPixel,
    },
    draw_target::DrawTarget,
    framebuffer::Framebuffer,
    geometry::{OriginDimensions, Point, Size},
    image::ImageRaw,
    pixelcolor::{raw::order::DataOrder, StorablePixelColor},
    Pixel,
};

/// A framebuffer stored in an array.
///
/// # Examples
///
/// ```
/// use embedded_graphics::{
///     framebuffer::ArrayFramebuffer,
///     common::{Horizontal, buffer_size},
///     pixelcolor::{Rgb565, raw::order::LittleEndian},
///     prelude::*,
///     primitives::PrimitiveStyle,
/// };
///
/// const SIZE: Size = Size::new(320, 240);
/// let mut fb = ArrayFramebuffer::<
///     { buffer_size::<Rgb565, Horizontal>(SIZE) },
///     Rgb565,
///     LittleEndian,
///     Horizontal,
/// >::new(SIZE);
///
/// fb.bounding_box()
///     .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 1))
///     .draw(&mut fb)
///     .unwrap();
/// ```
// MSRV: consider to change N to a generic argument of type `Size` when constant generic
//       expressions and custom types are stabilized
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct ArrayFramebuffer<const N: usize, C, O, A = Horizontal>
where
    C: StorablePixelColor,
    O: DataOrder<C::Raw>,
    A: PixelArrangement,
{
    data: PixelArray<C, O, N>,
    dimensions: BufferDimensions<C, A>,
}

impl<const N: usize, C, O, A> ArrayFramebuffer<N, C, O, A>
where
    C: StorablePixelColor,
    O: DataOrder<C::Raw>,
    A: PixelArrangement,
{
    /// Creates a new framebuffer.
    ///
    /// # Panics
    ///
    /// This constructor panics if the `N` parameter is too small to fit a framebuffer of the given
    /// size. Use [`buffer_size`] to determine the correct value for `N`.
    pub const fn new(size: Size) -> Self {
        let expected_buffer_size = buffer_size::<C, A>(size);

        if N < expected_buffer_size {
            panic!("invalid ArrayFramebuffer N parameter, see documentation for more information");
        }

        Self {
            data: PixelArray::new(),
            dimensions: BufferDimensions::new_unchecked(size),
        }
    }

    /// Creates a new framebuffer with a custom stride.
    ///
    /// Note that the stride is specified in pixels and not in bytes.
    ///
    /// # Panics
    ///
    /// This constructor panics if the `N` parameter is too small to fit a framebuffer of the given
    /// size. Use [`buffer_size_with_stride`] to determine the correct value for `N`.
    pub const fn with_stride(size: Size, stride: usize) -> Self {
        let expected_buffer_size = buffer_size_with_stride::<C, A>(size, stride);

        if N < expected_buffer_size {
            panic!("invalid ArrayFramebuffer N parameter, see documentation for more information");
        }

        Self {
            data: PixelArray::new(),
            dimensions: BufferDimensions::with_stride_unchecked(size, stride),
        }
    }
}

impl<const N: usize, C, O, A> ColorType for ArrayFramebuffer<N, C, O, A>
where
    C: StorablePixelColor,
    O: DataOrder<C::Raw>,
    A: PixelArrangement,
{
    type Color = C;
}

impl<const N: usize, C, O, A> Framebuffer for ArrayFramebuffer<N, C, O, A>
where
    C: StorablePixelColor,
    O: DataOrder<C::Raw>,
    A: PixelArrangement,
{
    type DataOrder = O;
    type PixelArrangement = A;

    fn data(&self) -> &[u8] {
        self.data.data()
    }

    fn data_mut(&mut self) -> &mut [u8] {
        self.data.data_mut()
    }

    fn as_image(&self) -> ImageRaw<'_, C, O, A> {
        ImageRaw::with_dimensions(&self.data(), self.dimensions)
    }
}

impl<const N: usize, C, O, A> GetPixel for ArrayFramebuffer<N, C, O, A>
where
    C: StorablePixelColor,
    O: DataOrder<C::Raw>,
    A: PixelArrangement,
{
    fn pixel(&self, point: Point) -> Option<Self::Color> {
        self.dimensions
            .index(point)
            .ok()
            .and_then(|index| self.data.get(index))
    }
}

impl<const N: usize, C, O, A> SetPixel for ArrayFramebuffer<N, C, O, A>
where
    C: StorablePixelColor,
    O: DataOrder<C::Raw>,
    A: PixelArrangement,
{
    fn try_set_pixel(&mut self, point: Point, color: Self::Color) -> Result<(), OutOfBoundsError> {
        self.dimensions
            .index(point)
            .map(|index| self.data.set(index, color))
    }
}

impl<const N: usize, C, O, A> DrawTarget for ArrayFramebuffer<N, C, O, A>
where
    C: StorablePixelColor,
    O: DataOrder<C::Raw>,
    A: PixelArrangement,
{
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(p, c) in pixels {
            self.set_pixel(p, c);
        }

        Ok(())
    }
}

impl<const N: usize, C, O, A> OriginDimensions for ArrayFramebuffer<N, C, O, A>
where
    C: StorablePixelColor,
    O: DataOrder<C::Raw>,
    A: PixelArrangement,
{
    fn size(&self) -> Size {
        self.dimensions.size()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        common::{Horizontal, Vertical},
        framebuffer::test_common::*,
        geometry::{Dimensions, Point},
        image::Image,
        mock_display::MockDisplay,
        pixelcolor::{
            raw::order::{BigEndian, LittleEndian, Lsb0, Msb0},
            BinaryColor, Gray4,
        },
        primitives::{PointsIter, Primitive, PrimitiveStyle},
        Drawable,
    };

    macro_rules! impl_test {
        ($name:ident, $test:ty, ($size:expr), <$order:ty, $arrangement:ty>) => {
            #[test]
            fn $name() {
                const SIZE: Size = $size;
                let mut fb = ArrayFramebuffer::<
                    { buffer_size::<<$test as FbTest>::Color, $arrangement>(SIZE) },
                    <$test as FbTest>::Color,
                    $order,
                    $arrangement,
                >::new(SIZE);

                <$test>::test(&mut fb)
            }
        };

        ($name:ident, $test:ty, <$order:ty, Horizontal>) => {
            impl_test!($name, $test, (<$test as FbTest>::HORIZONTAL_SIZE), <$order, Horizontal>);
        };

        ($name:ident, $test:ty, <$order:ty, Vertical>) => {
            impl_test!($name, $test, (<$test as FbTest>::HORIZONTAL_SIZE.swap_xy()), <$order, Vertical>);
        };
    }

    impl_framebuffer_tests!();

    #[test]
    fn as_image() {
        const SIZE: Size = Size::new(10, 10);
        let mut fb = ArrayFramebuffer::<
            { buffer_size::<BinaryColor, Horizontal>(SIZE) },
            BinaryColor,
            Msb0,
        >::new(SIZE);

        fb.bounding_box()
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut fb)
            .unwrap();

        fb.draw_iter(
            [(7, 2), (8, 8)]
                .into_iter()
                .map(|p| Pixel(Point::from(p), BinaryColor::On)),
        )
        .unwrap();

        let mut display = MockDisplay::<BinaryColor>::new();
        Image::new(&fb.as_image(), Point::new(2, 1))
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "            ",
            "  ##########",
            "  #........#",
            "  #......#.#",
            "  #........#",
            "  #........#",
            "  #........#",
            "  #........#",
            "  #........#",
            "  #.......##",
            "  ##########",
        ]);
    }

    #[test]
    fn pixel() {
        const SIZE: Size = Size::new(10, 10);
        let mut fb = ArrayFramebuffer::<
            { buffer_size::<BinaryColor, Horizontal>(SIZE) },
            BinaryColor,
            Msb0,
        >::new(SIZE);

        fb.draw_iter(
            [(7, 2), (8, 8)]
                .into_iter()
                .map(|p| Pixel(Point::from(p), BinaryColor::On)),
        )
        .unwrap();

        let expected = [
            ((0, 0), Some(BinaryColor::Off)),
            ((1, 0), Some(BinaryColor::Off)),
            ((1, 1), Some(BinaryColor::Off)),
            ((7, 2), Some(BinaryColor::On)),
            ((8, 8), Some(BinaryColor::On)),
            ((-1, 0), None),
            ((0, -1), None),
            ((10, 0), None),
            ((0, 10), None),
        ];
        for (p, c) in expected {
            assert_eq!(fb.pixel(p.into()), c, "{p:?}");
        }
    }

    #[test]
    fn oversized_buffer() {
        const SIZE: Size = Size::new(10, 5);
        let fb = ArrayFramebuffer::<
            { buffer_size::<BinaryColor, Horizontal>(SIZE) * 2 },
            BinaryColor,
            Msb0,
        >::new(SIZE);

        assert_eq!(fb.size(), Size::new(10, 5));
        assert_eq!(fb.as_image().size(), Size::new(10, 5));

        let outside_x = Point::zero() + fb.size().x_axis();
        let outside_y = Point::zero() + fb.size().y_axis();

        assert_eq!(fb.pixel(outside_x), None);
        assert_eq!(fb.pixel(outside_y), None);

        let mut fb2 = fb.clone();
        fb2.set_pixel(outside_x, BinaryColor::On);
        fb2.set_pixel(outside_y, BinaryColor::On);

        assert_eq!(fb, fb2);
    }

    #[test]
    fn with_stride() {
        let mut fb = ArrayFramebuffer::<3, Gray4, Msb0>::with_stride(Size::new(3, 2), 3);
        fb.draw_iter(
            fb.bounding_box()
                .points()
                .enumerate()
                .map(|(c, p)| Pixel(p, Gray4::new(c as u8))),
        )
        .unwrap();

        assert_eq!(fb.data(), &[0x01, 0x23, 0x45]);
    }
}
