//! Text drawable.
//!
//! TODO: - Add an overview of the e-g text rendering, e.g. what a `Text` object is used for and
//!         how it can be used with different text styles/renderers.
//!       - Describe the difference between `TextStyle` and `MonoTextStyle`
//!       - How is `Text::position` used by text renders dependent on `TextStyle::baseline`
//!       - Add examples:
//!         - Basic text without `TextStyle`
//!         - Advanced text with `TextStyle`
//!         - Draw text with multiple styles by using the result of `Text.draw(...)`
//!       - Link to `renderer` module docs for users who want to implement custom renderers.

pub mod renderer;
mod text;
mod text_style;

pub use text::Text;
pub use text_style::{TextStyle, TextStyleBuilder};

/// Text baseline.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Baseline {
    /// Top.
    Top,
    /// Bottom.
    Bottom,
    /// Middle.
    Middle,
    /// Alphabetic baseline.
    Alphabetic,
}

/// Horizontal text alignment.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Alignment {
    /// Left.
    Left,
    /// Center.
    Center,
    /// Right.
    Right,
}
/// Text decoration color.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum DecorationColor<C> {
    /// No text decoration.
    None,
    /// Text decoration with the same color as the text.
    TextColor,
    /// Text decoration with a custom color.
    Custom(C),
}

impl<C> DecorationColor<C> {
    /// Returns `true` if the decoration_color is `None`.
    pub fn is_none(&self) -> bool {
        // MSRV: replace with matches! for rust >= 1.42.0
        match self {
            Self::None => true,
            _ => false,
        }
    }

    /// Returns `true` if the decoration_color is `TextColor`.
    pub fn is_text_color(&self) -> bool {
        // MSRV: replace with matches! for rust >= 1.42.0
        match self {
            Self::TextColor => true,
            _ => false,
        }
    }

    /// Returns `true` if the decoration_color is `Custom`.
    pub fn is_custom(&self) -> bool {
        // MSRV: replace with matches! for rust >= 1.42.0
        match self {
            Self::Custom(_) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelcolor::BinaryColor;

    #[test]
    fn decoration_color_is_methods() {
        let none = DecorationColor::<BinaryColor>::None;
        assert!(none.is_none());
        assert!(!none.is_text_color());
        assert!(!none.is_custom());

        let text_color = DecorationColor::<BinaryColor>::TextColor;
        assert!(!text_color.is_none());
        assert!(text_color.is_text_color());
        assert!(!text_color.is_custom());

        let custom = DecorationColor::Custom(BinaryColor::On);
        assert!(!custom.is_none());
        assert!(!custom.is_text_color());
        assert!(custom.is_custom());
    }
}
