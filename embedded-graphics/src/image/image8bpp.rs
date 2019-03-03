use super::super::drawable::*;
use super::super::transform::*;
use super::Image;
use crate::coord::{Coord, ToUnsigned};
use crate::pixelcolor::PixelColor;
use crate::unsignedcoord::{ToSigned, UnsignedCoord};
use core::marker::PhantomData;

/// # 8 bits per pixel image
///
/// Each byte of input data defines the on/off state for each pixel. This currently only supports
/// monochrome displays, so if the pixel value is 0, it's off, anything above 0 is on.
///
/// You can convert an image to 8BPP for inclusion with `include_bytes!()` using the following
/// Imagemagick command:
///
/// ```bash
/// convert image.png -depth 8 gray:"image.raw"
/// ```
#[derive(Debug)]
pub struct Image8BPP<'a, C: PixelColor> {
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

impl<'a, C> Dimensions for Image8BPP<'a, C>
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

impl<'a, C> Image<'a> for Image8BPP<'a, C>
where
    C: PixelColor,
{
    /// Create a new 8BPP image with given data, width and height. Data length *must* equal
    /// `width * height`
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

impl<'a, C> IntoIterator for &'a Image8BPP<'a, C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = Image8BPPIterator<'a, C>;

    // NOTE: `self` is a reference already, no copies here!
    fn into_iter(self) -> Self::IntoIter {
        Image8BPPIterator {
            im: self,
            x: 0,
            y: 0,
        }
    }
}

#[derive(Debug)]
pub struct Image8BPPIterator<'a, C: 'a>
where
    C: PixelColor,
{
    x: u32,
    y: u32,
    im: &'a Image8BPP<'a, C>,
}

impl<'a, C> Iterator for Image8BPPIterator<'a, C>
where
    C: PixelColor,
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

            let offset = (y * w) + x;
            let bit_value = self.im.imagedata[offset as usize];

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

impl<'a, C> Drawable for Image8BPP<'a, C> where C: PixelColor {}

impl<'a, C> Transform for Image8BPP<'a, C>
where
    C: PixelColor,
{
    /// Translate the image from its current position to a new position by (x, y) pixels, returning
    /// a new `Image8BPP`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::image::{ Image, Image8BPP };
    /// # use embedded_graphics::transform::Transform;
    /// # use embedded_graphics::coord::Coord;
    /// # use embedded_graphics::pixelcolor::PixelColorU8;
    ///
    /// // 1px x 1px test image
    /// let image: Image8BPP<PixelColorU8> = Image8BPP::new(&[ 0xff ], 1, 1);
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
    /// # use embedded_graphics::image::{ Image, Image8BPP };
    /// # use embedded_graphics::transform::Transform;
    /// # use embedded_graphics::coord::Coord;
    /// # use embedded_graphics::pixelcolor::PixelColorU8;
    ///
    /// // 1px x 1px test image
    /// let mut image: Image8BPP<PixelColorU8> = Image8BPP::new(&[ 0xff ], 1, 1);
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
    use crate::pixelcolor::PixelColorU8;
    use crate::unsignedcoord::UnsignedCoord;

    #[test]
    fn negative_top_left() {
        let image: Image8BPP<PixelColorU8> = Image8BPP::new(
            &[0xff, 0x00, 0xbb, 0x00, 0xcc, 0x00, 0xee, 0x00, 0xaa],
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
        let image: Image8BPP<PixelColorU8> = Image8BPP::new(
            &[0xff, 0x00, 0xbb, 0x00, 0xcc, 0x00, 0xee, 0x00, 0xaa],
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
        let image: Image8BPP<PixelColorU8> = Image8BPP::new(
            &[0xff, 0x00, 0xbb, 0x00, 0xcc, 0x00, 0xee, 0x00, 0xaa],
            3,
            3,
        )
        .translate(Coord::new(-1, -1));
        let mut it = image.into_iter();

        assert_eq!(
            it.next(),
            Some(Pixel(UnsignedCoord::new(0, 0), 0xcc_u8.into()))
        );
        assert_eq!(
            it.next(),
            Some(Pixel(UnsignedCoord::new(1, 0), 0x00_u8.into()))
        );
        assert_eq!(
            it.next(),
            Some(Pixel(UnsignedCoord::new(0, 1), 0x00_u8.into()))
        );
        assert_eq!(
            it.next(),
            Some(Pixel(UnsignedCoord::new(1, 1), 0xaa_u8.into()))
        );

        assert_eq!(it.next(), None);
    }
}
