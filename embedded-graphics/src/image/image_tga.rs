use super::super::drawable::*;
use super::super::transform::*;
use super::ImageFile;
use crate::coord::{Coord, ToUnsigned};
use crate::pixelcolor::raw::RawData;
use crate::pixelcolor::PixelColor;
use crate::unsignedcoord::{ToSigned, UnsignedCoord};
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
/// Note that images must be passed to `Display#draw` by reference, or by explicitly calling
/// `.into_iter()` on them, unlike other embedded_graphics objects.
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::image::ImageTga;
/// # use embedded_graphics::mock_display::MockDisplay;
/// # use embedded_graphics::pixelcolor::Rgb888;
/// # let mut display: MockDisplay<Rgb888> = MockDisplay::default();
///
/// // Load `patch.tga`, a 32BPP 4x4px image
/// let image = ImageTga::new(include_bytes!("../../../assets/patch.tga")).unwrap();
///
/// // Equivalent behavior
/// display.draw(&image);
/// display.draw(image.into_iter());
/// ```
#[derive(Debug, Clone)]
pub struct ImageTga<'a, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    tga: Tga<'a>,

    /// Top left corner offset from display origin (0,0)
    pub offset: Coord,

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
            offset: Coord::new(0, 0),
            pixel_type: PhantomData,
        };

        Ok(im)
    }

    fn width(&self) -> u32 {
        self.tga.width() as u32
    }

    fn height(&self) -> u32 {
        self.tga.height() as u32
    }
}

impl<'a, C> Dimensions for ImageTga<'a, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    fn top_left(&self) -> Coord {
        self.offset
    }

    fn bottom_right(&self) -> Coord {
        self.top_left() + self.size().to_signed()
    }

    fn size(&self) -> UnsignedCoord {
        UnsignedCoord::new(self.tga.width() as u32, self.tga.height() as u32)
    }
}

impl<'a, C> IntoIterator for &'a ImageTga<'a, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    type Item = Pixel<C>;
    type IntoIter = ImageTgaIterator<'a, C>;

    // NOTE: `self` is a reference already, no copies here!
    fn into_iter(self) -> Self::IntoIter {
        ImageTgaIterator {
            im: self,
            image_data: self.tga.into_iter(),
            x: 0,
            y: 0,
        }
    }
}

#[derive(Debug)]
pub struct ImageTgaIterator<'a, C: 'a>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    x: u32,
    y: u32,
    im: &'a ImageTga<'a, C>,
    image_data: TgaIterator<'a>,
}

impl<'a, C> Iterator for ImageTgaIterator<'a, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        self.image_data.next().map(|color| {
            let pos = self.im.offset + Coord::new(self.x as i32, self.y as i32);

            let raw = C::Raw::from_u32(color);
            let out = Pixel(pos.to_unsigned(), raw.into());

            self.x += 1;

            if self.x >= self.im.width() {
                self.y += 1;
                self.x = 0;
            }

            out
        })
    }
}

impl<'a, C> Drawable for ImageTga<'a, C> where C: PixelColor + From<<C as PixelColor>::Raw> {}

impl<'a, C> Transform for ImageTga<'a, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    /// Translate the image from its current position to a new position by (x, y) pixels, returning
    /// a new `ImageTga`. For a mutating transform, see `translate_mut`.
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
    use crate::pixelcolor::{Rgb888, RgbColor};
    use crate::unsignedcoord::UnsignedCoord;

    const PIXEL_COLORS: [(u32, u32, Rgb888); 16] = [
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
                Some(Pixel(UnsignedCoord::new(*x, *y), *color)),
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
                Some(Pixel(UnsignedCoord::new(*x, *y), *color)),
                "Pixel color at index {} does not match",
                i
            );
        }

        // 17th iteration should have no pixels from 4x4px image
        assert_eq!(pixels.next(), None);

        Ok(())
    }
}
