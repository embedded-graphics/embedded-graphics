//! Bitmap header
//!
//! Information gleaned from [wikipedia](https://en.wikipedia.org/wiki/BMP_file_format) and
//! [this website](http://paulbourke.net/dataformats/bmp/)

use embedded_graphics::prelude::*;
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
    /// Total file size in bytes.
    pub file_size: u32,

    /// Byte offset from beginning of file at which pixel data begins.
    pub image_data_start: usize,

    /// Image size in pixels.
    pub image_size: Size,

    /// Number of bits per pixel.
    pub bpp: Bpp,

    /// Length in bytes of the image data.
    pub image_data_len: u32,

    /// Bit masks for the color channels.
    pub channel_masks: Option<ChannelMasks>,
}

impl Header {
    pub(crate) fn parse(input: &[u8]) -> IResult<&[u8], Header> {
        let (input, _) = tag("BM")(input)?;
        let (input, file_size) = le_u32(input)?;
        let (input, _reserved_1) = le_u16(input)?;
        let (input, _reserved_2) = le_u16(input)?;
        let (input, image_data_start) = le_u32(input)?;
        let (input, header_size) = le_u32(input)?;
        let (input, image_width) = le_u32(input)?;
        let (input, image_height) = le_u32(input)?;
        let (input, _color_planes) = le_u16(input)?;
        let (input, bpp) = Bpp::parse(input)?;
        let (input, compression_method) = CompressionMethod::parse(input)?;
        let (input, image_data_len) = le_u32(input)?;

        let (input, channel_masks) = if compression_method == CompressionMethod::Bitfields {
            // BMP header versions can be distinguished by the header length.
            // The color bit masks are only included in headers with at least version 3.
            if header_size >= 56 {
                let (input, _pels_per_meter_x) = le_u32(input)?;
                let (input, _pels_per_meter_y) = le_u32(input)?;
                let (input, _clr_used) = le_u32(input)?;
                let (input, _clr_important) = le_u32(input)?;
                let (input, mask_red) = le_u32(input)?;
                let (input, mask_green) = le_u32(input)?;
                let (input, mask_blue) = le_u32(input)?;
                let (input, mask_alpha) = le_u32(input)?;

                (
                    input,
                    Some(ChannelMasks {
                        red: mask_red,
                        green: mask_green,
                        blue: mask_blue,
                        alpha: mask_alpha,
                    }),
                )
            } else {
                // TODO: this should probably be an error
                (input, None)
            }
        } else {
            (input, None)
        };

        Ok((
            input,
            Header {
                file_size,
                image_data_start: image_data_start as usize,
                image_size: Size::new(image_width, image_height),
                image_data_len,
                bpp,
                channel_masks,
            },
        ))
    }
}

/// Masks for the color channels.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ChannelMasks {
    /// Red channel mask.
    pub red: u32,
    /// Green channel mask.
    pub green: u32,
    /// Blue channel mask.
    pub blue: u32,
    /// Alpha channel mask.
    pub alpha: u32,
}

impl ChannelMasks {
    /// Rgb555 color masks.
    pub const RGB555: Self = Self {
        red: 0b11111_00000_00000,
        green: 0b00000_11111_00000,
        blue: 0b00000_00000_11111,
        alpha: 0,
    };

    /// Rgb565 color masks.
    pub const RGB565: Self = Self {
        red: 0b11111_000000_00000,
        green: 0b00000_111111_00000,
        blue: 0b00000_000000_11111,
        alpha: 0,
    };

    /// Rgb888 color masks.
    pub const RGB888: Self = Self {
        red: 0xFF0000,
        green: 0x00FF00,
        blue: 0x0000FF,
        alpha: 0,
    };
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum CompressionMethod {
    Rgb,
    Bitfields,
}

impl CompressionMethod {
    fn new(value: u32) -> Option<Self> {
        Some(match value {
            0 => Self::Rgb,
            3 => Self::Bitfields,
            _ => return None,
        })
    }

    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map_opt(le_u32, Self::new)(input)
    }
}
