use crate::{
    drawable::{Drawable, Pixel},
    geometry::{Dimensions, Point, Size},
    pixelcolor::{
        raw::{BigEndian, ByteOrder, LittleEndian, RawData, RawDataIter},
        PixelColor,
    },
    transform::Transform,
    DrawTarget,
};
use core::marker::PhantomData;

/// Image with little endian data.
pub type ImageLE<'a, C> = Image<'a, C, LittleEndian>;

/// Image with big endian data.
pub type ImageBE<'a, C> = Image<'a, C, BigEndian>;

/// An image constructed from a slice.
///
/// The `Image` struct can be used to construct an image drawable from a slice
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
/// # Examples
///
/// This example creates an image from 1 bit per pixel data:
///
/// ```
/// use embedded_graphics::{image::Image, pixelcolor::BinaryColor, prelude::*};
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
/// fn main() {
///     // The type annotation `Image<BinaryColor>` is used to specify the format
///     // of the stored raw data (`PixelColor::Raw`) and which color type the
///     // raw data gets converted into.
///     let image: Image<BinaryColor> = Image::new(DATA, 12, 5);
///
///     let mut display = Display::default();
///     image.draw(&mut display).unwrap();
/// }
/// ```
///
/// Colors with more than one byte per pixel need an additional type annotation
/// for the byte order. The [`ImageBE`] and [`ImageLE`] type aliases can be used
/// to abbreviate the type.
///
/// ```
/// use embedded_graphics::{
///     image::{Image, ImageBE, ImageLE},
///     pixelcolor::{
///         raw::{BigEndian, LittleEndian},
///         Rgb565, Rgb888,
///     },
///     prelude::*,
/// };
/// # const DATA: &[u8] = &[0x55; 8 * 8 * 3];
///
/// // Rgb888 image with 24 bits per pixel and big endian byte order
/// let image1: ImageBE<Rgb888> = Image::new(DATA, 8, 8);
/// // or:
/// let image2: Image<Rgb888, BigEndian> = Image::new(DATA, 8, 8);
/// # assert_eq!(image1, image2);
///
/// // Rgb565 image with 16 bits per pixel and little endian byte order
/// let image1: ImageLE<Rgb565> = Image::new(DATA, 16, 6);
/// // or:
/// let image2: Image<Rgb565, LittleEndian> = Image::new(DATA, 16, 6);
/// # assert_eq!(image1, image2);
/// ```
///
/// [`raw` module documentation]: ../pixelcolor/raw/index.html
/// [`ImageBE`]: type.ImageBE.html
/// [`ImageLE`]: type.ImageLE.html
/// [`PixelColor`]: ../pixelcolor/trait.PixelColor.html
/// [`ByteOrder`]: ../pixelcolor/raw/trait.ByteOrder.html
#[derive(Debug, PartialEq, Eq)]
pub struct Image<'a, C, BO = BigEndian>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    BO: ByteOrder,
{
    /// Image data, packed as dictated by raw data type `C::Raw`
    data: &'a [u8],

    /// Image size in pixels
    size: Size,

    /// Image offset in pixels from screen origin (0,0)
    offset: Point,

    pixel_type: PhantomData<C>,
    byte_order: PhantomData<BO>,
}

impl<'a, C, BO> Image<'a, C, BO>
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
        let ret = Image {
            data,
            size: Size::new(width, height),
            offset: Point::new(0, 0),
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

    /// Returns the offset.
    pub fn offset(&self) -> Point {
        self.offset
    }
}

impl<'a, C, BO> Dimensions for Image<'a, C, BO>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    BO: ByteOrder,
{
    fn top_left(&self) -> Point {
        self.offset
    }

    fn bottom_right(&self) -> Point {
        self.top_left() + self.size()
    }

    fn size(&self) -> Size {
        self.size
    }
}

impl<'a, C, BO> Transform for Image<'a, C, BO>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    BO: ByteOrder,
{
    /// Translate the image from its current position to a new position by (x, y) pixels, returning
    /// a new `Image`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::pixelcolor::BinaryColor;
    /// # use embedded_graphics::image::Image;
    /// # use embedded_graphics::transform::Transform;
    /// # use embedded_graphics::geometry::Point;
    /// #
    /// // 8px x 1px test image
    /// let image: Image<BinaryColor> = Image::new(&[0xff], 8, 1);
    /// let moved = image.translate(Point::new(25, 30));
    ///
    /// assert_eq!(image.offset(), Point::new(0, 0));
    /// assert_eq!(moved.offset(), Point::new(25, 30));
    /// ```
    fn translate(&self, by: Point) -> Self {
        Self {
            data: self.data,
            offset: self.offset + by,
            ..*self
        }
    }

