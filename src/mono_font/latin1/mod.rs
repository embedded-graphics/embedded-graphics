//! Latin1.
//!
//! TODO: docs

mod generated;

pub use generated::*;

use crate::mono_font::{GlyphIndices, GlyphRange};

/// Glyph ranges for Latin-1 fonts.
const LATIN1_GLYPH_RANGES: &[GlyphRange] = &[
    GlyphRange::new(' ', '\x7F', 0),
    GlyphRange::new('\u{00A0}', '\u{00FF}', 0x60),
];

/// Glyph indices for Latin-1 fonts.
pub const LATIN1_GLYPH_INDICES: GlyphIndices =
    GlyphIndices::new(LATIN1_GLYPH_RANGES, '?' as u32 - ' ' as u32);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mono_font::tests::test_baseline;

    fn c(index: u32) -> char {
        core::char::from_u32(index).unwrap()
    }

    #[test]
    fn char_offset_valid() {
        assert_eq!(LATIN1_GLYPH_INDICES.get(c(0x20)), 0);
        assert_eq!(LATIN1_GLYPH_INDICES.get(c(0x40)), 2 * 16);
        assert_eq!(LATIN1_GLYPH_INDICES.get(c(0x7F)), 6 * 16 - 1);
        assert_eq!(LATIN1_GLYPH_INDICES.get(c(0xA0)), 6 * 16);
        assert_eq!(LATIN1_GLYPH_INDICES.get(c(0xD0)), 9 * 16);
        assert_eq!(LATIN1_GLYPH_INDICES.get(c(0xFF)), 12 * 16 - 1);
    }

    #[test]
    fn char_offset_fallback() {
        assert_eq!(
            LATIN1_GLYPH_INDICES.get(c(0x1F)),
            LATIN1_GLYPH_INDICES.get('?')
        );
        assert_eq!(
            LATIN1_GLYPH_INDICES.get(c(0x80)),
            LATIN1_GLYPH_INDICES.get('?')
        );
        assert_eq!(
            LATIN1_GLYPH_INDICES.get(c(0x9F)),
            LATIN1_GLYPH_INDICES.get('?')
        );
        assert_eq!(
            LATIN1_GLYPH_INDICES.get(c(0x100)),
            LATIN1_GLYPH_INDICES.get('?')
        );
    }

    #[test]
    fn baseline() {
        test_baseline(&FONT_4X6);
        test_baseline(&FONT_5X7);
        test_baseline(&FONT_5X8);
        test_baseline(&FONT_6X10);
        test_baseline(&FONT_6X12);
        test_baseline(&FONT_6X13BOLD);
        test_baseline(&FONT_6X13);
        test_baseline(&FONT_6X13ITALIC);
        test_baseline(&FONT_6X9);
        test_baseline(&FONT_7X13BOLD);
        test_baseline(&FONT_7X13);
        test_baseline(&FONT_7X13ITALIC);
        test_baseline(&FONT_7X14BOLD);
        test_baseline(&FONT_7X14);
        test_baseline(&FONT_8X13BOLD);
        test_baseline(&FONT_8X13);
        test_baseline(&FONT_8X13ITALIC);
        test_baseline(&FONT_9X15BOLD);
        test_baseline(&FONT_9X15);
        test_baseline(&FONT_9X18BOLD);
        test_baseline(&FONT_9X18);
        test_baseline(&FONT_10X20);
    }
}
