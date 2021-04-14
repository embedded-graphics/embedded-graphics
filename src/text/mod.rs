//! Text drawing.
//!
//! The [`Text`] drawable can be used to draw text on a draw target. To construct a [`Text`] object
//! at least a text string, position and character style are required. For advanced formatting
//! options an additional [`TextStyle`] object might be required.
//!
//! Text rendering in embedded-graphics is designed to be extendable by text renderers for different
//! font formats. To use a text renderer in an embedded-graphics project each renderer provides a
//! character style object. This object is used to set the appearance of characters, like the text
//! color or the used font. The available settings vary between different text renderer and are
//! documented in the text renderer documentation.
//!
//! See the [`renderer` module] docs for more information about implementing custom text renderers.
//!
//! Embedded-graphics includes a text renderer for monospaced fonts in the [`mono_font`] module.
//! Most examples will use this renderer and the associated [`MonoTextStyle`] character style.
//! But they should be easily adaptable to any external renderer listed in the
//! [external crates list].
//!
//! # Text style
//!
//! In addition to styling the individual characters the [`Text`] drawable also contains a
//! [`TextStyle`] setting. The text style is used to set the alignment and line spacing of text
//! objects.
//!
//! The [`alignment`] setting sets the horizontal alignment of the text. With the default value
//! `Left` the text will be rendered to the right of the given text position. Analogously `Right`
//! aligned text will be rendered to the left of the given position. `Center`ed text will extend
//! equally to the left and right of the text position.
//!
//! The [`baseline`] setting defines the vertical alignment of the first line of text. With the default
//! setting of `Alphabetic` the glyphs will be drawn with their descenders below the given position.
//! This means that the bottom of glyphs without descender (like 'A') will be on the same Y
//! coordinate as the given position. The other baseline settings will position the glyphs relative
//! to the EM box, without considering the baseline.
//!
//! If the text contains multiple lines only the first line will be vertically aligned based on the
//! baseline setting. All following lines will be spaced relative to the first line, according to the [`line_height`] setting.
//!
//! # Examples
//!
//! ## Draw basic text
//!
//! ```
//! use embedded_graphics::{
//!     mono_font::{ascii::FONT_6X10, MonoTextStyle},
//!     pixelcolor::Rgb565,
//!     prelude::*,
//!     text::Text,
//! };
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display: MockDisplay<Rgb565> = MockDisplay::default();
//! # display.set_allow_out_of_bounds_drawing(true);
//!
//! // Create a new character style
//! let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
//!
//! // Create a text at position (20, 30) and draw it using the previously defined style
//! Text::new("Hello Rust!", Point::new(20, 30), style).draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//! ## Draw centered text
//!
//! [`Text`] provides the [`with_alignment`] and [`with_baseline`] constructors to easily set
//! these commonly used settings without having to build a [`TextStyle`] object first.
//!
//! ```
//! use embedded_graphics::{
//!     mono_font::{ascii::FONT_6X10, MonoTextStyle},
//!     pixelcolor::Rgb565,
//!     prelude::*,
//!     text::{Text, Alignment},
//! };
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display: MockDisplay<Rgb565> = MockDisplay::default();
//! # display.set_allow_out_of_bounds_drawing(true);
//!
//! // Create a new character style
//! let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
//!
//! // Create a text at position (20, 30) and draw it using the previously defined style
//! Text::with_alignment(
//!     "First line\nSecond line",
//!     Point::new(20, 30),
//!     style,
//!     Alignment::Center,
//! )
//! .draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//!
//! ## Draw text with `TextStyle`
//!
//! For more advanced text styles a [`TextStyle`] object can be build using the
//! [`TextStyleBuilder`] and then passed to the [`with_text_style`] constructor.
//!
//! ```
//! use embedded_graphics::{
//!     mono_font::{ascii::FONT_6X10, MonoTextStyle},
//!     pixelcolor::Rgb565,
//!     prelude::*,
//!     text::{Alignment, LineHeight, Text, TextStyleBuilder},
//! };
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display: MockDisplay<Rgb565> = MockDisplay::default();
//! # display.set_allow_out_of_bounds_drawing(true);
//!
//! // Create a new character style.
//! let character_style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
//!
//! // Create a new text style.
//! let text_style = TextStyleBuilder::new()
//!     .alignment(Alignment::Center)
//!     .line_height(LineHeight::Percent(150))
//!     .build();
//!
//! // Create a text at position (20, 30) and draw it using the previously defined style.
//! Text::with_text_style(
//!     "First line\nSecond line",
//!     Point::new(20, 30),
//!     character_style,
//!     text_style,
//! )
//! .draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//!
//! ## Combine different character styles
//!
//! The `draw` method for text drawables returns the position of the next character. This can be
//! used to combine text with different character styles on a single line of text.
//!
//! ```
//! use embedded_graphics::{
//!     mono_font::{ascii::{FONT_6X10, FONT_10X20}, MonoTextStyle},
//!     pixelcolor::Rgb565,
//!     prelude::*,
//!     text::{Alignment, LineHeight, Text, TextStyleBuilder},
//! };
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display: MockDisplay<Rgb565> = MockDisplay::default();
//! # display.set_allow_out_of_bounds_drawing(true);
//!
//! // Create a small and a large character style.
//! let small_style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
//! let large_style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
//!
//! // Draw the first text at (20, 30) using the small character style.
//! let next = Text::new("small ", Point::new(20, 30), small_style).draw(&mut display)?;
//!
//! // Draw the second text after the first text using the large character style.
//! let next = Text::new("large", next, large_style).draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//!
//! [`Text`]: struct.Text.html
//! [`Text::new`]: struct.TextStyle.html#method.new
//! [`with_alignment`]: struct.Text.html#method.with_alignment
//! [`with_baseline`]: struct.Text.html#method.with_baseline
//! [`with_text_style`]: struct.Text.html#method.with_text_style
//! [`TextStyle`]: struct.TextStyle.html
//! [`alignment`]: struct.TextStyle.html#structfield.alignment
//! [`baseline`]: struct.TextStyle.html#structfield.baseline
//! [`line_height`]: struct.TextStyle.html#structfield.line_height
//! [`TextStyleBuilder`]: struct.TextStyleBuilder.html
//! [`mono_font`]: ../mono_font/index.html
//! [`MonoTextStyle`]: ../mono_font/struct.MonoTextStyle.html
//! [`renderer` module]: renderer/index.html
//! [external crates list]: ../index.html#additional-functions-provided-by-external-crates

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
