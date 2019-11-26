//! Monospaced bitmap fonts.
//!
//! This module contains support for drawing monospaced bitmap fonts and provides
//! several [built-in fonts].
//!
//! Additional custom fonts can be added by the application or other crates. This
//! is demonstrated in the `custom-font` example in the simulator crate.
//!
//! # Examples
//!
//! The examples below use the [`Font6x8`] font, however any of the [built-in fonts]
//! in this module or custom fonts can be substituted.
//!
//! ## Draw text without using `egtext` macro
//!
//! Text can be drawn to a display by creating a [`Text`] object and attaching a
//! text style to it by using a [`Styled`] object. By creating the text style manually,
//! without using the [`egtext`] macro, it can be reused to style multiple text objects.
//!
//! ```rust
//! use embedded_graphics::prelude::*;
//! use embedded_graphics::fonts::{Text, Font6x8};
//! use embedded_graphics::style::TextStyle;
//! use embedded_graphics::pixelcolor::Rgb565;
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display: MockDisplay<Rgb565> = MockDisplay::default();
//!
//! // Create a new text style
//! let style = TextStyle {
//!     font: Font6x8,
//!     text_color: Some(Rgb565::YELLOW),
//!     background_color: Some(Rgb565::BLUE),
//! };
//!
//! // Create a text at position (20, 30) and draw it using the previously defined style
//! Text::new("Hello Rust!", Point::new(20, 30))
//!     .into_styled(style)
//!     .draw(&mut display);
//! ```
//! ## Draw text using the `egtext` macro
//!
//! Creating styled texts can be simplified by using the [`egtext`] macro.
//! All style properties in [`TextStyle`] can be set by using assignments inside
//! the [`egtext`] macro call.
//!
//! The following example draws the same text as the previous example but uses
//! the [`egtext`] macro to build the necessary styled text objects.
//!
//! ```rust
//! use embedded_graphics::prelude::*;
//! use embedded_graphics::fonts::Font6x8;
//! use embedded_graphics::egtext;
//! use embedded_graphics::pixelcolor::Rgb565;
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display: MockDisplay<Rgb565> = MockDisplay::default();
//!
//! egtext!(
//!     "Hello Rust!",
//!     Point::new(20, 30),
//!     font = Font6x8,
//!     text_color = Some(Rgb565::YELLOW),
//!     background_color = Some(Rgb565::BLUE),
//! ).draw(&mut display);
//! ```
//! ## Translate text by (20px, 30px)
//!
//! ```rust
//! use embedded_graphics::prelude::*;
//! use embedded_graphics::fonts::{Text, Font6x8};
//! use embedded_graphics::style::TextStyle;
//! use embedded_graphics::pixelcolor::BinaryColor;
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display: MockDisplay<BinaryColor> = MockDisplay::default();
//!
//! Text::new("Hello Rust!", Point::zero())
//!     .into_styled(TextStyle::new(Font6x8, BinaryColor::On))
//!     .translate(Point::new(20, 30))
//!     .draw(&mut display);
//!
//! // this is equivalent to:
//!
//! Text::new("Hello Rust!", Point::new(20, 30))
//!     .into_styled(TextStyle::new(Font6x8, BinaryColor::On))
//!     .draw(&mut display);
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
//! use embedded_graphics::fonts::Font6x8;
//! use embedded_graphics::pixelcolor::Rgb565;
//! use embedded_graphics::prelude::*;
//! use embedded_graphics::egtext;
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display = MockDisplay::default();
//!
//! let value = 12.34567;
//!
//! // Create a fixed buffer of length 12
//! let mut buf = ArrayString::<[_; 12]>::new();
//!
//! // Output `Value: 12.35`
//! write!(&mut buf, "Value: {:.2}", value).expect("Failed to write to buffer");
//!
//! egtext!(
//!     &buf,
//!     Point::zero(),
//!     font = Font6x8,
//!     text_color = Some(Rgb565::YELLOW),
//!     background_color = Some(Rgb565::BLUE),
//! ).draw(&mut display);
//! ```
//!
//! # Built-in fonts
//!
//! | Type | Screenshot |
//! |------|------------|
//! | [`Font6x8`] | ![6x8 font spritemap screenshot](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/embedded-graphics/data/font6x8.png) |
//! | [`Font6x12`] | ![6x12 font spritemap screenshot](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/embedded-graphics/data/font6x12.png) |
//! | [`Font8x16`] | ![8x16 font spritemap screenshot](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/embedded-graphics/data/font8x16.png) |
//! | [`Font12x16`] | ![12x16 font spritemap screenshot](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/embedded-graphics/data/font12x16.png) |
//! | [`Font24x32`] | The 24x32 font is a pixel doubled version of the 12x16 font. |
//!
//! [built-in fonts]: #built-in-fonts
//! [`egtext`]: ../macro.egtext.html
//! [`Font6x8`]: struct.Font6x8.html
//! [`Font6x12`]: struct.Font6x12.html
//! [`Font8x16`]: struct.Font8x16.html
//! [`Font12x16`]: struct.Font12x16.html
//! [`Font24x32`]: struct.Font24x32.html
//! [`Text`]: struct.Text.html
//! [`Styled`]: ../style/struct.Styled.html
//! [`TextStyle`]: ../style/struct.TextStyle.html
//! [`ArrayString`]: https://docs.rs/arrayvec/0.4.11/arrayvec/struct.ArrayString.html
//! [`write!()`]: https://doc.rust-lang.org/nightly/std/macro.write.html

