use core::marker::PhantomData;

use crate::{
    array::{PixelData, PixelSlice},
    common::{
        BufferDimensions, BufferError, ColorType, GetPixel, Horizontal, PixelArrangement,
        PixelArrangementEnum,
    },
    draw_target::DrawTarget,
    geometry::{Dimensions, OriginDimensions, Point, Size},
    image::ImageDrawable,
    pixelcolor::{raw::order::DataOrder, StorablePixelColor},
    primitives::Rectangle,
    Pixel,
};

/// An image constructed from a slice of raw pixel data.
///
/// The `ImageRaw` struct is constructed from a slice of raw image data. Because the raw image data
/// doesn't contain metadata to define the storage format, the type parameters must be used to set
/// the format.
///
/// The `C`and `O` type parameters specify how individual pixels are stored in the pixel data.
/// All color types which implement the [`StorablePixelColor`] trait can be used for the `C` parameter.
/// The `O` parameter defines the bit order for colors with less than 8 BPP and the byte order for
/// colors with more than 8 BPP.
///
/// For color types with less than 8 bits per pixel each row (or column if `A` is `Vertical`) is
/// expected to be aligned with the start of a byte.
///
/// The `A` parameter is used to specify how parameters are arranged in the raw image data.
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
///     common::Horizontal,
///     pixelcolor::{BinaryColor, raw::order::Msb0},
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
/// let raw_image = ImageRaw::<BinaryColor, Msb0, Horizontal>::new(DATA, Size::new(12, 5)).unwrap();
/// // Because `Horizontal` is the default value this can be abbreviated to:
/// let raw_image = ImageRaw::<BinaryColor, Msb0>::new(DATA, Size::new(12, 5)).unwrap();
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
///         raw::order::{BigEndian, LittleEndian},
///         Rgb565, Rgb888,
///     },
///     prelude::*,
/// };
/// # const DATA: &[u8] = &[0x55; 8 * 8 * 3];
///
/// // Rgb888 image with 24 bits per pixel and big endian byte order
/// let image = ImageRaw::<Rgb888, BigEndian>::new(DATA, Size::new(8, 8))?;
///
/// // Rgb565 image with 16 bits per pixel and little endian byte order
/// let image = ImageRaw::<Rgb565, LittleEndian>::new(DATA, Size::new(16, 6))?;
/// # Ok::<_, embedded_graphics::common::BufferError>(())
/// ```
///
/// ## Creating image constants
///
/// Because of limitation in Rust's const generics it is currently not possible to use the default
/// [`new`](ImageRaw::new) and [`with_stride`](ImageRaw::with_stride) constructors to create image
/// constants.  Two additional constructors exist as a workaround:
/// [`new_const`](ImageRaw::new_const) and [`with_stride_const`](ImageRaw::with_stride_const). They
/// only difference to the regular constructors is that these constructors will panic instead of
/// reporting an error using a
/// [`Result`].
///
/// ```
/// use embedded_graphics::{
///     image::ImageRaw,
///     pixelcolor::{raw::order::BigEndian, Rgb888},
///     prelude::*,
/// };
///
/// # const DATA: &[u8] = &[0x55; 8 * 8 * 3];
/// // The image data can be included in the binary with the include_bytes! macro.
/// // const DATA: &[u8] = include_bytes!("image_data.raw");
///
/// const IMAGE: ImageRaw<Rgb888, BigEndian> = ImageRaw::new_const(DATA, Size::new(8, 8));
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
pub struct ImageRaw<'a, C: StorablePixelColor, O, A = Horizontal> {
    data: PixelSlice<'a, C, O>,
    dimensions: BufferDimensions<C, A>,
}

