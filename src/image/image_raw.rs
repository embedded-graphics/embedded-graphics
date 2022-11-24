use core::marker::PhantomData;

use crate::{
    draw_target::DrawTarget,
    geometry::{Dimensions, OriginDimensions, Point, Size},
    image::{arrangement::Horizontal, GetPixel, ImageDrawable},
    iterator::raw::RawDataSlice,
    pixelcolor::{
        raw::{storage::ColorStorage, RawData},
        PixelColor,
    },
    primitives::Rectangle,
    Pixel,
};

use super::arrangement::PixelArrangement;

/// An image constructed from a slice of raw pixel data.
///
/// The `ImageRaw` struct is constructed from a slice of raw image data. Because the raw image data
/// doesn't contain metadata to define the storage format, the `CS` and `A` type parameters must be
/// used to set the format.
///
/// `CS` is used to specify which [`PixelColor`] is stored inside the image. In addition to the
/// color type the `CS` parameter is also used to set the bit order for colors with less than 8 bits
/// per pixel, or the byte order (endianness) for colors with more than 8 bits per pixel. If no
/// order is specified the default order is [`Msb0`] for < 8 BPP and [`LittleEndian`] for > 8 BPP.
///
/// For color types with less than 8 bits per pixel each row (or column if `A` is `Vertical`) is
/// expected to be aligned with the start of a byte.
///
/// The `A` parameter is used to set if the image data is arranged in rows or columns.
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
///     image::{Image, ImageRaw, arrangement::Horizontal},
///     pixelcolor::{BinaryColor, raw::storage::Msb0},
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
/// // by using the turbofish syntax. For the image dimensions only the width must be
/// // passed to the `new` function. The image height will be calculated based on the
/// // length of the image data and the data format.
/// let raw_image = ImageRaw::<Msb0<BinaryColor>, Horizontal>::new(DATA, 12);
/// // Because `Msb0` and `Horizontal` are the default values this can be abbreviated to:
/// let raw_image = ImageRaw::<BinaryColor>::new(DATA, 12);
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
///
/// ```
/// use embedded_graphics::{
///     image::{Image, ImageRaw},
///     pixelcolor::{
///         raw::storage::{BigEndian, LittleEndian},
///         Rgb565, Rgb888,
///     },
///     prelude::*,
/// };
/// # const DATA: &[u8] = &[0x55; 8 * 8 * 3];
///
/// // Rgb888 image with 24 bits per pixel and big endian byte order
/// let image = ImageRaw::<BigEndian<Rgb888>>::new(DATA, 8);
///
/// // Rgb565 image with 16 bits per pixel and little endian byte order
/// let image = ImageRaw::<LittleEndian<Rgb565>>::new(DATA, 16);
/// ```
///
/// [`raw` module documentation]: crate::pixelcolor::raw
/// [`Image`]: crate::image::Image
/// [`Drawable`]: crate::drawable::Drawable
/// [`PixelColor`]: crate::pixelcolor::PixelColor
/// [`ByteOrder`]: crate::pixelcolor::raw::ByteOrder
/// [`Msb0`]: crate::pixelcolor::raw::storage::Msb0
/// [`LittleEndian`]: crate::pixelcolor::raw::storage::LittleEndian
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct ImageRaw<'a, CS, A = Horizontal> {
    /// Image data, packed as dictated by raw data type `C::Raw`
    data: &'a [u8],

    main_size: u32,
    cross_size: u32,

    color_storage_type: PhantomData<CS>,
    arrangement: PhantomData<A>,
}

