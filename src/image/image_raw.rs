use crate::{
    geometry::{OriginDimensions, Size},
    image::ImageDrawable,
    pixelcolor::{
        raw::{BigEndian, ByteOrder, LittleEndian, RawData, RawDataIter},
        PixelColor,
    },
    prelude::Dimensions,
};
use core::marker::PhantomData;

/// Image with little endian data.
pub type ImageRawLE<'a, C> = ImageRaw<'a, C, LittleEndian>;

/// Image with big endian data.
pub type ImageRawBE<'a, C> = ImageRaw<'a, C, BigEndian>;

/// An image constructed from a slice of raw pixel data.
///
/// The `ImageRaw` struct can be used to construct an image from a slice
/// of raw image data. The storage format is determined by the [`PixelColor`]
/// type `C` and the [`ByteOrder`] `BO`. The byteorder doesn't need to be
/// specified for colors which aren't stored in multiple bytes.
///
/// For color types with less than 8 bits per pixels the start of each row is
/// aligned to the next whole byte.
///
/// Details about the conversion of raw data to color types are explained in the
/// [`raw` module documentation].
///
/// To draw an `ImageRaw` object it needs to be wrapped in an [`Image`] object.
///
/// # Examples
///
/// ## Draw a 1BPP image
///
/// This example creates an image from 1 bit per pixel data.
///
/// ```
/// use embedded_graphics::{
///     image::{Image, ImageRaw},
///     pixelcolor::BinaryColor,
///     prelude::*,
/// };
/// # use embedded_graphics::mock_display::MockDisplay as Display;
///
/// /// Image data with 12 x 5 pixels.
/// /// The data for each row is 12 bits long and is padded with zeros on the
/// /// end because each row needs to contain a whole number of bytes.
/// #[rustfmt::skip]
/// const DATA: &[u8] = &[
///     0b11101111, 0b0101_0000,
///     0b10001000, 0b0101_0000,
///     0b11101011, 0b0101_0000,
///     0b10001001, 0b0101_0000,
///     0b11101111, 0b0101_0000,
/// ];
///
/// // The type annotation `ImageRaw<BinaryColor>` is used to specify the format
/// // of the stored raw data (`PixelColor::Raw`) and which color type the
/// // raw data gets converted into.
/// let raw_image: ImageRaw<BinaryColor> = ImageRaw::new(DATA, 12, 5);
///
/// let image = Image::new(&raw_image, Point::zero());
///
/// let mut display = Display::default();
///
/// image.draw(&mut display)?;
/// # Ok::<(), core::convert::Infallible>(())
/// ```
///
/// ## Draw an image that uses multibyte pixel encoding
///
/// Colors with more than one byte per pixel need an additional type annotation for the byte order.
/// For convenience, the [`ImageRawBE`] and [`ImageRawLE`] type aliases can be used to abbreviate
/// the type.
///
/// ```
/// use embedded_graphics::{
///     image::{Image, ImageRaw, ImageRawBE, ImageRawLE},
///     pixelcolor::{
///         raw::{BigEndian, LittleEndian},
///         Rgb565, Rgb888,
///     },
///     prelude::*,
/// };
/// # const DATA: &[u8] = &[0x55; 8 * 8 * 3];
///
/// // Rgb888 image with 24 bits per pixel and big endian byte order
/// let image1: ImageRawBE<Rgb888> = ImageRaw::new(DATA, 8, 8);
/// // or:
/// let image2: ImageRaw<Rgb888, BigEndian> = ImageRaw::new(DATA, 8, 8);
/// # assert_eq!(image1, image2);
///
/// // Rgb565 image with 16 bits per pixel and little endian byte order
/// let image1: ImageRawLE<Rgb565> = ImageRaw::new(DATA, 16, 6);
/// // or:
/// let image2: ImageRaw<Rgb565, LittleEndian> = ImageRaw::new(DATA, 16, 6);
/// # assert_eq!(image1, image2);
/// ```
///
/// [`raw` module documentation]: ../pixelcolor/raw/index.html
/// [`Drawable`]: ../drawable/trait.Drawable.html
/// [`ImageRawBE`]: type.ImageRawBE.html
/// [`ImageRawLE`]: type.ImageRawLE.html
/// [`Image`]: struct.Image.html
/// [`PixelColor`]: ../pixelcolor/trait.PixelColor.html
/// [`ByteOrder`]: ../pixelcolor/raw/trait.ByteOrder.html
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ImageRaw<'a, C, BO = BigEndian>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    BO: ByteOrder,
{
    /// Image data, packed as dictated by raw data type `C::Raw`
    data: &'a [u8],

    /// Image size in pixels
    size: Size,

    pixel_type: PhantomData<C>,
    byte_order: PhantomData<BO>,
}

