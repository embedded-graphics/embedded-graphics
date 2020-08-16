use nom::{bytes::complete::take, number::complete::le_u8, IResult};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Hash, Eq, Ord)]
pub enum PacketType {
    Raw,
    Rle,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Hash, Eq, Ord)]
pub struct Packet<'a> {
    packet_type: PacketType,
    pixel_count: usize,
    data: &'a [u8],
    bytes_per_pixel: u8,
}

impl<'a> Packet<'a> {
    /// Parses a packet in a RLE encoded file.
    pub fn parse(input: &'a [u8], bytes_per_pixel: u8) -> IResult<&'a [u8], Self> {
        let (input, type_and_count) = le_u8(input)?;

        // The pixel count is encoded in the lower 7 bits and the actual number of pixels
        // is one more than the value stored in the packet.
        let pixel_count = usize::from(type_and_count & 0x7F) + 1;

        // The packet type is encoded in the upper bit: 0 -> Raw, 1 -> Rle
        let packet_type;
        let (input, data) = if type_and_count & 0x80 != 0 {
            packet_type = PacketType::Rle;

            // RLE packets always contain a single pixel
            take(bytes_per_pixel)(input)?
        } else {
            packet_type = PacketType::Raw;

            // Raw packets contain `pixel_count` pixels
            take(pixel_count * usize::from(bytes_per_pixel))(input)?
        };

        Ok((
            input,
            Self {
                packet_type,
                pixel_count,
                data,
                bytes_per_pixel,
            },
        ))
    }

    pub fn from_uncompressed(
        image_data: &'a [u8],
        pixel_count: usize,
        bytes_per_pixel: u8,
    ) -> Self {
        Self {
            packet_type: PacketType::Raw,
            pixel_count,
            data: image_data,
            bytes_per_pixel,
        }
    }
}

impl Iterator for Packet<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes_per_pixel = usize::from(self.bytes_per_pixel);

        if self.pixel_count == 0 || self.data.len() < bytes_per_pixel {
            return None;
        }

        self.pixel_count -= 1;

        let color = match self.bytes_per_pixel {
            1 => u32::from(self.data[0]),
            2 => u32::from_le_bytes([self.data[0], self.data[1], 0, 0]),
            3 => u32::from_le_bytes([self.data[0], self.data[1], self.data[2], 0]),
            4 => u32::from_le_bytes([self.data[0], self.data[1], self.data[2], self.data[3]]),
            _ => 0,
        };

        if self.packet_type == PacketType::Raw {
            self.data = &self.data[bytes_per_pixel..];
        }

        Some(color)
    }
}
