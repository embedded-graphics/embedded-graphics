//! Monospaced bitmap fonts.
//!
//! This module contains support for drawing monospaced bitmap fonts and provides
//! several [built-in fonts].
//!
//! Additional custom fonts can be added by the application or other crates. This
//! is demonstrated in the `text-custom-font` example in the simulator crate.
//!
//! # Examples
//!
//! The examples below use the `Font6x8` font, however any of the [built-in fonts]
//! in this module or custom fonts can be substituted.
//!
//! ## Print styled "Hello Rust!"
//!
//! Text can be drawn to a display by creating a [`Text`] object and attaching a
//! text style to it by using a [`Styled`] object. This example prints
//! "Hello Rust" with a yellow text on a blue background.
//!
//! ```rust
//! use embedded_graphics::{
//!     mono_font::{ascii::FONT_6X9, MonoTextStyle, MonoTextStyleBuilder},
//!     pixelcolor::Rgb565,
//!     prelude::*,
//!     text::Text,
//! };
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display: MockDisplay<Rgb565> = MockDisplay::default();
//! # display.set_allow_out_of_bounds_drawing(true);
//!
//! // Create a new text style
//! let style = MonoTextStyleBuilder::new()
//!     .font(&FONT_6X9)
//!     .text_color(Rgb565::YELLOW)
//!     .background_color(Rgb565::BLUE)
//!     .build();
//!
//! // Create a text at position (20, 30) and draw it using the previously defined style
//! Text::new("Hello Rust!", Point::new(20, 30))
//!     .into_styled(style)
//!     .draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//!
//! ## Translate text by (20px, 30px)
//!
//! ```rust
//! use embedded_graphics::{
//!     mono_font::{ascii::FONT_6X9, MonoTextStyle},
//!     pixelcolor::BinaryColor,
//!     prelude::*,
//!     text::Text,
//! };
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display: MockDisplay<BinaryColor> = MockDisplay::default();
//! # display.set_allow_out_of_bounds_drawing(true);
//!
//! Text::new("Hello Rust!", Point::zero())
//!     .into_styled(MonoTextStyle::new(&FONT_6X9, BinaryColor::On))
//!     .translate(Point::new(20, 30))
//!     .draw(&mut display)?;
//!
//! // this is equivalent to:
//!
//! # let mut display: MockDisplay<BinaryColor> = MockDisplay::default();
//! # display.set_allow_out_of_bounds_drawing(true);
//! Text::new("Hello Rust!", Point::new(20, 30))
//!     .into_styled(MonoTextStyle::new(&FONT_6X9, BinaryColor::On))
//!     .draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//!
//! ## Use `write!()` and arrayvec to render a formatted string
//!
//! This example uses arrayvec's [`ArrayString`] to render a floating point value using the
//! [`write!()`] macro. These strings have a fixed maximum length, but allow the use of
//! Rust's builtin string formatting.
//!
//! ```rust
//! use arrayvec::ArrayString;
//! use core::fmt::Write;
//! use embedded_graphics::{
//!     mono_font::{ascii::FONT_6X9, MonoTextStyleBuilder},
//!     pixelcolor::Rgb565,
//!     prelude::*,
//!     text::Text,
//! };
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display = MockDisplay::default();
//! # display.set_allow_out_of_bounds_drawing(true);
//!
//! let value = 12.34567;
//!
//! // Create a fixed buffer of length 12
//! let mut buf = ArrayString::<[_; 12]>::new();
//!
//! // Output `Value: 12.35`
//! write!(&mut buf, "Value: {:.2}", value).expect("Failed to write to buffer");
//!
//! Text::new(&buf, Point::zero())
//!     .into_styled(
//!         MonoTextStyleBuilder::new()
//!             .font(&FONT_6X9)
//!             .text_color(Rgb565::YELLOW)
//!             .background_color(Rgb565::BLUE)
//!             .build(),
//!     )
//!     .draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//!
//! # Built-in fonts
//!
//! TODO: Replace with generated fonts table
//!
//! | Type | Screenshot |
//! |------|------------|
//! | `Font6x8` | ![6x8 font spritemap screenshot](https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/master/data/font6x8.png) |
//! | `Font6x12` | ![6x12 font spritemap screenshot](https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/master/data/font6x12.png) |
//! | `Font8x16` | ![8x16 font spritemap screenshot](https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/master/data/font8x16.png) |
//! | `Font12x16` | ![12x16 font spritemap screenshot](https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/master/data/font12x16.png) |
//! | `Font24x32` | The 24x32 font is a pixel doubled version of the 12x16 font. |
//!
//! [built-in fonts]: #built-in-fonts
//! [`Text`]: ../text/struct.Text.html
//! [`Styled`]: ../struct.Styled.html
//! [`MonoTextStyle`]: struct.MonoTextStyle.html
//! [`ArrayString`]: https://docs.rs/arrayvec/0.4.11/arrayvec/struct.ArrayString.html
//! [`write!()`]: https://doc.rust-lang.org/nightly/std/macro.write.html

