use nom::*;

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

named!(pub footer<&[u8], TgaFooter>,
    do_parse!(
        extension_area_offset: le_u32 >>
        developer_directory_offset: le_u32 >>
        tag!("TRUEVISION-XFILE.") >>
        ({
            TgaFooter {
                extension_area_offset,
                developer_directory_offset
            }
        })
    )
);
