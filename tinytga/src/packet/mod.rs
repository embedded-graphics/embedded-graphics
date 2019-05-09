mod raw_packet;
mod rle_packet;

pub use self::raw_packet::{raw_packet, RawPacket};
pub use self::rle_packet::{rle_packet, RlePacket};
use nom::*;

#[derive(Debug)]
pub enum Packet<'a> {
    RlePacket(RlePacket<'a>),
    RawPacket(RawPacket<'a>),
}

named_args!(pub any_packet(bytes_per_pixel: u8)<&[u8], Packet>,
    alt!(
        map!(call!(rle_packet, bytes_per_pixel), |p| Packet::RlePacket(p)) |
        map!(call!(raw_packet, bytes_per_pixel), |p| Packet::RawPacket(p))
    )
);
