//! No-std compatible TGA parser designed for embedded systems, but usable anywhere

#![no_std]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unsafe_code)]
#![deny(unstable_features)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]

mod footer;
mod header;
mod packet;
mod parse_error;

use crate::footer::*;
use crate::header::*;
use crate::packet::{any_packet, Packet, RawPacket, RlePacket};
use crate::parse_error::ParseError;

pub use crate::footer::TgaFooter;
pub use crate::header::{ImageType, TgaHeader};

/// TGA image
#[derive(Debug, Copy, Clone)]
pub struct Tga<'a> {
    /// TGA header
    pub header: TgaHeader,

    /// TGA footer (last 26 bytes of file)
    pub footer: TgaFooter,

    /// Image pixel data
    pub pixel_data: &'a [u8],
}

impl<'a> Tga<'a> {
    /// Parse a TGA image from a byte slice
    pub fn from_bytes(bytes: &'a [u8]) -> Result<Self, ParseError> {
        let (after_header, header) = header(bytes).map_err(|_| ParseError::Header)?;

        // Read last 26 bytes as TGA footer
        let (_remaining, footer) =
            footer(&bytes[(bytes.len() - FOOTER_LEN)..]).map_err(|_| ParseError::Footer)?;

        let header_len = bytes.len() - after_header.len();

        // TODO: Support color maps with by color map size with
        // (header.color_map_len * header.color_map_entry_size)
        let image_data_start = header_len + header.id_len as usize;

        let image_data_end = match footer
            .extension_area_offset
            .min(footer.developer_directory_offset) as usize
        {
            0 => bytes.len() - FOOTER_LEN,
            non_empty => non_empty,
        };

        let pixel_data = &bytes[image_data_start..image_data_end];

        Ok(Self {
            header,
            footer,
            pixel_data,
        })
    }

    /// Get the bit depth (BPP) of this image
    pub fn bpp(&self) -> u8 {
        self.header.pixel_depth
    }

    /// Get the image width in pixels
    pub fn width(&self) -> u16 {
        self.header.width
    }

    /// Get the image height in pixels
    pub fn height(&self) -> u16 {
        self.header.height
    }

    /// Get the raw image data contained in this image
    ///
    /// TGA images are encoded as packets, either [`RawPacket`]s or [`RlePacket`]s
    pub fn image_data(&self) -> &[u8] {
        self.pixel_data
    }
}

impl<'a> IntoIterator for &'a Tga<'a> {
    type Item = u32;
    type IntoIter = TgaIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let (bytes_to_consume, current_packet) = any_packet(self.image_data(), self.bpp() / 8)
            .expect("Failed to parse first image data packet");

        TgaIterator {
            x: 0,
            y: 0,
            tga: self,
            bytes_to_consume,
            current_packet,
            current_packet_position: 0,
        }
    }
}

/// Iterator over individual TGA pixels
#[derive(Debug)]
pub struct TgaIterator<'a> {
    /// Current pixel X coordinate
    x: u32,

    /// Current pixel Y coordinate
    y: u32,

    /// Reference to original TGA image
    tga: &'a Tga<'a>,

    /// Remaining bytes (after current packet) to consume
    bytes_to_consume: &'a [u8],

    /// Reference to current packet definition (either RLE or raw)
    current_packet: Packet<'a>,

    /// Current position within the current packet's pixel run
    current_packet_position: u8,
}

impl<'a> Iterator for TgaIterator<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