impl<'a, CS, A> ImageRaw<'a, CS, A>
where
    CS: ColorStorage,
    A: PixelArrangement,
{
    /// Creates a new image.
    ///
    /// Only one dimension of the image needs to be specified by the `main_size` parameter.
    /// The other dimension will be calculated based on the length of the image data.
    /// For [`Horizontal`] the `main_size`is the image width and for [`Vertical`] it is the height.
    ///
    /// If the length of the image data isn't an integer multiple of the data length for a single
    /// row the last partial row will be ignored.
    ///
    /// [`Vertical`]: crate::image::arrangement::Vertical
    pub const fn new(data: &'a [u8], main_size: u32) -> Self {
        // Prevent panic for `main_size == 0` by returning a zero sized image.
        if main_size == 0 {
            return Self {
                data: &[],
                main_size: 0,
                cross_size: 0,
                color_storage_type: PhantomData,
                arrangement: PhantomData,
            };
        }

        let bpp = <CS::Color as PixelColor>::Raw::BITS_PER_PIXEL;
        let cross_size = (data.len() / bytes_per_main(main_size, bpp)) as u32;

        Self {
            data,
            main_size,
            cross_size,
            color_storage_type: PhantomData,
            arrangement: PhantomData,
        }
    }

    /// Returns the actual main size in pixels.
    ///
    /// For images with less than 8 bits per pixel each row is padded to contain an integer number
    /// of bytes. This method returns the width of each row including the padding pixels.
    const fn padded_main_size(&self) -> u32 {
        let bpp = <CS::Color as PixelColor>::Raw::BITS_PER_PIXEL;

        if bpp < 8 {
            let pixels_per_byte = 8 / bpp as u32;

            bytes_per_main(self.main_size, bpp) as u32 * pixels_per_byte
        } else {
            self.main_size
        }
    }

    /// Returns the number of padding pixels.
    const fn padding_pixels(&self) -> u32 {
        self.padded_main_size() - self.main_size
    }
}

/// Returns the length along the main axis in bytes.
const fn bytes_per_main(width: u32, bits_per_pixel: usize) -> usize {
    (width as usize * bits_per_pixel + 7) / 8
}

impl<'a, CS, A> ImageDrawable for ImageRaw<'a, CS, A>
where
    CS: ColorStorage,
    A: PixelArrangement,
    <<CS as ColorStorage>::Color as PixelColor>::Raw: Into<CS::Color>,
    RawDataSlice<'a, CS::RawStorage>: IntoIterator<Item = <CS::Color as PixelColor>::Raw>,
{
    type Color = CS::Color;

    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let colors = ContiguousPixels::new(
            self,
            0,
            self.main_size,
            self.cross_size,
            self.padding_pixels() as usize,
        );

        if A::IS_HORIZONTAL {
            target.fill_contiguous(&self.bounding_box(), colors)
        } else {
            target.draw_iter(
                self.bounding_box()
                    .points_vertical()
                    .zip(colors)
                    .map(|(p, c)| Pixel(p, c)),
            )
        }
    }

    fn draw_sub_image<D>(&self, target: &mut D, area: &Rectangle) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let size = self.size();

        // Don't draw anything if `area` is zero sized or partially outside the image.
        if area.is_zero_sized()
            || area.top_left.x < 0
            || area.top_left.y < 0
            || area.top_left.x as u32 + area.size.width > size.width
            || area.top_left.y as u32 + area.size.height > size.height
        {
            return Ok(());
        }

        let data_width = self.padded_main_size() as usize;

        let initial_skip;
        let main_size;
        let cross_size;

        if A::IS_HORIZONTAL {
            main_size = area.size.width;
            cross_size = area.size.height;
            initial_skip = area.top_left.y as usize * data_width + area.top_left.x as usize;
        } else {
            main_size = area.size.height;
            cross_size = area.size.width;
            initial_skip = area.top_left.x as usize * data_width + area.top_left.y as usize;
        };

        let cross_skip = data_width - main_size as usize;

        let area = Rectangle::new(Point::zero(), area.size);
        let colors = ContiguousPixels::new(self, initial_skip, main_size, cross_size, cross_skip);

        if A::IS_HORIZONTAL {
            target.fill_contiguous(&area, colors)
        } else {
            target.draw_iter(area.points_vertical().zip(colors).map(|(p, c)| Pixel(p, c)))
        }
    }
}

