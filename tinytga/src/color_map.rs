use crate::{parse_error::ParseError, Bpp, TgaHeader};
use nom::bytes::complete::take;

/// Color map.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ColorMap<'a> {
    /// First color index.
    start_index: u16,
    /// Number of entries.
    length: u16,
    /// Entry bit depth.
    entry_bpp: Bpp,
    /// Color map data.
    data: &'a [u8],
}

impl<'a> ColorMap<'a> {
    pub(crate) fn parse(
        input: &'a [u8],
        header: &TgaHeader,
    ) -> Result<(&'a [u8], Option<Self>), ParseError> {
        if !header.has_color_map {
            return Ok((input, None));
        }

        let entry_bpp = header.color_map_depth.ok_or(ParseError::ColorMap)?;

        let length = usize::from(header.color_map_len) * usize::from(entry_bpp.bytes());

        let (input, color_map_data) =
            take(length)(input).map_err(|_: nom::Err<()>| ParseError::ColorMap)?;

        Ok((
            input,
            Some(Self {
                start_index: header.color_map_start,
                length: header.color_map_len,
                entry_bpp,
                data: color_map_data,
            }),
        ))
    }

    /// Returns the bit depth for the entries in the color map.
    pub fn entry_bpp(&self) -> Bpp {
        self.entry_bpp
    }

    /// Returns the raw color value for a color map entry.
    pub fn get_raw(&self, index: usize) -> Option<u32> {
        //TODO: use start_index
        if index >= usize::from(self.length) {
            return None;
        }

        let start = index * usize::from(self.entry_bpp.bytes());

        Some(match self.entry_bpp {
            Bpp::Bits8 => self.data[start] as u32,
            Bpp::Bits16 => u32::from_le_bytes([self.data[start], self.data[start + 1], 0, 0]),
            Bpp::Bits24 => u32::from_le_bytes([
                self.data[start],
                self.data[start + 1],
                self.data[start + 2],
                0,
            ]),
            Bpp::Bits32 => u32::from_le_bytes([
                self.data[start],
                self.data[start + 1],
                self.data[start + 2],
                self.data[start + 3],
            ]),
        })
    }
}