impl<'a, C, O, A> ImageRaw<'a, C, O, A>
where
    C: StorablePixelColor,
    O: DataOrder<C::Raw>,
    A: PixelArrangement,
{
    /// Creates a new image.
    ///
    /// Returns an error if the data slice isn't large enough to store an image of the given size.
    ///
    /// This method cannot be used in a const context, use [`new_const`](Self::new_const) to create
    /// image constants.
    pub fn new(data: &'a [u8], size: Size) -> Result<Self, BufferError> {
        BufferDimensions::new(data, size).map(|dimensions| Self::with_dimensions(data, dimensions))
    }

    /// Creates a new image.
    ///
    /// Alternative to the regular [`new`](Self::new) constructor, which cannot currently be used in
    /// a const context.
    ///
    /// # Panics
    ///
    /// In contrast to the regular constructor this variant panics if the data size is insufficient
    /// for the given image dimensions.
    pub const fn new_const(data: &'a [u8], size: Size) -> Self {
        let dimensions = match BufferDimensions::new(data, size) {
            Ok(dimensions) => dimensions,
            Err(_) => panic!("insufficient image data"),
        };

        Self::with_dimensions(data, dimensions)
    }

    /// Creates a new image with custom stride.
    ///
    /// Note that the stride value is in pixels and not bytes. This allows colors with < 8 BPP to
    /// be packed tightly if the row length in bytes is not an integer value.
    ///
    /// Returns an error if the data slice isn't large enough or the stride is too small.
    pub fn with_stride(data: &'a [u8], size: Size, stride: usize) -> Result<Self, BufferError> {
        BufferDimensions::with_stride(data, size, stride)
            .map(|dimensions| Self::with_dimensions(data, dimensions))
    }

    /// Creates a new image with custom stride.
    ///
    /// Alternative to the regular [`with_stride`](Self::with_stride) constructor, which cannot
    /// currently be used in a const context.
    ///
    /// # Panics
    ///
    /// In contrast to the regular constructor this variant panics if the data size is insufficient
    /// for the given image dimensions.
    pub const fn with_stride_const(data: &'a [u8], size: Size, stride: usize) -> Self {
        let dimensions = match BufferDimensions::with_stride(data, size, stride) {
            Ok(dimensions) => dimensions,
            Err(_) => panic!("insufficient image data"),
        };

        Self::with_dimensions(data, dimensions)
    }

    pub(crate) const fn with_dimensions(
        data: &'a [u8],
        dimensions: BufferDimensions<C, A>,
    ) -> Self {
        Self {
            data: PixelSlice::new(data),
            dimensions,
        }
    }
}

impl<'a, C, O, A> ImageDrawable for ImageRaw<'a, C, O, A>
where
    C: StorablePixelColor,
    O: DataOrder<C::Raw>,
    A: PixelArrangement,
{
    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let colors = ContiguousPixels::new(
            self,
            0,
            A::ARRANGEMENT.size_to_main(self.dimensions.size()),
            A::ARRANGEMENT.size_to_cross(self.dimensions.size()),
            self.dimensions.stride(),
        );

        match A::ARRANGEMENT {
            PixelArrangementEnum::Horizontal => {
                target.fill_contiguous(&self.bounding_box(), colors)
            }
            PixelArrangementEnum::Vertical => target.draw_iter(
                self.bounding_box()
                    .points_vertical()
                    .zip(colors)
                    .map(|(p, c)| Pixel(p, c)),
            ),
        }
    }

    fn draw_sub_image<T>(&self, target: &mut T, area: &Rectangle) -> Result<(), T::Error>
    where
        T: DrawTarget<Color = Self::Color>,
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

        let (main_size, cross_size) = match A::ARRANGEMENT {
            PixelArrangementEnum::Horizontal => (area.size.width, area.size.height),
            PixelArrangementEnum::Vertical => (area.size.height, area.size.width),
        };

        let colors = ContiguousPixels::new(
            self,
            self.dimensions.index(area.top_left).unwrap_or_default(),
            main_size,
            cross_size,
            self.dimensions.stride(),
        );

        let area = Rectangle::new(Point::zero(), area.size);
        match A::ARRANGEMENT {
            PixelArrangementEnum::Horizontal => target.fill_contiguous(&area, colors),
            PixelArrangementEnum::Vertical => {
                target.draw_iter(area.points_vertical().zip(colors).map(|(p, c)| Pixel(p, c)))
            }
        }
    }
}

