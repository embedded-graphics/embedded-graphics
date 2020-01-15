use crate::{
    drawable::{Drawable, Pixel},
    geometry::{Dimensions, Point, Size},
    image::ImageFile,
    pixelcolor::{raw::RawData, PixelColor},
    transform::Transform,
    DrawTarget,
};
use core::marker::PhantomData;
use tinytga::{Tga, TgaIterator};

/// TGA format image
///
/// `ImageTga` is available with the `tga` feature turned on
///
/// # Examples
///
/// ## Load a 16 bit per pixel image from a raw byte slice and draw it to a display
///
/// ```rust
/// use embedded_graphics::{image::ImageTga, prelude::*};
/// # use embedded_graphics::mock_display::MockDisplay;
/// # use embedded_graphics::pixelcolor::Rgb888;
/// # let mut display: MockDisplay<Rgb888> = MockDisplay::default();
///
/// // Load `patch.tga`, a 32BPP 4x4px image
/// let image = ImageTga::new(include_bytes!("../../../assets/patch.tga")).unwrap();
///
/// image.draw(&mut display)?;
/// # Ok::<(), core::convert::Infallible>(())
/// ```
#[derive(Debug, Clone)]
pub struct ImageTga<'a, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    tga: Tga<'a>,

    /// Top left corner offset from display origin (0,0)
    pub offset: Point,

    pixel_type: PhantomData<C>,
}

impl<'a, C> ImageFile<'a> for ImageTga<'a, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    /// Create a new TGA from a byte slice
    fn new(image_data: &'a [u8]) -> Result<Self, ()> {
        let im = Self {
            tga: Tga::from_slice(image_data).map_err(|_| ())?,
            offset: Point::zero(),
            pixel_type: PhantomData,
        };

        Ok(im)
    }

    fn width(&self) -> u32 {
        u32::from(self.tga.width())
    }

    fn height(&self) -> u32 {
        u32::from(self.tga.height())
    }
}

impl<'a, C> Dimensions for ImageTga<'a, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    fn top_left(&self) -> Point {
        self.offset
    }

    fn bottom_right(&self) -> Point {
        self.top_left() + self.size()
    }

    fn size(&self) -> Size {
        Size::new(u32::from(self.tga.width()), u32::from(self.tga.height()))
    }
}

impl<'a, 'b, C> IntoIterator for &'b ImageTga<'a, C>
where
    'b: 'a,
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    type Item = Pixel<C>;
    type IntoIter = ImageTgaIterator<'a, 'b, C>;

    // NOTE: `self` is a reference already, no copies here!
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            im: self,
            image_data: self.tga.into_iter(),
            x: 0,
            y: 0,
        }
    }
}

#[derive(Debug)]
pub struct ImageTgaIterator<'a, 'b, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    x: u32,
    y: u32,
    im: &'b ImageTga<'a, C>,
    image_data: TgaIterator<'a>,
}

impl<'a, 'b, C> Iterator for ImageTgaIterator<'a, 'b, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        self.image_data.next().map(|color| {
            let pos = self.im.offset + Point::new(self.x as i32, self.y as i32);

            let raw = C::Raw::from_u32(color);
            let out = Pixel(pos, raw.into());

            self.x += 1;

            if self.x >= self.im.width() {
                self.y += 1;
                self.x = 0;
            }

            out
        })
    }
}

impl<'a, C: 'a> Drawable<C> for &'a ImageTga<'a, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        display.draw_iter(self.into_iter())
    }
}

