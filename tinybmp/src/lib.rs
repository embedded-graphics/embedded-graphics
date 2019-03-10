//! Small BMP format image parser supporting no-std environments. Specifically designed to work with
//! [embedded-graphics](https://crates.io/crates/embedded-graphics)

#![no_std]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

mod header;

use crate::header::parse_header;
pub use crate::header::{FileType, Header};

/// A BMP-format bitmap
#[derive(Debug, Clone, PartialEq)]
pub struct Bmp<'a> {
    /// Image header
    pub header: Header,

    image_data: &'a [u8],
}

impl<'a> Bmp<'a> {
    /// Create a bitmap object from a byte array
    ///
    /// This method keeps a slice of the original input and does not dynamically allocate memory.
    /// The input data must live for as long as this BMP instance does.
    pub fn from_slice(bytes: &'a [u8]) -> Result<Self, ()> {
        let (_remaining, header) = parse_header(bytes).map_err(|_| ())?;

        let image_data = &bytes[header.image_data_start..];

        Ok(Bmp { header, image_data })
    }

    /// Get a reference to the range of bytes that represents the pixel data in the image
    pub fn image_data(&'a self) -> &'a [u8] {
        self.image_data
    }

    /// Get the image width in pixels
    pub fn width(&self) -> u32 {
        self.header.image_width
    }

    /// Get the image height in pixels
    pub fn height(&self) -> u32 {
        self.header.image_height
    }

    /// Get image dimensions as `(width, height)` in pixels
    pub fn dimensions(&self) -> (u32, u32) {
        (self.header.image_width, self.header.image_height)
    }

    /// Get the BPP (bits per pixel) for this image
    // TODO: Should this return an enum?
    pub fn bpp(&self) -> u32 {
        self.header.bpp as u32
    }
}
