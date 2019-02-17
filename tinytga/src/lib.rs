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
mod parse_error;
mod raw_packet;
mod rle_packet;

use crate::footer::*;
use crate::header::*;
use crate::parse_error::ParseError;
use crate::raw_packet::raw_packet;
use crate::rle_packet::rle_packet;

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
}
