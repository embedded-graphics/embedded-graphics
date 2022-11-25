//! Framebuffer.

use core::{convert::Infallible, marker::PhantomData};

use crate::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Point, Size},
    image::{arrangement::PixelArrangement, GetPixel, ImageRaw},
    iterator::raw::RawDataSlice,
    pixelcolor::{
        raw::{
            storage::{BigEndian, LittleEndian, Lsb0, Msb0, RawStorage, ColorStorage},
            RawData, RawU1, RawU16, RawU2, RawU24, RawU32, RawU4, RawU8, ToBytes,
        },
        PixelColor,
    },
    Pixel,
};

/// Calculates the required buffer size.
///
/// This function is a workaround for current limitations in Rust const generics.
/// It can be used to calculate the `N` parameter based on the size and color type of the framebuffer.
pub const fn buffer_size<C: PixelColor>(width: usize, height: usize) -> usize {
    buffer_size_bpp(width, height, C::Raw::BITS_PER_PIXEL)
}

/// Calculates the required buffer size.
///
/// This function is a workaround for current limitations in Rust const generics. It can be used to
/// calculate the `N` parameter based on the size and bit depth of the framebuffer.
pub const fn buffer_size_bpp(width: usize, height: usize, bpp: usize) -> usize {
    (width * bpp + 7) / 8 * height
}

/// A framebuffer.
///
/// # Examples
///
/// ```
/// use embedded_graphics::{
///     framebuffer,
///     framebuffer::{Framebuffer, buffer_size},
///     pixelcolor::{Rgb565, raw::storage::LittleEndian},
///     prelude::*,
///     primitives::PrimitiveStyle,
/// };
///
/// let mut fb = Framebuffer::<LittleEndian<Rgb565>, 320, 240, {buffer_size::<Rgb565>(320, 240)}>::new();
///
/// fb.bounding_box()
///     .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 1))
///     .draw(&mut fb)
///     .unwrap();
/// ```
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Framebuffer<CS, DIR, const WIDTH: usize, const HEIGHT: usize, const N: usize> {
    data: [u8; N],
    color_type: PhantomData<C>,
    raw_type: PhantomData<R>,
    byte_order: PhantomData<BO>,
    color_storage_type: PhantomData<CS>,
    direction: PhantomData<DIR>,
    n_assert: (),
}

impl<CS, DIR, const WIDTH: usize, const HEIGHT: usize, const N: usize>
    Framebuffer<CS, DIR, WIDTH, HEIGHT, N>
where
    CS: ColorStorage,
    DIR: PixelArrangement,
{
    const BUFFER_SIZE: usize = buffer_size::<CS::Color>(WIDTH, HEIGHT);

    /// Static assertion that N is correct.
    // MSRV: remove N when constant generic expressions are stabilized
    const CHECK_N: () = assert!(
        N >= Self::BUFFER_SIZE,
        "Invalid N: see Framebuffer documentation for more information"
    );

    /// Creates a new framebuffer.
    pub const fn new() -> Self {
        Self {
            data: [0; N],
            color_type: PhantomData,
            raw_type: PhantomData,
            byte_order: PhantomData,
            color_storage_type: PhantomData,
            direction: PhantomData,
            n_assert: Self::CHECK_N,
        }
    }

    /// Returns a reference to the raw framebuffer data.
    pub const fn data(&self) -> &[u8; N] {
        &self.data
    }

    /// Returns a mutable reference to the raw framebuffer data.
    pub fn data_mut(&mut self) -> &mut [u8; N] {
        &mut self.data
    }
}

impl<CS, DIR, const WIDTH: usize, const HEIGHT: usize, const N: usize>
    Framebuffer<CS, DIR, WIDTH, HEIGHT, N>
where
    CS: ColorStorage,
    <<CS as ColorStorage>::Color as PixelColor>::Raw: Into<CS::Color>,
    DIR: PixelArrangement,
{
    /// Returns an image based on the framebuffer content.
    pub fn as_image(&self) -> ImageRaw<'_, CS> {
        ImageRaw::new(&self.data[0..Self::BUFFER_SIZE], WIDTH as u32)
    }
}

impl<CS, DIR, const WIDTH: usize, const HEIGHT: usize, const N: usize> GetPixel
    for Framebuffer<CS, DIR, WIDTH, HEIGHT, N>
