use crate::text::{Alignment, Baseline, LineHeight};

/// Text style.
///
/// A text style is used to set how text lines are laid out in a text drawable.
///
/// Use [`TextStyleBuilder`] to build a text style object.
///
/// See the [module-level documentation](super) for more information about text styles and examples.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
#[non_exhaustive]
pub struct TextStyle {
    /// Horizontal text alignment.
    pub alignment: Alignment,

    /// Text baseline.
    pub baseline: Baseline,

    /// Line height.
    pub line_height: LineHeight,
}

impl TextStyle {
    /// Creates a new text style with the given baseline.
    pub const fn with_baseline(baseline: Baseline) -> Self {
        TextStyleBuilder::new().baseline(baseline).build()
    }

    /// Creates a new text style with the given alignment.
    pub const fn with_alignment(alignment: Alignment) -> Self {
        TextStyleBuilder::new().alignment(alignment).build()
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        TextStyleBuilder::new().build()
    }
}

/// Builder for text styles.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct TextStyleBuilder {
    style: TextStyle,
}

impl TextStyleBuilder {
    /// Creates a new text style builder.
    pub const fn new() -> Self {
        Self {
            style: TextStyle {
                alignment: Alignment::Left,
                baseline: Baseline::Alphabetic,
                line_height: LineHeight::Percent(100),
            },
        }
    }
}

impl TextStyleBuilder {
    /// Sets the horizontal text alignment.
    pub const fn alignment(mut self, alignment: Alignment) -> Self {
        self.style.alignment = alignment;

        self
    }

    /// Sets the text baseline.
    pub const fn baseline(mut self, baseline: Baseline) -> Self {
        self.style.baseline = baseline;

        self
    }

    /// Sets the line height.
    pub const fn line_height(mut self, line_height: LineHeight) -> Self {
        self.style.line_height = line_height;

        self
    }

    /// Builds the text style.
    pub const fn build(self) -> TextStyle {
        self.style
    }
}

impl From<&TextStyle> for TextStyleBuilder {
    fn from(style: &TextStyle) -> Self {
        Self { style: *style }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder() {
        let text_style = TextStyleBuilder::new()
            .alignment(Alignment::Right)
            .baseline(Baseline::Top)
            .line_height(LineHeight::Pixels(123))
            .build();

        assert_eq!(text_style.alignment, Alignment::Right);
        assert_eq!(text_style.baseline, Baseline::Top);
        assert_eq!(text_style.line_height, LineHeight::Pixels(123));
    }

    #[test]
    fn builder_default() {
        let text_style = TextStyleBuilder::new().build();

        assert_eq!(text_style.alignment, Alignment::Left);
        assert_eq!(text_style.baseline, Baseline::Alphabetic);
        assert_eq!(text_style.line_height, LineHeight::Percent(100));
    }
}
