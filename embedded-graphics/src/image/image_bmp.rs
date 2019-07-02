use super::super::drawable::*;
use super::super::transform::*;
use super::ImageFile;
use crate::coord::{Coord, ToUnsigned};
use crate::pixelcolor::{FromRawData, PixelColor};
use crate::unsignedcoord::{ToSigned, UnsignedCoord};
use byteorder::{ByteOrder, LittleEndian};
use core::marker::PhantomData;
use tinybmp::Bmp;

/// BMP format image
///
/// `ImageBmp` is available with the `bmp` feature turned on
///
/// # Examples
///
/// ## Load a 16 bit per pixel image from a raw byte slice and draw it to a display
///
/// Note that images must be passed to `Display#draw` by reference, or by explicitly calling
/// `.into_iter()` on them, unlike other embedded_graphics objects.
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::image::ImageBmp;
/// # use embedded_graphics::mock_display::MockDisplay;
/// # use embedded_graphics::pixelcolor::Rgb565;
/// # let mut display: MockDisplay<Rgb565> = MockDisplay::default();
///
/// // Load `patch_16bpp.bmp`, a 16BPP 4x4px image
/// let image = ImageBmp::new(include_bytes!("../../../assets/patch_16bpp.bmp")).unwrap();
///
/// // Equivalent behaviour
/// display.draw(&image);
/// display.draw(image.into_iter());
/// ```
#[derive(Debug, Clone)]
pub struct ImageBmp<'a, C: PixelColor> {
    bmp: Bmp<'a>,

    /// Top left corner offset from display origin (0,0)
    pub offset: Coord,

    pixel_type: PhantomData<C>,
}

impl<'a, C> ImageFile<'a> for ImageBmp<'a, C>
where
    C: PixelColor + FromRawData,
{
    /// Create a new BMP from a byte slice
    fn new(image_data: &'a [u8]) -> Result<Self, ()> {
        let im = Self {
            bmp: Bmp::from_slice(image_data)?,
            offset: Coord::new(0, 0),
            pixel_type: PhantomData,
        };

        Ok(im)
    }

    fn width(&self) -> u32 {
        self.bmp.width()
    }

    fn height(&self) -> u32 {
        self.bmp.height()
    }
}

impl<'a, C> Dimensions for ImageBmp<'a, C>
where
    C: PixelColor + FromRawData,
{
    fn top_left(&self) -> Coord {
        self.offset
    }

    fn bottom_right(&self) -> Coord {
        self.top_left() + self.size().to_signed()
    }

    fn size(&self) -> UnsignedCoord {
        UnsignedCoord::new(self.bmp.width(), self.bmp.height())
    }
}

impl<'a, C> IntoIterator for &'a ImageBmp<'a, C>
where
    C: PixelColor + FromRawData,
{
    type Item = Pixel<C>;
    type IntoIter = ImageBmpIterator<'a, C>;

    // NOTE: `self` is a reference already, no copies here!
    fn into_iter(self) -> Self::IntoIter {
        ImageBmpIterator {
            im: self,
            image_data: self.bmp.image_data(),
            x: 0,
            y: 0,
        }
    }
}

#[derive(Debug)]
pub struct ImageBmpIterator<'a, C: 'a>
where
    C: PixelColor + FromRawData,
{
    x: u32,
    y: u32,
    im: &'a ImageBmp<'a, C>,
    image_data: &'a [u8],
}

impl<'a, C> Iterator for ImageBmpIterator<'a, C>
where
    C: PixelColor + FromRawData,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        let current_pixel = loop {
            let w = self.im.bmp.width();
            let h = self.im.bmp.height();
            let x = self.x;
            let y = self.y;

            // End iterator if we've run out of stuff
            if x >= w || y >= h {
                return None;
            }

            let offset = (((h - 1 - y) * w) + x) as usize;

            let data = match self.im.bmp.bpp() {
                8 => self.image_data[offset] as u32,
                16 => LittleEndian::read_u16(&self.image_data[offset * 2..]) as u32, // * 2 as two bytes per pixel
                24 => LittleEndian::read_u24(&self.image_data[offset * 3..]), // * 3 as three bytes per pixel
                32 => LittleEndian::read_u32(&self.image_data[offset * 4..]), // * 4 as four bytes per pixel
                _ => panic!("Bit depth {} not supported", self.im.bmp.bpp()),
            };

            let current_pixel = self.im.offset + Coord::new(x as i32, y as i32);

            // Increment stuff
            self.x += 1;

            // Step down a row if we've hit the end of this one
            if self.x >= w {
                self.x = 0;
                self.y += 1;
            }

            if current_pixel[0] >= 0 && current_pixel[1] >= 0 {
                break Pixel(current_pixel.to_unsigned(), C::from_raw_data(data));
            }
        };

        Some(current_pixel)
    }
}

impl<'a, C> Drawable for ImageBmp<'a, C> where C: PixelColor {}

