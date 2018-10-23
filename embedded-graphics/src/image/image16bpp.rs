//! 16 bit per pixel image. Each byte of input data defines the on/off state for each pixel. This
//! currently only supports monochrome displays, so if the pixel value is 0, it's off, anything
//! above 0 is on.
//!
//! You can convert an image to 16BPP for inclusion with `include_bytes!()` using the following
//! Imagemagick command:
//!
//! ```bash
//! convert image.png -depth 16 gray:"image.raw"
//! ```

use super::super::drawable::*;
use super::super::transform::*;
use super::Image;
use coord::{Coord, ToUnsigned};
use pixelcolor::PixelColorU16;

/// 16 bit per pixel image
#[derive(Debug)]
pub struct Image16BPP<'a> {
    /// Image width
    width: u32,

    /// Image height
    height: u32,

    /// Image data, 1 byte per pixel
    imagedata: &'a [u8],

    /// Top left corner offset from display origin (0,0)
    pub offset: Coord,

    pixel_type: PixelColorU16,
}

impl<'a> Image<'a> for Image16BPP<'a>{
    /// Create a new 16BPP image with given data, width and height. Data length *must* equal
    /// `width * height`
    fn new(imagedata: &'a [u8], width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            imagedata,
            offset: Coord::new(0, 0),
            pixel_type: PixelColorU16(0u16),
        }
    }
}

impl<'a> IntoIterator for &'a Image16BPP<'a>{
    type Item = Pixel<PixelColorU16>;
    type IntoIter = Image16BPPIterator<'a>;

    // NOTE: `self` is a reference already, no copies here!
    fn into_iter(self) -> Self::IntoIter {
        Image16BPPIterator {
            im: self,
            x: 0,
            y: 0,
        }
    }
}

#[derive(Debug)]
pub struct Image16BPPIterator<'a>{
    x: u32,
    y: u32,
    im: &'a Image16BPP<'a>,
}

impl<'a> Iterator for Image16BPPIterator<'a>{
    type Item = Pixel<PixelColorU16>;

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

            // let offset = (y * w) + x;
            // let bit_value = self.im.imagedata[offset as usize];

            let offset = ((y * w) + x) * 2; // * 2 as two bytes per pixel
            // let bit_value = self.im.imagedata[offset as usize];
            let bit_value = (self.im.imagedata[(offset + 1) as usize] as u16) << 8 | self.im.imagedata[offset as usize] as u16;

            let current_pixel = self.im.offset + Coord::new(x as i32, y as i32);

            // Increment stuff
            self.x += 1;

            // Step down a row if we've hit the end of this one
            if self.x >= w {
                self.x = 0;
                self.y += 1;
            }

            if current_pixel[0] >= 0 && current_pixel[1] >= 0 {
                break Pixel(current_pixel.to_unsigned(), bit_value.into()); // something like this? PixelColorU16(bit_value)
            }
        };

        Some(current_pixel)
    }
}

impl<'a> Drawable for Image16BPP<'a> {}

impl<'a> Transform for Image16BPP<'a>{
    /// Translate the image from its current position to a new position by (x, y) pixels, returning
    /// a new `Image16BPP`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::image::{ Image, Image16BPP };
    /// # use embedded_graphics::transform::Transform;
    /// # use embedded_graphics::coord::Coord;
    /// # use embedded_graphics::pixelcolor::PixelColorU16;
    ///
    /// // 1px x 1px test image
    /// let image: Image16BPP<PixelColorU16> = Image16BPP::new(&[ 0xff ], 1, 1);
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
    /// # use embedded_graphics::pixelcolor::PixelColorU16;
    ///
    /// // 1px x 1px test image
    /// let mut image: Image16BPP<PixelColorU16> = Image16BPP::new(&[ 0xff ], 1, 1);
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
    use unsignedcoord::UnsignedCoord;

    #[test]
    fn it_can_have_negative_offsets() {
        let image: Image16BPP = Image16BPP::new(
            &[0xff, 0x00, 0xbb, 0x00, 0xcc, 0x00, 0xee, 0x00, 0xaa],
            3,
            3,
        ).translate(Coord::new(-1, -1));
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
