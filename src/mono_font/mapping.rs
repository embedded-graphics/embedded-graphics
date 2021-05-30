//! Glyph mapping.
//!
//! A glyph mapping defines the position of characters in a [`MonoFont`] image. This module provides
//! predefined mappings for common glyph subsets, but custom mappings are also supported.
//!
//! # Custom mappings
//!
//! Custom mappings can be defined in three different ways:
//! * The [`StrGlyphMapping`] type can be used to specify a character mapping by encoding the
//!   mapping as a string.
//! * The [`GlyphMapping`] trait is implemented for all functions `Fn(char) -> usize`.
//! * The [`GlyphMapping`] trait can be implemented by a custom type.
//!
//! # `StrGlyphMapping` encoding
//!
//! Strings without a `\0` character can be used to directly map a character to its position in
//! the mapping string:
//!
//! ```
//! use embedded_graphics::mono_font::mapping::{GlyphMapping, StrGlyphMapping};
//!
//! let mapping = StrGlyphMapping::new("abcdef1234", 0);
//! assert_eq!(mapping.index('a'), 0);
//! assert_eq!(mapping.index('b'), 1);
//! assert_eq!(mapping.index('1'), 6);
//! assert_eq!(mapping.index('2'), 7);
//! ```
//!
//! This direct mapping is inefficient for mappings that map consecutive ranges of characters to
//! consecutive index ranges. To define a range of characters a `\0` character followed by the
//! start and end characters of the inclusive range can be used. This way the mapping in the previous
//! example can be abbreviated to:
//!
//! ```
//! use embedded_graphics::mono_font::mapping::{GlyphMapping, StrGlyphMapping};
//!
//! let mapping = StrGlyphMapping::new("\0af\014", 0);
//! assert_eq!(mapping.index('a'), 0);
//! assert_eq!(mapping.index('b'), 1);
//! assert_eq!(mapping.index('1'), 6);
//! assert_eq!(mapping.index('2'), 7);
//! ```
//!
//! [`MonoFont`]: ../struct.MonoFont.html
//! [`StrGlyphMapping`]: struct.StrGlyphMapping.html
//! [`GlyphMapping`]: trait.GlyphMapping.html

use core::ops::RangeInclusive;

/// Mapping from characters to glyph indices.
pub trait GlyphMapping {
    /// Maps a character to a glyph index.
    ///
    /// If `c` isn't included in the font the index of a suitable replacement glyph is returned.
    fn index(&self, c: char) -> usize;
}

impl<F> GlyphMapping for F
where
    F: Fn(char) -> usize,
{
    fn index(&self, c: char) -> usize {
        self(c)
    }
}

/// Glyph mapping stored as a UTF-8 string.
///
/// See the [module-level documentation] for more details.
///
/// [module-level documentation]: index.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StrGlyphMapping<'a> {
    data: &'a str,
    replacement_index: usize,
}

impl<'a> StrGlyphMapping<'a> {
    /// Creates a new glyph mapping.
    pub const fn new(data: &'a str, replacement_index: usize) -> Self {
        Self {
            data,
            replacement_index,
        }
    }

    /// Returns an iterator over the character ranges.
    pub fn ranges(&self) -> impl Iterator<Item = (usize, RangeInclusive<char>)> + '_ {
        let mut chars = self.data.chars();
        let mut index = 0;

        core::iter::from_fn(move || {
            let start_index = index;

            let range = match chars.next()? {
                '\0' => {
                    let start = chars.next()?;
                    let end = chars.next()?;

                    index += end as usize - start as usize + 1;

                    start..=end
                }
                c => {
                    index += 1;

                    c..=c
                }
            };

            Some((start_index, range))
        })
    }

    /// Returns an iterator over the characters in this mapping.
    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        let mut chars = self.data.chars();

        core::iter::from_fn(move || {
            let range = match chars.next()? {
                '\0' => {
                    let start = chars.next()?;
                    let end = chars.next()?;

                    start as u32..=end as u32
                }
                c => c as u32..=c as u32,
            };

            Some(range)
        })
        .flatten()
        //MSRV: directly iterator over char ranges
        .filter_map(core::char::from_u32)
    }

    /// Returns if the mapping contains the given char.
    pub fn contains(&self, c: char) -> bool {
        self.chars().any(|v| v == c)
    }
}

