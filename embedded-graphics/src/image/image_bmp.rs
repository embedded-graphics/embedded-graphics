use super::super::drawable::*;
use super::super::transform::*;
use super::ImageFile;
use crate::coord::{Coord, ToUnsigned};
use crate::pixelcolor::PixelColor;
use crate::unsignedcoord::{ToSigned, UnsignedCoord};
use core::marker::PhantomData;
use tinybmp::Bmp;

/// BMP format image
#[derive(Debug, Clone)]
pub struct ImageBmp<'a, C: PixelColor> {
    bmp: Bmp<'a>,

    /// Top left corner offset from display origin (0,0)
    pub offset: Coord,

    pixel_type: PhantomData<C>,
}

impl<'a, C> ImageFile<'a> for ImageBmp<'a, C>
where
    C: PixelColor,
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
    C: PixelColor,
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
    C: PixelColor + From<u8> + From<u16>,
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
    C: PixelColor,
{
    x: u32,
    y: u32,
    im: &'a ImageBmp<'a, C>,
    image_data: &'a [u8],
}

impl<'a, C> Iterator for ImageBmpIterator<'a, C>
where
    C: PixelColor + From<u8> + From<u16>,
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

            let offset = ((h - 1 - y) * w) + x;

            let bit_value = if self.im.bmp.bpp() == 8 {
                self.image_data[offset as usize] as u16
            } else if self.im.bmp.bpp() == 16 {
                let offset = offset * 2; // * 2 as two bytes per pixel

                (self.image_data[offset as usize] as u16)
                    | ((self.image_data[(offset + 1) as usize] as u16) << 8)
            } else {
                panic!("Bit depth {} not supported", self.im.bmp.bpp());
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
                break Pixel(current_pixel.to_unsigned(), bit_value.into());
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
    use crate::pixelcolor::PixelColorU16;
    use crate::unsignedcoord::UnsignedCoord;

    #[test]
    fn negative_top_left() {
        let image: ImageBmp<PixelColorU16> = ImageBmp::new(include_bytes!(
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
        let image: ImageBmp<PixelColorU16> = ImageBmp::new(include_bytes!(
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
        let image: ImageBmp<PixelColorU16> = ImageBmp::new(include_bytes!(
            "../../tests/chessboard-4px-colour-16bit.bmp"
        ))
        .unwrap()
        .translate(Coord::new(-1, -1));
        let it = image.into_iter();

        // Image is RGB565 encoded in BPP
        let red = 0b11111_000000_00000_u16;
        let green = 0b00000_111111_00000_u16;
        let blue = 0b00000_000000_11111_u16;
        let black = 0x0000_u16;
        let white = 0xffff_u16;

        let expected: [Pixel<PixelColorU16>; 9] = [
            Pixel(UnsignedCoord::new(0, 0), red.into()),
            Pixel(UnsignedCoord::new(1, 0), black.into()),
            Pixel(UnsignedCoord::new(2, 0), green.into()),
            //
            Pixel(UnsignedCoord::new(0, 1), black.into()),
            Pixel(UnsignedCoord::new(1, 1), blue.into()),
            Pixel(UnsignedCoord::new(2, 1), black.into()),
            //
            Pixel(UnsignedCoord::new(0, 2), white.into()),
            Pixel(UnsignedCoord::new(1, 2), black.into()),
            Pixel(UnsignedCoord::new(2, 2), white.into()),
        ];

        assert_eq!(image.into_iter().count(), 9);

        for (idx, pixel) in it.enumerate() {
            assert_eq!(pixel, expected[idx]);
        }
    }
}
