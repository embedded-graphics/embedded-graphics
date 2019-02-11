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

/// Image type
#[derive(Debug, Copy, Clone)]
pub enum ImageType {
    /// Image contains no pixel data
    Empty = 0,

    /// Color mapped image
    ColorMapped = 1,

    /// Truecolor image
    Truecolor = 2,

    /// Monochrome (greyscale) image
    Monochrome = 3,

    /// Run length encoded color mapped image
    RleColorMapped = 9,

    /// Run length encoded truecolor image
    RleTruecolor = 10,

    /// Run length encoded monochrome (greyscale) image
    RleMonochrome = 11,
}

/// Color map type
#[derive(Debug, Copy, Clone)]
pub enum ColorMapType {
    /// Each pixel is represented by an index into a color pallette
    Pseudocolor,

    /// Similar to pseudocolor, but each RGB component has its own index into a color pallette
    Directcolor,

    /// Store pixel information in the image, no pallette is used
    Truecolor,
}

/// TGA header structure, referenced from <https://www.fileformat.info/format/tga/egff.htm>
#[derive(Debug, Copy, Clone)]
pub struct TgaHeader {
    /// Image ID field length
    id_len: u8,

    /// Color map type
    color_map_type: ColorMapType,

    /// Image type
    image_type: ImageType,

    /// Color map origin
    color_map_start: u16,

    /// Length of color map
    color_map_len: u16,

    /// Color map entry depth
    color_map_depth: u8,

    /// Image origin (X)
    x_origin: u16,

    /// Image origin (Y)
    y_origin: u16,

    /// Image width in pixels
    width: u16,

    /// Image heigh in pixels
    height: u16,

    /// Pixel bit depth
    pixel_depth: u8,

    /// Image descriptor (unused)
    image_descriptor: u8,
}
