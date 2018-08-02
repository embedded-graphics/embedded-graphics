//! 8 bit per pixel image. Each byte of input data defines the on/off state for each pixel. This
//! currently only supports monochrome displays, so if the pixel value is 0, it's off, anything
//! above 0 is on.
//!
//! You can convert an image to 8BPP for inclusion with `include_bytes!()` using the following
//! Imagemagick command:
//!
//! ```bash
//! convert image.png -depth 8 gray:"image.raw"
//! ```

use super::super::drawable::*;
use super::super::transform::*;
use super::Image;
use coord::{Coord, ToUnsigned};

/// 8 bit per pixel image
#[derive(Debug)]
pub struct Image8BPP<'a> {
    /// Image width
    width: u32,

    /// Image height
    height: u32,

    /// Image data, 1 byte per pixel
    imagedata: &'a [u8],

    /// Top left corner offset from display origin (0,0)
    pub offset: Coord,
}

impl<'a> Image<'a> for Image8BPP<'a> {
    /// Create a new 8BPP image with given data, width and height. Data length *must* equal
    /// `width * height`
    fn new(imagedata: &'a [u8], width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            imagedata,
            offset: Coord::new(0, 0),
        }
    }
}

impl<'a> IntoIterator for &'a Image8BPP<'a> {
    type Item = Pixel<u8>;
    type IntoIter = Image8BPPIterator<'a>;

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
pub struct Image8BPPIterator<'a> {
    x: u32,
    y: u32,
    im: &'a Image8BPP<'a>,
}

impl<'a> Iterator for Image8BPPIterator<'a> {
    type Item = Pixel<u8>;

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
                break (current_pixel.to_unsigned(), Color::new(bit_value));
            }
        };

        Some(current_pixel)
    }
}

impl<'a> Drawable for Image8BPP<'a> {}

impl<'a> Transform for Image8BPP<'a> {
    /// Translate the image from its current position to a new position by (x, y) pixels, returning
    /// a new `Image8BPP`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::image::{ Image, Image8BPP };
    /// # use embedded_graphics::transform::Transform;
    /// # use embedded_graphics::coord::Coord;
    ///
    /// // 1px x 1px test image
    /// let image = Image8BPP::new(&[ 0xff ], 1, 1);
    /// let moved = image.translate(Coord::new(25, 30));
    ///
    /// assert_eq!(image.offset, Coord::new(0, 0));
    /// assert_eq!(moved.offset, Coord::new(25, 30));
    /// ```
    fn translate(&self, by: Coord) -> Self {
        Self {
            offset: self.offset + by,
            ..*self
        }
    }

    /// Translate the image from its current position to a new position by (x, y) pixels.
    ///
    /// ```
    /// # use embedded_graphics::image::{ Image, Image8BPP };
    /// # use embedded_graphics::transform::Transform;
    /// # use embedded_graphics::coord::Coord;
    ///
    /// // 1px x 1px test image
    /// let mut image = Image8BPP::new(&[ 0xff ], 1, 1);
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
        let image = Image8BPP::new(
            &[0xff, 0x00, 0xbb, 0x00, 0xcc, 0x00, 0xee, 0x00, 0xaa],
            3,
            3,
        ).translate(Coord::new(-1, -1));
        let mut it = image.into_iter();

        assert_eq!(it.next(), Some((UnsignedCoord::new(0, 0), Color::new(0xcc))));
        assert_eq!(it.next(), Some((UnsignedCoord::new(1, 0), Color::new(0x00))));
        assert_eq!(it.next(), Some((UnsignedCoord::new(0, 1), Color::new(0x00))));
        assert_eq!(it.next(), Some((UnsignedCoord::new(1, 1), Color::new(0xaa))));

        assert_eq!(it.next(), None);
    }
}
