//! Pixel based fonts
//!
//! # Examples
//!
//! The examples below use the [`Font6x8`] font and the [`text_6x8`] macro, however any of the [font
//! types in this module](#types) or [`text_*`] macros can be substituted.
//!
//! ## Write some text to the screen at the default `(0, 0)` position
//!
//! ```rust
//! use embedded_graphics::prelude::*;
//! use embedded_graphics::fonts::{Text, FONT6X8};
//! use embedded_graphics::egtext;
//! use embedded_graphics::style::TextStyle;
//! # use embedded_graphics::mock_display::MockDisplay;
//! # use embedded_graphics::pixelcolor::BinaryColor;
//! # let mut display: MockDisplay<BinaryColor> = MockDisplay::default();
//!
//! // Use struct methods directly
//! Text::new("Hello Rust!", Point::zero()).into_styled(TextStyle::with_text_color(FONT6X8, BinaryColor::On)).draw(&mut display);
//!
//! // Use a macro instead
//! egtext!("Hello Rust!", font = FONT6X8).draw(&mut display);
//! ```
//!
//! ## Translate text by (20px, 30px)
//!
//! ```rust
//! use embedded_graphics::prelude::*;
//! use embedded_graphics::fonts::{Text, FONT6X8};
//! use embedded_graphics::style::TextStyle;
//! # use embedded_graphics::mock_display::MockDisplay;
//! # use embedded_graphics::pixelcolor::BinaryColor;
//! # let mut display: MockDisplay<BinaryColor> = MockDisplay::default();
//!
//! Text::new("Hello Rust!", Point::zero()).into_styled(TextStyle::with_text_color(FONT6X8, BinaryColor::On))
//!     .translate(Point::new(20, 30))
//!     .draw(&mut display)
//! ```
//!
//! ## Add some styling to the text
//!
//! Text can be styled by setting style properties on a [`TextStyle`] object.
//! The style properties provided by [`TextStyle`] are also accessible using the
//! [`text_*`] macros.
//!
//! ```rust
//! use embedded_graphics::prelude::*;
//! use embedded_graphics::egtext;
//! use embedded_graphics::fonts::{Text, FONT6X8};
//! use embedded_graphics::pixelcolor::Rgb565;
//! use embedded_graphics::style::TextStyle;
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display = MockDisplay::default();
//!
//! egtext!(
//!     "Hello Rust!",
//!     font = FONT6X8,
//!     text_color = Some(Rgb565::YELLOW),
//!     background_color = Some(Rgb565::BLUE),
//! ).draw(&mut display);
//!
//! let style = TextStyle {
//!     font: FONT6X8,
//!     text_color: Some(Rgb565::YELLOW),
//!     background_color: Some(Rgb565::BLUE),
//! };
//!
//! Text::new("Hello Rust!", Point::zero())
//!      .into_styled(style)
//!     .translate(Point::new(20, 30))
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
//! use embedded_graphics::fonts::FONT6X8;
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
//!     font = FONT6X8,
//!     text_color = Some(Rgb565::YELLOW),
//!     background_color = Some(Rgb565::BLUE),
//! ).draw(&mut display);
//! ```
//!
//! [`text_6x8`]: ../macro.text_6x8.html
//! [`text_*`]: ../index.html#macros
//! [`Font6x8`]: ./type.Font6x8.html
//! [`TextStyle`]: ../style/struct.TextStyle.html
//! [`ArrayString`]: https://docs.rs/arrayvec/0.4.11/arrayvec/struct.ArrayString.html
//! [`write!()`]: https://doc.rust-lang.org/nightly/std/macro.write.html

//pub mod font_builder;
mod font;
mod font12x16;
mod font24x32;
mod font6x12;
mod font6x8;
mod font8x16;
mod text;

pub use font::{Font, FontIterator};
pub use text::Text;

pub use font12x16::{Font12x16, FONT12X16};
pub use font24x32::{Font24x32, FONT24X32};
pub use font6x12::{Font6x12, FONT6X12};
pub use font6x8::{Font6x8, FONT6X8};
pub use font8x16::{Font8x16, FONT8X16};

//use crate::geometry::Dimensions;
//use crate::pixelcolor::PixelColor;
//use crate::style::TextStyle;

/// TODO: docs
/// TODO: don't require font as first parameter
/// TODO: position parameter
#[macro_export]
macro_rules! egtext {
    ($text:expr, font = $font:expr $(, $style_key:ident = $style_value:expr )* $(,)?) => {{
        use $crate::pixelcolor::BinaryColor;
        use $crate::style::TextStyle;
        use $crate::geometry::Point;

        #[allow(unused_mut)]
        let mut style = TextStyle::with_text_color($font, BinaryColor::On.into());
        $( style.$style_key = $style_value; )*

        $crate::fonts::Text::new($text, Point::zero()).into_styled(style)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelcolor::{BinaryColor, Rgb565, RgbColor};
    use crate::style::{Styled, TextStyle};

    #[test]
    fn font_macros() {
        let _text: Styled<Text<'_>, TextStyle<'_, BinaryColor, Font6x8>> =
            egtext!("Hello!", font = FONT6X8);
        let _text: Styled<Text<'_>, TextStyle<'_, BinaryColor, Font6x12>> =
            egtext!("Hello!", font = FONT6X12);
        let _text: Styled<Text<'_>, TextStyle<'_, BinaryColor, Font8x16>> =
            egtext!("Hello!", font = FONT8X16);
        let _text: Styled<Text<'_>, TextStyle<'_, BinaryColor, Font12x16>> =
            egtext!("Hello!", font = FONT12X16);
        let _text: Styled<Text<'_>, TextStyle<'_, BinaryColor, Font24x32>> =
            egtext!("Hello!", font = FONT24X32);
    }

    #[test]
    fn styled_text() {
        let _text: Styled<Text<'_>, TextStyle<'_, Rgb565, Font6x8>> =
            egtext!("Hello!", font = FONT6X8, text_color = Some(Rgb565::RED));
        let _text: Styled<Text<'_>, TextStyle<'_, Rgb565, Font6x12>> =
            egtext!("Hello!", font = FONT6X12, text_color = Some(Rgb565::GREEN));
        let _text: Styled<Text<'_>, TextStyle<'_, Rgb565, Font8x16>> =
            egtext!("Hello!", font = FONT8X16, text_color = Some(Rgb565::BLUE));
        let _text: Styled<Text<'_>, TextStyle<'_, Rgb565, Font12x16>> = egtext!(
            "Hello!",
            font = FONT12X16,
            text_color = Some(Rgb565::YELLOW)
        );
        let _text: Styled<Text<'_>, TextStyle<'_, Rgb565, Font24x32>> = egtext!(
            "Hello!",
            font = FONT24X32,
            text_color = Some(Rgb565::MAGENTA)
        );
    }
}
