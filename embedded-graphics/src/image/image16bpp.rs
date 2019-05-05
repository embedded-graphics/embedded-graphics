use super::super::drawable::*;
use super::image::{Image, ImageIterator, ImageType};
use crate::coord::{Coord, ToUnsigned};
use crate::pixelcolor::PixelColor;

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
pub type Image16BPP<'a, C> = Image<'a, C, ImageType16BPP>;

/// 16 bits per pixel image type
#[derive(Debug, Copy, Clone)]
pub enum ImageType16BPP {}

impl ImageType for ImageType16BPP {}

impl<'a, C> IntoIterator for &'a Image16BPP<'a, C>
where
    C: PixelColor + From<u16>,
{
    type Item = Pixel<C>;
    type IntoIter = ImageIterator<'a, C, ImageType16BPP>;

    fn into_iter(self) -> Self::IntoIter {
        ImageIterator::new(self)
    }
}

impl<'a, C> Iterator for ImageIterator<'a, C, ImageType16BPP>
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelcolor::PixelColorU16;
    use crate::transform::Transform;
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
