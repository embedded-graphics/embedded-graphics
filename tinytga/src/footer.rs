use core::num::NonZeroUsize;
use nom::{bytes::complete::tag, combinator::map, number::complete::le_u32, IResult, Needed};

/// TGA footer length in bytes
const TGA_FOOTER_LENGTH: usize = 26;

/// TGA footer structure, referenced from <http://tfc.duke.free.fr/coding/tga_specs.pdf>
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub(crate) struct TgaFooter {
    /// Footer start offset
    footer_start: usize,

    /// Extension area offset
    extension_area_offset: Option<NonZeroUsize>,

    /// Developer directory
    developer_directory_offset: Option<NonZeroUsize>,
}

impl TgaFooter {
    /// Parses the TGA footer.
    ///
    /// Returns `None` if the file doesn't contain a valid footer.
    pub fn parse(image_data: &[u8]) -> Option<Self> {
        parse_footer(image_data).ok().map(|(_, footer)| footer)
    }

    /// Returns the length of the footer section of the TGA file.
    ///
    /// The length includes the footer, extension area and developer directory.
    pub fn length(&self, image_data: &[u8]) -> usize {
        let mut length = TGA_FOOTER_LENGTH;

        if let Some(offset) = self.extension_area_offset {
            length = length.max(image_data.len() - offset.get());
        }

        if let Some(offset) = self.developer_directory_offset {
            length = length.max(image_data.len() - offset.get());
        }

        length
    }

    /// Returns the extension area.
    ///
    /// Returns `None` if the file doesn't contain an extension area.
    pub fn extension_area<'a>(&self, image_data: &'a [u8]) -> Option<&'a [u8]> {
        self.extension_area_offset
            .map(NonZeroUsize::get)
            .and_then(|start| {
                let end = self
                    .developer_directory_offset
                    .map(NonZeroUsize::get)
                    .filter(|offset| *offset > start)
                    .unwrap_or(self.footer_start);

                image_data.get(start..end)
            })
    }

    /// Returns the developer directory.
    ///
    /// Returns `None` if the file doesn't contain a developer directory.
    pub fn developer_directory<'a>(&self, image_data: &'a [u8]) -> Option<&'a [u8]> {
        self.developer_directory_offset
            .map(NonZeroUsize::get)
            .and_then(|start| {
                let end = self
                    .extension_area_offset
                    .map(NonZeroUsize::get)
                    .filter(|offset| *offset > start)
                    .unwrap_or(self.footer_start);

                image_data.get(start..end)
            })
    }
}

fn offset(input: &[u8]) -> IResult<&[u8], Option<NonZeroUsize>> {
    map(le_u32, |offset| NonZeroUsize::new(offset as usize))(input)
}

fn parse_footer<'a>(input: &'a [u8]) -> IResult<&[u8], TgaFooter> {
    let footer_start = input
        .len()
        .checked_sub(TGA_FOOTER_LENGTH)
        .ok_or(nom::Err::Incomplete(Needed::Size(TGA_FOOTER_LENGTH)))?;
    let input = &input[footer_start..input.len()];

    let (input, extension_area_offset) = offset(input)?;
    let (input, developer_directory_offset) = offset(input)?;
    let (input, _) = tag("TRUEVISION-XFILE.\0")(input)?;

    Ok((
        input,
        TgaFooter {
            footer_start,
            extension_area_offset,
            developer_directory_offset,
        },
    ))
}
