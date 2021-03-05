use embedded_graphics_core::text::CharacterStyle;

use crate::text::{Alignment, Baseline, TextRenderer};

/// Text style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextStyle<S> {
    /// Character style.
    pub character_style: S,

    /// Horizontal text alignment.
    pub alignment: Alignment,

    /// Text baseline.
    pub baseline: Baseline,
}

impl<S> TextStyle<S> {}

/// Text style builder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextStyleBuilder<S> {
    style: TextStyle<S>,
}

impl TextStyleBuilder<UndefinedCharacterStyle> {
    /// Creates a new text style builder.
    pub fn new() -> Self {
        Self {
            style: TextStyle {
                character_style: UndefinedCharacterStyle,
                alignment: Alignment::Left,
                baseline: Baseline::Alphabetic,
            },
        }
    }
}

impl<S> TextStyleBuilder<S> {
    /// Sets the horizontal text alignment.
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.style.alignment = alignment;

        self
    }

    /// Sets the text baseline.
    pub fn baseline(mut self, baseline: Baseline) -> Self {
        self.style.baseline = baseline;

        self
    }

    /// Sets the character style.
    pub fn character_style<Style>(self, character_style: Style) -> TextStyleBuilder<Style> {
        TextStyleBuilder {
            style: TextStyle {
                character_style,
                alignment: self.style.alignment,
                baseline: self.style.baseline,
            },
        }
    }
}

impl<S> TextStyleBuilder<S>
where
    S: CharacterStyle + TextRenderer,
{
    /// Builds the text style.
    pub fn build(self) -> TextStyle<S> {
        self.style
    }
}

/// Undefined character style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UndefinedCharacterStyle;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        mono_font::{ascii::Font6x9, MonoTextStyleBuilder},
        pixelcolor::BinaryColor,
    };

    // TODO: add tests

    #[test]
    fn builder_alignments() {
        let character_style = MonoTextStyleBuilder::<BinaryColor, _>::new()
            .font(Font6x9)
            .build();

        let text_style = TextStyleBuilder::new()
            .character_style(character_style)
            .alignment(Alignment::Right)
            .baseline(Baseline::Top)
            .build();

        assert_eq!(text_style.alignment, Alignment::Right);
        assert_eq!(text_style.baseline, Baseline::Top);
    }
}
