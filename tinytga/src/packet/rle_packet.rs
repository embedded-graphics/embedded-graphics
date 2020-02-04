use super::pixel_count;
use nom::{
    bits::{bits, complete::tag},
    bytes::complete::take,
    sequence::preceded,
    IResult,
};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct RlePacket<'a> {
    /// Number of pixels in this run
    pub run_length: u8,

    /// Pixel data
    pub pixel_data: &'a [u8],
}

impl<'a> RlePacket<'a> {
    /// Get the number of pixels in this packet
    pub fn len(&self) -> usize {
        self.pixel_data.len() * self.run_length as usize
    }
}

pub fn rle_packet(bytes_per_pixel: u8) -> impl Fn(&[u8]) -> IResult<&[u8], RlePacket> {
    move |input| {
        // 0x00 = raw packet, 0x01 = RLE packet
        let (input, run_length) = bits(preceded(tag(1, 1u8), pixel_count))(input)?;
        let (input, pixel_data) = take(bytes_per_pixel)(input)?;

        Ok((
            input,
            RlePacket {
                run_length,
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
            // 2 pixels worth of RLE data
            0b1000_0001,
            // 32BPP pixel
            0xAA,
            0xBB,
            0xCC,
            0xDD,
        ];

        let (remaining, packet) = rle_packet(4)(&input).unwrap();

        assert_eq!(remaining, &[]);
        assert_eq!(
            packet,
            RlePacket {
                run_length: 2,
                pixel_data: &[0xAA, 0xBB, 0xCC, 0xDD]
            }
        );
    }

    #[test]
    fn ignore_raw_packet() {
        let input = [
            // 2 pixels worth of raw (non-RLE) data
            0b0000_0001,
            // 32BPP pixel
            0xAA,
            0xBB,
            0xCC,
            0xDD,
        ];

        let result = rle_packet(4)(&input);

        assert!(result.is_err());
    }

    #[test]
    fn stop_at_packet_end() {
        let input = [
            // 2 pixels worth of RLE data
            0b1000_0001,
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

        let (remaining, packet) = rle_packet(4)(&input).unwrap();

        assert_eq!(remaining, &[0x11, 0x22, 0x33, 0x44]);
        assert_eq!(
            packet,
            RlePacket {
                run_length: 2,
                pixel_data: &[0xAA, 0xBB, 0xCC, 0xDD]
            }
        );
    }
}