impl<C, O, A> ColorType for ImageRaw<'_, C, O, A>
where
    C: StorablePixelColor,
    O: DataOrder<C::Raw>,
    A: PixelArrangement,
{
    type Color = C;
}

impl<C, O, A> OriginDimensions for ImageRaw<'_, C, O, A>
where
    C: StorablePixelColor,
    O: DataOrder<C::Raw>,
    A: PixelArrangement,
{
    fn size(&self) -> Size {
        self.dimensions.size()
    }
}

impl<'a, C, O, A> GetPixel for ImageRaw<'a, C, O, A>
where
    C: StorablePixelColor,
    O: DataOrder<C::Raw>,
    A: PixelArrangement,
{
    fn pixel(&self, p: Point) -> Option<Self::Color> {
        self.dimensions
            .index(p)
            .ok()
            .and_then(|index| self.data.get(index))
    }
}

struct ContiguousPixels<'a, C, O, A>
where
    C: StorablePixelColor,
    O: DataOrder<C::Raw>,
    A: PixelArrangement,
{
    data: PixelSlice<'a, C, O>,
    index: usize,

    main_remaining: u32,
    main_size: u32,

    cross_remaining: u32,
    cross_skip: usize,

    pixel_arrangement: PhantomData<A>,
}

impl<'a, C, O, A> ContiguousPixels<'a, C, O, A>
where
    C: StorablePixelColor,
    O: DataOrder<C::Raw>,
    A: PixelArrangement,
{
    fn new(
        image: &ImageRaw<'a, C, O, A>,
        initial_skip: usize,
        main_size: u32,
        cross_size: u32,
        stride: usize,
    ) -> Self {
        // Set `cross_remaining` to `0` if `main_size == 0` to prevent integer underflow in `next`.
        let cross_remaining = if main_size > 0 { cross_size } else { 0 };
        let cross_skip = stride - main_size as usize;

        Self {
            data: image.data,
            index: initial_skip,
            main_remaining: main_size,
            main_size,
            cross_remaining,
            cross_skip,
            pixel_arrangement: PhantomData,
        }
    }
}

