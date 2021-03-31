use crate::text::{renderer::CharacterStyle, Alignment, Baseline, LineHeight};

/// Text style.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[non_exhaustive]
pub struct TextStyle<S> {
    /// Character style.
    pub character_style: S,

    /// Horizontal text alignment.
    pub alignment: Alignment,

    /// Text baseline.
    pub baseline: Baseline,

    /// Line height.
    pub line_height: LineHeight,
}

/// Text style builder.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
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
                line_height: LineHeight::default(),
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

    /// Sets the line height.
    pub fn line_height(mut self, line_height: LineHeight) -> Self {
        self.style.line_height = line_height;

        self
    }

    /// Sets the character style.
    pub fn character_style<Style>(self, character_style: Style) -> TextStyleBuilder<Style> {
        TextStyleBuilder {
            style: TextStyle {
                character_style,
                alignment: self.style.alignment,
                baseline: self.style.baseline,
                line_height: self.style.line_height,
            },
        }
    }
}

impl<S> TextStyleBuilder<S>
where
    S: CharacterStyle,
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
        mono_font::{ascii::FONT_6X9, MonoTextStyleBuilder},
        pixelcolor::BinaryColor,
    };

    #[test]
    fn builder() {
        let character_style = MonoTextStyleBuilder::<BinaryColor>::new()
            .font(&FONT_6X9)
            .build();

        let text_style = TextStyleBuilder::new()
            .character_style(character_style)
            .alignment(Alignment::Right)
            .baseline(Baseline::Top)
            .line_height(LineHeight::Pixels(123))
            .build();

        assert_eq!(text_style.character_style, character_style);
        assert_eq!(text_style.alignment, Alignment::Right);
        assert_eq!(text_style.baseline, Baseline::Top);
        assert_eq!(text_style.line_height, LineHeight::Pixels(123));
    }

    #[test]
    fn builder_default() {
        let character_style = MonoTextStyleBuilder::<BinaryColor>::new()
            .font(&FONT_6X9)
            .build();

        // Setting a `character_style` is required to be able to call `build`.
        let text_style = TextStyleBuilder::new()
            .character_style(character_style)
            .build();

        assert_eq!(text_style.character_style, character_style);
        assert_eq!(text_style.alignment, Alignment::Left);
        assert_eq!(text_style.baseline, Baseline::Alphabetic);
        assert_eq!(text_style.line_height, LineHeight::Percent(100));
    }
}
