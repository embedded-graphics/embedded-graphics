use nom::{bytes::complete::tag, number::complete::le_u32, IResult};

/// TGA footer length in bytes
pub const FOOTER_LEN: usize = 26;

/// TGA footer structure, referenced from <http://tfc.duke.free.fr/coding/tga_specs.pdf>
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TgaFooter {
    /// Extension area byte offset from beginning of file
    pub extension_area_offset: u32,

    /// Developer directory area byte offset from beginning of file
    pub developer_directory_offset: u32,
}

pub fn footer(input: &[u8]) -> IResult<&[u8], TgaFooter> {
    let (input, extension_area_offset) = le_u32(input)?;
    let (input, developer_directory_offset) = le_u32(input)?;
    let (input, _) = tag("TRUEVISION-XFILE.")(input)?;

    Ok((
        input,
        TgaFooter {
            extension_area_offset,
            developer_directory_offset,
        },
    ))
}
