use crate::header::{Bpp, ImageType};

/// Possible parse errors
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[non_exhaustive]
pub enum ParseError {
    /// An error occurred when parsing the color map.
    ColorMap,

    /// An error occurred when parsing the TGA header.
    Header,

    /// An error occurred when parsing the TGA footer.
    Footer,

    /// An unsupported image type value was encountered.
    UnsupportedImageType(u8),

    /// An unsupported bits per pixel value was encountered.
    UnsupportedBpp(u8),

    /// Mismatched bits per pixel.
    ///
    /// The bit depth of the image doesn't match the depth that was specified
    /// when `Tga::from_slice` was called.
    ///
    /// [`Tga::from_slice`]: struct.Tga.html#method.from_slice
    MismatchedBpp(u8),

    /// The image type and bits per pixel combination isn't supported by `DynamicTga`.
    UnsupportedDynamicTgaType(ImageType, Bpp),
}
