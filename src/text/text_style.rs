use embedded_graphics_core::text::CharacterStyle;

use crate::text::{HorizontalAlignment, TextRenderer, VerticalAlignment};

/// Text style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextStyle<S> {
    /// Character style.
    pub character_style: S,

    /// Horizontal alignment.
    pub horizontal_alignment: HorizontalAlignment,

    /// Vertical alignment.
    pub vertical_alignment: VerticalAlignment,
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
                horizontal_alignment: HorizontalAlignment::Left,
                vertical_alignment: VerticalAlignment::Baseline,
            },
        }
    }
}

impl<S> TextStyleBuilder<S> {
    /// Sets the horizontal alignment.
    pub fn horizontal_alignment(mut self, horizontal_alignment: HorizontalAlignment) -> Self {
        self.style.horizontal_alignment = horizontal_alignment;

        self
    }

    /// Sets the vertical alignment.
    pub fn vertical_alignment(mut self, vertical_alignment: VerticalAlignment) -> Self {
        self.style.vertical_alignment = vertical_alignment;

        self
    }

    /// Sets the character style.
    pub fn character_style<Style>(self, character_style: Style) -> TextStyleBuilder<Style> {
        TextStyleBuilder {
            style: TextStyle {
                character_style,
                horizontal_alignment: self.style.horizontal_alignment,
                vertical_alignment: self.style.vertical_alignment,
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
            .horizontal_alignment(HorizontalAlignment::Right)
            .vertical_alignment(VerticalAlignment::Top)
            .build();

        assert_eq!(text_style.horizontal_alignment, HorizontalAlignment::Right);
        assert_eq!(text_style.vertical_alignment, VerticalAlignment::Top);
    }
}
