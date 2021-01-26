use anyhow::{anyhow, bail, ensure, Result};
use bdf_parser::{BdfFont, Glyph};
use std::str::FromStr;

// TODO: move to bdf-parser crate
fn get_glyph(font: &BdfFont, c: char) -> Option<&Glyph> {
    font.glyphs.iter().find(|glyph| glyph.encoding == Some(c))
}

pub fn bdf_to_bitmap(input: &str) -> Result<Bitmap> {
    let font = BdfFont::from_str(&input).map_err(|_| anyhow!("couldn't parse BDF file"))?;

    let glyph_bb = get_glyph(&font, 'A')
        .ok_or_else(|| anyhow!("font doesn't contain 'A' glyph"))?
        .bounding_box;
    // TODO: get baseline from offset
    //assert_eq!(glyph_bb.offset, (0, 0));
    let glyph_width = glyph_bb.size.0 as usize;
    let glyph_width_bytes = (glyph_width + 7) / 8;
    let glyph_height = glyph_bb.size.1 as usize;

    let bitmap_width = ((glyph_width * 16 + 7) / 8) * 8;
    let bitmap_height = glyph_height * 6;
    let mut bitmap = vec![false; bitmap_width * bitmap_height];

    for (index, c) in (' '..='~').enumerate() {
        if let Some(glyph) = font.glyphs.iter().find(|glyph| glyph.encoding == Some(c)) {
            ensure!(
                glyph.bounding_box == glyph_bb,
                "glyph '{}' has a different bounding",
                c
            );

            let bitmap_x = (index % 16) * glyph_width;
            let bitmap_y = (index / 16) * glyph_height;

            for y in 0..glyph_height {
                for x in 0..glyph_width {
                    let byte_index = y * glyph_width_bytes + x / 8;
                    let bit_index = x % 8;

                    if glyph.bitmap[byte_index] & (0x80 >> bit_index) != 0 {
                        bitmap[bitmap_x + x + (bitmap_y + y) * bitmap_width] = true;
                    }
                }
            }
        } else {
            bail!("font doesn't contain '{}' glyph", c);
        }
    }

    let data = bitmap
        .chunks_exact(8)
        .map(|byte| {
            byte.iter()
                .enumerate()
                .filter(|(_, bit)| **bit)
                .map(|(i, _)| 0x80 >> i)
                .sum()
        })
        .collect::<Vec<_>>();

    Ok(Bitmap {
        data,
        width: bitmap_width,
        height: bitmap_height,
    })
}

pub struct Bitmap {
    pub data: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl Bitmap {
    pub fn pixel(&self, x: usize, y: usize) -> bool {
        self.data[x / 8 + y * (self.width / 8)] & (0x80 >> x % 8) != 0
    }
}