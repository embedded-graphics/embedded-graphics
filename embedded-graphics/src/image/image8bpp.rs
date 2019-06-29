use super::super::drawable::*;
use super::image::{Image, ImageIterator, ImageType};
use crate::coord::{Coord, ToUnsigned};
use crate::pixelcolor::{FromRawData, PixelColor};

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
///
/// # Examples
///
/// ## Load an 8 bit per pixel image from a raw byte slice and draw it to a display
///
/// Note that images must be passed to `Display#draw` by reference, or by explicitly calling
/// `.into_iter()` on them, unlike other embedded_graphics objects.
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::image::Image8BPP;
/// # use embedded_graphics::mock_display::MockDisplay;
/// # use embedded_graphics::pixelcolor::Y8;
/// # let mut display: MockDisplay<Y8> = MockDisplay::default();
///
/// // Load `patch_8bpp.raw`, an 8BPP 4x4px image
/// let image = Image8BPP::new(include_bytes!("../../../assets/patch_8bpp.raw"), 4, 4);
///
/// // Equivalent behaviour
/// display.draw(&image);
/// display.draw(image.into_iter());
/// ```
pub type Image8BPP<'a, C> = Image<'a, C, ImageType8BPP>;

/// 8 bits per pixel image type
#[derive(Debug, Copy, Clone)]
pub enum ImageType8BPP {}

impl ImageType for ImageType8BPP {}

impl<'a, C> IntoIterator for &'a Image8BPP<'a, C>
where
    C: PixelColor + FromRawData,
{
    type Item = Pixel<C>;
    type IntoIter = ImageIterator<'a, C, ImageType8BPP>;

    // NOTE: `self` is a reference already, no copies here!
    fn into_iter(self) -> Self::IntoIter {
        ImageIterator::new(self)
    }
}

impl<'a, C> Iterator for ImageIterator<'a, C, ImageType8BPP>
where
    C: PixelColor + FromRawData,
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
            let data = self.im.imagedata[offset as usize] as u32;

            let current_pixel = self.im.offset + Coord::new(x as i32, y as i32);

            // Increment stuff
            self.x += 1;

            // Step down a row if we've hit the end of this one
            if self.x >= w {
                self.x = 0;
                self.y += 1;
            }

            if current_pixel[0] >= 0 && current_pixel[1] >= 0 {
                break Pixel(current_pixel.to_unsigned(), C::from_raw_data(data));
            }
        };

        Some(current_pixel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelcolor::Y8;
    use crate::transform::Transform;
    use crate::unsignedcoord::UnsignedCoord;

    #[test]
    fn negative_top_left() {
        let image: Image8BPP<Y8> = Image8BPP::new(
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
        let image: Image8BPP<Y8> = Image8BPP::new(
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
        let image: Image8BPP<Y8> = Image8BPP::new(
            &[0xff, 0x00, 0xbb, 0x00, 0xcc, 0x00, 0xee, 0x00, 0xaa],
            3,
            3,
        )
        .translate(Coord::new(-1, -1));
        let mut it = image.into_iter();

        assert_eq!(
            it.next(),
            Some(Pixel(UnsignedCoord::new(0, 0), Y8::new(0xcc)))
        );
        assert_eq!(it.next(), Some(Pixel(UnsignedCoord::new(1, 0), Y8::BLACK)));
        assert_eq!(it.next(), Some(Pixel(UnsignedCoord::new(0, 1), Y8::BLACK)));
        assert_eq!(
            it.next(),
            Some(Pixel(UnsignedCoord::new(1, 1), Y8::new(0xaa)))
        );

        assert_eq!(it.next(), None);
    }
}