impl GlyphMapping for StrGlyphMapping<'_> {
    fn index(&self, c: char) -> usize {
        // PERF: use ranges instead of chars iter
        self.chars()
            .enumerate()
            .find(|(_, v)| c == *v)
            .map(|(index, _)| index)
            .unwrap_or(self.replacement_index)
    }
}

macro_rules! impl_mapping {
    ($( $(#[$meta:meta])* ($enum_variant:ident, $constant:ident, $mapping:expr), )*) => {
        /// Mapping.
        ///
        /// This enum lists all mappings that are included in embedded-graphics. It is used
        /// to automatically generate font data for all mappings and isn't normally used in
        /// applications.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Mapping {
            $(
                $(#[$meta])*
                $enum_variant,
            )*
        }

        impl Mapping {
            /// Returns an iterator over all mappings.
            pub fn iter() -> impl Iterator<Item = Self> {
                const ALL: &[Mapping] = &[$(Mapping::$enum_variant),*];

                ALL.iter().copied()
            }

            /// Returns the MIME identifier for this mapping.
            pub fn mime(self) -> &'static str {
                match self {
                    $(Mapping::$enum_variant => stringify!($constant)),*
                }
            }

            /// Returns a glyph mapping for this mapping.
            pub fn glyph_mapping(self) -> &'static StrGlyphMapping<'static> {
                match self {
                    $(Self::$enum_variant => &$constant),*
                }
            }
        }

        $(
            $(#[$meta])*
            pub const $constant: StrGlyphMapping = StrGlyphMapping::new($mapping, '?' as usize - ' ' as usize);
        )*
    };
}

// TODO: Add Iso8859_6 (Latin/Arabic), Iso8859_8 (Latin/Hebrew) and Iso8859_11 (Latin/Thai) when we support RTL and combining characters.
impl_mapping!(
    /// ASCII.
    (Ascii, ASCII, "\0\u{20}\u{7f}"),

    /// ISO/IEC 8859 Part 1: Latin-1, Western European.
    (Iso8859_1, ISO_8859_1, "\0\u{20}\u{7f}\0\u{a0}\u{ff}"),

    /// ISO/IEC 8859 Part 2: Latin-2, Central European.
    (Iso8859_2, ISO_8859_2, "\0\u{20}\u{7f}\u{a0}\u{104}\u{2d8}\u{141}\u{a4}\u{13d}\u{15a}\u{a7}\u{a8}\u{160}\u{15e}\u{164}\u{179}\u{ad}\u{17d}\u{17b}\u{b0}\u{105}\u{2db}\u{142}\u{b4}\u{13e}\u{15b}\u{2c7}\u{b8}\u{161}\u{15f}\u{165}\u{17a}\u{2dd}\u{17e}\u{17c}\u{154}\u{c1}\u{c2}\u{102}\u{c4}\u{139}\u{106}\u{c7}\u{10c}\u{c9}\u{118}\u{cb}\u{11a}\u{cd}\u{ce}\u{10e}\u{110}\u{143}\u{147}\u{d3}\u{d4}\u{150}\u{d6}\u{d7}\u{158}\u{16e}\u{da}\u{170}\u{dc}\u{dd}\u{162}\u{df}\u{155}\u{e1}\u{e2}\u{103}\u{e4}\u{13a}\u{107}\u{e7}\u{10d}\u{e9}\u{119}\u{eb}\u{11b}\u{ed}\u{ee}\u{10f}\u{111}\u{144}\u{148}\u{f3}\u{f4}\u{151}\u{f6}\u{f7}\u{159}\u{16f}\u{fa}\u{171}\u{fc}\u{fd}\u{163}\u{2d9}"),

    /// ISO/IEC 8859 Part 3: Latin-3, South European.
    (Iso8859_3, ISO_8859_3, "\0\u{20}\u{7f}\u{a0}\u{126}\u{2d8}\0\u{a3}\u{a5}\u{124}\u{a7}\u{a8}\u{130}\u{15e}\u{11e}\u{134}\u{ad}\u{ae}\u{17b}\u{b0}\u{127}\0\u{b2}\u{b5}\u{125}\u{b7}\u{b8}\u{131}\u{15f}\u{11f}\u{135}\u{bd}\u{be}\u{17c}\0\u{c0}\u{c4}\u{10a}\u{108}\0\u{c7}\u{d4}\u{120}\u{d6}\u{d7}\u{11c}\0\u{d9}\u{dc}\u{16c}\u{15c}\0\u{df}\u{e4}\u{10b}\u{109}\0\u{e7}\u{f4}\u{121}\u{f6}\u{f7}\u{11d}\0\u{f9}\u{fc}\u{16d}\u{15d}\u{2d9}"),

    /// ISO/IEC 8859 Part 4: Latin-4, North European.
    (Iso8859_4, ISO_8859_4, "\0\u{20}\u{7f}\u{a0}\u{104}\u{138}\u{156}\u{a4}\u{128}\u{13b}\u{a7}\u{a8}\u{160}\u{112}\u{122}\u{166}\u{ad}\u{17d}\u{af}\u{b0}\u{105}\u{2db}\u{157}\u{b4}\u{129}\u{13c}\u{2c7}\u{b8}\u{161}\u{113}\u{123}\u{167}\u{14a}\u{17e}\u{14b}\u{100}\0\u{c1}\u{c6}\u{12e}\u{10c}\u{c9}\u{118}\u{cb}\u{116}\u{cd}\u{ce}\u{12a}\u{110}\u{145}\u{14c}\u{136}\0\u{d4}\u{d8}\u{172}\0\u{da}\u{dc}\u{168}\u{16a}\u{df}\u{101}\0\u{e1}\u{e6}\u{12f}\u{10d}\u{e9}\u{119}\u{eb}\u{117}\u{ed}\u{ee}\u{12b}\u{111}\u{146}\u{14d}\u{137}\0\u{f4}\u{f8}\u{173}\0\u{fa}\u{fc}\u{169}\u{16b}\u{2d9}"),

    /// ISO/IEC 8859 Part 5: Latin/Cyrillic.
    (Iso8859_5, ISO_8859_5, "\0\u{20}\u{7f}\u{a0}\0\u{401}\u{40c}\u{ad}\0\u{40e}\u{44f}\u{2116}\0\u{451}\u{45c}\u{a7}\u{45e}\u{45f}"),

    /// ISO/IEC 8859 Part 7: Latin/Greek.
    (Iso8859_7, ISO_8859_7, "\0\u{20}\u{7f}\u{a0}\u{2018}\u{2019}\u{a3}\u{20ac}\u{20af}\0\u{a6}\u{a9}\u{37a}\0\u{ab}\u{ae}\u{2015}\0\u{b0}\u{b3}\0\u{384}\u{386}\u{b7}\0\u{388}\u{38a}\u{bb}\u{38c}\u{bd}\0\u{38e}\u{3cf}"),

    /// ISO/IEC 8859 Part 9: Latin-5, Turkish.
    (Iso8859_9, ISO_8859_9, "\0\u{20}\u{7f}\0\u{a0}\u{cf}\u{11e}\0\u{d1}\u{dc}\u{130}\u{15e}\0\u{df}\u{ef}\u{11f}\0\u{f1}\u{fc}\u{131}\u{15f}\u{ff}"),

    /// ISO/IEC 8859 Part 10: Latin-6, Nordic.
    (Iso8859_10, ISO_8859_10, "\0\u{20}\u{7f}\u{a0}\u{104}\u{112}\u{122}\u{12a}\u{128}\u{136}\u{a7}\u{13b}\u{110}\u{160}\u{166}\u{17d}\u{ad}\u{16a}\u{14a}\u{b0}\u{105}\u{113}\u{123}\u{12b}\u{129}\u{137}\u{b7}\u{13c}\u{111}\u{161}\u{167}\u{17e}\u{2015}\u{16b}\u{14b}\u{100}\0\u{c1}\u{c6}\u{12e}\u{10c}\u{c9}\u{118}\u{cb}\u{116}\0\u{cd}\u{d0}\u{145}\u{14c}\0\u{d3}\u{d6}\u{168}\u{d8}\u{172}\0\u{da}\u{df}\u{101}\0\u{e1}\u{e6}\u{12f}\u{10d}\u{e9}\u{119}\u{eb}\u{117}\0\u{ed}\u{f0}\u{146}\u{14d}\0\u{f3}\u{f6}\u{169}\u{f8}\u{173}\0\u{fa}\u{fe}\u{138}"),

    /// ISO/IEC 8859 Part 13: Latin-7, Baltic Rim.
    (Iso8859_13, ISO_8859_13, "\0\u{20}\u{7f}\u{a0}\u{201d}\0\u{a2}\u{a4}\u{201e}\u{a6}\u{a7}\u{d8}\u{a9}\u{156}\0\u{ab}\u{ae}\u{c6}\0\u{b0}\u{b3}\u{201c}\0\u{b5}\u{b7}\u{f8}\u{b9}\u{157}\0\u{bb}\u{be}\u{e6}\u{104}\u{12e}\u{100}\u{106}\u{c4}\u{c5}\u{118}\u{112}\u{10c}\u{c9}\u{179}\u{116}\u{122}\u{136}\u{12a}\u{13b}\u{160}\u{143}\u{145}\u{d3}\u{14c}\0\u{d5}\u{d7}\u{172}\u{141}\u{15a}\u{16a}\u{dc}\u{17b}\u{17d}\u{df}\u{105}\u{12f}\u{101}\u{107}\u{e4}\u{e5}\u{119}\u{113}\u{10d}\u{e9}\u{17a}\u{117}\u{123}\u{137}\u{12b}\u{13c}\u{161}\u{144}\u{146}\u{f3}\u{14d}\0\u{f5}\u{f7}\u{173}\u{142}\u{15b}\u{16b}\u{fc}\u{17c}\u{17e}\u{2019}"),

    /// ISO/IEC 8859 Part 14: Latin-8, Celtic.
    (Iso8859_14, ISO_8859_14, "\0\u{20}\u{7f}\u{a0}\u{1e02}\u{1e03}\u{a3}\u{10a}\u{10b}\u{1e0a}\u{a7}\u{1e80}\u{a9}\u{1e82}\u{1e0b}\u{1ef2}\u{ad}\u{ae}\u{178}\u{1e1e}\u{1e1f}\u{120}\u{121}\u{1e40}\u{1e41}\u{b6}\u{1e56}\u{1e81}\u{1e57}\u{1e83}\u{1e60}\u{1ef3}\u{1e84}\u{1e85}\u{1e61}\0\u{c0}\u{cf}\u{174}\0\u{d1}\u{d6}\u{1e6a}\0\u{d8}\u{dd}\u{176}\0\u{df}\u{ef}\u{175}\0\u{f1}\u{f6}\u{1e6b}\0\u{f8}\u{fd}\u{177}\u{ff}"),

    /// ISO/IEC 8859 Part 15: Latin-9 (revised Latin-1).
    (Iso8859_15, ISO_8859_15, "\0\u{20}\u{7f}\0\u{a0}\u{a3}\u{20ac}\u{a5}\u{160}\u{a7}\u{161}\0\u{a9}\u{b3}\u{17d}\0\u{b5}\u{b7}\u{17e}\0\u{b9}\u{bb}\u{152}\u{153}\u{178}\0\u{bf}\u{ff}"),

    /// ISO/IEC 8859 Part 16: Latin-10: South-East European.
    (Iso8859_16, ISO_8859_16, "\0\u{20}\u{7f}\u{a0}\u{104}\u{105}\u{141}\u{20ac}\u{201e}\u{160}\u{a7}\u{161}\u{a9}\u{218}\u{ab}\u{179}\u{ad}\u{17a}\u{17b}\u{b0}\u{b1}\u{10c}\u{142}\u{17d}\u{201d}\u{b6}\u{b7}\u{17e}\u{10d}\u{219}\u{bb}\u{152}\u{153}\u{178}\u{17c}\0\u{c0}\u{c2}\u{102}\u{c4}\u{106}\0\u{c6}\u{cf}\u{110}\u{143}\0\u{d2}\u{d4}\u{150}\u{d6}\u{15a}\u{170}\0\u{d9}\u{dc}\u{118}\u{21a}\0\u{df}\u{e2}\u{103}\u{e4}\u{107}\0\u{e6}\u{ef}\u{111}\u{144}\0\u{f2}\u{f4}\u{151}\u{f6}\u{15b}\u{171}\0\u{f9}\u{fc}\u{119}\u{21b}\u{ff}"),

    /// JIS X 0201: Japanese katakana (halfwidth).
    (JisX0201, JIS_X0201, "\0\u{20}\u{7f}\0\u{ff60}\u{ff9f}"),
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mapping = StrGlyphMapping::new("", 0);

        let mut iter = mapping.ranges();
        assert_eq!(iter.next(), None);

        let mut iter = mapping.chars();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn one_char() {
        let mapping = StrGlyphMapping::new("a", 0);

        let mut iter = mapping.ranges();
        assert_eq!(iter.next(), Some((0, 'a'..='a')));
        assert_eq!(iter.next(), None);

        let mut iter = mapping.chars();
        assert_eq!(iter.next(), Some('a'));
        assert_eq!(iter.next(), None);

        assert_eq!(mapping.index('a'), 0);
    }

    #[test]
    fn three_chars() {
        let mapping = StrGlyphMapping::new("abc", 1);

        let mut iter = mapping.ranges();
        assert_eq!(iter.next(), Some((0, 'a'..='a')));
        assert_eq!(iter.next(), Some((1, 'b'..='b')));
        assert_eq!(iter.next(), Some((2, 'c'..='c')));
        assert_eq!(iter.next(), None);

        let mut iter = mapping.chars();
        assert_eq!(iter.next(), Some('a'));
        assert_eq!(iter.next(), Some('b'));
        assert_eq!(iter.next(), Some('c'));
        assert_eq!(iter.next(), None);

        assert_eq!(mapping.index('a'), 0);
        assert_eq!(mapping.index('b'), 1);
        assert_eq!(mapping.index('c'), 2);
        assert_eq!(mapping.index('$'), 1);
    }

    #[test]
    fn one_range() {
        let mapping = StrGlyphMapping::new("\x00ac", 2);

        let mut iter = mapping.ranges();
        assert_eq!(iter.next(), Some((0, 'a'..='c')));
        assert_eq!(iter.next(), None);

        let mut iter = mapping.chars();
        assert_eq!(iter.next(), Some('a'));
        assert_eq!(iter.next(), Some('b'));
        assert_eq!(iter.next(), Some('c'));
        assert_eq!(iter.next(), None);

        assert_eq!(mapping.index('a'), 0);
        assert_eq!(mapping.index('b'), 1);
        assert_eq!(mapping.index('c'), 2);
        assert_eq!(mapping.index('$'), 2);
    }

    #[test]
    fn incomplete_range() {
        let mapping = StrGlyphMapping::new("\x00a", 0);

        let mut iter = mapping.ranges();
        assert_eq!(iter.next(), None);

        let mut iter = mapping.chars();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn mixed_ranges_and_chars() {
        let mapping = StrGlyphMapping::new("a\x00bde", 3);

        let mut iter = mapping.ranges();
        assert_eq!(iter.next(), Some((0, 'a'..='a')));
        assert_eq!(iter.next(), Some((1, 'b'..='d')));
        assert_eq!(iter.next(), Some((4, 'e'..='e')));
        assert_eq!(iter.next(), None);

        let mut iter = mapping.chars();
        assert_eq!(iter.next(), Some('a'));
        assert_eq!(iter.next(), Some('b'));
        assert_eq!(iter.next(), Some('c'));
        assert_eq!(iter.next(), Some('d'));
        assert_eq!(iter.next(), Some('e'));
        assert_eq!(iter.next(), None);

        assert_eq!(mapping.index('a'), 0);
        assert_eq!(mapping.index('b'), 1);
        assert_eq!(mapping.index('c'), 2);
        assert_eq!(mapping.index('d'), 3);
        assert_eq!(mapping.index('e'), 4);
        assert_eq!(mapping.index('$'), 3);
    }

    #[test]
    fn dyn_str_glyph_mapping() {
        let mapping = StrGlyphMapping::new("ab", 0);
        let dyn_mapping: &dyn GlyphMapping = &mapping;

        assert_eq!(dyn_mapping.index('b'), 1);
    }

    #[test]
    fn dyn_fn_glyph_mapping() {
        fn map(c: char) -> usize {
            match c {
                'a' => 0,
                'b' => 1,
                _ => 2,
            }
        }

        let dyn_mapping: &dyn GlyphMapping = &map;

        assert_eq!(dyn_mapping.index('a'), 0);
        assert_eq!(dyn_mapping.index('b'), 1);
        assert_eq!(dyn_mapping.index('?'), 2);
    }
}