impl<'a, C> Transform for ImageBmp<'a, C>
where
    C: PixelColor,
{
    /// Translate the image from its current position to a new position by (x, y) pixels, returning
    /// a new `ImageBmp`. For a mutating transform, see `translate_mut`.
    fn translate(&self, by: Coord) -> Self {
        Self {
            offset: self.offset + by,
            ..self.clone()
        }
    }

    /// Translate the image from its current position to a new position by (x, y) pixels.
    fn translate_mut(&mut self, by: Coord) -> &mut Self {
        self.offset += by;

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelcolor::{Rgb555, Rgb565, Rgb888, RgbColor, Gray8};
    use crate::unsignedcoord::UnsignedCoord;

    #[test]
    fn negative_top_left() {
        let image: ImageBmp<Rgb565> = ImageBmp::new(include_bytes!(
            "../../tests/chessboard-4px-colour-16bit.bmp"
        ))
        .unwrap()
        .translate(Coord::new(-1, -1));

        assert_eq!(image.top_left(), Coord::new(-1, -1));
        assert_eq!(image.bottom_right(), Coord::new(3, 3));
        assert_eq!(image.size(), UnsignedCoord::new(4, 4));
    }

    #[test]
    fn dimensions() {
        let image: ImageBmp<Rgb565> = ImageBmp::new(include_bytes!(
            "../../tests/chessboard-4px-colour-16bit.bmp"
        ))
        .unwrap()
        .translate(Coord::new(100, 200));

        assert_eq!(image.top_left(), Coord::new(100, 200));
        assert_eq!(image.bottom_right(), Coord::new(104, 204));
        assert_eq!(image.size(), UnsignedCoord::new(4, 4));
    }

    #[test]
    fn it_can_have_negative_offsets() {
        let image: ImageBmp<Rgb565> = ImageBmp::new(include_bytes!(
            "../../tests/chessboard-4px-colour-16bit.bmp"
        ))
        .unwrap()
        .translate(Coord::new(-1, -1));
        let it = image.into_iter();

        let expected: [Pixel<Rgb565>; 9] = [
            Pixel(UnsignedCoord::new(0, 0), Rgb565::RED),
            Pixel(UnsignedCoord::new(1, 0), Rgb565::BLACK),
            Pixel(UnsignedCoord::new(2, 0), Rgb565::GREEN),
            //
            Pixel(UnsignedCoord::new(0, 1), Rgb565::BLACK),
            Pixel(UnsignedCoord::new(1, 1), Rgb565::BLUE),
            Pixel(UnsignedCoord::new(2, 1), Rgb565::BLACK),
            //
            Pixel(UnsignedCoord::new(0, 2), Rgb565::WHITE),
            Pixel(UnsignedCoord::new(1, 2), Rgb565::BLACK),
            Pixel(UnsignedCoord::new(2, 2), Rgb565::WHITE),
        ];

        assert_eq!(image.into_iter().count(), 9);

        for (idx, pixel) in it.enumerate() {
            assert_eq!(pixel, expected[idx]);
        }
    }

    fn create_color_pattern<C>() -> [[C; 4]; 2]
    where
        C: RgbColor,
    {
        [
            [C::BLACK, C::RED, C::GREEN, C::YELLOW],
            [C::BLUE, C::MAGENTA, C::CYAN, C::WHITE],
        ]
    }

    fn test_pattern<C>(image_data: &[u8])
    where
        C: PixelColor + RgbColor + FromRawData,
    {
        let image: ImageBmp<C> = ImageBmp::new(image_data).unwrap();

        let pattern = create_color_pattern();

        assert_eq!(image.size(), UnsignedCoord::new(4, 2));

        let mut iter = image.into_iter();
        for (y, row) in pattern.iter().enumerate() {
            for (x, exepcted_color) in row.iter().enumerate() {
                let pos = UnsignedCoord::new(x as u32, y as u32);
                let pixel = iter.next().unwrap();

                assert_eq!(pixel.0, pos);
                assert_eq!(pixel.1, *exepcted_color);
            }
        }

        assert!(iter.next().is_none());
    }

    #[test]
    fn colors_rgb555() {
        test_pattern::<Rgb555>(include_bytes!("../../tests/colors_rgb555.bmp"));
    }

    #[test]
    fn colors_rgb565() {
        test_pattern::<Rgb565>(include_bytes!("../../tests/colors_rgb565.bmp"));
    }

    #[test]
    fn colors_rgb888_24bit() {
        test_pattern::<Rgb888>(include_bytes!("../../tests/colors_rgb888_24bit.bmp"));
    }

    #[test]
    fn colors_rgb888_32bit() {
        test_pattern::<Rgb888>(include_bytes!("../../tests/colors_rgb888_32bit.bmp"));
    }

    #[test]
    fn colors_grey8() {
        let image: ImageBmp<Gray8> =
            ImageBmp::new(include_bytes!("../../tests/colors_grey8.bmp")).unwrap();

        assert_eq!(image.size(), UnsignedCoord::new(3, 1));

        let mut iter = image.into_iter();

        let p = iter.next().unwrap();
        assert_eq!(p.0, UnsignedCoord::new(0, 0));
        assert_eq!(p.1, Gray8::BLACK);

        let p = iter.next().unwrap();
        assert_eq!(p.0, UnsignedCoord::new(1, 0));
        assert_eq!(p.1, Gray8::new(128));

        let p = iter.next().unwrap();
        assert_eq!(p.0, UnsignedCoord::new(2, 0));
        assert_eq!(p.1, Gray8::WHITE);

        assert!(iter.next().is_none());
    }
}
