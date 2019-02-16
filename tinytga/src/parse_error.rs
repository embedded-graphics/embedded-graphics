/// Possible parse errors
#[derive(Debug, Copy, Clone)]
pub enum ParseError {
    /// An invalid color map value was encountered. Valid values are `0` (no color map) or `1`
    /// (color map included)
    InvalidColorMap(u8),

    /// An invalid image type was encountered. Valid values are presented in [`ImageType`]
    InvalidImageType(u8),

    /// Parse was incomplete. Holds the remaining number of bytes
    Incomplete(usize),

    /// An error occurred when parsing the TGA header
    Header,

    /// An error occurred when parsing the TGA footer
    Footer,

    /// Any other type of parse error
    Other,
}
