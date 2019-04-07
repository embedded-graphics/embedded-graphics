use super::super::drawable::*;
use super::image::{Image, ImageIterator, ImageType};
use crate::coord::{Coord, ToUnsigned};
use crate::pixelcolor::PixelColor;

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
pub type Image1BPP<'a, C> = Image<'a, C, ImageType1BPP>;

/// 1 bit per pixel image type
#[derive(Debug, Copy, Clone)]
pub enum ImageType1BPP {}

impl ImageType for ImageType1BPP {}

impl<'a, C> IntoIterator for &'a Image1BPP<'a, C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = ImageIterator<'a, C, ImageType1BPP>;

    // NOTE: `self` is a reference already, no copies here!
    fn into_iter(self) -> Self::IntoIter {
        ImageIterator::new(self)
    }
}

/// Iterator over every pixel in the source image
impl<'a, C> Iterator for ImageIterator<'a, C, ImageType1BPP>
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelcolor::PixelColorU16;
    use crate::transform::Transform;
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
