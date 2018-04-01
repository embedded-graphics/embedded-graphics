use super::super::drawable::*;
use super::super::transform::*;
use super::Image;

#[derive(Debug)]
pub struct Image1BPP<'a> {
    width: u32,
    height: u32,
    imagedata: &'a [u8],
    pub offset: Coord,
}

impl<'a> Image<'a> for Image1BPP<'a> {
    fn new(imagedata: &'a [u8], width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            imagedata,
            offset: (0, 0),
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

impl<'a> Iterator for Image1BPPIterator<'a> {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
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

        let current_pixel: Self::Item = ((x + self.im.offset.0, y + self.im.offset.1), bit_value);

        // Increment stuff
        self.x += 1;

        // Step down a row if we've hit the end of this one
        if self.x >= w {
            self.x = 0;
            self.y += 1;
        }

        Some(current_pixel)
    }
}

impl<'a> Drawable for Image1BPP<'a> {}

impl<'a> Transform for Image1BPP<'a> {
    /// Translate the image from its current position to a new position by (x, y) pixels, returning
    /// a new `Image1BPP`.
    ///
    /// ```
    /// # use embedded_graphics::image::{ Image, Image1BPP };
    /// # use embedded_graphics::transform::Transform;
    ///
    /// // 8px x 1px test image
    /// let image = Image1BPP::new(&[ 0xff ], 8, 1);
    /// let moved = image.translate((25, 30));
    ///
    /// assert_eq!(image.offset, (0, 0));
    /// assert_eq!(moved.offset, (25, 30));
    /// ```
    fn translate(&self, by: Coord) -> Self {
        Self {
            offset: (self.offset.0 + by.0, self.offset.1 + by.1),
            ..*self
        }
    }
}
