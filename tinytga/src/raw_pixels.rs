use crate::{packet::Packet, raw_tga::RawTga};
use embedded_graphics::prelude::*;

/// Iterator over individual TGA pixels.
///
/// See the [`pixels`] method for additional information.
///
/// [`pixels`]: struct.RawTga.html#method.pixels
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct RawPixels<'a, 'b> {
    /// Reference to original TGA image
    raw_tga: &'a RawTga<'b>,

    position: Point,

    packet: Packet<'b>,

    remaining_data: &'b [u8],
}

impl<'a, 'b> RawPixels<'a, 'b> {
    pub(crate) fn new(raw_tga: &'a RawTga<'b>) -> Self {
        let size = raw_tga.size();
        let remaining_pixels = size.width as usize * size.height as usize;

        let image_data = raw_tga.image_data();

        let (first_packet_pixels, data) = if raw_tga.image_type().is_rle() {
            (0, image_data)
        } else {
            (remaining_pixels, &image_data[0..0])
        };

        let packet = Packet::from_uncompressed(
            raw_tga.image_data(),
            first_packet_pixels,
            raw_tga.image_data_bpp().bytes(),
        );

        let start_y = if raw_tga.image_origin().is_bottom() {
            raw_tga.size().height.saturating_sub(1)
        } else {
            0
        };

        Self {
            raw_tga: raw_tga,
            packet,
            remaining_data: data,
            position: Point::new(0, start_y as i32),
        }
    }

    /// Returns the next pixel position.
    fn next_position(&mut self) -> Option<Point> {
        if self.position.y < 0 || self.position.y >= self.raw_tga.size().height as i32 {
            return None;
        }

        let position = self.position;

        self.position.x += 1;

        if self.position.x >= self.raw_tga.size().width as i32 {
            self.position.x = 0;

            if self.raw_tga.image_origin().is_bottom() {
                self.position.y -= 1;
            } else {
                self.position.y += 1;
            }
        }

        Some(position)
    }
}

impl<'a, 'b> Iterator for RawPixels<'a, 'b> {
    type Item = RawPixel;

    fn next(&mut self) -> Option<Self::Item> {
        let position = self.next_position()?;

        let color = if let Some(color) = self.packet.next() {
            color
        } else {
            match Packet::parse(self.remaining_data, self.raw_tga.image_data_bpp().bytes()) {
                Ok((data, packet)) => {
                    self.remaining_data = data;
                    self.packet = packet;

                    self.packet.next().unwrap_or(0)
                }
                Err(_) => 0,
            }
        };

        let color = if let Some(color_map) = self.raw_tga.color_map() {
            color_map.get_raw(color as usize).unwrap_or(0)
        } else {
            color
        };

        Some(RawPixel::new(position, color))
    }
}

/// Pixel with raw pixel color.
///
/// This struct is returned by the [`RawPixels`] iterator.
///
/// [`RawPixels`]: struct.RawPixels.html
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct RawPixel {
    /// The position relative to the top left corner of the image.
    pub position: Point,

    /// The raw pixel color.
    pub color: u32,
}

impl RawPixel {
    /// Creates a new raw pixel.
    pub fn new(position: Point, color: u32) -> Self {
        Self { position, color }
    }
}
