use core::marker::PhantomData;

use crate::{
    draw_target::DrawTarget,
    geometry::{Dimensions, OriginDimensions, Point, Size},
    image::{GetPixel, ImageDrawable},
    iterator::raw::RawDataSlice,
    pixelcolor::{
        raw::{BigEndian, ByteOrder, LittleEndian, RawData},
        PixelColor,
    },
    primitives::Rectangle,
};

/// Image with little endian data.
pub type ImageRawLE<'a, C> = ImageRaw<'a, C, LittleEndian>;

/// Image with big endian data.
pub type ImageRawBE<'a, C> = ImageRaw<'a, C, BigEndian>;

/// Error returned by [`ImageRaw::new`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImageRawError {
    /// Invalid data size.
    InvalidDataSize {
        /// The expected data size in bytes.
        expected_data_size: usize,
    },
}

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
/// /// 12 x 5 pixel image with 1 bit per pixel.
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
/// // The image dimensions and the format of the stored raw data must be specified
/// // when the `new` function is called. The data format can, for example, be specified
/// // by using the turbofish syntax.
/// let raw_image = ImageRaw::<BinaryColor>::new(DATA, Size::new(12, 5)).unwrap();
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
/// let image1 = ImageRawBE::<Rgb888>::new(DATA, Size::new(8, 8)).unwrap();
/// // or:
/// let image2 = ImageRaw::<Rgb888, BigEndian>::new(DATA, Size::new(8, 8)).unwrap();
/// # assert_eq!(image1, image2);
///
/// // Rgb565 image with 16 bits per pixel and little endian byte order
/// let image1 = ImageRawLE::<Rgb565>::new(DATA, Size::new(12, 8)).unwrap();
/// // or:
/// let image2 = ImageRaw::<Rgb565, LittleEndian>::new(DATA, Size::new(12, 8)).unwrap();
/// # assert_eq!(image1, image2);
/// ```
///
/// [`raw` module documentation]: crate::pixelcolor::raw
/// [`Image`]: crate::image::Image
/// [`Drawable`]: crate::drawable::Drawable
/// [`PixelColor`]: crate::pixelcolor::PixelColor
/// [`ByteOrder`]: crate::pixelcolor::raw::ByteOrder
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
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
    /// Returns an error if the length of the data size doesn't match the
    /// expected size based on the given image dimensions and the color depth.
    pub const fn new(data: &'a [u8], size: Size) -> Result<Self, ImageRawError> {
        let expected_size =
            bytes_per_row(size.width, C::Raw::BITS_PER_PIXEL) * size.height as usize;

        if data.len() != expected_size {
            return Err(ImageRawError::InvalidDataSize {
                expected_data_size: expected_size,
            });
        }

        Ok(Self {
            data,
            size,
            pixel_type: PhantomData,
            byte_order: PhantomData,
        })
    }

    /// Creates a new image in a const context.
    ///
    /// While [`ImageRaw::new`] is also `const` it cannot easily be used to
    /// initialize a constant, because `unwrap()` isn't available in a const
    /// context in stable Rust. This method provides an alternative that panics
    /// instead of returning an error.
    ///
    /// # Panics
    ///
    /// Panics if the given slice of data doesn't have the correct length to
    /// match the image size and bit depth.
    pub const fn new_const(data: &'a [u8], size: Size) -> Self {
        match Self::new(data, size) {
            Ok(image) => image,
            Err(ImageRawError::InvalidDataSize { .. }) => panic!("Invalid data size"),
        }
    }

    /// Returns the actual row width in pixels.
    ///
    /// For images with less than 8 bits per pixel each row is padded to contain an integer number
    /// of bytes. This method returns the width of each row including the padding pixels.
    const fn data_width(&self) -> u32 {
        if C::Raw::BITS_PER_PIXEL < 8 {
            let pixels_per_byte = 8 / C::Raw::BITS_PER_PIXEL as u32;

            bytes_per_row(self.size.width, C::Raw::BITS_PER_PIXEL) as u32 * pixels_per_byte
        } else {
            self.size.width
        }
    }
}

/// Returns the length of each row in bytes.
const fn bytes_per_row(width: u32, bits_per_pixel: usize) -> usize {
    (width as usize * bits_per_pixel + 7) / 8
}