impl<'a, C, BO> ImageRaw<'a, C, BO>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    BO: ByteOrder,
{
    /// Creates a new image.
    ///
    /// # Panics
    ///
    /// If `data` doesn't have the correct length.
    pub fn new(data: &'a [u8], width: u32, height: u32) -> Self {
        let ret = Self {
            data,
            size: Size::new(width, height),
            pixel_type: PhantomData,
            byte_order: PhantomData,
        };

        assert_eq!(data.len(), height as usize * ret.bytes_per_row());

        ret
    }

    /// Returns the length of each row in bytes.
    fn bytes_per_row(&self) -> usize {
        (self.size.width as usize * C::Raw::BITS_PER_PIXEL + 7) / 8
    }
}

impl<'a, C, BO> ImageDrawable for ImageRaw<'a, C, BO>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    BO: ByteOrder,
    RawDataIter<'a, C::Raw, BO>: Iterator<Item = C::Raw>,
{
    type Color = C;

    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: crate::draw_target::DrawTarget<Color = C>,
    {
        target.fill_contiguous(&self.bounding_box(), ContiguousPixels::new(self))
    }
}

impl<C, BO> OriginDimensions for ImageRaw<'_, C, BO>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    BO: ByteOrder,
{
    fn size(&self) -> Size {
        self.size
    }
}

#[derive(Clone, Debug)]
pub struct ContiguousPixels<'a, C, BO>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    BO: ByteOrder,
{
    iter: RawDataIter<'a, C::Raw, BO>,

    x: u32,
    width: u32,

    rows_remaining: u32,
}

impl<'a, C, BO> ContiguousPixels<'a, C, BO>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    BO: ByteOrder,
    RawDataIter<'a, C::Raw, BO>: Iterator<Item = C::Raw>,
{
    fn new(image: &ImageRaw<'a, C, BO>) -> Self {
        Self {
            iter: RawDataIter::new(image.data),
            x: 0,
            width: image.size.width,
            rows_remaining: image.size.height,
        }
    }
}