impl<CS, A: PixelArrangement> OriginDimensions for ImageRaw<'_, CS, A> {
    fn size(&self) -> Size {
        if A::IS_HORIZONTAL {
            Size::new(self.main_size, self.cross_size)
        } else {
            Size::new(self.cross_size, self.main_size)
        }
    }
}

impl<'a, CS, A> GetPixel for ImageRaw<'a, CS, A>
where
    CS: ColorStorage,
    A: PixelArrangement,
    <<CS as ColorStorage>::Color as PixelColor>::Raw: Into<CS::Color>,
    RawDataSlice<'a, CS::RawStorage>: IntoIterator<Item = <CS::Color as PixelColor>::Raw>,
{
    type Color = CS::Color;

    fn pixel(&self, p: Point) -> Option<Self::Color> {
        if p.x < 0 || p.y < 0 || p.x >= self.size().width as i32 || p.y >= self.size().height as i32
        {
            return None;
        }

        let index = if A::IS_HORIZONTAL {
            p.x as usize + p.y as usize * self.padded_main_size() as usize
        } else {
            p.x as usize * self.padded_main_size() as usize + p.y as usize
        };

        RawDataSlice::new(self.data)
            .into_iter()
            .nth(index)
            .map(Into::into)
    }
}

struct ContiguousPixels<'a, CS>
where
    CS: ColorStorage,
    RawDataSlice<'a, CS::RawStorage>: IntoIterator<Item = <CS::Color as PixelColor>::Raw>,
{
    iter: <RawDataSlice<'a, CS::RawStorage> as IntoIterator>::IntoIter,

    main_remaining: u32,
    main_size: u32,

    cross_remaining: u32,
    cross_skip: usize,
}

impl<'a, CS> ContiguousPixels<'a, CS>
where
    CS: ColorStorage,
    RawDataSlice<'a, CS::RawStorage>: IntoIterator<Item = <CS::Color as PixelColor>::Raw>,
{
    fn new<A>(
        image: &ImageRaw<'a, CS, A>,
        initial_skip: usize,
        main_size: u32,
        cross_size: u32,
        cross_skip: usize,
    ) -> Self {
        let mut iter = RawDataSlice::new(image.data).into_iter();

        if initial_skip > 0 {
            iter.nth(initial_skip - 1);
        }

        // Set `cross_remaining` to `0` if `main_size == 0` to prevent integer underflow in `next`.
        let cross_remaining = if main_size > 0 { cross_size } else { 0 };

        Self {
            iter,
            main_remaining: main_size,
            main_size,
            cross_remaining,
            cross_skip,
        }
    }
}