mod font12x16;
mod font24x32;
mod font6x12;
mod font6x8;
mod font8x16;
mod text;

pub use text::{StyledTextIterator, Text};

pub use font12x16::Font12x16;
pub use font24x32::Font24x32;
pub use font6x12::Font6x12;
pub use font6x8::Font6x8;
pub use font8x16::Font8x16;

use crate::geometry::Size;

/// Monospaced bitmap font.
pub trait Font {
    /// Raw image data containing the font.
    const FONT_IMAGE: &'static [u8];

    /// The width of the raw image data.
    ///
    /// The width must be divisible by `8` and `CHARACTER_SIZE.width`.
    const FONT_IMAGE_WIDTH: u32;

    /// Size of a single character in pixel.
    const CHARACTER_SIZE: Size;

    /// Spacing between characters.
    ///
    /// The spacing defines how many empty pixels are added horizontally between adjacent characters
    /// on a single line of text.
    const CHARACTER_SPACING: u32 = 0;

    /// Returns the position a character in the font.
    fn char_offset(_: char) -> u32;
}

/// Creates a styled text.
///
/// The `egtext` macro expects the text, the position and styling properties as arguments.
///
/// # Examples
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::pixelcolor::Rgb888;
/// use embedded_graphics::fonts::Font6x8;
/// use embedded_graphics::egtext;
///
/// let text = egtext!(
///     "text",
///     Point::zero(),
///     font = Font6x8, // font needs to be the first styling property
///     text_color = Some(Rgb888::RED),
/// );
/// ```
#[macro_export]
macro_rules! egtext {
    ($text:expr, $position:expr,
        font = $font:expr $(, $style_key:ident = $style_value:expr )* $(,)?) => {{

        let color = $crate::pixelcolor::BinaryColor::On.into();
        #[allow(unused_mut)]
        let mut style = $crate::style::TextStyle::new($font, color);
        $( style.$style_key = $style_value; )*

        $crate::fonts::Text::new($text, $position).into_styled(style)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;
    use crate::pixelcolor::{BinaryColor, Rgb565, RgbColor};
    use crate::style::{Styled, TextStyle};

    #[test]
    fn font_macros() {
        let _text: Styled<Text<'_>, TextStyle<BinaryColor, Font6x8>> =
            egtext!("Hello!", Point::zero(), font = Font6x8);
        let _text: Styled<Text<'_>, TextStyle<BinaryColor, Font6x12>> =
            egtext!("Hello!", Point::zero(), font = Font6x12);
        let _text: Styled<Text<'_>, TextStyle<BinaryColor, Font8x16>> =
            egtext!("Hello!", Point::zero(), font = Font8x16);
        let _text: Styled<Text<'_>, TextStyle<BinaryColor, Font12x16>> =
            egtext!("Hello!", Point::zero(), font = Font12x16);
        let _text: Styled<Text<'_>, TextStyle<BinaryColor, Font24x32>> =
            egtext!("Hello!", Point::zero(), font = Font24x32);
    }

    #[test]
    fn styled_text() {
        let _text: Styled<Text<'_>, TextStyle<Rgb565, Font6x8>> = egtext!(
            "Hello!",
            Point::zero(),
            font = Font6x8,
            text_color = Some(Rgb565::RED)
        );
        let _text: Styled<Text<'_>, TextStyle<Rgb565, Font6x12>> = egtext!(
            "Hello!",
            Point::zero(),
            font = Font6x12,
            text_color = Some(Rgb565::GREEN)
        );
        let _text: Styled<Text<'_>, TextStyle<Rgb565, Font8x16>> = egtext!(
            "Hello!",
            Point::zero(),
            font = Font8x16,
            text_color = Some(Rgb565::BLUE)
        );
        let _text: Styled<Text<'_>, TextStyle<Rgb565, Font12x16>> = egtext!(
            "Hello!",
            Point::zero(),
            font = Font12x16,
            text_color = Some(Rgb565::YELLOW)
        );
        let _text: Styled<Text<'_>, TextStyle<Rgb565, Font24x32>> = egtext!(
            "Hello!",
            Point::zero(),
            font = Font24x32,
            text_color = Some(Rgb565::MAGENTA)
        );
    }
}