    /// Translate the image from its current position to a new position by (x, y) pixels.
    ///
    /// ```
    /// # use embedded_graphics::pixelcolor::BinaryColor;
    /// # use embedded_graphics::image::Image;
    /// # use embedded_graphics::transform::Transform;
    /// # use embedded_graphics::geometry::Point;
    /// #
    /// let mut image: Image<BinaryColor> = Image::new(&[0xff], 8, 1);
    /// image.translate_mut(Point::new(25, 30));
    ///
    /// assert_eq!(image.offset(), Point::new(25, 30));
    /// ```
    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.offset += by;

        self
    }
}

impl<'a, 'b: 'a, C: 'a, BO: 'a> IntoIterator for &'b Image<'a, C, BO>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    BO: ByteOrder,
    RawDataIter<'a, C::Raw, BO>: Iterator<Item = C::Raw>,
{
    type Item = Pixel<C>;
    type IntoIter = ImageIterator<'a, C, BO>;

    fn into_iter(self) -> Self::IntoIter {
        ImageIterator {
            data: RawDataIter::new(self.data),
            x: 0,
            y: 0,
            image: self,
        }
    }
}

impl<'a, C: 'a, BO: 'a> Drawable<C> for &'a Image<'a, C, BO>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    BO: ByteOrder,
    RawDataIter<'a, C::Raw, BO>: Iterator<Item = C::Raw>,
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        display.draw_iter(self.into_iter())
    }
}

#[derive(Debug)]
pub struct ImageIterator<'a, C, BO>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    BO: ByteOrder,
{
    data: RawDataIter<'a, C::Raw, BO>,

    x: u32,
    y: u32,

    image: &'a Image<'a, C, BO>,
}

