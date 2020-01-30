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

mod check_readme;
mod footer;
mod header;
mod packet;
mod parse_error;
mod pixel;

use crate::footer::*;
use crate::header::*;
use crate::packet::{next_rle_packet, Packet};
use crate::parse_error::ParseError;

pub use crate::footer::TgaFooter;
pub use crate::header::{ImageType, TgaHeader};
pub use crate::pixel::Pixel;

/// TGA image
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Tga<'a> {
    /// TGA header
    pub header: TgaHeader,

    /// TGA footer (last 26 bytes of file)
    pub footer: Option<TgaFooter>,

    /// Color map
    pub color_map: Option<&'a [u8]>,

    /// Image pixel data
    pub pixel_data: &'a [u8],
}

impl<'a> Tga<'a> {
    /// Parse a TGA image from a byte slice
    pub fn from_slice(bytes: &'a [u8]) -> Result<Self, ParseError> {
        let (_remaining, header) = header(bytes).map_err(|_| ParseError::Header)?;

        // Read last 26 bytes as TGA footer
        let footer = footer(&bytes[bytes.len() - FOOTER_LEN..])
            .map(|(_remaining, footer)| footer)
            .ok();

        let header_len = HEADER_LEN + header.id_len as usize;

        let color_map = if header.has_color_map {
            let len =
                usize::from(header.color_map_len) * (usize::from(header.color_map_depth + 7) / 8);

            Some(&bytes[header_len..header_len + len])
        } else {
            None
        };

        let image_data_start = header_len + color_map.map_or(0, |map| map.len());

        let image_data_end = if let Some(footer) = footer {
            [
                footer.extension_area_offset as usize,
                footer.developer_directory_offset as usize,
            ]
            .iter()
            .filter_map(|v| if *v > 0 { Some(*v) } else { None })
            .min()
            .unwrap_or(bytes.len() - FOOTER_LEN)
        } else {
            bytes.len()
        };

        let pixel_data = &bytes[image_data_start..image_data_end];

        Ok(Self {
            header,
            footer,
            color_map,
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
    pub fn image_data(&self) -> &[u8] {
        self.pixel_data
    }
}

impl<'a> IntoIterator for &'a Tga<'a> {
    type Item = Pixel;
    type IntoIter = TgaIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let (bytes_to_consume, current_packet) = match self.header.image_type {
            ImageType::Monochrome | ImageType::Truecolor | ImageType::ColorMapped => {
                let data = Packet::from_slice(self.image_data());

                (Some(self.image_data()), data)
            }
            ImageType::RleMonochrome | ImageType::RleTruecolor | ImageType::RleColorMapped => {
                next_rle_packet(self.image_data(), self.bpp() / 8)
                    .map(|(remaining, packet)| (Some(remaining), packet))
                    .expect("Failed to parse first image RLE data packet")
            }
            image_type => panic!("Image type {:?} not supported", image_type),
        };

        // Explicit match to prevent integer division rounding errors
        let stride = match self.bpp() {
            8 => 1,
            16 => 2,
            24 => 3,
            32 => 4,
            depth => panic!("Bit depth {} not supported", depth),
        };

        let current_packet_len = current_packet.len();

        TgaIterator {
            tga: self,
            bytes_to_consume,
            current_packet,
            current_packet_position: 0,
            current_packet_pixel_length: current_packet_len / stride,
            stride,
            x: 0,
            y: 0,
        }
    }
}

/// Iterator over individual TGA pixels
///
/// This can be used to build a raw image buffer to pass around
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct TgaIterator<'a> {
    /// Reference to original TGA image
    tga: &'a Tga<'a>,

    /// Remaining bytes (after current packet) to consume
    bytes_to_consume: Option<&'a [u8]>,

    /// Reference to current packet definition (either RLE or raw)
    current_packet: Packet<'a>,

    /// Current position within the current packet's pixel run
    current_packet_position: usize,

    /// Current packet length in pixels
    current_packet_pixel_length: usize,

    /// Number of bytes contained within each pixel
    stride: usize,

    /// Current X coordinate from top-left of image
    x: u32,

    /// Current Y coordinate from top-left of image
    y: u32,
}

