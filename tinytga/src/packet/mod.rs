mod raw_packet;
mod rle_packet;

pub use self::{raw_packet::raw_packet, rle_packet::rle_packet};
use nom::{bits::complete::take, branch::alt, combinator::map, IResult};

/// A Run Length Encoded (RLE) packet
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Packet<'a> {
    /// Data in this packet is Run Length Encoded
    RlePacket {
        /// Number of pixels in this run
        run_length: u8,

        /// Pixel data
        pixel_data: &'a [u8],
    },

    /// Data is in a packet, but is not compressed at all
    RawPacket {
        /// Number of pixels of this packet
        num_pixels: u8,

        /// Pixel data in this packet, up to 32 bits (4 bytes) per pixel
        pixel_data: &'a [u8],
    },

    /// File is not packeted. Contains reference to all image data in the file
    FullContents(&'a [u8]),
}

impl<'a> Packet<'a> {
    /// Get the number of pixels in this packet
    pub fn len(&self) -> usize {
        match self {
            Self::RlePacket {
                pixel_data,
                run_length,
            } => pixel_data.len() * *run_length as usize,
            Self::RawPacket { pixel_data, .. } => pixel_data.len(),
            Self::FullContents(data) => data.len(),
        }
    }

    /// Create a `FullContents` packet from a slice
    pub fn from_slice(data: &'a [u8]) -> Self {
        Packet::FullContents(data)
    }
}

pub fn next_packet(input: &[u8], bytes_per_pixel: u8) -> IResult<&[u8], Packet> {
    alt((rle_packet(bytes_per_pixel), raw_packet(bytes_per_pixel)))(input)
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