impl<'a, C, BO> Iterator for ImageIterator<'a, C, BO>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    BO: ByteOrder,
    RawDataIter<'a, C::Raw, BO>: Iterator<Item = C::Raw>,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y < self.image.size.height {
            let data = self.data.next()?;
            let mut point = Point::new(self.x as i32, self.y as i32);
            point += self.image.offset;

            self.x += 1;
            if self.x >= self.image.size.width {
                self.data.align();

                self.y += 1;
                self.x = 0;
            }

            Some(Pixel(point, data.into()))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        drawable::Pixel,
        pixelcolor::{raw::RawU32, *},
        transform::Transform,
    };

    #[derive(Debug, PartialEq, Clone, Copy)]
    struct TestColorU32(RawU32);

    impl PixelColor for TestColorU32 {
        type Raw = RawU32;
    }

    impl From<RawU32> for TestColorU32 {
        fn from(data: RawU32) -> Self {
            Self(data)
        }
    }

    fn assert_next<I, C>(iter: &mut I, x: i32, y: i32, color: C)
    where
        I: Iterator<Item = Pixel<C>>,
        C: PixelColor + core::fmt::Debug,
    {
        let p = Point::new(x, y);
        assert_eq!(iter.next(), Some(Pixel(p, color)));
    }

    #[test]
    fn negative_top_left() {
        let image: Image<BinaryColor> =
            Image::new(&[0xff, 0x00, 0xff, 0x00], 4, 4).translate(Point::new(-1, -1));

        assert_eq!(image.top_left(), Point::new(-1, -1));
        assert_eq!(image.bottom_right(), Point::new(3, 3));
        assert_eq!(image.size(), Size::new(4, 4));
    }

    #[test]
    fn dimensions() {
        let image: Image<BinaryColor> =
            Image::new(&[0xff, 0x00, 0xFF, 0x00], 4, 4).translate(Point::new(100, 200));

        assert_eq!(image.top_left(), Point::new(100, 200));
        assert_eq!(image.bottom_right(), Point::new(104, 204));
        assert_eq!(image.size(), Size::new(4, 4));
    }

    #[test]
    fn it_can_have_negative_offsets() {
        let image: Image<Gray8> = Image::new(
            &[0xff, 0x00, 0xbb, 0x00, 0xcc, 0x00, 0xee, 0x00, 0xaa],
            3,
            3,
        )
        .translate(Point::new(-1, -1));

        let mut iter = image.into_iter();
        assert_next(&mut iter, -1, -1, Gray8::WHITE);
        assert_next(&mut iter, 0, -1, Gray8::BLACK);
        assert_next(&mut iter, 1, -1, Gray8::new(0xbb));
        assert_next(&mut iter, -1, 0, Gray8::BLACK);
        assert_next(&mut iter, 0, 0, Gray8::new(0xcc));
        assert_next(&mut iter, 1, 0, Gray8::BLACK);
        assert_next(&mut iter, -1, 1, Gray8::new(0xee));
        assert_next(&mut iter, 0, 1, Gray8::BLACK);
        assert_next(&mut iter, 1, 1, Gray8::new(0xaa));
        assert!(iter.next().is_none());
    }

    #[test]
    fn bpp1() {
        let data = [0xAA, 0x00, 0x55, 0xFF, 0xAA, 0x00];
        let image: Image<BinaryColor> = Image::new(&data, 9, 3);

        let mut iter = image.into_iter();
        assert_next(&mut iter, 0, 0, BinaryColor::On);
        assert_next(&mut iter, 1, 0, BinaryColor::Off);
        assert_next(&mut iter, 2, 0, BinaryColor::On);
        assert_next(&mut iter, 3, 0, BinaryColor::Off);
        assert_next(&mut iter, 4, 0, BinaryColor::On);
        assert_next(&mut iter, 5, 0, BinaryColor::Off);
        assert_next(&mut iter, 6, 0, BinaryColor::On);
        assert_next(&mut iter, 7, 0, BinaryColor::Off);

        assert_next(&mut iter, 8, 0, BinaryColor::Off);

        assert_next(&mut iter, 0, 1, BinaryColor::Off);
        assert_next(&mut iter, 1, 1, BinaryColor::On);
        assert_next(&mut iter, 2, 1, BinaryColor::Off);
        assert_next(&mut iter, 3, 1, BinaryColor::On);
        assert_next(&mut iter, 4, 1, BinaryColor::Off);
        assert_next(&mut iter, 5, 1, BinaryColor::On);
        assert_next(&mut iter, 6, 1, BinaryColor::Off);
        assert_next(&mut iter, 7, 1, BinaryColor::On);

        assert_next(&mut iter, 8, 1, BinaryColor::On);

        assert_next(&mut iter, 0, 2, BinaryColor::On);
        assert_next(&mut iter, 1, 2, BinaryColor::Off);
        assert_next(&mut iter, 2, 2, BinaryColor::On);
        assert_next(&mut iter, 3, 2, BinaryColor::Off);
        assert_next(&mut iter, 4, 2, BinaryColor::On);
        assert_next(&mut iter, 5, 2, BinaryColor::Off);
        assert_next(&mut iter, 6, 2, BinaryColor::On);
        assert_next(&mut iter, 7, 2, BinaryColor::Off);

        assert_next(&mut iter, 8, 2, BinaryColor::Off);

        assert!(iter.next().is_none());
    }

    #[test]
    fn bpp2() {
        let data = [0b00011011, 0x0, 0b11100100, 0xFF];
        let image: Image<Gray2> = Image::new(&data, 5, 2);

        let mut iter = image.into_iter();
        assert_next(&mut iter, 0, 0, Gray2::new(0));
        assert_next(&mut iter, 1, 0, Gray2::new(1));
        assert_next(&mut iter, 2, 0, Gray2::new(2));
        assert_next(&mut iter, 3, 0, Gray2::new(3));
        assert_next(&mut iter, 4, 0, Gray2::new(0));

        assert_next(&mut iter, 0, 1, Gray2::new(3));
        assert_next(&mut iter, 1, 1, Gray2::new(2));
        assert_next(&mut iter, 2, 1, Gray2::new(1));
        assert_next(&mut iter, 3, 1, Gray2::new(0));
        assert_next(&mut iter, 4, 1, Gray2::new(3));

        assert!(iter.next().is_none());
    }

    #[test]
    fn bpp4() {
        let data = [0b00011000, 0b11110000, 0b01011010, 0x0];
        let image: Image<Gray4> = Image::new(&data, 3, 2);

        let mut iter = image.into_iter();
        assert_next(&mut iter, 0, 0, Gray4::new(0x1));
        assert_next(&mut iter, 1, 0, Gray4::new(0x8));
        assert_next(&mut iter, 2, 0, Gray4::new(0xF));

        assert_next(&mut iter, 0, 1, Gray4::new(0x5));
        assert_next(&mut iter, 1, 1, Gray4::new(0xA));
        assert_next(&mut iter, 2, 1, Gray4::new(0x0));

        assert!(iter.next().is_none());
    }

    #[test]
    fn bpp8() {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let image: Image<Gray8> = Image::new(&data, 2, 3);

        let mut iter = image.into_iter();
        assert_next(&mut iter, 0, 0, Gray8::new(1));
        assert_next(&mut iter, 1, 0, Gray8::new(2));
        assert_next(&mut iter, 0, 1, Gray8::new(3));
        assert_next(&mut iter, 1, 1, Gray8::new(4));
        assert_next(&mut iter, 0, 2, Gray8::new(5));
        assert_next(&mut iter, 1, 2, Gray8::new(6));

        assert!(iter.next().is_none());
    }

    #[test]
    fn bpp16_little_endian() {
        let data = [0x00, 0xF8, 0xE0, 0x07, 0x1F, 0x00, 0x00, 0x00];
        let image: ImageLE<Rgb565> = Image::new(&data, 1, 4);

        let mut iter = image.into_iter();
        assert_next(&mut iter, 0, 0, Rgb565::RED);
        assert_next(&mut iter, 0, 1, Rgb565::GREEN);
        assert_next(&mut iter, 0, 2, Rgb565::BLUE);
        assert_next(&mut iter, 0, 3, Rgb565::BLACK);

        assert!(iter.next().is_none());
    }

    #[test]
    fn bpp16_big_endian() {
        let data = [0xF8, 0x00, 0x07, 0xE0, 0x00, 0x1F, 0x00, 0x00];
        let image: ImageBE<Rgb565> = Image::new(&data, 2, 2);

        let mut iter = image.into_iter();
        assert_next(&mut iter, 0, 0, Rgb565::RED);
        assert_next(&mut iter, 1, 0, Rgb565::GREEN);
        assert_next(&mut iter, 0, 1, Rgb565::BLUE);
        assert_next(&mut iter, 1, 1, Rgb565::BLACK);

        assert!(iter.next().is_none());
    }

    #[test]
    fn bpp24_little_endian() {
        let data = [
            0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00,
        ];
        let image: ImageLE<Bgr888> = Image::new(&data, 1, 4);

        let mut iter = image.into_iter();
        assert_next(&mut iter, 0, 0, Bgr888::RED);
        assert_next(&mut iter, 0, 1, Bgr888::GREEN);
        assert_next(&mut iter, 0, 2, Bgr888::BLUE);
        assert_next(&mut iter, 0, 3, Bgr888::BLACK);

        assert!(iter.next().is_none());
    }

    #[test]
    fn bpp24_big_endian() {
        let data = [
            0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00,
        ];
        let image: ImageBE<Rgb888> = Image::new(&data, 4, 1);

        let mut iter = image.into_iter();
        assert_next(&mut iter, 0, 0, Rgb888::RED);
        assert_next(&mut iter, 1, 0, Rgb888::GREEN);
        assert_next(&mut iter, 2, 0, Rgb888::BLUE);
        assert_next(&mut iter, 3, 0, Rgb888::BLACK);

        assert!(iter.next().is_none());
    }

    #[test]
    fn bpp32_little_endian() {
        #[rustfmt::skip]
        let data = [
            0x12, 0x34, 0x56, 0x78,
            0x9A, 0xBC, 0xDE, 0xF0,
            0x00, 0x00, 0x00, 0x00,
            0xFF, 0xFF, 0xFF, 0xFF,
        ];
        let image: ImageLE<TestColorU32> = Image::new(&data, 2, 2);

        let mut iter = image.into_iter();
        assert_next(&mut iter, 0, 0, TestColorU32(RawU32::new(0x78563412)));
        assert_next(&mut iter, 1, 0, TestColorU32(RawU32::new(0xF0DEBC9A)));
        assert_next(&mut iter, 0, 1, TestColorU32(RawU32::new(0x00000000)));
        assert_next(&mut iter, 1, 1, TestColorU32(RawU32::new(0xFFFFFFFF)));

        assert!(iter.next().is_none());
    }

    #[test]
    fn bpp32_big_endian() {
        #[rustfmt::skip]
        let data = [
            0x12, 0x34, 0x56, 0x78,
            0x9A, 0xBC, 0xDE, 0xF0,
            0x00, 0x00, 0x00, 0x00,
            0xFF, 0xFF, 0xFF, 0xFF,
        ];
        let image: ImageBE<TestColorU32> = Image::new(&data, 4, 1);

        let mut iter = image.into_iter();
        assert_next(&mut iter, 0, 0, TestColorU32(RawU32::new(0x12345678)));
        assert_next(&mut iter, 1, 0, TestColorU32(RawU32::new(0x9ABCDEF0)));
        assert_next(&mut iter, 2, 0, TestColorU32(RawU32::new(0x00000000)));
        assert_next(&mut iter, 3, 0, TestColorU32(RawU32::new(0xFFFFFFFF)));

        assert!(iter.next().is_none());
    }

    #[test]
    #[should_panic]
    fn panics_if_length_of_data_is_too_short() {
        let data = [0u8; 3];
        let _: Image<BinaryColor> = Image::new(&data, 12, 2);
    }
}
