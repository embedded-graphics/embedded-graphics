//! Bitmap header
//!
//! Information gleaned from [wikipedia](https://en.wikipedia.org/wiki/BMP_file_format) and
//! [this website](http://paulbourke.net/dataformats/bmp/)

use nom::{
    bytes::complete::tag,
    number::complete::{le_u16, le_u32},
    IResult,
};

/// Bitmap file type
#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    /// Default "BM" magic bytes marker for most commonly encountered bitmaps
    BM,
}

/// BMP header information
#[derive(Debug, Clone, PartialEq)]
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

    /// Image width in pixels
    pub image_width: u32,

    /// Image height in pixels
    pub image_height: u32,

    /// Number of bits per pixel
    pub bpp: u16,

    /// Length in bytes of the image data
    pub image_data_len: u32,
}

pub fn parse_header(input: &[u8]) -> IResult<&[u8], Header> {
    let (input, _) = tag("BM")(input)?;
    let (input, file_size) = le_u32(input)?;
    let (input, reserved_1) = le_u16(input)?;
    let (input, reserved_2) = le_u16(input)?;
    let (input, image_data_start) = le_u32(input)?;
    let (input, _header_size) = le_u32(input)?;
    let (input, image_width) = le_u32(input)?;
    let (input, image_height) = le_u32(input)?;
    let (input, _color_planes) = le_u16(input)?;
    let (input, bpp) = le_u16(input)?;
    let (input, _compression_method) = le_u32(input)?;
    let (input, image_data_len) = le_u32(input)?;

    Ok((
        input,
        Header {
            file_type: FileType::BM,
            file_size,
            reserved_1,
            reserved_2,
            image_data_start: image_data_start as usize,
            image_width,
            image_height,
            image_data_len,
            bpp,
        },
    ))
}
