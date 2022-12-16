use core::convert::Infallible;

use crate::{
    array::{PixelData, PixelDataMut, PixelMutSlice},
    common::{
        BufferDimensions, BufferError, GetPixel, Horizontal, OutOfBoundsError, PixelArrangement,
        SetPixel,
    },
    draw_target::DrawTarget,
    framebuffer::Framebuffer,
    geometry::{OriginDimensions, Point, Size},
    image::ImageRaw,
    pixelcolor::{raw::order::DataOrder, StorablePixelColor},
    Pixel,
};

/// A framebuffer stored in a slice.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SliceFramebuffer<'a, C, O, A = Horizontal>
where
    C: StorablePixelColor,
    O: DataOrder<C::Raw>,
    A: PixelArrangement,
{
    data: PixelMutSlice<'a, C, O>,
    dimensions: BufferDimensions<C, A>,
}

impl<'a, C, O, A> SliceFramebuffer<'a, C, O, A>
where
    C: StorablePixelColor,
    O: DataOrder<C::Raw>,
    A: PixelArrangement,
{
    /// Creates a new framebuffer.
    ///
    /// Returns an error if the buffer size is insufficient for the given framebuffer dimensions.
    pub fn new(data: &'a mut [u8], size: Size) -> Result<Self, BufferError> {
        let dimensions = BufferDimensions::new(data, size)?;
        let data = PixelMutSlice::new(data);

        Ok(Self { data, dimensions })
    }

    /// Creates a new framebuffer with a custom stride.
    /// 
    /// Note that the stride is specified in pixels and not in bytes.
    ///
    /// Returns an error if the buffer size is insufficient for the given framebuffer dimensions.
    pub fn with_stride(data: &'a mut [u8], size: Size, stride: usize) -> Result<Self, BufferError> {
        let dimensions = BufferDimensions::with_stride(data, size, stride)?;
        let data = PixelMutSlice::new(data);

        Ok(Self { data, dimensions })
    }
}

impl<'a, C, O, A> Framebuffer for SliceFramebuffer<'a, C, O, A>
where
    C: StorablePixelColor,
    O: DataOrder<C::Raw>,
    A: PixelArrangement,
{
    type Color = C;
    type DataOrder = O;
    type PixelArrangement = A;

    fn data(&self) -> &[u8] {
        self.data.data()
    }

    fn data_mut(&mut self) -> &mut [u8] {
        self.data.data_mut()
    }

    fn as_image(&self) -> ImageRaw<'_, Self::Color, Self::DataOrder, Self::PixelArrangement> {
        ImageRaw::with_dimensions(&self.data(), self.dimensions)
    }
}

impl<'a, C, O, A> GetPixel<C> for SliceFramebuffer<'a, C, O, A>
where
    C: StorablePixelColor,
    O: DataOrder<C::Raw>,
    A: PixelArrangement,
{
    fn pixel(&self, point: Point) -> Option<C> {
        self.dimensions
            .index(point)
            .ok()
            .and_then(|index| self.data.get(index))
    }
}

impl<'a, C, O, A> SetPixel<C> for SliceFramebuffer<'a, C, O, A>
where
    C: StorablePixelColor,
    O: DataOrder<C::Raw>,
    A: PixelArrangement,
{
    fn try_set_pixel(&mut self, point: Point, color: C) -> Result<(), OutOfBoundsError> {
        self.dimensions
            .index(point)
            .map(|index| self.data.set(index, color))
    }
}

impl<'a, C, O, A> DrawTarget for SliceFramebuffer<'a, C, O, A>
where
    C: StorablePixelColor,
    O: DataOrder<C::Raw>,
    A: PixelArrangement,
{
    type Color = C;
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

impl<'a, C, O, A> OriginDimensions for SliceFramebuffer<'a, C, O, A>
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
    use embedded_graphics_core::primitives::PointsIter;

    use super::*;

    use crate::{
        common::{buffer_size, Vertical},
        framebuffer::test_common::*,
        geometry::{Dimensions, Point},
        image::Image,
        mock_display::MockDisplay,
        pixelcolor::{
            raw::order::{BigEndian, LittleEndian, Lsb0, Msb0},
            BinaryColor, Gray4, Gray8,
        },
        primitives::{Primitive, PrimitiveStyle},
        Drawable,
    };

    macro_rules! impl_test {
        ($name:ident, $test:ty, ($size:expr), <$order:ty, $arrangement:ty>) => {
            #[test]
            fn $name() {
                const SIZE: Size = $size;
                let mut data = [0; { buffer_size::<<$test as FbTest>::Color, $arrangement>(SIZE) }];
                let mut fb = SliceFramebuffer::<<$test as FbTest>::Color, $order, $arrangement>::new(&mut data, SIZE).unwrap();

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
        let mut data = [0; 128];
        let mut fb =
            SliceFramebuffer::<BinaryColor, Msb0>::new(&mut data, Size::new(10, 10)).unwrap();

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
        let mut data = [0u8; 128];
        let mut fb =
            SliceFramebuffer::<BinaryColor, Msb0>::new(&mut data, Size::new(10, 10)).unwrap();

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
    fn undersized_buffer() {
        let mut data = [];
        assert_eq!(
            SliceFramebuffer::<Gray8, LittleEndian>::new(&mut data, Size::new(3, 3)),
            Err(BufferError::TruncatedData {
                expected_buffer_size: 3 * 3
            })
        );
    }

    #[test]
    fn oversized_buffer() {
        const SIZE: Size = Size::new(10, 5);
        let mut data = [0u8; { buffer_size::<BinaryColor, Horizontal>(SIZE) * 2 }];
        let fb = SliceFramebuffer::<BinaryColor, Msb0>::new(&mut data, SIZE).unwrap();

        assert_eq!(fb.size(), Size::new(10, 5));
        assert_eq!(fb.as_image().size(), Size::new(10, 5));

        let outside_x = Point::zero() + fb.size().x_axis();
        let outside_y = Point::zero() + fb.size().y_axis();

        assert_eq!(fb.pixel(outside_x), None);
        assert_eq!(fb.pixel(outside_y), None);

        let mut data2 = data.clone();
        let mut fb2 = SliceFramebuffer::<BinaryColor, Msb0>::new(&mut data2, SIZE).unwrap();
        fb2.set_pixel(outside_x, BinaryColor::On);
        fb2.set_pixel(outside_y, BinaryColor::On);

        assert_eq!(data, data2);
    }

    #[test]
    fn with_stride() {
        let mut data = [0; 3];
        let mut fb =
            SliceFramebuffer::<Gray4, Msb0>::with_stride(&mut data, Size::new(3, 2), 3).unwrap();
        fb.draw_iter(
            fb.bounding_box()
                .points()
                .enumerate()
                .map(|(c, p)| Pixel(p, Gray4::new(c as u8))),
        )
        .unwrap();

        assert_eq!(&data, &[0x01, 0x23, 0x45]);
    }
}
