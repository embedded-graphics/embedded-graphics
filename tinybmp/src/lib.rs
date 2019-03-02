//! Small BMP format image parser supporting no-std environments. Specifically designed to work with
//! [embedded-graphics](https://crates.io/crates/embedded-graphics)

#![deny(missing_docs)]

mod header;

use header::parse_header;
pub use header::{FileType, Header};

/// A BMP-format bitmap
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
    pub fn from_bytes(bytes: &'a [u8]) -> Result<Self, ()> {
        let (_remaining, header) = parse_header(bytes).map_err(|_| ())?;

        let image_data = &bytes[header.image_data_start..];

        Ok(Bmp { header, image_data })
    }
}
