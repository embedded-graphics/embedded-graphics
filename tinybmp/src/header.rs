//! Bitmap header
//!
//! Information gleaned from [wikipedia](https://en.wikipedia.org/wiki/BMP_file_format) and
//! [this website](http://paulbourke.net/dataformats/bmp/)

use nom::*;

/// Bitmap file type
#[derive(Debug, PartialEq)]
pub enum FileType {
    /// Default "BM" magic bytes marker for most commonly encountered bitmaps
    BM,
}

/// BMP header information
#[derive(Debug, PartialEq)]
pub struct Header {
    /// Bitmap file type
    pub file_type: FileType,

    /// Total file size in bytes
    pub file_size: u32,

    /// Reserved field 1
    pub reserved_1: u16,

    /// Reserved field 2
    pub reserved_2: u16,

    /// Byte offset from beginning of file at which pixel data begins
    pub image_data_start: usize,
}

named!(pub(crate) parse_header<&[u8], Header>,
    do_parse!(
        tag!("BM") >>
        file_size: le_u32 >>
        reserved_1: le_u16 >>
        reserved_2: le_u16 >>
        image_data_start: le_u32 >>
        (Header{
            file_type: FileType::BM,
            file_size,
            reserved_1,
            reserved_2,
            image_data_start: image_data_start as usize
        })
    )
);
