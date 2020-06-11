use super::pixel_count;
use nom::{
    bits::{bits, complete::tag},
    bytes::complete::take,
    sequence::preceded,
    IResult,
};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct RawPacket<'a> {
    /// Number of pixels of this packet
    pub num_pixels: u8,

    /// Pixel data in this packet, up to 32 bits (4 bytes) per pixel
    pub pixel_data: &'a [u8],
}

impl<'a> RawPacket<'a> {
    /// Get the number of bytes in this packet
    pub fn len(&self) -> usize {
        self.pixel_data.len()
    }
}

pub fn raw_packet(bytes_per_pixel: u8) -> impl Fn(&[u8]) -> IResult<&[u8], RawPacket> {
    move |input| {
        // 0x00 = raw packet, 0x01 = RLE packet
        let (input, num_pixels) = bits(preceded(tag(0, 1u8), pixel_count))(input)?;
        let (input, pixel_data) = take(num_pixels as usize * bytes_per_pixel as usize)(input)?;

        Ok((
            input,
            RawPacket {
                num_pixels,
                pixel_data,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let input = [
            // 2 pixels worth of RAW data
            0b0000_0001,
            // 32BPP pixel
            0xAA,
            0xBB,
            0xCC,
            0xDD,
            // 32BPP pixel
            0x11,
            0x22,
            0x33,
            0x44,
        ];

        let (remaining, packet) = raw_packet(4)(&input).unwrap();

        assert_eq!(remaining.len(), 0);
        assert_eq!(
            packet,
            RawPacket {
                num_pixels: 2,
                pixel_data: &[
                    0xAA, 0xBB, 0xCC, 0xDD, //
                    0x11, 0x22, 0x33, 0x44, //
                ]
            }
        );
    }

    #[test]
    fn ignore_rle_packet() {
        let input = [
            // 2 pixels worth of RLE data
            0b1000_0001,
            // 32BPP pixel
            0xAA,
            0xBB,
            0xCC,
            0xDD,
        ];

        let result = raw_packet(4)(&input);

        assert!(result.is_err());
    }

    #[test]
    fn stop_at_packet_end() {
        let input = [
            // 2 pixels worth of non-RLE data
            0b0000_0001,
            // 32BPP pixel
            0xAA,
            0xBB,
            0xCC,
            0xDD,
            // 32BPP pixel
            0x11,
            0x22,
            0x33,
            0x44,
            // 32BPP pixel (extra, invalid)
            0x55,
            0x66,
            0x77,
            0x88,
        ];

        let (remaining, packet) = raw_packet(4)(&input).unwrap();

        assert_eq!(remaining, &[0x55, 0x66, 0x77, 0x88]);
        assert_eq!(
            packet,
            RawPacket {
                num_pixels: 2,
                pixel_data: &[
                    0xAA, 0xBB, 0xCC, 0xDD, //
                    0x11, 0x22, 0x33, 0x44, //
                ]
            }
        );
    }
}