impl<'a> Iterator for TgaIterator<'a> {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_packet_position >= self.current_packet_pixel_length {
            // Parse next packet from remaining bytes
            match self.tga.header.image_type {
                ImageType::Monochrome | ImageType::Truecolor | ImageType::ColorMapped => {
                    return None;
                }
                ImageType::RleMonochrome | ImageType::RleTruecolor | ImageType::RleColorMapped => {
                    if self.bytes_to_consume.is_none() {
                        return None;
                    } else {
                        self.current_packet_position = 0;
                    }

                    let (bytes_to_consume, current_packet) =
                        next_rle_packet(self.bytes_to_consume.unwrap(), self.tga.bpp() / 8)
                            .map(|(remaining, packet)| {
                                (
                                    if !remaining.is_empty() {
                                        Some(remaining)
                                    } else {
                                        None
                                    },
                                    packet,
                                )
                            })
                            .expect("Failed to parse first image RLE data packet");

                    self.bytes_to_consume = bytes_to_consume;
                    self.current_packet_pixel_length = current_packet.len() / self.stride;
                    self.current_packet = current_packet;
                }
                image_type => panic!("Image type {:?} not supported", image_type),
            };
            // }
        }

        let (start, px): (usize, &[u8]) = match self.current_packet {
            // RLE packets use the same 4 bytes for the color of every pixel in the packet, so
            // there is no start offet like `RawPacket`s have
            Packet::RlePacket(ref p) => (0, p.pixel_data),
            // Raw packets need to look within the byte array to find the correct bytes to
            // convert to a pixel value, hence the calculation of `start = position * stride`
            Packet::RawPacket(ref p) => {
                let px = p.pixel_data;
                let start = self.current_packet_position * self.stride;

                (start, px)
            }
            // Uncompressed data just walks along the byte array in steps of `self.stride`
            Packet::FullContents(px) => {
                let start = self.current_packet_position * self.stride;

                (start, px)
            }
        };

        let mut pixel_value = {
            let out = match self.stride {
                1 => u32::from(px[start]),
                2 => u32::from_le_bytes([px[start], px[start + 1], 0, 0]),
                3 => u32::from_le_bytes([px[start], px[start + 1], px[start + 2], 0]),
                4 => u32::from_le_bytes([px[start], px[start + 1], px[start + 2], px[start + 3]]),
                depth => unreachable!("Depth {} is not supported", depth),
            };

            self.current_packet_position += 1;

            out
        };

        if let Some(color_map) = self.tga.color_map {
            let entry_size = usize::from(self.tga.header.color_map_depth + 7) / 8;
            let start = pixel_value as usize * entry_size;

            pixel_value = match entry_size {
                1 => color_map[start] as u32,
                2 => u32::from_le_bytes([color_map[start], color_map[start + 1], 0, 0]),
                3 => u32::from_le_bytes([
                    color_map[start],
                    color_map[start + 1],
                    color_map[start + 2],
                    0,
                ]),
                4 => u32::from_le_bytes([
                    color_map[start],
                    color_map[start + 1],
                    color_map[start + 2],
                    color_map[start + 3],
                ]),
                depth => unreachable!("Depth {} is not supported", depth),
            };
        }

        let x = self.x;
        let y = self.y;

        self.x += 1;

        if self.x >= self.tga.width().into() {
            self.x = 0;
            self.y += 1;
        }

        Some(Pixel {
            x,
            y,
            color: pixel_value,
        })
    }
}

#[cfg(feature = "graphics")]
mod e_g {
    use super::*;
    use core::marker::PhantomData;
    use embedded_graphics::{
        drawable::Pixel as EgPixel,
        geometry::Point,
        image::ImageData,
        pixelcolor::{raw::RawData, PixelColor},
    };

    /// A thin wrapper over [`TgaIterator`] to support [`embedded-graphics`] integration
    ///
    /// [`TgaIterator`]: ./struct.TgaIterator.html
    /// [`embedded-graphics`]: https://docs.rs/embedded-graphics
    #[derive(Debug)]
    pub struct EgPixelIterator<'a, C> {
        it: TgaIterator<'a>,
        c: PhantomData<C>,
    }

    impl<'a, C> Iterator for EgPixelIterator<'a, C>
    where
        C: PixelColor + From<<C as PixelColor>::Raw>,
    {
        type Item = EgPixel<C>;

        fn next(&mut self) -> Option<Self::Item> {
            self.it.next().map(|p| {
                let raw = C::Raw::from_u32(p.color);
                EgPixel(Point::new(p.x as i32, p.y as i32), raw.into())
            })
        }
    }

    /// TODO: Docs
    impl<'a, C> ImageData<C> for &'a Tga<'a>
    where
        C: PixelColor + From<<C as PixelColor>::Raw>,
    {
        type PixelIterator = EgPixelIterator<'a, C>;

        fn width(&self) -> u32 {
            self.header.width.into()
        }

        fn height(&self) -> u32 {
            self.header.height.into()
        }

        fn pixel_iter(&self) -> Self::PixelIterator {
            EgPixelIterator {
                it: self.into_iter(),
                c: PhantomData,
            }
        }
    }
}

#[cfg(feature = "graphics")]
pub use e_g::*;