pub mod ascii;
mod draw_target;
pub mod latin1;
mod mono_text_style;

use core::ops::RangeInclusive;

pub use mono_text_style::{MonoTextStyle, MonoTextStyleBuilder};

use crate::{
    geometry::{OriginDimensions, Point, Size},
    image::{ImageRaw, SubImage},
    pixelcolor::BinaryColor,
    primitives::Rectangle,
};

/// Monospaced bitmap font.
///
/// See the [module documentation] for more information about using fonts.
///
/// # Implementation notes
///
/// Use [`MonoFontBuilder`] to create new font objects.
///
/// [module documentation]: index.html
/// [`MonoFontBuilder`]: struct.MonoFontBuilder.html
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub struct MonoFont<'a, 'b> {
    /// Raw image data containing the font.
    pub image: ImageRaw<'a, BinaryColor>,

    /// Size of a single character in pixel.
    pub character_size: Size,

    /// Spacing between characters.
    ///
    /// The spacing defines how many empty pixels are added horizontally between adjacent characters
    /// on a single line of text.
    pub character_spacing: u32,

    /// The baseline.
    ///
    /// Offset from the top of the glyph bounding box to the baseline.
    pub baseline: u32,

    /// Strikethrough decoration dimensions.
    pub strikethrough: DecorationDimensions,

    /// Underline decoration dimensions.
    pub underline: DecorationDimensions,

    /// Glyph indices.
    pub glyph_indices: GlyphIndices<'b>,
}

impl MonoFont<'_, '_> {
    /// Returns a subimage for a glyph.
    pub(crate) fn glyph(&self, c: char) -> SubImage<'_, ImageRaw<BinaryColor>> {
        let glyphs_per_row = self.image.size().width / self.character_size.width;

        // Char _code_ offset from first char, most often a space
        // E.g. first char = ' ' (32), target char = '!' (33), offset = 33 - 32 = 1
        let glyph_index = self.glyph_indices.get(c);
        let row = glyph_index / glyphs_per_row;

        // Top left corner of character, in pixels
        let char_x = (glyph_index - (row * glyphs_per_row)) * self.character_size.width;
        let char_y = row * self.character_size.height;

        SubImage::new_unchecked(
            &self.image,
            Rectangle::new(
                Point::new(char_x as i32, char_y as i32),
                self.character_size,
            ),
        )
    }
}

/// Mono font builder.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct MonoFontBuilder<'a, 'b> {
    font: MonoFont<'a, 'b>,
}

impl<'a, 'b> MonoFontBuilder<'a, 'b> {
    /// Creates a new mono font builder.
    pub const fn new() -> Self {
        Self { font: NULL_FONT }
    }

    /// Sets the image.
    pub const fn image(mut self, image: ImageRaw<'a, BinaryColor>) -> Self {
        self.font.image = image;
        self
    }

    /// Sets the character size.
    ///
    /// Setting the character size resets the baseline and decoration parameters to their default
    /// value.
    ///
    /// # Panics
    ///
    /// This methods panics if the character height is 0.
    // MSRV 1.46: Don't reset baseline, underline and strikethrough (see other methods).
    // MSRV 1.47: Use `saturating_sub` to calculate baseline to remove panic.
    pub const fn character_size(mut self, character_size: Size) -> Self {
        self.font.character_size = character_size;
        self.font.baseline = character_size.height - 1;
        self.font.underline = DecorationDimensions::new(character_size.height + 1, 1);
        self.font.strikethrough = DecorationDimensions::new(self.font.baseline / 2, 1);

        self
    }

    /// Sets the character spacing.
    pub const fn character_spacing(mut self, character_spacing: u32) -> Self {
        self.font.character_spacing = character_spacing;
        self
    }

    /// Sets the baseline.
    ///
    /// To set a custom baseline this method must be called after `character_size`.
    // MSRV 1.46: Use `Option<u32>` internally and resolve the final value in `build` to make the
    //            builder behave the same independent of the call order.
    pub const fn baseline(mut self, baseline: u32) -> Self {
        self.font.baseline = baseline;
        self
    }

    /// Sets the underline decoration.
    ///
    /// To set a custom underline offset and height this method must be called after `character_size`
    /// and `baseline`.
    // MSRV 1.46: Use `Option<Decoration>` internally and resolve the final value in `build` to make
    //            the builder behave the same independent of the call order.
    pub const fn underline(mut self, offset: u32, height: u32) -> Self {
        self.font.underline = DecorationDimensions::new(offset, height);
        self
    }

    /// Sets the strikethrough decoration.
    ///
    /// To set a custom strikethrough offset and height this method must be called after `character_size`.
    // MSRV 1.46: Use `Option<Decoration>` internally and resolve the final value in `build` to make
    //            the builder behave the same independent of the call order.
    pub const fn strikethrough(mut self, offset: u32, height: u32) -> Self {
        self.font.strikethrough = DecorationDimensions::new(offset, height);
        self
    }

