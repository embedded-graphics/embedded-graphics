//! 1 bit per pixel image. Each byte of input data defines the on/off state of 8 horizontal pixels
//! to be displayed on the screen.
//!
//! You can convert an image to 1BPP for inclusion with `include_bytes!()` using the following
//! Imagemagick command:
//!
//! ```bash
//! convert image.png -depth 1 gray:"image.raw"
//! ```

use super::super::drawable::*;
use super::super::transform::*;
use super::Image;
use coord::Coord;

/// 1 bit per pixel image
#[derive(Debug)]
pub struct Image1BPP<'a> {
    /// Image width in pixels
    width: u32,

    /// Image height in pixels
    height: u32,

    /// Image data, 1 bit per byte, 1 byte per 8 horizontal pixels
    imagedata: &'a [u8],

    /// Image offset in pixels from screen origin (0,0)
    pub offset: Coord,
}

impl<'a> Image<'a> for Image1BPP<'a> {
    fn new(imagedata: &'a [u8], width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            imagedata,
            offset: Coord::new(0, 0),
        }
    }
}

impl<'a> IntoIterator for &'a Image1BPP<'a> {
    type Item = Pixel;
    type IntoIter = Image1BPPIterator<'a>;

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
pub struct Image1BPPIterator<'a> {
    x: u32,
    y: u32,
    im: &'a Image1BPP<'a>,
}

/// Iterator over every pixel in the source image
impl<'a> Iterator for Image1BPPIterator<'a> {
    type Item = Pixel;

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
                break (current_pixel, bit_value);
            }
        };

        Some(current_pixel)
    }
}

impl<'a> Drawable for Image1BPP<'a> {}

impl<'a> Transform for Image1BPP<'a> {
    /// Translate the image from its current position to a new position by (x, y) pixels, returning
    /// a new `Image1BPP`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::image::{ Image, Image1BPP };
    /// # use embedded_graphics::transform::Transform;
    /// # use embedded_graphics::coord::Coord;
    ///
    /// // 8px x 1px test image
    /// let image = Image1BPP::new(&[ 0xff ], 8, 1);
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
    /// # use embedded_graphics::image::{ Image, Image1BPP };
    /// # use embedded_graphics::transform::Transform;
    /// # use embedded_graphics::coord::Coord;
    ///
    /// let mut image = Image1BPP::new(&[ 0xff ], 8, 1);
    /// image.translate_mut(Coord::new(25, 30));
    ///
    /// assert_eq!(image.offset, Coord::new(25, 30));
    /// ```
    fn translate_mut(&mut self, by: Coord) -> &mut Self {
        self.offset += by;

        self
    }
}
