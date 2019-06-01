mod raw_packet;
mod rle_packet;

pub use self::raw_packet::{raw_packet, RawPacket};
pub use self::rle_packet::{rle_packet, RlePacket};
use nom::*;

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
    /// Get the number of pixels in this packet
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

named_args!(pub next_rle_packet(bytes_per_pixel: u8)<&[u8], Packet>,
    alt_complete!(
        map!(call!(rle_packet, bytes_per_pixel), |p| Packet::RlePacket(p)) |
        map!(call!(raw_packet, bytes_per_pixel), |p| Packet::RawPacket(p))
    )
);