impl<'a, C, O, A> Iterator for ContiguousPixels<'a, C, O, A>
where
    C: StorablePixelColor,
    O: DataOrder<C::Raw>,
    A: PixelArrangement,
{
    type Item = C;

    fn next(&mut self) -> Option<Self::Item> {
        if self.main_remaining > 0 {
            self.main_remaining -= 1;
        } else {
            if self.cross_remaining == 0 {
                return None;
            }

            self.cross_remaining -= 1;
            self.main_remaining = self.main_size - 1;

            self.index += self.cross_skip;
        }

        let color = self.data.get(self.index)?;
        self.index += 1;

        Some(color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        common::{tests::U32Color, Horizontal, Vertical},
        draw_target::DrawTarget,
        geometry::Point,
        image::Image,
        iterator::PixelIteratorExt,
        mock_display::{ColorMapping, MockDisplay},
        pixelcolor::{
            raw::order::{BigEndian, LittleEndian, Lsb0, Msb0},
            Bgr888, BinaryColor, Gray2, Gray4, Gray8, Rgb565, Rgb888,
        },
        prelude::ImageDrawableExt,
        primitives::PointsIter,
        Drawable, Pixel,
    };

    fn draw_image<'a, C, O, A>(
        image_data: &'a [u8],
        size: Size,
    ) -> (ImageRaw<'a, C, O, A>, MockDisplay<C>)
    where
        C: StorablePixelColor + ColorMapping,
        O: DataOrder<C::Raw>,
        A: PixelArrangement,
    {
        let image_raw = ImageRaw::new(&image_data, size).unwrap();

        let mut display = MockDisplay::new();
        Image::new(&image_raw, Point::zero())
            .draw(&mut display)
            .unwrap();

        (image_raw, display)
    }

    // Draws the image using the `GetPixel` implementation.
    fn draw_image_get_pixel<'a, C, O, A>(image_raw: &ImageRaw<'a, C, O, A>) -> MockDisplay<C>
    where
        C: StorablePixelColor,
        ImageRaw<'a, C, O, A>: GetPixel + ColorType<Color = C>,
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
    fn draw_image_subimage<'a, C, O, A>(image_raw: &ImageRaw<'a, C, O, A>) -> MockDisplay<C>
    where
        C: StorablePixelColor,
        ImageRaw<'a, C, O, A>: ImageDrawable<Color = C>,
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
    fn assert_image<C, O>(image_data: &[u8], size: Size, expected_pattern: &[&str])
    where
        C: StorablePixelColor + ColorMapping,
        O: DataOrder<C::Raw>,
    {
        let h_size = size;
        let v_size = Size::new(size.height, size.width);

        // Horizontal (ImageDrawable)

        let (image_horizontal, display_horizontal) =
            draw_image::<C, O, Horizontal>(&image_data, h_size);
        assert_eq!(image_horizontal.size(), h_size);

        display_horizontal.assert_pattern(expected_pattern);

        // Horizontal (GetPixel)

        draw_image_get_pixel(&image_horizontal).assert_pattern(expected_pattern);

        // Horizontal SubImage

        let subimage_horizontal = draw_image_subimage(&image_horizontal);
        subimage_horizontal.assert_eq(&display_horizontal);

        // Vertical (ImageDrawable)

        let expected_vertical = MockDisplay::from_pattern(expected_pattern).swap_xy();

        let (image_vertical, display_vertical) = draw_image::<C, O, Vertical>(&image_data, v_size);
        assert_eq!(image_vertical.size(), v_size);

        display_vertical.assert_eq(&expected_vertical);

        // Vertical (GetPixel)

        draw_image_get_pixel(&image_vertical).assert_eq(&expected_vertical);

        // Vertical SubImage

        let subimage_horizontal = draw_image_subimage(&image_vertical);
        subimage_horizontal.assert_eq(&expected_vertical);
    }

    #[test]
    fn truncated_data_new() {
        let data = &[
            0xAA, 0x00, //
            0x55, 0xFF, //
            0xAA, 0x00, //
        ];
        assert!(ImageRaw::<BinaryColor, Msb0>::new(data, Size::new(9, 3)).is_ok());

        let (_, truncated_data) = data.split_last().unwrap();
        assert_eq!(
            ImageRaw::<BinaryColor, Msb0>::new(truncated_data, Size::new(9, 3)),
            Err(BufferError::TruncatedData {
                expected_buffer_size: data.len()
            })
        );
    }

    #[test]
    #[should_panic = "insufficient image data"]
    fn truncated_data_new_const() {
        let data = &[
            0xAA, 0x00, //
            0x55, 0xFF, //
            0xAA, 0x00, //
        ];
        let (_, truncated_data) = data.split_last().unwrap();

        ImageRaw::<BinaryColor, Msb0>::new_const(truncated_data, Size::new(9, 3));
    }

    #[test]
    fn truncated_data_with_stride() {
        let data = &[0; 4];
        assert!(
            ImageRaw::<BinaryColor, Msb0, Vertical>::with_stride(data, Size::new(3, 9), 10).is_ok()
        );

        let (_, truncated_data) = data.split_last().unwrap();
        assert_eq!(
            ImageRaw::<BinaryColor, Msb0, Vertical>::with_stride(
                truncated_data,
                Size::new(3, 9),
                10
            ),
            Err(BufferError::TruncatedData {
                expected_buffer_size: data.len()
            })
        );
    }

    #[test]
    #[should_panic = "insufficient image data"]
    fn truncated_data_with_stride_const() {
        let data = &[0; 4];

        let (_, truncated_data) = data.split_last().unwrap();
        ImageRaw::<BinaryColor, Msb0, Vertical>::with_stride_const(
            truncated_data,
            Size::new(3, 9),
            10,
        );
    }

    #[test]
    fn bpp1_msb0() {
        assert_image::<BinaryColor, Msb0>(
            &[
                0xAA, 0x00, //
                0x55, 0xFF, //
                0xAA, 0x80, //
            ],
            Size::new(9, 3),
            &[
                "#.#.#.#..", //
                ".#.#.#.##", //
                "#.#.#.#.#", //
            ],
        );
    }

    #[test]
    fn bpp1_lsb0() {
        assert_image::<BinaryColor, Lsb0>(
            &[
                0xAA, 0x00, //
                0x55, 0xFF, //
                0xAA, 0x80, //
            ],
            Size::new(9, 3),
            &[
                ".#.#.#.#.", //
                "#.#.#.#.#", //
                ".#.#.#.#.", //
            ],
        );
    }

    #[test]
    fn bpp2_msb0() {
        assert_image::<Gray2, Msb0>(
            &[
                0b00_01_10_11, //
                0b00_00_00_00, //
                0b11_10_01_00, //
                0b11_11_11_11, //
            ],
            Size::new(5, 2),
            &[
                "01230", //
                "32103", //
            ],
        );
    }

    #[test]
    fn bpp2_lsb0() {
        assert_image::<Gray2, Lsb0>(
            &[
                0b00_01_10_11, //
                0b00_00_00_00, //
                0b11_10_01_00, //
                0b11_11_11_11, //
            ],
            Size::new(5, 2),
            &[
                "32100", //
                "01233", //
            ],
        );
    }

    #[test]
    fn bpp4_msb0() {
        assert_image::<Gray4, Msb0>(
            &[
                0b0001_1000, //
                0b1111_0000, //
                0b0101_1010, //
                0b0000_0000, //
            ],
            Size::new(3, 2),
            &[
                "18F", //
                "5A0", //
            ],
        );
    }

    #[test]
    fn bpp4_lsb0() {
        assert_image::<Gray4, Lsb0>(
            &[
                0b0001_1000, //
                0b1111_0000, //
                0b0101_1010, //
                0b0000_0000, //
            ],
            Size::new(3, 2),
            &[
                "810", //
                "A50", //
            ],
        );
    }

    #[test]
    fn bpp8_1() {
        assert_image::<Gray8, LittleEndian>(
            &[
                0x11, 0x22, //
                0x33, 0x44, //
                0x55, 0x66, //
            ],
            Size::new(2, 3),
            &[
                "12", //
                "34", //
                "56", //
            ],
        );
    }

    #[test]
    fn bpp8_little_endian() {
        assert_image::<Gray8, LittleEndian>(
            &[
                0x11, 0x22, //
                0x33, 0x44, //
                0x55, 0x66, //
            ],
            Size::new(2, 3),
            &[
                "12", //
                "34", //
                "56", //
            ],
        );
    }

    #[test]
    fn bpp8_big_endian() {
        assert_image::<Gray8, BigEndian>(
            &[
                0x11, 0x22, //
                0x33, 0x44, //
                0x55, 0x66, //
            ],
            Size::new(2, 3),
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
        let image_data = ImageRaw::<Gray8, LittleEndian>::new(&data, Size::new(4, 1)).unwrap();

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
        assert_image::<Rgb565, LittleEndian>(
            &[
                0x00, 0xF8, //
                0xE0, 0x07, //
                0x1F, 0x00, //
                0x00, 0x00, //
            ],
            Size::new(1, 4),
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
        assert_image::<Rgb565, BigEndian>(
            &[
                0xF8, 0x00, //
                0x07, 0xE0, //
                0x00, 0x1F, //
                0x00, 0x00, //
            ],
            Size::new(2, 2),
            &[
                "RG", //
                "BK", //
            ],
        );
    }

    #[test]
    fn bpp24_little_endian() {
        assert_image::<Bgr888, LittleEndian>(
            &[
                0xFF, 0x00, 0x00, //
                0x00, 0xFF, 0x00, //
                0x00, 0x00, 0xFF, //
                0x00, 0x00, 0x00, //
            ],
            Size::new(1, 4),
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
        assert_image::<Rgb888, BigEndian>(
            &[
                0xFF, 0x00, 0x00, //
                0x00, 0xFF, 0x00, //
                0x00, 0x00, 0xFF, //
                0x00, 0x00, 0x00, //
            ],
            Size::new(4, 1),
            &["RGBK"],
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
        let image_data: ImageRaw<U32Color, LittleEndian> =
            ImageRaw::new(&data, Size::new(2, 2)).unwrap();

        let mut display = MockDisplay::new();
        Image::new(&image_data, Point::zero())
            .draw(&mut display)
            .unwrap();

        let expected = [
            Pixel(Point::new(0, 0), U32Color(0x78563412)),
            Pixel(Point::new(1, 0), U32Color(0xF0DEBC9A)),
            Pixel(Point::new(0, 1), U32Color(0x00000000)),
            Pixel(Point::new(1, 1), U32Color(0xFFFFFFFF)),
        ];

        let mut expected_display = MockDisplay::new();
        expected
            .iter()
            .copied()
            .draw(&mut expected_display)
            .unwrap();

        // assert_eq can't be used here because ColorMapping isn't implemented for U32Color
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
        let image_data: ImageRaw<U32Color, BigEndian> =
            ImageRaw::new(&data, Size::new(4, 1)).unwrap();

        let mut display = MockDisplay::new();
        Image::new(&image_data, Point::zero())
            .draw(&mut display)
            .unwrap();

        let expected = [
            Pixel(Point::new(0, 0), U32Color(0x12345678)),
            Pixel(Point::new(1, 0), U32Color(0x9ABCDEF0)),
            Pixel(Point::new(2, 0), U32Color(0x00000000)),
            Pixel(Point::new(3, 0), U32Color(0xFFFFFFFF)),
        ];

        let mut expected_display = MockDisplay::new();
        expected
            .iter()
            .copied()
            .draw(&mut expected_display)
            .unwrap();

        // assert_eq can't be used here because ColorMapping isn't implemented for U32Color
        assert!(display.eq(&expected_display));
    }

    #[test]
    fn zero_sized_image() {
        let image = ImageRaw::<BinaryColor, Msb0>::new(&[], Size::zero()).unwrap();
        assert_eq!(image.size(), Size::zero());

        let mut display = MockDisplay::<BinaryColor>::new();
        Image::new(&image, Point::zero())
            .draw(&mut display)
            .unwrap();
        display.assert_eq(&MockDisplay::new());

        assert_eq!(image.pixel(Point::zero()), None);
        assert_eq!(image.pixel(Point::new(1, 0)), None);
        assert_eq!(image.pixel(Point::new(0, 1)), None);
    }

    #[test]
    fn const_zero_sized_image() {
        const IMAGE: ImageRaw<BinaryColor, Msb0> = ImageRaw::new_const(&[], Size::zero());
        assert_eq!(IMAGE.size(), Size::zero());

        let mut display = MockDisplay::<BinaryColor>::new();
        Image::new(&IMAGE, Point::zero())
            .draw(&mut display)
            .unwrap();
        display.assert_eq(&MockDisplay::new());

        assert_eq!(IMAGE.pixel(Point::zero()), None);
        assert_eq!(IMAGE.pixel(Point::new(1, 0)), None);
        assert_eq!(IMAGE.pixel(Point::new(0, 1)), None);
    }

    #[test]
    fn const_larger_data() {
        let data = [0u8; 10];
        let image = ImageRaw::<Gray8, LittleEndian>::new_const(&data, Size::new(1, 5));
        assert_eq!(image.size(), Size::new(1, 5));
    }

    #[test]
    fn pixel_out_of_bounds() {
        let data = [
            0xAA, 0x00, //
            0x55, 0xFF, //
            0xAA, 0x80, //
        ];
        let image_data = ImageRaw::<BinaryColor, Msb0>::new(&data, Size::new(9, 3)).unwrap();

        assert_eq!(image_data.pixel(Point::new(-1, 0)), None);
        assert_eq!(image_data.pixel(Point::new(0, -1)), None);
        assert_eq!(image_data.pixel(Point::new(9, 0)), None);
        assert_eq!(image_data.pixel(Point::new(9, 3)), None);
    }

    #[test]
    fn with_stride() {
        let data = [0x12, 0x34, 0x56, 0x78];

        let image = ImageRaw::<Gray4, Msb0>::new(&data, Size::new(3, 2)).unwrap();
        let mut display = MockDisplay::new();
        Image::new(&image, Point::zero())
            .draw(&mut display)
            .unwrap();
        display.assert_pattern(&[
            "123", //
            "567", //
        ]);

        let image = ImageRaw::<Gray4, Msb0>::with_stride(&data, Size::new(3, 2), 3).unwrap();
        let mut display = MockDisplay::new();
        Image::new(&image, Point::zero())
            .draw(&mut display)
            .unwrap();
        display.assert_pattern(&[
            "123", //
            "456", //
        ]);

        let image =
            ImageRaw::<Gray4, Msb0, Vertical>::with_stride(&data, Size::new(2, 3), 3).unwrap();
        let mut display = MockDisplay::new();
        Image::new(&image, Point::zero())
            .draw(&mut display)
            .unwrap();
        display.assert_pattern(&[
            "14", //
            "25", //
            "36", //
        ]);
    }

    #[test]
    fn with_stride_const() {
        const DATA: &[u8] = &[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB];

        const IMAGE1: ImageRaw<Gray4, Msb0> = ImageRaw::with_stride_const(DATA, Size::new(3, 3), 3);
        let mut display = MockDisplay::new();
        Image::new(&IMAGE1, Point::zero())
            .draw(&mut display)
            .unwrap();
        display.assert_pattern(&[
            "123", //
            "456", //
            "789", //
        ]);

        const IMAGE2: ImageRaw<Gray4, Msb0, Vertical> =
            ImageRaw::with_stride_const(DATA, Size::new(3, 3), 3);
        let mut display = MockDisplay::new();
        Image::new(&IMAGE2, Point::zero())
            .draw(&mut display)
            .unwrap();
        display.assert_pattern(&[
            "147", //
            "258", //
            "369", //
        ]);
    }

    #[test]
    fn stride_too_large() {
        let data = [0; 3 * 3];

        let stride = 3 * 8 + 1;
        assert_eq!(
            ImageRaw::<BinaryColor, Msb0>::with_stride(&data, Size::new(3 * 8, 3), stride),
            Err(BufferError::TruncatedData {
                expected_buffer_size: (stride * 3 + 7) / 8
            })
        );

        let stride = 3 + 2;
        assert_eq!(
            ImageRaw::<Gray8, LittleEndian>::with_stride(&data, Size::new(3, 3), stride),
            Err(BufferError::TruncatedData {
                expected_buffer_size: stride * 3
            })
        );
    }

    #[test]
    fn stride_too_small() {
        let data = [0; 3 * 3];

        assert_eq!(
            ImageRaw::<Gray4, Msb0>::with_stride(&data, Size::new(3, 3), 2),
            Err(BufferError::InvalidStride { minimum_stride: 3 })
        );
    }

    #[test]
    fn stride_too_small_const() {
        let data = [0x12, 0x34, 0x56];

        let image = ImageRaw::<Gray4, Msb0>::with_stride_const(&data, Size::new(3, 2), 3);
        assert_eq!(image.size(), Size::new(3, 2));

        let mut display = MockDisplay::new();
        Image::new(&image, Point::zero())
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "123", //
            "456", //
        ]);
    }
}
