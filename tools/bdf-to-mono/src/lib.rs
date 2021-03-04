use anyhow::{anyhow, Result};
use bdf_parser::BdfFont;
use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Encoding {
    Ascii,
    Latin1,
}

impl fmt::Display for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        match self {
            Self::Ascii => f.write_str("ascii"),
            Self::Latin1 => f.write_str("latin1"),
        }
    }
}

pub fn bdf_to_bitmap(font: &BdfFont, encoding: Encoding) -> Result<Bitmap> {
    let rows: Vec<u32> = match encoding {
        Encoding::Ascii => (0x20..=0x7F).step_by(16).collect(),
        Encoding::Latin1 => (0x20..=0x7F).chain(0xA0..=0xFF).step_by(16).collect(),
    };
    let chars = rows.iter().flat_map(|start| {
        std::char::from_u32(*start).unwrap()..std::char::from_u32(*start + 16).unwrap()
    });

    let fallback_glyph = font
        .glyphs
        .get('?')
        .ok_or_else(|| anyhow!("font doesn't contain the fallback '?' glyph"))?;
    let mut glyph_bb = font
        .glyphs
        .get('A')
        .ok_or_else(|| anyhow!("font doesn't contain 'A' glyph"))?
        .bounding_box;
    // used to optimize the glyph height in case all glyph offsets are e.g. -1
    let mut max_y_offset = i32::MIN;
    // We should take the DWIDTH value into account. If all glyph bounding boxes are smaller than
    // DWIDTH the CHARACTER_SPACING in the MonoFont impl should be set to DWIDTH - (width of the BB)
    // to maintain the correct kerning.
    let mut char_spacing = None;

    // find the maximum bounding box that can fit all glyphs taking offsets into account
    // glyph_bb.offset.y will contain the baseline
    // additionally build the `selected_glyphs` vector to save on the glyph lookup later in the code
    let selected_glyphs = chars
        .into_iter()
        .enumerate()
        .map(|(index, c)| {
            let glyph = font.glyphs.get(c).unwrap_or(&fallback_glyph);
            let off_x = glyph.bounding_box.offset.x;
            if off_x < glyph_bb.offset.x {
                glyph_bb.offset.x = off_x;
            }
            let off_y = glyph.bounding_box.offset.y;
            if off_y < glyph_bb.offset.y {
                glyph_bb.offset.y = off_y;
            }
            if off_y > max_y_offset {
                max_y_offset = off_y;
            }
            let gl_w = glyph.bounding_box.size.x + off_x.abs();
            if gl_w > glyph_bb.size.x {
                glyph_bb.size.x = gl_w;
            }
            let gl_h = glyph.bounding_box.size.y + off_y.abs();
            if gl_h > glyph_bb.size.y {
                glyph_bb.size.y = gl_h;
            }
            let x_size_diff = glyph.device_width.x - glyph.bounding_box.size.x;
            if char_spacing.map_or(true, |char_spacing_compensation| {
                x_size_diff < char_spacing_compensation
            }) {
                char_spacing = Some(x_size_diff);
            }
            (index, glyph)
        })
        .collect::<Vec<_>>();
    let glyph_width = usize::try_from(glyph_bb.size.x)?;
    let mut glyph_height = usize::try_from(glyph_bb.size.y)?;
    // compensate the glyph height if case all offsets are either > 0 or < 0
    let y_offset_compensation;
    if max_y_offset.signum() == glyph_bb.offset.y.signum() {
        y_offset_compensation = max_y_offset.abs().min(glyph_bb.offset.y.abs());
        glyph_height = usize::try_from(i32::try_from(glyph_height)? - y_offset_compensation)?;
    } else {
        y_offset_compensation = 0;
    }
    let bitmap_width = ((glyph_width * 16 + 7) / 8) * 8;
    let bitmap_height = glyph_height * rows.len();
    let mut bitmap = vec![false; bitmap_width * bitmap_height];

    for (index, glyph) in selected_glyphs {
        let mut bitmap_x = (index % 16) * glyph_width;
        let mut bitmap_y = (index / 16) * glyph_height;

        let dx = glyph.bounding_box.offset.x - glyph_bb.offset.x;
        // calculate the baseline and make it an origin for vertical positioning
        let dy = glyph_bb.size.y + glyph_bb.offset.y
            - glyph.bounding_box.offset.y
            - glyph.bounding_box.size.y
            - y_offset_compensation;

        bitmap_x = usize::try_from(i32::try_from(bitmap_x)? + dx)?;
        bitmap_y = usize::try_from(i32::try_from(bitmap_y)? + dy)?;

        for y in 0..usize::try_from(glyph.bounding_box.size.y)? {
            for x in 0..usize::try_from(glyph.bounding_box.size.x)? {
                if glyph.pixel(x, y) {
                    bitmap[bitmap_x + x + (bitmap_y + y) * bitmap_width] = true;
                }
            }
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

    assert_eq!(glyph_bb.offset.x, 0);
    assert!(glyph_bb.offset.y <= 0);

    Ok(Bitmap {
        data,
        width: bitmap_width,
        height: bitmap_height,
        glyph_width,
        glyph_height,
        rows: rows.len(),
        columns: 16,
        baseline: glyph_height - -glyph_bb.offset.y as usize - 1,
        character_spacing: u32::try_from(char_spacing.unwrap_or(0).max(0))?,
    })
}

pub struct Bitmap {
    pub data: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub glyph_width: usize,
    pub glyph_height: usize,
    pub rows: usize,
    pub columns: usize,
    pub baseline: usize,
    pub character_spacing: u32,
}

impl Bitmap {
    pub fn pixel(&self, x: usize, y: usize) -> bool {
        self.data[x / 8 + y * (self.width / 8)] & (0x80 >> x % 8) != 0
    }
}
