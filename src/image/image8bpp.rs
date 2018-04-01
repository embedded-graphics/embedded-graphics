use super::super::drawable::*;
use super::super::transform::*;
use super::Image;

#[derive(Debug)]
pub struct Image8BPP<'a> {
    width: u32,
    height: u32,
    imagedata: &'a [u8],
    pub offset: Coord,
}

impl<'a> Image<'a> for Image8BPP<'a> {
    fn new(imagedata: &'a [u8], width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            imagedata,
            offset: (0, 0),
        }
    }
}

impl<'a> IntoIterator for &'a Image8BPP<'a> {
    type Item = Pixel;
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

        let offset = (y * w) + x;
        let bit_value = self.im.imagedata[offset as usize];

        let current_pixel: Self::Item = ((self.im.offset.0 + x, self.im.offset.1 + y), bit_value);

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

impl<'a> Drawable for Image8BPP<'a> {}

impl<'a> Transform for Image8BPP<'a> {
    /// Translate the image from its current position to a new position by (x, y) pixels, returning
    /// a new `Image8BPP`.
    ///
    /// ```
    /// # use embedded_graphics::image::{ Image, Image8BPP };
    /// # use embedded_graphics::transform::Transform;
    ///
    /// // 8px x 1px test image
    /// let image = Image8BPP::new(&[ 0xff ], 8, 1);
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
