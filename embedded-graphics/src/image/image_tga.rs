use super::super::drawable::*;
use super::super::transform::*;
use super::ImageFile;
use crate::coord::{Coord, ToUnsigned};
use crate::pixelcolor::PixelColor;
use crate::unsignedcoord::{ToSigned, UnsignedCoord};
use core::marker::PhantomData;
use tinytga::{Tga, TgaIterator};

/// TGA format image
///
/// `ImageTga` is available with the `tga` feature turned on
#[derive(Debug, Clone)]
pub struct ImageTga<'a, C: PixelColor> {
    tga: Tga<'a>,

    /// Top left corner offset from display origin (0,0)
    pub offset: Coord,

    pixel_type: PhantomData<C>,
}

impl<'a, C> ImageFile<'a> for ImageTga<'a, C>
where
    C: PixelColor,
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
    C: PixelColor,
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
    C: PixelColor + From<u8> + From<u16> + From<u32>,
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
    C: PixelColor,
{
    x: u32,
    y: u32,
    im: &'a ImageTga<'a, C>,
    image_data: TgaIterator<'a>,
}

impl<'a, C> Iterator for ImageTgaIterator<'a, C>
where
    C: PixelColor + From<u8> + From<u16> + From<u32>,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        self.image_data.next().map(|color| {
            let x = self.x;
            let y = self.y;

            let pos = self.im.offset + Coord::new(x as i32, y as i32);

            let out = Pixel(pos.to_unsigned(), color.into());

            self.x += 1;

            if self.x >= self.im.width() {
                self.y += 1;
                self.x = 0;
            }

            out
        })
    }
}

impl<'a, C> Drawable for ImageTga<'a, C> where C: PixelColor {}

impl<'a, C> Transform for ImageTga<'a, C>
where
    C: PixelColor,
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
    use crate::unsignedcoord::UnsignedCoord;

    const PIXEL_COLORS: [(u32, u32, u32); 16] = [
        (0, 0, 0x00ffffffu32),
        (1, 0, 0x00000000),
        (2, 0, 0x00ffffff),
        (3, 0, 0x00000000),
        (0, 1, 0x00000000),
        (1, 1, 0x00ff0000),
        (2, 1, 0x00000000),
        (3, 1, 0x0000ff00),
        (0, 2, 0x00ffffff),
        (1, 2, 0x00000000),
        (2, 2, 0x000000ff),
        (3, 2, 0x00000000),
        (0, 3, 0x00000000),
        (1, 3, 0x00ffffff),
        (2, 3, 0x00000000),
        (3, 3, 0x00ffffff),
    ];

    #[test]
    fn chessboard_compressed() -> Result<(), ()> {
        let im: ImageTga<u32> = ImageTga::new(include_bytes!("../../tests/chessboard_rle.tga"))?;

        let mut pixels = im.into_iter();

        for (i, (x, y, color)) in PIXEL_COLORS.iter().enumerate() {
            assert_eq!(
                pixels.next(),
                Some(Pixel(UnsignedCoord::new(*x, *y), u32::from(*color))),
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
        let im: ImageTga<u32> = ImageTga::new(include_bytes!("../../tests/chessboard_raw.tga"))?;

        let mut pixels = im.into_iter();

        for (i, (x, y, color)) in PIXEL_COLORS.iter().enumerate() {
            assert_eq!(
                pixels.next(),
                Some(Pixel(UnsignedCoord::new(*x, *y), u32::from(*color))),
                "Pixel color at index {} does not match",
                i
            );
        }

        // 17th iteration should have no pixels from 4x4px image
        assert_eq!(pixels.next(), None);

        Ok(())
    }
}
