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

mod header;

use crate::header::*;

/// TGA image
#[derive(Debug, Copy, Clone)]
pub struct Tga {
    header: TgaHeader,
}

impl Tga {
    /// Parse a TGA image from a byte slice
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ParseError> {
        let (_remaining, header) = header(bytes).map_err(|_| ParseError::Other)?;

        // if remaining.len() > 0 {
        //     Err(ParseError::Incomplete(remaining.len()))
        // } else {
        //     Ok(Self { header })
        // }

        Ok(Self { header })
    }
}