impl<'a, CS> Iterator for ContiguousPixels<'a, CS>
where
    CS: ColorStorage,
    <<CS as ColorStorage>::Color as PixelColor>::Raw: Into<CS::Color>,
    RawDataSlice<'a, CS::RawStorage>: IntoIterator<Item = <CS::Color as PixelColor>::Raw>,
{
    type Item = CS::Color;

    fn next(&mut self) -> Option<Self::Item> {
        if self.main_remaining > 0 {
            self.main_remaining -= 1;

            self.iter.next()
        } else {
            if self.cross_remaining == 0 {
                return None;
            }

            self.cross_remaining -= 1;
            self.main_remaining = self.main_size - 1;

            self.iter.nth(self.cross_skip)
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
        image::{arrangement::Vertical, Image},
        iterator::PixelIteratorExt,
        mock_display::{ColorMapping, MockDisplay},
        pixelcolor::{
            raw::{
                storage::{BigEndian, LittleEndian, Lsb0, Msb0},
                RawU32,
            },
            Bgr888, BinaryColor, Gray2, Gray4, Gray8, Rgb565, Rgb888,
        },
        prelude::ImageDrawableExt,
        primitives::PointsIter,
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

    fn draw_image<'a, CS, A>(
        image_data: &'a [u8],
        main_size: u32,
    ) -> (ImageRaw<'a, CS, A>, MockDisplay<CS::Color>)
    where
        CS: ColorStorage,
        A: PixelArrangement,
        ImageRaw<'a, CS, A>: ImageDrawable<Color = CS::Color>,
    {
        let image_raw = ImageRaw::<CS, A>::new(&image_data, main_size);

        let mut display = MockDisplay::new();
        Image::new(&image_raw, Point::zero())
            .draw(&mut display)
            .unwrap();

        (image_raw, display)
    }

    // Draws the image using the `GetPixel` implementation.
    fn draw_image_get_pixel<'a, CS, A>(image_raw: &ImageRaw<'a, CS, A>) -> MockDisplay<CS::Color>
    where
        CS: ColorStorage,
        A: PixelArrangement,
        ImageRaw<'a, CS, A>: GetPixel<Color = CS::Color>,
    {
        let mut display = MockDisplay::new();
        display
            .bounding_box()
            .points()
            .filter_map(|p| image_raw.pixel(p).map(|c| Pixel(p, c)))
            .draw(&mut display)
            .unwrap();

        display
    }

    // Draws the image by splitting the image in 2x2 subimages.
    fn draw_image_subimage<'a, CS, A>(image_raw: &ImageRaw<'a, CS, A>) -> MockDisplay<CS::Color>
    where
        CS: ColorStorage,
        A: PixelArrangement,
        ImageRaw<'a, CS, A>: ImageDrawable<Color = CS::Color>,
    {
        let mut display = MockDisplay::new();

        let bb = image_raw.bounding_box();
        for x in bb.columns().step_by(2) {
            for y in bb.rows().step_by(2) {
                let area = Rectangle::new(Point::new(x, y), Size::new_equal(2));

                Image::new(&image_raw.sub_image(&area), area.top_left)
                    .draw(&mut display)
                    .unwrap();
            }
        }

        display
    }

    /// Tests if the given image data matches an excepted `MockDisplay` pattern.
    fn assert_image<CS>(
        image_data: &[u8],
        main_size: u32,
        expected_pattern: &[&str],
        expected_size: Size,
    ) where
        CS: ColorStorage,
        <<CS as ColorStorage>::Color as PixelColor>::Raw: Into<CS::Color>,
        CS::Color: ColorMapping + core::fmt::Debug,
        for<'a> RawDataSlice<'a, CS::RawStorage>:
            IntoIterator<Item = <CS::Color as PixelColor>::Raw>,
    {
        // Horizontal (ImageDrawable)

        let (image_horizontal, display_horizontal) =
            draw_image::<CS, Horizontal>(&image_data, main_size);
        assert_eq!(image_horizontal.size(), expected_size);

        display_horizontal.assert_pattern(expected_pattern);

        // Horizontal (GetPixel)

        draw_image_get_pixel(&image_horizontal).assert_pattern(expected_pattern);

        // Horizontal SubImage

        let subimage_horizontal = draw_image_subimage(&image_horizontal);
        subimage_horizontal.assert_eq(&display_horizontal);

        // Vertical (ImageDrawable)

        let expected_vertical = MockDisplay::from_pattern(expected_pattern).swap_xy();

        let (image_vertical, display_vertical) = draw_image::<CS, Vertical>(&image_data, main_size);
        assert_eq!(
            image_vertical.size(),
            Size::new(expected_size.height, expected_size.width)
        );

        display_vertical.assert_eq(&expected_vertical);

        // Vertical (GetPixel)

        draw_image_get_pixel(&image_vertical).assert_eq(&expected_vertical);

        // Vertical SubImage

        let subimage_horizontal = draw_image_subimage(&image_vertical);
        subimage_horizontal.assert_eq(&expected_vertical);
    }

    #[test]
    fn truncated_data() {
        assert_image::<Msb0<BinaryColor>>(
            &[
                0xAA, 0x00, //
                0x55, 0xFF, //
                0xAA, //
            ],
            9,
            &[
                "#.#.#.#..", //
                ".#.#.#.##", //
            ],
            Size::new(9, 2),
        );
    }

    #[test]
    fn bpp1_msb0() {
        assert_image::<Msb0<BinaryColor>>(
            &[
                0xAA, 0x00, //
                0x55, 0xFF, //
                0xAA, 0x80, //
            ],
            9,
            &[
                "#.#.#.#..", //
                ".#.#.#.##", //
                "#.#.#.#.#", //
            ],
            Size::new(9, 3),
        );
    }

    #[test]
    fn bpp1_msb0_is_default() {
        assert_image::<BinaryColor>(
            &[
                0xAA, 0x00, //
                0x55, 0xFF, //
                0xAA, 0x80, //
            ],
            9,
            &[
                "#.#.#.#..", //
                ".#.#.#.##", //
                "#.#.#.#.#", //
            ],
            Size::new(9, 3),
        );
    }

    #[test]
    fn bpp1_lsb0() {
        assert_image::<Lsb0<BinaryColor>>(
            &[
                0xAA, 0x00, //
                0x55, 0xFF, //
                0xAA, 0x80, //
            ],
            9,
            &[
                ".#.#.#.#.", //
                "#.#.#.#.#", //
                ".#.#.#.#.", //
            ],
            Size::new(9, 3),
        );
    }

    #[test]
    fn bpp2_msb0() {
        assert_image::<Msb0<Gray2>>(
            &[
                0b00_01_10_11, //
                0b00_00_00_00, //
                0b11_10_01_00, //
                0b11_11_11_11, //
            ],
            5,
            &[
                "01230", //
                "32103", //
            ],
            Size::new(5, 2),
        );
    }

    #[test]
    fn bpp2_lsb0() {
        assert_image::<Lsb0<Gray2>>(
            &[
                0b00_01_10_11, //
                0b00_00_00_00, //
                0b11_10_01_00, //
                0b11_11_11_11, //
            ],
            5,
            &[
                "32100", //
                "01233", //
            ],
            Size::new(5, 2),
        );
    }

    #[test]
    fn bpp4_msb0() {
        assert_image::<Msb0<Gray4>>(
            &[
                0b0001_1000, //
                0b1111_0000, //
                0b0101_1010, //
                0b0000_0000, //
            ],
            3,
            &[
                "18F", //
                "5A0", //
            ],
            Size::new(3, 2),
        );
    }

    #[test]
    fn bpp4_lsb0() {
        assert_image::<Lsb0<Gray4>>(
            &[
                0b0001_1000, //
                0b1111_0000, //
                0b0101_1010, //
                0b0000_0000, //
            ],
            3,
            &[
                "810", //
                "A50", //
            ],
            Size::new(3, 2),
        );
    }

    #[test]
    fn bpp8_1() {
        assert_image::<Gray8>(
            &[
                0x11, 0x22, //
                0x33, 0x44, //
                0x55, 0x66, //
            ],
            2,
            &[
                "12", //
                "34", //
                "56", //
            ],
            Size::new(2, 3),
        );
    }

    #[test]
    fn bpp8_little_endian() {
        assert_image::<LittleEndian<Gray8>>(
            &[
                0x11, 0x22, //
                0x33, 0x44, //
                0x55, 0x66, //
            ],
            2,
            &[
                "12", //
                "34", //
                "56", //
            ],
            Size::new(2, 3),
        );
    }

    #[test]
    fn bpp8_big_endian() {
        assert_image::<BigEndian<Gray8>>(
            &[
                0x11, 0x22, //
                0x33, 0x44, //
                0x55, 0x66, //
            ],
            2,
            &[
                "12", //
                "34", //
                "56", //
            ],
            Size::new(2, 3),
        );
    }

    /// Additional test for luma values with different low and high nibbles,
    /// which are not supported by `MockDisplay` patterns.
    #[test]
    fn bpp8_2() {
        let data = [0x01, 0x08, 0x10, 0x80];
        let image_data = ImageRaw::<Gray8>::new(&data, 4);

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
    fn bpp16_little_endian_is_default() {
        assert_image::<Rgb565>(
            &[
                0x00, 0xF8, //
                0xE0, 0x07, //
                0x1F, 0x00, //
                0x00, 0x00, //
            ],
            1,
            &[
                "R", //
                "G", //
                "B", //
                "K", //
            ],
            Size::new(1, 4),
        );
    }

    #[test]
    fn bpp16_little_endian() {
        assert_image::<LittleEndian<Rgb565>>(
            &[
                0x00, 0xF8, //
                0xE0, 0x07, //
                0x1F, 0x00, //
                0x00, 0x00, //
            ],
            1,
            &[
                "R", //
                "G", //
                "B", //
                "K", //
            ],
            Size::new(1, 4),
        );
    }

    #[test]
    fn bpp16_big_endian() {
        assert_image::<BigEndian<Rgb565>>(
            &[
                0xF8, 0x00, //
                0x07, 0xE0, //
                0x00, 0x1F, //
                0x00, 0x00, //
            ],
            2,
            &[
                "RG", //
                "BK", //
            ],
            Size::new(2, 2),
        );
    }

    #[test]
    fn bpp24_little_endian() {
        assert_image::<LittleEndian<Bgr888>>(
            &[
                0xFF, 0x00, 0x00, //
                0x00, 0xFF, 0x00, //
                0x00, 0x00, 0xFF, //
                0x00, 0x00, 0x00, //
            ],
            1,
            &[
                "R", //
                "G", //
                "B", //
                "K", //
            ],
            Size::new(1, 4),
        );
    }

    #[test]
    fn bpp24_big_endian() {
        assert_image::<BigEndian<Rgb888>>(
            &[
                0xFF, 0x00, 0x00, //
                0x00, 0xFF, 0x00, //
                0x00, 0x00, 0xFF, //
                0x00, 0x00, 0x00, //
            ],
            4,
            &["RGBK"],
            Size::new(4, 1),
        );
    }

    #[test]
    fn bpp32_little_endian() {
        let data = [
            0x12, 0x34, 0x56, 0x78, //
            0x9A, 0xBC, 0xDE, 0xF0, //
            0x00, 0x00, 0x00, 0x00, //
            0xFF, 0xFF, 0xFF, 0xFF, //
        ];
        let image_data: ImageRaw<LittleEndian<TestColorU32>> = ImageRaw::new(&data, 2);

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
        let image_data: ImageRaw<BigEndian<TestColorU32>> = ImageRaw::new(&data, 4);

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
    fn calculated_height() {
        let data = [0u8; 1];
        assert_eq!(
            ImageRaw::<Msb0<BinaryColor>>::new(&data, 12).size().height,
            0
        );

        let data = [0u8; 2];
        assert_eq!(
            ImageRaw::<Msb0<BinaryColor>>::new(&data, 12).size().height,
            1
        );

        let data = [0u8; 3];
        assert_eq!(
            ImageRaw::<Msb0<BinaryColor>>::new(&data, 12).size().height,
            1
        );

        let data = [0u8; 4];
        assert_eq!(
            ImageRaw::<Msb0<BinaryColor>>::new(&data, 12).size().height,
            2
        );
    }

    #[test]
    fn binary_image_with_zero_width() {
        let image = ImageRaw::<Msb0<BinaryColor>>::new(&[], 0);

        assert_eq!(image.size(), Size::zero());
    }

    #[test]
    fn pixel_out_of_bounds() {
        let data = [
            0xAA, 0x00, //
            0x55, 0xFF, //
            0xAA, 0x80, //
        ];
        let image_data = ImageRaw::<Msb0<BinaryColor>>::new(&data, 9);

        assert_eq!(image_data.pixel(Point::new(-1, 0)), None);
        assert_eq!(image_data.pixel(Point::new(0, -1)), None);
        assert_eq!(image_data.pixel(Point::new(9, 0)), None);
        assert_eq!(image_data.pixel(Point::new(9, 3)), None);
    }
}