impl<'a, C, BO> ImageDrawable for ImageRaw<'a, C, BO>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    BO: ByteOrder,
    RawDataSlice<'a, C::Raw, BO>: IntoIterator<Item = C::Raw>,
{
    type Color = C;

    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        let row_skip = self.data_width() - self.size.width;

        target.fill_contiguous(
            &self.bounding_box(),
            ContiguousPixels::new(self, self.size, 0, row_skip as usize),
        )
    }

    fn draw_sub_image<D>(&self, target: &mut D, area: &Rectangle) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        // Don't draw anything if `area` is zero sized or partially outside the image.
        if area.is_zero_sized()
            || area.top_left.x < 0
            || area.top_left.y < 0
            || area.top_left.x as u32 + area.size.width > self.size.width
            || area.top_left.y as u32 + area.size.height > self.size.height
        {
            return Ok(());
        }

        let data_width = self.data_width() as usize;

        let initial_skip = area.top_left.y as usize * data_width + area.top_left.x as usize;
        let row_skip = data_width - area.size.width as usize;

        target.fill_contiguous(
            &Rectangle::new(Point::zero(), area.size),
            ContiguousPixels::new(self, area.size, initial_skip, row_skip),
        )
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

impl<'a, C, BO> GetPixel for ImageRaw<'a, C, BO>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    BO: ByteOrder,
    RawDataSlice<'a, C::Raw, BO>: IntoIterator<Item = C::Raw>,
{
    type Color = C;

    fn pixel(&self, p: Point) -> Option<Self::Color> {
        if p.x < 0 || p.y < 0 || p.x >= self.size.width as i32 || p.y >= self.size.height as i32 {
            return None;
        }

        RawDataSlice::new(self.data)
            .into_iter()
            .nth(p.x as usize + p.y as usize * self.data_width() as usize)
            .map(|r| r.into())
    }
}

struct ContiguousPixels<'a, C, BO>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    BO: ByteOrder,
    RawDataSlice<'a, C::Raw, BO>: IntoIterator<Item = C::Raw>,
{
    iter: <RawDataSlice<'a, C::Raw, BO> as IntoIterator>::IntoIter,

    remaining_x: u32,
    width: u32,

    remaining_y: u32,
    row_skip: usize,
}

impl<'a, C, BO> ContiguousPixels<'a, C, BO>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    BO: ByteOrder,
    RawDataSlice<'a, C::Raw, BO>: IntoIterator<Item = C::Raw>,
{
    fn new(image: &ImageRaw<'a, C, BO>, size: Size, initial_skip: usize, row_skip: usize) -> Self {
        let mut iter = RawDataSlice::new(image.data).into_iter();

        if initial_skip > 0 {
            iter.nth(initial_skip - 1);
        }

        // Set `remaining_y` to `0` if `width == 0` to prevent integer underflow in `next`.
        let remaining_y = if size.width > 0 { size.height } else { 0 };

        Self {
            iter,
            remaining_x: size.width,
            width: size.width,
            remaining_y,
            row_skip,
        }
    }
}