    /// Sets the glyph indices.
    pub const fn glyph_indices(mut self, glyph_indices: GlyphIndices<'b>) -> Self {
        self.font.glyph_indices = glyph_indices;
        self
    }

    /// Builds the font.
    pub const fn build(self) -> MonoFont<'a, 'b> {
        self.font
    }
}

/// Glyph indices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlyphIndices<'a> {
    ranges: &'a [GlyphRange],
    fallback: u32,
}

impl<'a> GlyphIndices<'a> {
    /// Creates new character offsets.
    pub const fn new(ranges: &'a [GlyphRange], fallback: u32) -> Self {
        Self { ranges, fallback }
    }

    /// Gets the index of a character.
    pub fn get(&self, c: char) -> u32 {
        self.ranges
            .iter()
            .find_map(|range| range.get(c))
            .unwrap_or(self.fallback)
    }
}

/// Glyph range.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GlyphRange {
    characters: RangeInclusive<char>,
    start_offset: u32,
}

impl GlyphRange {
    /// Creates a new glyph range.
    pub const fn new(first_char: char, last_char: char, start_offset: u32) -> Self {
        Self {
            characters: first_char..=last_char,
            start_offset,
        }
    }

    /// Gets the glyph index of a character.
    ///
    /// Returns `None` if the glyph isn't inside this character range.
    pub fn get(&self, c: char) -> Option<u32> {
        if self.characters.contains(&c) {
            let delta = c as u32 - *self.characters.start() as u32;

            Some(self.start_offset + delta)
        } else {
            None
        }
    }
}

/// Decoration dimensions.
///
/// `DecorationDimensions` is used to specify the position and height of underline and strikethrough
/// decorations in [`MonoFont`]s.
///
/// [`MonoFont`]: struct.MonoFont.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DecorationDimensions {
    /// Offset from the top of the character to the top of the decoration.
    pub offset: u32,
    /// Height of the decoration.
    pub height: u32,
}

impl DecorationDimensions {
    /// Creates new decoration dimensions.
    pub const fn new(offset: u32, height: u32) -> Self {
        Self { offset, height }
    }

    fn to_rectangle(&self, position: Point, width: u32) -> Rectangle {
        let top_left = position + Size::new(0, self.offset);
        let size = Size::new(width, self.height);

        Rectangle::new(top_left, size)
    }
}

const NULL_FONT: MonoFont = MonoFont {
    image: ImageRaw::new_binary(&[], 1),
    character_size: Size::zero(),
    character_spacing: 0,
    baseline: 0,
    strikethrough: DecorationDimensions::new(0, 0),
    underline: DecorationDimensions::new(0, 0),
    glyph_indices: ascii::ASCII_GLYPH_INDICES,
};

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::{
        geometry::{AnchorPoint, Point},
        mock_display::MockDisplay,
        mono_font::MonoTextStyleBuilder,
        pixelcolor::BinaryColor,
        text::{Baseline, Text, TextStyleBuilder},
        Drawable,
    };

    /// Draws a text using the given font and checks it against the expected pattern.
    // MSRV: Add `track_caller` attribute for rust version >= 1.46.0
    // #[track_caller]
    pub fn assert_text_from_pattern(text: &str, font: &MonoFont, pattern: &[&str]) {
        let character_style = MonoTextStyleBuilder::new()
            .font(font)
            .text_color(BinaryColor::On)
            .build();

        let text_style = TextStyleBuilder::new()
            .character_style(character_style)
            .baseline(Baseline::Top)
            .build();

        let mut display = MockDisplay::new();
        Text::new(text, Point::zero())
            .into_styled(text_style)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(pattern);
    }

    /// Test if the baseline constant is set correctly.
    ///
    /// This test assumes that the character `A` is on the baseline.
    pub fn test_baseline(font: &MonoFont) {
        let character_style = MonoTextStyleBuilder::new()
            .font(font)
            .text_color(BinaryColor::On)
            .build();

        let text_style = TextStyleBuilder::new()
            .character_style(character_style)
            .baseline(Baseline::Top)
            .build();

        // Draw 'A' character to determine it's baseline
        let mut display = MockDisplay::new();
        Text::new("A", Point::zero())
            .into_styled(text_style)
            .draw(&mut display)
            .unwrap();

        let baseline = display.affected_area().bottom_right().unwrap().y as u32;

        assert_eq!(font.baseline, baseline);
    }

    #[test]
    fn default_baseline_and_decorations() {
        let glyph_bb = Rectangle::new(Point::zero(), Size::new(10, 15));
        let baseline = glyph_bb.anchor_point(AnchorPoint::BottomLeft).y as u32;
        let strikethrough = glyph_bb.anchor_point(AnchorPoint::Center).y as u32;
        let underline = glyph_bb.size.height + 1; // 1px gap between baseline and underline

        let font = MonoFontBuilder::new().character_size(glyph_bb.size).build();

        assert_eq!(font.baseline, baseline);
        assert_eq!(
            font.strikethrough,
            DecorationDimensions::new(strikethrough, 1)
        );
        assert_eq!(font.underline, DecorationDimensions::new(underline, 1));
    }
}