impl<'a, C> Transform for ImageTga<'a, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    /// Translate the image from its current position to a new position by (x, y) pixels, returning
    /// a new `ImageTga`. For a mutating transform, see `translate_mut`.
    fn translate(&self, by: Point) -> Self {
        Self {
            offset: self.offset + by,
            ..self.clone()
        }
    }

    /// Translate the image from its current position to a new position by (x, y) pixels.
    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.offset += by;

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        mock_display::MockDisplay,
        pixelcolor::{Gray8, Rgb888, RgbColor},
    };

    const PIXEL_COLORS: [(i32, i32, Rgb888); 16] = [
        (0, 0, Rgb888::WHITE),
        (1, 0, Rgb888::BLACK),
        (2, 0, Rgb888::WHITE),
        (3, 0, Rgb888::BLACK),
        (0, 1, Rgb888::BLACK),
        (1, 1, Rgb888::RED),
        (2, 1, Rgb888::BLACK),
        (3, 1, Rgb888::GREEN),
        (0, 2, Rgb888::WHITE),
        (1, 2, Rgb888::BLACK),
        (2, 2, Rgb888::BLUE),
        (3, 2, Rgb888::BLACK),
        (0, 3, Rgb888::BLACK),
        (1, 3, Rgb888::WHITE),
        (2, 3, Rgb888::BLACK),
        (3, 3, Rgb888::WHITE),
    ];

    #[test]
    fn chessboard_compressed() -> Result<(), ()> {
        let im: ImageTga<Rgb888> = ImageTga::new(include_bytes!("../../tests/chessboard_rle.tga"))?;

        let mut pixels = im.into_iter();

        for (i, (x, y, color)) in PIXEL_COLORS.iter().enumerate() {
            assert_eq!(
                pixels.next(),
                Some(Pixel(Point::new(*x, *y), *color)),
                "Pixel color at index {} does not match",
                i
            );
        }

        // 17th iteration should have no pixels from 4x4px image
        assert_eq!(pixels.next(), None);

        Ok(())
    }

    #[test]
    fn chessboard_uncompressed() -> Result<(), ()> {
        let im: ImageTga<Rgb888> = ImageTga::new(include_bytes!("../../tests/chessboard_raw.tga"))?;

        let mut pixels = im.into_iter();

        for (i, (x, y, color)) in PIXEL_COLORS.iter().enumerate() {
            assert_eq!(
                pixels.next(),
                Some(Pixel(Point::new(*x, *y), *color)),
                "Pixel color at index {} does not match",
                i
            );
        }

        // 17th iteration should have no pixels from 4x4px image
        assert_eq!(pixels.next(), None);

        Ok(())
    }

    fn test_color_tga(data: &[u8]) -> Result<(), ()> {
        let image: ImageTga<Rgb888> = ImageTga::new(data)?;

        let mut display = MockDisplay::new();
        image.draw(&mut display).map_err(|_| ())?;

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "WKRGBYMCW",
                "KKRGBYMCW",
                "WKRGBYMCW",
                "KKKKKKKKK",
                "WKWCMYBGR",
            ])
        );

        Ok(())
    }

    fn test_gray_tga(data: &[u8]) -> Result<(), ()> {
        let image: ImageTga<Gray8> = ImageTga::new(data)?;

        let mut display = MockDisplay::new();
        image.draw(&mut display).map_err(|_| ())?;

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "0F0F0F0F0",
                "00FF00FF0",
                "0000FFFF0",
                "012345670",
                "89ABCDEF0",
            ])
        );

        Ok(())
    }

    /// Tests color mapped, uncompressed, bottom left origin TGA file.
    #[test]
    #[ignore]
    fn type1_bl() -> Result<(), ()> {
        test_color_tga(include_bytes!("../../../tinytga/tests/type1_bl.tga"))
    }

    /// Tests color mapped, uncompressed, top left origin TGA file.
    #[test]
    fn type1_tl() -> Result<(), ()> {
        test_color_tga(include_bytes!("../../../tinytga/tests/type1_tl.tga"))
    }

    /// Tests true color, uncompressed, bottom left origin TGA file.
    #[test]
    #[ignore]
    fn type2_bl() -> Result<(), ()> {
        test_color_tga(include_bytes!("../../../tinytga/tests/type2_bl.tga"))
    }

    /// Tests true color, uncompressed, top left origin TGA file.
    #[test]
    fn type2_tl() -> Result<(), ()> {
        test_color_tga(include_bytes!("../../../tinytga/tests/type2_tl.tga"))
    }

    /// Tests grayscale, uncompressed, bottom left origin TGA file.
    #[test]
    #[ignore]
    fn type3_bl() -> Result<(), ()> {
        test_gray_tga(include_bytes!("../../../tinytga/tests/type3_bl.tga"))
    }

    /// Tests grayscale, uncompressed, top left origin TGA file.
    #[test]
    fn type3_tl() -> Result<(), ()> {
        test_gray_tga(include_bytes!("../../../tinytga/tests/type3_tl.tga"))
    }

    /// Tests color mapped, RLE compressed, bottom left origin TGA file.
    #[test]
    #[ignore]
    fn type9_bl() -> Result<(), ()> {
        test_color_tga(include_bytes!("../../../tinytga/tests/type9_bl.tga"))
    }

    /// Tests color mapped, RLE compressed, top left origin TGA file.
    #[test]
    fn type9_tl() -> Result<(), ()> {
        test_color_tga(include_bytes!("../../../tinytga/tests/type9_tl.tga"))
    }

    /// Tests true color, RLE compressed, bottom left origin TGA file.
    #[test]
    #[ignore]
    fn type10_bl() -> Result<(), ()> {
        test_color_tga(include_bytes!("../../../tinytga/tests/type10_bl.tga"))
    }

    /// Tests true color, RLE compressed, top left origin TGA file.
    #[test]
    fn type10_tl() -> Result<(), ()> {
        test_color_tga(include_bytes!("../../../tinytga/tests/type10_tl.tga"))
    }

    /// Tests grayscale, RLE compressed, bottom left origin TGA file.
    #[test]
    #[ignore]
    fn type11_bl() -> Result<(), ()> {
        test_gray_tga(include_bytes!("../../../tinytga/tests/type11_bl.tga"))
    }

    /// Tests grayscale, RLE compressed, top left origin TGA file.
    #[test]
    fn type11_tl() -> Result<(), ()> {
        test_gray_tga(include_bytes!("../../../tinytga/tests/type11_tl.tga"))
    }
}