impl<'a, C, BO> Iterator for ContiguousPixels<'a, C, BO>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    BO: ByteOrder,
    RawDataSlice<'a, C::Raw, BO>: IntoIterator<Item = C::Raw>,
{
    type Item = C;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_x > 0 {
            self.remaining_x -= 1;

            self.iter.next()
        } else {
            if self.remaining_y == 0 {
                return None;
            }

            self.remaining_y -= 1;
            self.remaining_x = self.width - 1;

            self.iter.nth(self.row_skip)
        }
        .map(|c| c.into())
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
        for<'a> RawDataSlice<'a, C::Raw, BO>: IntoIterator<Item = C::Raw>,
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
        let image_data: ImageRaw<BinaryColor> = ImageRaw::new(&data, Size::new(9, 3)).unwrap();

        assert_eq!(image_data.size(), Size::new(9, 3));
    }

    #[test]
    fn bpp1_new() {
        let data = [
            0xAA, 0x00, //
            0x55, 0xFF, //
            0xAA, 0x80, //
        ];
        let image_data: ImageRaw<BinaryColor> = ImageRaw::new(&data, Size::new(9, 3)).unwrap();

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
    fn bpp1_get_pixel() {
        let data = [
            0xAA, 0x00, //
            0x55, 0xFF, //
            0xAA, 0x80, //
        ];
        let image_data: ImageRaw<BinaryColor> = ImageRaw::new(&data, Size::new(9, 3)).unwrap();

        assert_eq!(image_data.pixel(Point::new(0, 0)), Some(BinaryColor::On));
        assert_eq!(image_data.pixel(Point::new(8, 0)), Some(BinaryColor::Off));
        assert_eq!(image_data.pixel(Point::new(0, 1)), Some(BinaryColor::Off));
        assert_eq!(image_data.pixel(Point::new(8, 1)), Some(BinaryColor::On));
        assert_eq!(image_data.pixel(Point::new(0, 2)), Some(BinaryColor::On));
        assert_eq!(image_data.pixel(Point::new(8, 2)), Some(BinaryColor::On));
    }

    #[test]
    fn bpp2() {
        let data = [
            0b00_01_10_11, //
            0b00_00_00_00, //
            0b11_10_01_00, //
            0b11_11_11_11, //
        ];
        let image_data: ImageRaw<Gray2> = ImageRaw::new(&data, Size::new(5, 2)).unwrap();

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
        let image_data: ImageRaw<Gray4> = ImageRaw::new(&data, Size::new(3, 2)).unwrap();

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
        let image_data: ImageRaw<Gray8> = ImageRaw::new(&data, Size::new(2, 3)).unwrap();

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
        let image_data: ImageRaw<Gray8> = ImageRaw::new(&data, Size::new(4, 1)).unwrap();

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
        let image_data: ImageRawLE<Rgb565> = ImageRaw::new(&data, Size::new(1, 4)).unwrap();

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
        let image_data: ImageRawBE<Rgb565> = ImageRaw::new(&data, Size::new(2, 2)).unwrap();

        assert_pattern(
            image_data,
            &[
                "RG", //
                "BK", //
            ],
        );
    }

    #[test]
    fn bpp16_big_endian_get_pixel() {
        let data = [
            0xF8, 0x00, //
            0x07, 0xE0, //
            0x00, 0x1F, //
            0x00, 0x00, //
        ];
        let image_data: ImageRawBE<Rgb565> = ImageRaw::new(&data, Size::new(2, 2)).unwrap();

        assert_eq!(image_data.pixel(Point::new(0, 0)), Some(Rgb565::RED));
        assert_eq!(image_data.pixel(Point::new(1, 0)), Some(Rgb565::GREEN));
        assert_eq!(image_data.pixel(Point::new(0, 1)), Some(Rgb565::BLUE));
        assert_eq!(image_data.pixel(Point::new(1, 1)), Some(Rgb565::BLACK));
    }

    #[test]
    fn bpp24_little_endian() {
        let data = [
            0xFF, 0x00, 0x00, //
            0x00, 0xFF, 0x00, //
            0x00, 0x00, 0xFF, //
            0x00, 0x00, 0x00, //
        ];
        let image_data: ImageRawLE<Bgr888> = ImageRaw::new(&data, Size::new(1, 4)).unwrap();

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
        let image_data: ImageRawBE<Rgb888> = ImageRaw::new(&data, Size::new(4, 1)).unwrap();

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
        let image_data: ImageRawLE<TestColorU32> = ImageRaw::new(&data, Size::new(2, 2)).unwrap();

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
        let image_data: ImageRawBE<TestColorU32> = ImageRaw::new(&data, Size::new(4, 1)).unwrap();

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
    fn binary_image_with_zero_width() {
        let image = ImageRaw::<BinaryColor>::new(&[], Size::new(0, 10)).unwrap();
        assert_eq!(image.size, Size::new(0, 10));

        let mut display = MockDisplay::new();
        Image::new(&image, Point::zero())
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[]);
    }

    #[test]
    fn binary_image_with_zero_height() {
        let image = ImageRaw::<BinaryColor>::new(&[], Size::new(5, 0)).unwrap();
        assert_eq!(image.size, Size::new(5, 0));

        let mut display = MockDisplay::new();
        Image::new(&image, Point::zero())
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[]);
    }

    #[test]
    fn pixel_out_of_bounds() {
        let data = [
            0xAA, 0x00, //
            0x55, 0xFF, //
            0xAA, 0x80, //
        ];
        let image_data = ImageRaw::<BinaryColor>::new(&data, Size::new(9, 3)).unwrap();

        assert_eq!(image_data.pixel(Point::new(-1, 0)), None);
        assert_eq!(image_data.pixel(Point::new(0, -1)), None);
        assert_eq!(image_data.pixel(Point::new(9, 0)), None);
        assert_eq!(image_data.pixel(Point::new(9, 3)), None);
    }
}
