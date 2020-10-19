//! Bitmap header
//!
//! Information gleaned from [wikipedia](https://en.wikipedia.org/wiki/BMP_file_format) and
//! [this website](http://paulbourke.net/dataformats/bmp/)

use nom::{
    bytes::complete::tag,
    combinator::map_opt,
    number::complete::{le_u16, le_u32},
    IResult,
};

/// Bits per pixel.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[non_exhaustive]
pub enum Bpp {
    /// 1 bit per pixel.
    Bits1,
    /// 8 bits per pixel.
    Bits8,
    /// 16 bits per pixel.
    Bits16,
    /// 24 bits per pixel.
    Bits24,
    /// 32 bits per pixel.
    Bits32,
}

impl Bpp {
    fn new(value: u16) -> Option<Self> {
        Some(match value {
            1 => Self::Bits1,
            8 => Self::Bits8,
            16 => Self::Bits16,
            24 => Self::Bits24,
            32 => Self::Bits32,
            _ => return None,
        })
    }

    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map_opt(le_u16, Bpp::new)(input)
    }

    /// Returns the number of bits.
    pub fn bits(self) -> u16 {
        match self {
            Self::Bits1 => 1,
            Self::Bits8 => 8,
            Self::Bits16 => 16,
            Self::Bits24 => 24,
            Self::Bits32 => 32,
        }
    }
}

/// BMP header information
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Header {
    /// Total file size in bytes
    pub file_size: u32,

    /// Byte offset from beginning of file at which pixel data begins
    pub image_data_start: usize,

    /// Image width in pixels
    pub image_width: u32,

    /// Image height in pixels
    pub image_height: u32,

    /// Number of bits per pixel
    pub bpp: Bpp,

    /// Length in bytes of the image data
    pub image_data_len: u32,
}

pub fn parse_header(input: &[u8]) -> IResult<&[u8], Header> {
    let (input, _) = tag("BM")(input)?;
    let (input, file_size) = le_u32(input)?;
    let (input, _reserved_1) = le_u16(input)?;
    let (input, _reserved_2) = le_u16(input)?;
    let (input, image_data_start) = le_u32(input)?;
    let (input, _header_size) = le_u32(input)?;
    let (input, image_width) = le_u32(input)?;
    let (input, image_height) = le_u32(input)?;
    let (input, _color_planes) = le_u16(input)?;
    let (input, bpp) = Bpp::parse(input)?;
    let (input, _compression_method) = le_u32(input)?;
    let (input, image_data_len) = le_u32(input)?;

    Ok((
        input,
        Header {
            file_size,
            image_data_start: image_data_start as usize,
            image_width,
            image_height,
            image_data_len,
            bpp,
        },
    ))
}
