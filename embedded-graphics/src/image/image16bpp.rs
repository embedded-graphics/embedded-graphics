use super::super::drawable::*;
use super::super::transform::*;
use super::Image;
use crate::coord::{Coord, ToUnsigned};
use crate::pixelcolor::PixelColor;
use crate::unsignedcoord::{ToSigned, UnsignedCoord};
use core::marker::PhantomData;

/// # 16 bits per pixel images
///
/// Every two bytes define the color for each pixel.
///
/// You can convert an image to 16BPP for inclusion with `include_bytes!()` doing the following
///
/// ```bash
/// convert image.png -flip -flop -type truecolor -define bmp:subtype=RGB565 -resize '64x64!' -depth 16 -strip image.bmp
/// ```
/// then
/// ```bash
/// tail -c $bytes image.bmp > image.raw // where $bytes is w * h * 2
/// ```
/// This will remove the BMP header leaving the raw pixel data
/// E.g 64x64 image will have `64 * 64 * 2` bytes of raw data.
#[derive(Debug)]
pub struct Image16BPP<'a, C: PixelColor> {
    /// Image width
    width: u32,

    /// Image height
    height: u32,

    /// Image data, 1 byte per pixel
    imagedata: &'a [u8],

    /// Top left corner offset from display origin (0,0)
    pub offset: Coord,

    pixel_type: PhantomData<C>,
}

impl<'a, C> Dimensions for Image16BPP<'a, C>
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
        let height = self.height;
        let width = self.width;

        UnsignedCoord::new(width, height)
    }
}

impl<'a, C> Image<'a> for Image16BPP<'a, C>
where
    C: PixelColor,
{
    /// Create a new 16BPP image with given data, width and height. Data length *must* equal
    /// `width * height * 2`
    fn new(imagedata: &'a [u8], width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            imagedata,
            offset: Coord::new(0, 0),
            pixel_type: PhantomData,
        }
    }
}

impl<'a, C> IntoIterator for &'a Image16BPP<'a, C>
where
    C: PixelColor + From<u16>,
{
    type Item = Pixel<C>;
    type IntoIter = Image16BPPIterator<'a, C>;

    fn into_iter(self) -> Self::IntoIter {
        Image16BPPIterator {
            im: self,
            x: 0,
            y: 0,
        }
    }
}

#[derive(Debug)]
pub struct Image16BPPIterator<'a, C: 'a>
where
    C: PixelColor,
{
    x: u32,
    y: u32,
    im: &'a Image16BPP<'a, C>,
}

impl<'a, C> Iterator for Image16BPPIterator<'a, C>
where
    C: PixelColor + From<u16>,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        let current_pixel = loop {
            let w = self.im.width;
            let h = self.im.height;
            let x = self.x;
            let y = self.y;

            // End iterator if we've run out of stuff
            if x >= w || y >= h {
                return None;
            }

            let offset = ((y * w) + x) * 2; // * 2 as two bytes per pixel
                                            // merge two bytes into a u16
            let bit_value = (self.im.imagedata[(offset + 1) as usize] as u16) << 8
                | self.im.imagedata[offset as usize] as u16;

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

impl<'a, C> Drawable for Image16BPP<'a, C> where C: PixelColor {}

impl<'a, C> Transform for Image16BPP<'a, C>
where
    C: PixelColor,
{
    /// Translate the image from its current position to a new position by (x, y) pixels, returning
    /// a new `Image16BPP`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::image::{ Image, Image16BPP };
    /// # use embedded_graphics::transform::Transform;
    /// # use embedded_graphics::coord::Coord;
    /// # use embedded_graphics::pixelcolor::PixelColorU8;
    ///
    /// // 1px x 1px test image
    /// let image: Image16BPP<PixelColorU8> = Image16BPP::new(&[ 0xff ], 1, 1);
    /// let moved = image.translate(Coord::new(25, 30));
    ///
    /// assert_eq!(image.offset, Coord::new(0, 0));
    /// assert_eq!(moved.offset, Coord::new(25, 30));
    /// ```
    fn translate(&self, by: Coord) -> Self {
        Self {
            offset: self.offset + by,
            ..*self.clone()
        }
    }

    /// Translate the image from its current position to a new position by (x, y) pixels.
    ///
    /// ```
    /// # use embedded_graphics::image::{ Image, Image16BPP };
    /// # use embedded_graphics::transform::Transform;
    /// # use embedded_graphics::coord::Coord;
    /// # use embedded_graphics::pixelcolor::PixelColorU8;
    ///
    /// // 1px x 1px test image
    /// let mut image: Image16BPP<PixelColorU8> = Image16BPP::new(&[ 0xff ], 1, 1);
    /// image.translate_mut(Coord::new(25, 30));
    ///
    /// assert_eq!(image.offset, Coord::new(25, 30));
    /// ```
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
        let image: Image16BPP<PixelColorU16> = Image16BPP::new(
            &[
                0xff, 0x00, 0x00, 0x00, 0xbb, 0x00, //
                0x00, 0x00, 0xcc, 0x00, 0x00, 0x00, //
                0xee, 0x00, 0x00, 0x00, 0xaa, 0x00,
            ],
            3,
            3,
        )
        .translate(Coord::new(-1, -1));

        assert_eq!(image.top_left(), Coord::new(-1, -1));
        assert_eq!(image.bottom_right(), Coord::new(2, 2));
        assert_eq!(image.size(), UnsignedCoord::new(3, 3));
    }

    #[test]
    fn dimensions() {
        let image: Image16BPP<PixelColorU16> = Image16BPP::new(
            &[
                0xff, 0x00, 0x00, 0x00, 0xbb, 0x00, //
                0x00, 0x00, 0xcc, 0x00, 0x00, 0x00, //
                0xee, 0x00, 0x00, 0x00, 0xaa, 0x00,
            ],
            3,
            3,
        )
        .translate(Coord::new(100, 200));

        assert_eq!(image.top_left(), Coord::new(100, 200));
        assert_eq!(image.bottom_right(), Coord::new(103, 203));
        assert_eq!(image.size(), UnsignedCoord::new(3, 3));
    }

    #[test]
    fn it_can_have_negative_offsets() {
        let image: Image16BPP<PixelColorU16> = Image16BPP::new(
            &[
                0xff, 0x00, 0x00, 0x00, 0xbb, 0x00, //
                0x00, 0x00, 0xcc, 0x00, 0x00, 0x00, //
                0xee, 0x00, 0x00, 0x00, 0xaa, 0x00,
            ],
            3,
            3,
        )
        .translate(Coord::new(-1, -1));
        let mut it = image.into_iter();

        assert_eq!(
            it.next(),
            Some(Pixel(UnsignedCoord::new(0, 0), 0xcc_u16.into()))
        );
        assert_eq!(
            it.next(),
            Some(Pixel(UnsignedCoord::new(1, 0), 0x00_u16.into()))
        );
        assert_eq!(
            it.next(),
            Some(Pixel(UnsignedCoord::new(0, 1), 0x00_u16.into()))
        );
        assert_eq!(
            it.next(),
            Some(Pixel(UnsignedCoord::new(1, 1), 0xaa_u16.into()))
        );

        assert_eq!(it.next(), None);
    }
}