where
    CS: ColorStorage,
    <<CS as ColorStorage>::Color as PixelColor>::Raw: Into<CS::Color>,
    for<'a> RawDataSlice<'a, <CS as ColorStorage>::RawStorage>:
        IntoIterator<Item = <<CS as ColorStorage>::Color as PixelColor>::Raw>,
    DIR: PixelArrangement,
{
    type Color = CS::Color;

    fn pixel(&self, p: Point) -> Option<Self::Color> {
        self.as_image().pixel(p)
    }
}

impl<CS, DIR, const WIDTH: usize, const HEIGHT: usize, const N: usize>
    Framebuffer<CS, DIR, WIDTH, HEIGHT, N>
where
    CS: ColorStorage,
    CS::Color: Into<<CS::Color as PixelColor>::Raw>,
    Self: SetPixelRaw<CS::RawStorage>,
{
    /// Sets the color of a pixel.
    ///
    /// Trying to set a pixel outside the framebuffer is a noop.
    pub fn set_pixel(&mut self, p: Point, c: CS::Color) {
        self.set_pixel_raw(p, c.into())
    }
}

impl<CS, DIR, const WIDTH: usize, const HEIGHT: usize, const N: usize> DrawTarget
    for Framebuffer<CS, DIR, WIDTH, HEIGHT, N>
