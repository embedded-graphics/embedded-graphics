mod raw_packet;
mod rle_packet;

pub use self::raw_packet::{raw_packet, RawPacket};
pub use self::rle_packet::{rle_packet, RlePacket};
use nom::{bits::complete::take, branch::alt, combinator::map, IResult};

/// A Run Length Encoded (RLE) packet
#[derive(Debug)]
pub enum Packet<'a> {
    /// Data in this packet is Run Length Encoded
    RlePacket(RlePacket<'a>),

    /// Data is in a packet, but is not compressed at all
    RawPacket(RawPacket<'a>),

    /// File is not packeted. Contains reference to all image data in the file
    FullContents(&'a [u8]),
}

impl<'a> Packet<'a> {
    /// Get the length in bytes of the pixel data in this packet
    pub fn len(&self) -> usize {
        match self {
            Packet::RlePacket(p) => p.len(),
            Packet::RawPacket(p) => p.len(),
            Packet::FullContents(p) => p.len(),
        }
    }

    /// Create a `FullContents` packet from a slice
    pub fn from_slice(data: &'a [u8]) -> Self {
        Packet::FullContents(data)
    }
}

pub fn next_rle_packet(input: &[u8], bytes_per_pixel: u8) -> IResult<&[u8], Packet> {
    alt((
        map(rle_packet(bytes_per_pixel), |p| Packet::RlePacket(p)),
        map(raw_packet(bytes_per_pixel), |p| Packet::RawPacket(p)),
    ))(input)
}

/// Parse pixel count in raw and RLE packets.
///
/// This parser expects bits as input!
fn pixel_count(input: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    map(
        take(7u8),
        // Run length is encoded as 0 = 1 pixel, 1 = 2 pixels, etc, hence this offset
        |count: u8| count + 1,
    )(input)
}