impl<'a, C, BO> Iterator for ContiguousPixels<'a, C, BO>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    BO: ByteOrder,
    RawDataIter<'a, C::Raw, BO>: Iterator<Item = C::Raw>,
{
    type Item = C;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rows_remaining == 0 {
            return None;
        }

        let color = self.iter.next()?.into();

        self.x += 1;
        if self.x >= self.width {
            self.x = 0;
            self.rows_remaining -= 1;

            self.iter.align();
        }

        Some(color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        draw_target::DrawTarget,
        geometry::Point,
        image::Image,
        iterator::PixelIteratorExt,
        mock_display::{ColorMapping, MockDisplay},
        pixelcolor::{raw::RawU32, *},
        Drawable, Pixel,
    };

    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
    struct TestColorU32(RawU32);

    impl PixelColor for TestColorU32 {
        type Raw = RawU32;
    }

    impl From<RawU32> for TestColorU32 {
        fn from(data: RawU32) -> Self {
            Self(data)
        }
    }

    /// Tests if the given image data matches an excepted `MockDisplay` pattern.
    fn assert_pattern<C, BO>(image_data: ImageRaw<C, BO>, expected_pattern: &[&str])
    where
        C: PixelColor + From<<C as PixelColor>::Raw> + ColorMapping,
        BO: ByteOrder,
        for<'a> RawDataIter<'a, C::Raw, BO>: Iterator<Item = C::Raw>,
    {
        let image = Image::new(&image_data, Point::zero());
        let mut display = MockDisplay::new();
        image.draw(&mut display).unwrap();

        display.assert_pattern(expected_pattern);
    }

    #[test]
    fn image_dimensions() {
        let data = [
            0xAA, 0x00, //
            0x55, 0xFF, //
            0xAA, 0x80, //
        ];
        let image_data: ImageRaw<BinaryColor> = ImageRaw::new(&data, 9, 3);

        assert_eq!(image_data.size(), Size::new(9, 3));
    }

    #[test]
    fn bpp1() {
        let data = [
            0xAA, 0x00, //
            0x55, 0xFF, //
            0xAA, 0x80, //
        ];
        let image_data: ImageRaw<BinaryColor> = ImageRaw::new(&data, 9, 3);

        assert_pattern(
            image_data,
            &[
                "#.#.#.#..", //
                ".#.#.#.##", //
                "#.#.#.#.#", //
            ],
        );
    }

    #[test]
    fn bpp2() {
        let data = [
            0b00_01_10_11, //
            0b00_00_00_00, //
            0b11_10_01_00, //
            0b11_11_11_11, //
        ];
        let image_data: ImageRaw<Gray2> = ImageRaw::new(&data, 5, 2);

        assert_pattern(
            image_data,
            &[
                "01230", //
                "32103", //
            ],
        );
    }

    #[test]
    fn bpp4() {
        let data = [
            0b0001_1000, //
            0b1111_0000, //
            0b0101_1010, //
            0b0000_0000, //
        ];
        let image_data: ImageRaw<Gray4> = ImageRaw::new(&data, 3, 2);

        assert_pattern(
            image_data,
            &[
                "18F", //
                "5A0", //
            ],
        );
    }

    #[test]
    fn bpp8_1() {
        let data = [
            0x11, 0x22, //
            0x33, 0x44, //
            0x55, 0x66, //
        ];
        let image_data: ImageRaw<Gray8> = ImageRaw::new(&data, 2, 3);

        assert_pattern(
            image_data,
            &[
                "12", //
                "34", //
                "56", //
            ],
        );
    }

    /// Additional test for luma values with different low and high nibbles,
    /// which are not supported by `MockDisplay` patterns.
    #[test]
    fn bpp8_2() {
        let data = [0x01, 0x08, 0x10, 0x80];
        let image_data: ImageRaw<Gray8> = ImageRaw::new(&data, 4, 1);

        let mut display = MockDisplay::new();
        Image::new(&image_data, Point::zero())
            .draw(&mut display)
            .unwrap();

        let mut expected = MockDisplay::new();
        expected
            .fill_contiguous(
                &expected.bounding_box(),
                data.iter().copied().map(Gray8::new),
            )
            .unwrap();

        display.assert_eq(&expected);
    }

    #[test]
    fn bpp16_little_endian() {
        let data = [
            0x00, 0xF8, //
            0xE0, 0x07, //
            0x1F, 0x00, //
            0x00, 0x00, //
        ];
        let image_data: ImageRawLE<Rgb565> = ImageRaw::new(&data, 1, 4);

        assert_pattern(
            image_data,
            &[
                "R", //
                "G", //
                "B", //
                "K", //
            ],
        );
    }

    #[test]
    fn bpp16_big_endian() {
        let data = [
            0xF8, 0x00, //
            0x07, 0xE0, //
            0x00, 0x1F, //
            0x00, 0x00, //
        ];
        let image_data: ImageRawBE<Rgb565> = ImageRaw::new(&data, 2, 2);

        assert_pattern(
            image_data,
            &[
                "RG", //
                "BK", //
            ],
        );
    }

    #[test]
    fn bpp24_little_endian() {
        let data = [
            0xFF, 0x00, 0x00, //
            0x00, 0xFF, 0x00, //
            0x00, 0x00, 0xFF, //
            0x00, 0x00, 0x00, //
        ];
        let image_data: ImageRawLE<Bgr888> = ImageRaw::new(&data, 1, 4);

        assert_pattern(
            image_data,
            &[
                "R", //
                "G", //
                "B", //
                "K", //
            ],
        );
    }

    #[test]
    fn bpp24_big_endian() {
        let data = [
            0xFF, 0x00, 0x00, //
            0x00, 0xFF, 0x00, //
            0x00, 0x00, 0xFF, //
            0x00, 0x00, 0x00, //
        ];
        let image_data: ImageRawBE<Rgb888> = ImageRaw::new(&data, 4, 1);

        assert_pattern(image_data, &["RGBK"]);
    }

    #[test]
    fn bpp32_little_endian() {
        let data = [
            0x12, 0x34, 0x56, 0x78, //
            0x9A, 0xBC, 0xDE, 0xF0, //
            0x00, 0x00, 0x00, 0x00, //
            0xFF, 0xFF, 0xFF, 0xFF, //
        ];
        let image_data: ImageRawLE<TestColorU32> = ImageRaw::new(&data, 2, 2);

        let mut display = MockDisplay::new();
        Image::new(&image_data, Point::zero())
            .draw(&mut display)
            .unwrap();

        let expected = [
            Pixel(Point::new(0, 0), TestColorU32(RawU32::new(0x78563412))),
            Pixel(Point::new(1, 0), TestColorU32(RawU32::new(0xF0DEBC9A))),
            Pixel(Point::new(0, 1), TestColorU32(RawU32::new(0x00000000))),
            Pixel(Point::new(1, 1), TestColorU32(RawU32::new(0xFFFFFFFF))),
        ];

        let mut expected_display = MockDisplay::new();
        expected
            .iter()
            .copied()
            .draw(&mut expected_display)
            .unwrap();

        // assert_eq can't be used here because ColorMapping isn't implemented for TestColorU32
        assert!(display.eq(&expected_display));
    }

    #[test]
    fn bpp32_big_endian() {
        let data = [
            0x12, 0x34, 0x56, 0x78, //
            0x9A, 0xBC, 0xDE, 0xF0, //
            0x00, 0x00, 0x00, 0x00, //
            0xFF, 0xFF, 0xFF, 0xFF, //
        ];
        let image_data: ImageRawBE<TestColorU32> = ImageRaw::new(&data, 4, 1);

        let mut display = MockDisplay::new();
        Image::new(&image_data, Point::zero())
            .draw(&mut display)
            .unwrap();

        let expected = [
            Pixel(Point::new(0, 0), TestColorU32(RawU32::new(0x12345678))),
            Pixel(Point::new(1, 0), TestColorU32(RawU32::new(0x9ABCDEF0))),
            Pixel(Point::new(2, 0), TestColorU32(RawU32::new(0x00000000))),
            Pixel(Point::new(3, 0), TestColorU32(RawU32::new(0xFFFFFFFF))),
        ];

        let mut expected_display = MockDisplay::new();
        expected
            .iter()
            .copied()
            .draw(&mut expected_display)
            .unwrap();

        // assert_eq can't be used here because ColorMapping isn't implemented for TestColorU32
        assert!(display.eq(&expected_display));
    }

    #[test]
    #[should_panic]
    fn panics_if_length_of_data_is_too_short() {
        let data = [0u8; 3];
        let _: ImageRaw<BinaryColor> = ImageRaw::new(&data, 12, 2);
    }
}
