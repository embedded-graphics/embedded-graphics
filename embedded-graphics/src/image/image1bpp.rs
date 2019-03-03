use super::super::drawable::*;
use super::super::transform::*;
use super::Image;
use crate::coord::{Coord, ToUnsigned};
use crate::pixelcolor::PixelColor;
use crate::unsignedcoord::{ToSigned, UnsignedCoord};
use core::marker::PhantomData;

/// # 1 bit per pixel image
///
/// Each byte of input data defines the on/off state of 8 horizontal pixels to be displayed on the
/// screen.
///
/// You can convert an image to 1BPP for inclusion with `include_bytes!()` using the following
/// Imagemagick command:
///
/// ```bash
/// convert image.png -depth 1 gray:"image.raw"
/// ```
#[derive(Debug)]
pub struct Image1BPP<'a, C> {
    /// Image width in pixels
    width: u32,

    /// Image height in pixels
    height: u32,

    /// Image data, 1 bit per byte, 1 byte per 8 horizontal pixels
    imagedata: &'a [u8],

    /// Image offset in pixels from screen origin (0,0)
    pub offset: Coord,

    pixel_type: PhantomData<C>,
}

impl<'a, C> Dimensions for Image1BPP<'a, C>
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

impl<'a, C> Image<'a> for Image1BPP<'a, C>
where
    C: PixelColor,
{
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

impl<'a, C> IntoIterator for &'a Image1BPP<'a, C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = Image1BPPIterator<'a, C>;

    // NOTE: `self` is a reference already, no copies here!
    fn into_iter(self) -> Self::IntoIter {
        Image1BPPIterator {
            im: self,
            x: 0,
            y: 0,
        }
    }
}

#[derive(Debug)]
pub struct Image1BPPIterator<'a, C: 'a> {
    x: u32,
    y: u32,
    im: &'a Image1BPP<'a, C>,
}

/// Iterator over every pixel in the source image
impl<'a, C> Iterator for Image1BPPIterator<'a, C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        // If we're outside the upper left screen bounds, bail
        if (self.im.offset[0] + self.im.width as i32) < 0
            && (self.im.offset[1] + self.im.height as i32) < 0
        {
            return None;
        }

        let current_pixel = loop {
            let w = self.im.width;
            let h = self.im.height;
            let x = self.x;
            let y = self.y;

            // End iterator if we've run out of stuff
            if x >= w || y >= h {
                return None;
            }

            // Rows are padded to a full byte. Rust integer division rounds down, so add 1 full byte if there are remaining pixels
            let bytes_in_row = (w / 8) + if w % 8 > 0 { 1 } else { 0 };

            let row_start = bytes_in_row * y;

            let row_byte_index = x / 8;
            let byte_index = row_start + row_byte_index;
            let bit_offset = 7 - (x - (row_byte_index * 8));
            let bit_value = (self.im.imagedata[byte_index as usize] >> bit_offset) & 1;

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

impl<'a, C> Drawable for Image1BPP<'a, C> {}

impl<'a, C> Transform for Image1BPP<'a, C> {
    /// Translate the image from its current position to a new position by (x, y) pixels, returning
    /// a new `Image1BPP`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::image::{ Image, Image1BPP };
    /// # use embedded_graphics::transform::Transform;
    /// # use embedded_graphics::coord::Coord;
    /// # use embedded_graphics::pixelcolor::PixelColorU8;
    /// #
    /// // 8px x 1px test image
    /// let image: Image1BPP<PixelColorU8> = Image1BPP::new(&[ 0xff ], 8, 1);
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
    /// # use embedded_graphics::image::{ Image, Image1BPP };
    /// # use embedded_graphics::transform::Transform;
    /// # use embedded_graphics::coord::Coord;
    /// # use embedded_graphics::pixelcolor::PixelColorU8;
    /// #
    /// let mut image: Image1BPP<PixelColorU8> = Image1BPP::new(&[ 0xff ], 8, 1);
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
        let image: Image1BPP<PixelColorU16> =
            Image1BPP::new(&[0xff, 0x00], 4, 4).translate(Coord::new(-1, -1));

        assert_eq!(image.top_left(), Coord::new(-1, -1));
        assert_eq!(image.bottom_right(), Coord::new(3, 3));
        assert_eq!(image.size(), UnsignedCoord::new(4, 4));
    }

    #[test]
    fn dimensions() {
        let image: Image1BPP<PixelColorU16> =
            Image1BPP::new(&[0xff, 0x00], 4, 4).translate(Coord::new(100, 200));

        assert_eq!(image.top_left(), Coord::new(100, 200));
        assert_eq!(image.bottom_right(), Coord::new(104, 204));
        assert_eq!(image.size(), UnsignedCoord::new(4, 4));
    }
}
