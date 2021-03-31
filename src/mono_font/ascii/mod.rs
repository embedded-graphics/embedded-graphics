//! ASCII.
//!
//! TODO: docs

mod generated;

pub use generated::*;

use crate::mono_font::{GlyphIndices, GlyphRange};

/// Glyph ranges for ASCII fonts.
pub const ASCII_GLYPH_RANGES: &[GlyphRange] = &[GlyphRange::new(' ', '\x7F', 0)];

/// Glyph indices for ASCII fonts.
pub const ASCII_GLYPH_INDICES: GlyphIndices =
    GlyphIndices::new(ASCII_GLYPH_RANGES, '?' as u32 - ' ' as u32);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mono_font::tests::test_baseline;

    fn c(index: u32) -> char {
        core::char::from_u32(index).unwrap()
    }

    #[test]
    fn glyph_indices_valid() {
        assert_eq!(ASCII_GLYPH_INDICES.get(c(0x20)), 0);
        assert_eq!(ASCII_GLYPH_INDICES.get(c(0x50)), 3 * 16);
        assert_eq!(ASCII_GLYPH_INDICES.get(c(0x7F)), 6 * 16 - 1);
    }

    #[test]
    fn glyph_indices_fallback() {
        assert_eq!(
            ASCII_GLYPH_INDICES.get(c(0x1F)),
            ASCII_GLYPH_INDICES.get('?')
        );
        assert_eq!(
            ASCII_GLYPH_INDICES.get(c(0x80)),
            ASCII_GLYPH_INDICES.get('?')
        );
        assert_eq!(
            ASCII_GLYPH_INDICES.get(c(0xA0)),
            ASCII_GLYPH_INDICES.get('?')
        );
        assert_eq!(
            ASCII_GLYPH_INDICES.get(c(0xFF)),
            ASCII_GLYPH_INDICES.get('?')
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
