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

use embedded_graphics_core::prelude::PixelColor;
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

impl<C: PixelColor> DecorationColor<C> {
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

    pub(crate) fn to_color(&self, text_color: Option<C>) -> Option<C> {
        match self {
            DecorationColor::TextColor => text_color,
            DecorationColor::Custom(custom_color) => Some(*custom_color),
            DecorationColor::None => None,
        }
    }
}

/// Text line height.
///
/// The line height is defined as the vertical distance between the baseline of two adjacent lines
/// of text.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum LineHeight {
    /// Absolute line height in pixels.
    Pixels(u32),

    /// Relative line height in percent of the default line height.
    Percent(u32),
}

impl LineHeight {
    /// Converts the line height to an absolute pixel distance.
    ///
    /// # Examples
    ///
    /// ```
    /// use embedded_graphics::text::LineHeight;
    ///
    /// let relative_height = LineHeight::Percent(150);
    /// assert_eq!(relative_height.to_absolute(20), 30);
    /// ```
    pub fn to_absolute(self, base_line_height: u32) -> u32 {
        match self {
            Self::Pixels(px) => px,
            Self::Percent(percent) => base_line_height * percent / 100,
        }
    }
}

impl Default for LineHeight {
    fn default() -> Self {
        Self::Percent(100)
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

    #[test]
    fn line_height_to_absolute() {
        assert_eq!(LineHeight::Pixels(100).to_absolute(20), 100);
        assert_eq!(LineHeight::Percent(100).to_absolute(20), 20);
        assert_eq!(LineHeight::Percent(150).to_absolute(20), 30);
    }
}