where
    CS: ColorStorage,
    CS::Color: Into<<CS::Color as PixelColor>::Raw>,
    Self: SetPixelRaw<CS::RawStorage>,
{
    type Color = CS::Color;
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

/// TODO: docs
// TODO: can this be hidden from the API?
pub trait SetPixelRaw<RS: RawStorage> {
    /// TODO: docs
    fn set_pixel_raw(&mut self, p: Point, r: RS::Raw);
}

#[inline(always)]
const fn shift_lsb0(input: usize) -> usize {
    7 - input
}

#[inline(always)]
const fn shift_msb0(input: usize) -> usize {
    input
}

macro_rules! impl_bit {
    (@impl $raw_storage_type:ty, $raw_type:ty, $shifter:expr) => {
        impl<CS, DIR, const WIDTH: usize, const HEIGHT: usize, const N: usize> SetPixelRaw<$raw_storage_type>
            for Framebuffer<CS, DIR, WIDTH, HEIGHT, N> where DIR: PixelArrangement
        {
            fn set_pixel_raw(&mut self, p: Point, r: $raw_type) {
                if let (Ok(x), Ok(y)) = (usize::try_from(p.x), usize::try_from(p.y)) {
                    if x < WIDTH && y < HEIGHT {
                      let (bit_index, byte_index) =  if DIR::IS_HORIZONTAL {
                            let pixels_per_bit = 8 / <$raw_type>::BITS_PER_PIXEL;
                            let bits_per_row = WIDTH * <$raw_type>::BITS_PER_PIXEL;
                            let bytes_per_row = (bits_per_row + 7) / 8;
                            let byte_index = bytes_per_row * y + (x / pixels_per_bit);
                            let bit_index = 8 - (x % pixels_per_bit + 1) * <$raw_type>::BITS_PER_PIXEL;

                            (bit_index, byte_index)
                        } else {
                            let y_start_byte = (y * <$raw_type>::BITS_PER_PIXEL) / 8;
                            let y_start_bit = (y * <$raw_type>::BITS_PER_PIXEL) % 8;

                            let byte_index = WIDTH * y_start_byte + x;

                            (y_start_bit, byte_index)
                        };

                        let shift = $shifter(bit_index);
                        let mask = !((<$raw_type>::BIT_MASK as u8) << shift);
                        let bits = r.into_inner() << shift;

                        self.data[byte_index] = self.data[byte_index] & mask | bits;
                    }
                }
            }
        }
    };

    ($raw_type:ty) => {
        impl_bit!(@impl Lsb0<$raw_type>, $raw_type, shift_lsb0);
        impl_bit!(@impl Msb0<$raw_type>, $raw_type, shift_msb0);
    };
}

impl_bit!(RawU1);
impl_bit!(RawU2);
impl_bit!(RawU4);

impl<CS, DIR, const WIDTH: usize, const HEIGHT: usize, const N: usize> SetPixelRaw<RawU8>
    for Framebuffer<CS, DIR, WIDTH, HEIGHT, N>
{
    fn set_pixel_raw(&mut self, p: Point, r: RawU8) {
        if let (Ok(x), Ok(y)) = (usize::try_from(p.x), usize::try_from(p.y)) {
            if x < WIDTH && y < HEIGHT {
                let x = p.x as usize;
                let y = p.y as usize;

                self.data[y * WIDTH + x] = r.into_inner();
            }
        }
    }
}

impl<CS, DIR, const WIDTH: usize, const HEIGHT: usize, const N: usize>
    SetPixelRaw<LittleEndian<RawU8>> for Framebuffer<CS, DIR, WIDTH, HEIGHT, N>
{
    fn set_pixel_raw(&mut self, p: Point, r: RawU8) {
        SetPixelRaw::<RawU8>::set_pixel_raw(self, p, r)
    }
}

impl<CS, DIR, const WIDTH: usize, const HEIGHT: usize, const N: usize> SetPixelRaw<BigEndian<RawU8>>
    for Framebuffer<CS, DIR, WIDTH, HEIGHT, N>
{
    fn set_pixel_raw(&mut self, p: Point, r: RawU8) {
        SetPixelRaw::<RawU8>::set_pixel_raw(self, p, r)
    }
}

macro_rules! impl_bytes {
    ($raw_storage_type:ty, $to_bytes_fn:ident) => {
        impl<CS, DIR, const WIDTH: usize, const HEIGHT: usize, const N: usize>
            SetPixelRaw<$raw_storage_type> for Framebuffer<CS, DIR, WIDTH, HEIGHT, N>
        {
            fn set_pixel_raw(&mut self, p: Point, r: <$raw_storage_type as RawStorage>::Raw) {
                const BYTES_PER_PIXEL: usize =
                    <<$raw_storage_type as RawStorage>::Raw>::BITS_PER_PIXEL / 8;

                if let (Ok(x), Ok(y)) = (usize::try_from(p.x), usize::try_from(p.y)) {
                    if x < WIDTH && y < HEIGHT {
                        let x = p.x as usize;
                        let y = p.y as usize;

                        let index = (y * WIDTH + x) * BYTES_PER_PIXEL;

                        self.data[index..index + BYTES_PER_PIXEL]
                            .copy_from_slice(&r.$to_bytes_fn());
                    }
                }
            }
        }
    };

    ($raw_type:ty) => {
        impl_bytes!(LittleEndian<$raw_type>, to_le_bytes);
        impl_bytes!(BigEndian<$raw_type>, to_be_bytes);
    };
}

impl_bytes!(RawU16);
impl_bytes!(RawU24);
impl_bytes!(RawU32);

impl<CS, DIR, const WIDTH: usize, const HEIGHT: usize, const N: usize> OriginDimensions
    for Framebuffer<CS, DIR, WIDTH, HEIGHT, N>
{
    fn size(&self) -> Size {
        Size::new(WIDTH as u32, HEIGHT as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::Dimensions,
        geometry::Point,
        image::{arrangement::Horizontal, Image},
        mock_display::MockDisplay,
        pixelcolor::{BinaryColor, Gray2, Gray4, Gray8, GrayColor, Rgb565, Rgb888, RgbColor},
        primitives::{Primitive, PrimitiveStyle},
        Drawable,
    };

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    struct U32Color(u32);

    impl PixelColor for U32Color {
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

    #[test]
    fn raw_u1_msb0() {
        let mut fb = Framebuffer::<
            Msb0<BinaryColor>,
            Horizontal,
            9,
            2,
            { buffer_size::<BinaryColor>(9, 2) },
        >::new();

        use BinaryColor::{Off, On};
        fb.draw_iter(
            [
                ((0, 0), On),  //
                ((8, 1), On),  //
                ((1, 1), On),  //
                ((1, 1), Off), //
                ((-1, 0), On), //
                ((0, -1), On), //
                ((9, 0), On),  //
                ((0, 2), On),  //
            ]
            .iter()
            .map(|(p, c)| Pixel(Point::from(*p), *c)),
        )
        .unwrap();

        assert_eq!(
            fb.data(),
            &[
                0b10000000, 0b00000000, //
                0b00000000, 0b10000000, //
            ]
        );
    }

    #[test]
    fn raw_u1_lsb0() {
        let mut fb = Framebuffer::<
            Lsb0<BinaryColor>,
            Horizontal,
            9,
            2,
            { buffer_size::<BinaryColor>(9, 2) },
        >::new();

        use BinaryColor::{Off, On};
        fb.draw_iter(
            [
                ((0, 0), On),  //
                ((8, 1), On),  //
                ((1, 1), On),  //
                ((1, 1), Off), //
                ((-1, 0), On), //
                ((0, -1), On), //
                ((9, 0), On),  //
                ((0, 2), On),  //
            ]
            .iter()
            .map(|(p, c)| Pixel(Point::from(*p), *c)),
        )
        .unwrap();

        assert_eq!(
            fb.data(),
            &[
                0b00000001, 0b00000000, //
                0b00000000, 0b00000001, //
            ]
        );
    }

    #[test]
    fn raw_u2_msb0() {
        let mut fb =
            Framebuffer::<Msb0<Gray2>, Horizontal, 6, 4, { buffer_size::<Gray2>(6, 4) }>::new();

        fb.draw_iter(
            [
                ((0, 0), 1),  //
                ((5, 3), 2),  //
                ((1, 1), 3),  //
                ((1, 2), 0),  //
                ((-1, 0), 3), //
                ((0, -1), 3), //
                ((6, 0), 3),  //
                ((0, 4), 3),  //
            ]
            .iter()
            .map(|(p, c)| Pixel(Point::from(*p), Gray2::new(*c))),
        )
        .unwrap();

        assert_eq!(
            fb.data(),
            &[
                0b01000000, 0b00000000, //
                0b00110000, 0b00000000, //
                0b00000000, 0b00000000, //
                0b00000000, 0b00100000, //
            ]
        );
    }

    #[test]
    fn raw_u4_msb0() {
        let mut fb =
            Framebuffer::<Msb0<Gray4>, Horizontal, 3, 2, { buffer_size::<Gray4>(3, 2) }>::new();

        fb.draw_iter(
            [
                ((0, 0), 0x1),  //
                ((2, 1), 0xF),  //
                ((1, 0), 0xA),  //
                ((1, 1), 0xB),  //
                ((-1, 0), 0xF), //
                ((0, -1), 0xF), //
                ((3, 0), 0xF),  //
                ((0, 2), 0xF),  //
            ]
            .iter()
            .map(|(p, c)| Pixel(Point::from(*p), Gray4::new(*c))),
        )
        .unwrap();

        assert_eq!(
            fb.data(),
            &[
                0x1A, 0x00, //
                0x0B, 0xF0, //
            ]
        );
    }

    #[test]
    fn raw_u8() {
        let mut fb = Framebuffer::<Gray8, Horizontal, 3, 2, { buffer_size::<Gray8>(3, 2) }>::new();

        fb.draw_iter(
            [
                ((0, 0), 0x10),  //
                ((2, 1), 0x22),  //
                ((1, 0), 0x3F),  //
                ((1, 1), 0xF0),  //
                ((-1, 0), 0xFF), //
                ((0, -1), 0xFF), //
                ((3, 0), 0xFF),  //
                ((0, 2), 0xFF),  //
            ]
            .iter()
            .map(|(p, c)| Pixel(Point::from(*p), Gray8::new(*c))),
        )
        .unwrap();

        assert_eq!(
            fb.data(),
            &[
                0x10, 0x3F, 0x00, //
                0x00, 0xF0, 0x22, //
            ]
        );
    }

    #[test]
    fn raw_u16_le() {
        let mut fb = Framebuffer::<
            LittleEndian<Rgb565>,
            Horizontal,
            3,
            2,
            { buffer_size::<Rgb565>(3, 2) },
        >::new();

        fb.draw_iter(
            [
                ((0, 0), 0x1000),  //
                ((2, 1), 0x0001),  //
                ((1, 0), 0x1234),  //
                ((1, 1), 0x8765),  //
                ((-1, 0), 0xFFFF), //
                ((0, -1), 0xFFFF), //
                ((3, 0), 0xFFFF),  //
                ((0, 2), 0xFFFF),  //
            ]
            .iter()
            .map(|(p, c)| Pixel(Point::from(*p), Rgb565::from(RawU16::new(*c)))),
        )
        .unwrap();

        assert_eq!(
            fb.data(),
            &[
                0x00, 0x10, 0x34, 0x12, 0x00, 0x00, //
                0x00, 0x00, 0x65, 0x87, 0x01, 0x00, //
            ]
        );
    }

    #[test]
    fn raw_u16_be() {
        let mut fb = Framebuffer::<
            BigEndian<Rgb565>,
            Horizontal,
            3,
            2,
            { buffer_size::<Rgb565>(3, 2) },
        >::new();

        fb.draw_iter(
            [
                ((0, 0), 0x1000),  //
                ((2, 1), 0x0001),  //
                ((1, 0), 0x1234),  //
                ((1, 1), 0x8765),  //
                ((-1, 0), 0xFFFF), //
                ((0, -1), 0xFFFF), //
                ((3, 0), 0xFFFF),  //
                ((0, 2), 0xFFFF),  //
            ]
            .iter()
            .map(|(p, c)| Pixel(Point::from(*p), Rgb565::from(RawU16::new(*c)))),
        )
        .unwrap();

        assert_eq!(
            fb.data(),
            &[
                0x10, 0x00, 0x12, 0x34, 0x00, 0x00, //
                0x00, 0x00, 0x87, 0x65, 0x00, 0x01, //
            ]
        );
    }

    #[test]
    fn raw_u24_le() {
        let mut fb = Framebuffer::<
            LittleEndian<Rgb888>,
            Horizontal,
            3,
            2,
            { buffer_size::<Rgb888>(3, 2) },
        >::new();

        fb.draw_iter(
            [
                ((0, 0), 0x100000),  //
                ((2, 1), 0x000001),  //
                ((1, 0), 0x123456),  //
                ((1, 1), 0x876543),  //
                ((-1, 0), 0xFFFFFF), //
                ((0, -1), 0xFFFFFF), //
                ((3, 0), 0xFFFFFF),  //
                ((0, 2), 0xFFFFFF),  //
            ]
            .iter()
            .map(|(p, c)| Pixel(Point::from(*p), Rgb888::from(RawU24::new(*c)))),
        )
        .unwrap();

        assert_eq!(
            fb.data(),
            &[
                0x00, 0x00, 0x10, 0x56, 0x34, 0x12, 0x00, 0x00, 0x00, //
                0x00, 0x00, 0x00, 0x43, 0x65, 0x87, 0x01, 0x00, 0x00, //
            ]
        );
    }

    #[test]
    fn raw_u24_be() {
        let mut fb = Framebuffer::<
            BigEndian<Rgb888>,
            Horizontal,
            3,
            2,
            { buffer_size::<Rgb888>(3, 2) },
        >::new();

        fb.draw_iter(
            [
                ((0, 0), 0x100000),  //
                ((2, 1), 0x000001),  //
                ((1, 0), 0x123456),  //
                ((1, 1), 0x876543),  //
                ((-1, 0), 0xFFFFFF), //
                ((0, -1), 0xFFFFFF), //
                ((3, 0), 0xFFFFFF),  //
                ((0, 2), 0xFFFFFF),  //
            ]
            .iter()
            .map(|(p, c)| Pixel(Point::from(*p), Rgb888::from(RawU24::new(*c)))),
        )
        .unwrap();

        assert_eq!(
            fb.data(),
            &[
                0x10, 0x00, 0x00, 0x12, 0x34, 0x56, 0x00, 0x00, 0x00, //
                0x00, 0x00, 0x00, 0x87, 0x65, 0x43, 0x00, 0x00, 0x01, //
            ]
        );
    }

    #[test]
    fn raw_u32_le() {
        let mut fb = Framebuffer::<
            LittleEndian<U32Color>,
            Horizontal,
            3,
            2,
            { buffer_size::<U32Color>(3, 2) },
        >::new();

        fb.draw_iter(
            [
                ((0, 0), 0x10000000),  //
                ((2, 1), 0x00000001),  //
                ((1, 0), 0x12345678),  //
                ((1, 1), 0x87654321),  //
                ((-1, 0), 0xFFFFFFFF), //
                ((0, -1), 0xFFFFFFFF), //
                ((3, 0), 0xFFFFFFFF),  //
                ((0, 2), 0xFFFFFFFF),  //
            ]
            .iter()
            .map(|(p, c)| Pixel(Point::from(*p), U32Color::from(RawU32::new(*c)))),
        )
        .unwrap();

        assert_eq!(
            fb.data(),
            &[
                0x00, 0x00, 0x00, 0x10, 0x78, 0x56, 0x34, 0x12, 0x00, 0x00, 0x00, 0x00, //
                0x00, 0x00, 0x00, 0x00, 0x21, 0x43, 0x65, 0x87, 0x01, 0x00, 0x00, 0x00, //
            ]
        );
    }

    #[test]
    fn raw_u32_be() {
        let mut fb = Framebuffer::<
            BigEndian<U32Color>,
            Horizontal,
            3,
            2,
            { buffer_size::<U32Color>(3, 2) },
        >::new();

        fb.draw_iter(
            [
                ((0, 0), 0x10000000),  //
                ((2, 1), 0x00000001),  //
                ((1, 0), 0x12345678),  //
                ((1, 1), 0x87654321),  //
                ((-1, 0), 0xFFFFFFFF), //
                ((0, -1), 0xFFFFFFFF), //
                ((3, 0), 0xFFFFFFFF),  //
                ((0, 2), 0xFFFFFFFF),  //
            ]
            .iter()
            .map(|(p, c)| Pixel(Point::from(*p), U32Color::from(RawU32::new(*c)))),
        )
        .unwrap();

        assert_eq!(
            fb.data(),
            &[
                0x10, 0x00, 0x00, 0x00, 0x12, 0x34, 0x56, 0x78, 0x00, 0x00, 0x00, 0x00, //
                0x00, 0x00, 0x00, 0x00, 0x87, 0x65, 0x43, 0x21, 0x00, 0x00, 0x00, 0x01, //
            ]
        );
    }

    #[test]
    fn as_image() {
        let mut fb = Framebuffer::<
            Msb0<BinaryColor>,
            Horizontal,
            10,
            10,
            { buffer_size::<BinaryColor>(10, 10) },
        >::new();

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
        let mut fb = Framebuffer::<
            Msb0<BinaryColor>,
            Horizontal,
            10,
            10,
            { buffer_size::<BinaryColor>(10, 10) },
        >::new();

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
    fn set_pixel() {
        // This tests only checks that the set_pixel methods are present for all BPPs.
        // The correct function is tested indirectly in the DrawTarget tests.

        type Fb<CS> = Framebuffer<CS, Horizontal, 10, 10, 1000>;

        Fb::<Msb0<BinaryColor>>::new().set_pixel(Point::zero(), BinaryColor::On);
        Fb::<Lsb0<BinaryColor>>::new().set_pixel(Point::zero(), BinaryColor::On);
        Fb::<Msb0<Gray2>>::new().set_pixel(Point::zero(), Gray2::WHITE);
        Fb::<Lsb0<Gray2>>::new().set_pixel(Point::zero(), Gray2::WHITE);
        Fb::<Msb0<Gray4>>::new().set_pixel(Point::zero(), Gray4::WHITE);
        Fb::<Lsb0<Gray4>>::new().set_pixel(Point::zero(), Gray4::WHITE);
        Fb::<Gray8>::new().set_pixel(Point::zero(), Gray8::WHITE);
        Fb::<LittleEndian<Gray8>>::new().set_pixel(Point::zero(), Gray8::WHITE);
        Fb::<BigEndian<Gray8>>::new().set_pixel(Point::zero(), Gray8::WHITE);
        Fb::<LittleEndian<Rgb565>>::new().set_pixel(Point::zero(), Rgb565::WHITE);
        Fb::<BigEndian<Rgb565>>::new().set_pixel(Point::zero(), Rgb565::WHITE);
        Fb::<LittleEndian<Rgb888>>::new().set_pixel(Point::zero(), Rgb888::WHITE);
        Fb::<BigEndian<Rgb888>>::new().set_pixel(Point::zero(), Rgb888::WHITE);
        Fb::<LittleEndian<U32Color>>::new().set_pixel(Point::zero(), U32Color(0));
        Fb::<BigEndian<U32Color>>::new().set_pixel(Point::zero(), U32Color(0));
    }

    #[test]
    fn oversized_buffer() {
        let fb = Framebuffer::<
            Msb0<BinaryColor>,
            Horizontal,
            10,
            5,
            { buffer_size::<BinaryColor>(10, 5) * 3 / 2 },
        >::new();

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
}
