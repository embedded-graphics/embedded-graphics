//! Pixel based fonts
//!
//! # Examples
//!
//! The examples below use the [`Font6x8`] font, however any of the [font
//! types in this module](#stucts) can be substituted.
//!
//! ## Write some text to the screen at the top left corner
//!
//! ```rust
//! use embedded_graphics::prelude::*;
//! use embedded_graphics::fonts::{Text, Font6x8};
//! use embedded_graphics::egtext;
//! use embedded_graphics::style::TextStyle;
//! # use embedded_graphics::mock_display::MockDisplay;
//! # use embedded_graphics::pixelcolor::BinaryColor;
//! # let mut display: MockDisplay<BinaryColor> = MockDisplay::default();
//!
//! // Create a text object and convert it into a styled text
//! Text::new("Hello Rust!", Point::zero())
//!     .into_styled(TextStyle::with_text_color(Font6x8, BinaryColor::On))
//!     .draw(&mut display);
//!
//! // Use the egtext macro instead
//! egtext!("Hello Rust!", Point::zero(), font = Font6x8).draw(&mut display);
//! ```
//!
//! ## Translate text by (20px, 30px)
//!
//! ```rust
//! use embedded_graphics::prelude::*;
//! use embedded_graphics::fonts::{Text, Font6x8};
//! use embedded_graphics::style::TextStyle;
//! # use embedded_graphics::mock_display::MockDisplay;
//! # use embedded_graphics::pixelcolor::BinaryColor;
//! # let mut display: MockDisplay<BinaryColor> = MockDisplay::default();
//!
//! Text::new("Hello Rust!", Point::zero())
//!     .into_styled(TextStyle::with_text_color(Font6x8, BinaryColor::On))
//!     .translate(Point::new(20, 30))
//!     .draw(&mut display);
//!
//! // this is equivalent to:
//!
//! Text::new("Hello Rust!", Point::new(20, 30))
//!     .into_styled(TextStyle::with_text_color(Font6x8, BinaryColor::On))
//!     .draw(&mut display);
//! ```
//!
//! ## Add some styling to the text
//!
//! Text can be styled by setting style properties on a [`TextStyle`] object.
//! The style properties provided by [`TextStyle`] are also accessible using the
//! [`egtext`] macro.
//!
//! ```rust
//! use embedded_graphics::prelude::*;
//! use embedded_graphics::egtext;
//! use embedded_graphics::fonts::{Text, Font6x8};
//! use embedded_graphics::pixelcolor::Rgb565;
//! use embedded_graphics::style::TextStyle;
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display = MockDisplay::default();
//!
//! egtext!(
//!     "Hello Rust!",
//!     Point::new(20, 30),
//!     font = Font6x8,
//!     text_color = Some(Rgb565::YELLOW),
//!     background_color = Some(Rgb565::BLUE),
//! ).draw(&mut display);
//!
//! // this is equivalent to:
//!
//! let style = TextStyle {
//!     font: Font6x8,
//!     text_color: Some(Rgb565::YELLOW),
//!     background_color: Some(Rgb565::BLUE),
//! };
//!
//! Text::new("Hello Rust!", Point::new(20, 30))
//!     .into_styled(style)
//!     .draw(&mut display);
//! ```
//!
//! ## Use `write!()` and arrayvec to render a formatted string
//!
//! This example uses arrayvec's [`ArrayString`] to render a floating point value using the
//! [`write!()`] macro. These strings have a fixed length, but allow the use of Rust's builtin
//! string formatting.
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
//! [`egtext`]: ../macro.egtext.html
//! [`Font6x8`]: ./type.Font6x8.html
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

/// Font
pub trait Font {
    /// Raw image containing the font
    const FONT_IMAGE: &'static [u8];
    /// `char` height of the font
    const CHAR_HEIGHT: u32;

    /// `char` width of the font
    const CHAR_WIDTH: u32;
    /// Font image width, must be divisible by `8` and `CHAR_WIDTH`.
    const FONT_IMAGE_WIDTH: u32 = 240;
    /// Returns the index in the font of the correponding `char`
    fn char_offset(_: char) -> u32;
}

/// TODO: docs
/// TODO: don't require font as first parameter
#[macro_export]
macro_rules! egtext {
    ($text:expr, $position:expr,
        font = $font:expr $(, $style_key:ident = $style_value:expr )* $(,)?) => {{

        use $crate::pixelcolor::BinaryColor;
        use $crate::style::TextStyle;
        use $crate::geometry::Point;

        #[allow(unused_mut)]
        let mut style = TextStyle::with_text_color($font, BinaryColor::On.into());
        $( style.$style_key = $style_value; )*

        $crate::fonts::Text::new($text, $position).into_styled(style)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
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
