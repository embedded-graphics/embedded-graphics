//! A small TGA parser designed for embedded, no-std environments but usable anywhere. Beyond
//! parsing the image header, no other allocations are made.
//!
//! To access the individual pixels in an image, the [`Tga`] struct implements `IntoIterator`. It is
//! also possible to access the unaltered raw image data by reading the [`pixel_data`] field. This
//! data will need to be interpreted according to the [`image_type`] specified in the header.
//!
//! # Features
//!
//! * `graphics` - enables [`embedded-graphics`] integration.
//!
//! # Examples
//!
//! ## Load a Run Length Encoded (RLE) TGA image
//!
//! ```rust
//! use tinytga::{ImageOrigin, ImageType, Pixel, Tga, TgaFooter, TgaHeader};
//!
//! // Include an image from a local path as bytes
//! let data = include_bytes!("../tests/chessboard_4px_rle.tga");
//!
//! // Create a TGA instance from a byte slice
//! let img = Tga::from_slice(data).unwrap();
//!
//! // Take a look at the header
//! assert_eq!(
//!     img.header,
//!     TgaHeader {
//!         id_len: 0,
//!         has_color_map: false,
//!         image_type: ImageType::RleTruecolor,
//!         color_map_start: 0,
//!         color_map_len: 0,
//!         color_map_depth: 0,
//!         x_origin: 0,
//!         y_origin: 4,
//!         width: 4,
//!         height: 4,
//!         pixel_depth: 24,
//!         image_origin: ImageOrigin::TopLeft,
//!         alpha_channel_depth: 0,
//!     }
//! );
//!
//! // Take a look at the footer
//! assert_eq!(
//!     img.footer,
//!     Some(TgaFooter {
//!         extension_area_offset: 0,
//!         developer_directory_offset: 0
//!     })
//! );
//!
//! // Collect pixels into a `Vec<Pixel>`
//! let pixels = img.into_iter().collect::<Vec<Pixel>>();
//! ```
//!
//! ## Use with `embedded-graphics`
//!
//! This example demonstrates [embedded-graphics] support by rendering a TGA image to a mock
//! display.
//!
//! The `graphics` feature of `tinytga` needs to be enabled in `Cargo.toml` to use the `Tga` object
//! with embedded-graphics.
//!
//! ```rust
//! # #[cfg(feature = "graphics")] { fn main() -> Result<(), core::convert::Infallible> {
//! # let mut display = embedded_graphics::mock_display::MockDisplay::default();
//! use embedded_graphics::{image::Image, pixelcolor::Rgb888, prelude::*};
//! use tinytga::EgTga;
//!
//! let tga: EgTga<Rgb888> = EgTga::from_slice(include_bytes!("../tests/rust-rle-bw-topleft.tga")).unwrap();
//!
//! let image = Image::new(&tga, Point::zero());
//!
//! image.draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(()) } }
//! ```
//!
//! [embedded-graphics]: https://docs.rs/embedded-graphics
//! [`Tga`]: ./struct.Tga.html
//! [`image_type`]: ./struct.TgaHeader.html#structfield.image_type
//! [`pixel_data`]: ./struct.Tga.html#structfield.pixel_data

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
mod pixel;

use crate::{
    footer::*,
    header::*,
    packet::{next_rle_packet, Packet},
    parse_error::ParseError,
};

pub use crate::{
    footer::TgaFooter,
    header::{ImageOrigin, ImageType, TgaHeader},
    pixel::Pixel,
};

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

        let y = if self.header.image_origin.is_bottom() {
            self.height().saturating_sub(1)
        } else {
            0
        };

        TgaIterator {
            tga: self,
            bytes_to_consume,
            current_packet,
            current_packet_position: 0,
            current_packet_pixel_length: current_packet_len / stride,
            stride,
            x: 0,
            y,
            done: false,
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
    x: u16,

    /// Current Y coordinate from top-left of image
    y: u16,

    /// Iteration is done
    done: bool,
}

impl<'a> Iterator for TgaIterator<'a> {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

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

        if self.x >= self.tga.width() {
            self.x = 0;

            if self.tga.header.image_origin.is_bottom() {
                if self.y > 0 {
                    self.y -= 1;
                } else {
                    self.done = true;
                }
            } else {
                self.y += 1;
                if self.y >= self.tga.height() {
                    self.done = true;
                }
            }
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
    use embedded_graphics::prelude::*;

    ///TODO: docs
    #[derive(Debug)]
    pub struct EgTga<'a, C> {
        tga: Tga<'a>,
        color_type: PhantomData<C>,
    }

    impl<'a, C> EgTga<'a, C> {
        ///TODO: docs
        pub fn from_slice(data: &'a [u8]) -> Result<Self, ParseError> {
            Ok(Self {
                tga: Tga::from_slice(data)?,
                color_type: PhantomData,
            })
        }
    }

    impl<C> ImageDrawable for EgTga<'_, C>
    where
        C: PixelColor + From<<C as PixelColor>::Raw>,
    {
        type Color = C;

        fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
        where
            D: DrawTarget<Color = C>,
        {
            // TGA files with the origin in the top left corner can be drawn using `fill_contiguous`.
            // All other origins are drawn by falling back to `draw_iter`.
            if self.tga.header.image_origin == ImageOrigin::TopLeft {
                target.fill_contiguous(
                    &self.bounding_box(),
                    self.tga
                        .into_iter()
                        .map(|p| C::Raw::from_u32(p.color).into()),
                )
            } else {
                target.draw_iter(self.tga.into_iter().map(|p| {
                    Pixel(
                        Point::new(i32::from(p.x), i32::from(p.y)),
                        C::Raw::from_u32(p.color).into(),
                    )
                }))
            }
        }
    }

    impl<C> OriginDimensions for EgTga<'_, C> {
        fn size(&self) -> Size {
            Size::new(self.tga.width().into(), self.tga.height().into())
        }
    }
}

#[cfg(feature = "graphics")]
pub use e_g::*;
