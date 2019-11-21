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
//! use embedded_graphics::fonts::Font6x8;
//! use embedded_graphics::text_6x8;
//! use embedded_graphics::style::TextStyle;
//! # use embedded_graphics::mock_display::MockDisplay;
//! # use embedded_graphics::pixelcolor::BinaryColor;
//! # let mut display: MockDisplay<BinaryColor> = MockDisplay::default();
//!
//! // Use struct methods directly
//! Font6x8::render_str("Hello Rust!", TextStyle::with_text_color(BinaryColor::On)).draw(&mut display);
//!
//! // Use a macro instead
//! text_6x8!("Hello Rust!").draw(&mut display);
//! ```
//!
//! ## Translate text by (20px, 30px)
//!
//! ```rust
//! use embedded_graphics::prelude::*;
//! use embedded_graphics::fonts::Font6x8;
//! use embedded_graphics::style::TextStyle;
//! # use embedded_graphics::mock_display::MockDisplay;
//! # use embedded_graphics::pixelcolor::BinaryColor;
//! # let mut display: MockDisplay<BinaryColor> = MockDisplay::default();
//!
//! Font6x8::render_str("Hello Rust!", TextStyle::with_text_color(BinaryColor::On))
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
//! use embedded_graphics::text_6x8;
//! use embedded_graphics::fonts::Font6x8;
//! use embedded_graphics::pixelcolor::Rgb565;
//! use embedded_graphics::style::TextStyle;
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display = MockDisplay::default();
//!
//! text_6x8!(
//!     "Hello Rust!",
//!     text_color = Some(Rgb565::YELLOW),
//!     background_color = Some(Rgb565::BLUE),
//! ).draw(&mut display);
//!
//! let style = TextStyle {
//!     text_color: Some(Rgb565::YELLOW),
//!     background_color: Some(Rgb565::BLUE),
//! };
//!
//! Font6x8::render_str("Hello Rust!", style)
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
//! use embedded_graphics::fonts::Font6x8;
//! use embedded_graphics::pixelcolor::Rgb565;
//! use embedded_graphics::prelude::*;
//! use embedded_graphics::text_6x8;
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
//! text_6x8!(
//!     &buf,
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

mod font12x16;
mod font24x32;
mod font6x12;
mod font6x8;
mod font8x16;
pub mod font_builder;

pub use self::font12x16::Font12x16;
pub use self::font24x32::Font24x32;
pub use self::font6x12::Font6x12;
pub use self::font6x8::Font6x8;
pub use self::font8x16::Font8x16;
use crate::geometry::Dimensions;
use crate::pixelcolor::PixelColor;
use crate::style::TextStyle;

/// Common methods for all fonts
pub trait Font<'a, C>: Dimensions
where
    C: PixelColor,
{
    /// Render a string in the implementing font's typeface.
    ///
    /// Defaults to 1u8 for stroke_color and 0u8 for fill_color
    ///
    /// ```rust
    /// use embedded_graphics::prelude::*;
    /// use embedded_graphics::fonts::Font6x8;
    /// use embedded_graphics::pixelcolor::Rgb565;
    /// use embedded_graphics::style::TextStyle;
    /// # use embedded_graphics::mock_display::MockDisplay as Display;
    ///
    /// fn main() {
    ///     let mut disp = Display::default();
    ///
    ///     // Render a string with red characters
    ///     let text = Font6x8::render_str("Hello world", TextStyle::with_text_color(Rgb565::RED));
    ///     text.draw(&mut disp);
    /// }
    /// ```
    fn render_str(chars: &'a str, style: TextStyle<C>) -> Self;
}

/// Internal macro used to implement `text_*` on fonts. Do not use directly!
#[doc(hidden)]
#[macro_export]
macro_rules! impl_text {
    ($Font:ident, $text:expr $(, $style_key:ident = $style_value:expr )* $(,)?) => {{
        use $crate::pixelcolor::BinaryColor;
        use $crate::style::TextStyle;

        #[allow(unused_mut)]
        let mut style = TextStyle::with_text_color(BinaryColor::On.into());
        $( style.$style_key = $style_value; )*

        $crate::fonts::$Font::render_str($text, style)
    }};
}

/// Render text using the [`Font6x8`](./fonts/type.Font6x8.html) font
///
/// ```rust
/// use embedded_graphics::{text_6x8, prelude::*, fonts::Font6x8, pixelcolor::Rgb565};
///
/// let text: Font6x8<Rgb565> = text_6x8!("Hello world!");
/// let styled_text: Font6x8<Rgb565> = text_6x8!(
///     "Hello world!",
///     text_color = Some(Rgb565::RED),
///     background_color = Some(Rgb565::GREEN)
/// );
/// ```
///
/// Style properties like `text_color` map to properties in the [`TextStyle`] struct.
///
/// [`TextStyle`]: style/struct.TextStyle.html
#[macro_export]
macro_rules! text_6x8 {
    ($text:expr $(, $style_key:ident = $style_value:expr )* $(,)?) => {
        $crate::impl_text!(Font6x8, $text $(, $style_key = $style_value )*)
    };
}

/// Render text using the [`Font6x12`](./fonts/type.Font6x12.html) font
///
/// ```rust
/// use embedded_graphics::{text_6x12, prelude::*, fonts::Font6x12, pixelcolor::Rgb565};
///
/// let text: Font6x12<Rgb565> = text_6x12!("Hello world!");
/// let styled_text: Font6x12<Rgb565> = text_6x12!(
///     "Hello world!",
///     text_color = Some(Rgb565::RED),
///     background_color = Some(Rgb565::GREEN)
/// );
/// ```
///
/// Style properties like `text_color` map to properties in the [`TextStyle`] struct.
///
/// [`TextStyle`]: style/struct.TextStyle.html
#[macro_export]
macro_rules! text_6x12 {
    ($text:expr $(, $style_key:ident = $style_value:expr )* $(,)?) => {
        $crate::impl_text!(Font6x12, $text $(, $style_key = $style_value )*)
    };
}

/// Render text using the [`Font8x16`](./fonts/type.Font8x16.html) font
///
/// ```rust
/// use embedded_graphics::{text_8x16, prelude::*, fonts::Font8x16, pixelcolor::Rgb565};
///
/// let text: Font8x16<Rgb565> = text_8x16!("Hello world!");
/// let styled_text: Font8x16<Rgb565> = text_8x16!(
///     "Hello world!",
///     text_color = Some(Rgb565::RED),
///     background_color = Some(Rgb565::GREEN)
/// );
/// ```
///
/// Style properties like `text_color` map to properties in the [`TextStyle`] struct.
///
/// [`TextStyle`]: style/struct.TextStyle.html
#[macro_export]
macro_rules! text_8x16 {
    ($text:expr $(, $style_key:ident = $style_value:expr )* $(,)?) => {
        $crate::impl_text!(Font8x16, $text $(, $style_key = $style_value )*)
    };
}

/// Render text using the [`Font12x16`](./fonts/type.Font12x16.html) font
///
/// ```rust
/// use embedded_graphics::{text_12x16, prelude::*, fonts::Font12x16, pixelcolor::Rgb565};
///
/// let text: Font12x16<Rgb565> = text_12x16!("Hello world!");
/// let styled_text: Font12x16<Rgb565> = text_12x16!(
///     "Hello world!",
///     text_color = Some(Rgb565::RED),
///     background_color = Some(Rgb565::GREEN)
/// );
/// ```
///
/// Style properties like `text_color` map to properties in the [`TextStyle`] struct.
///
/// [`TextStyle`]: style/struct.TextStyle.html
#[macro_export]
macro_rules! text_12x16 {
    ($text:expr $(, $style_key:ident = $style_value:expr )* $(,)?) => {
        $crate::impl_text!(Font12x16, $text $(, $style_key = $style_value )*)
    };
}

/// Render text using the [`Font24x32`](./fonts/type.Font24x32.html) font
///
/// ```rust
/// use embedded_graphics::{text_24x32, prelude::*, fonts::Font24x32, pixelcolor::Rgb565};
///
/// let text: Font24x32<Rgb565> = text_24x32!("Hello world!");
/// let styled_text: Font24x32<Rgb565> = text_24x32!(
///     "Hello world!",
///     text_color = Some(Rgb565::RED),
///     background_color = Some(Rgb565::GREEN)
/// );
/// ```
///
/// Style properties like `text_color` map to properties in the [`TextStyle`] struct.
///
/// [`TextStyle`]: style/struct.TextStyle.html
#[macro_export]
macro_rules! text_24x32 {
    ($text:expr $(, $style_key:ident = $style_value:expr )* $(,)?) => {
        $crate::impl_text!(Font24x32, $text $(, $style_key = $style_value )*)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelcolor::{BinaryColor, Rgb565, RgbColor};

    #[test]
    fn font_macros() {
        let _text: Font6x8<BinaryColor> = text_6x8!("Hello!");
        let _text: Font6x12<BinaryColor> = text_6x12!("Hello!");
        let _text: Font8x16<BinaryColor> = text_8x16!("Hello!");
        let _text: Font12x16<BinaryColor> = text_12x16!("Hello!");
        let _text: Font24x32<BinaryColor> = text_24x32!("Hello!");
    }

    #[test]
    fn styled_text() {
        let _text: Font6x8<Rgb565> = text_6x8!("Hello!", text_color = Some(Rgb565::RED));
        let _text: Font6x12<Rgb565> = text_6x12!("Hello!", text_color = Some(Rgb565::GREEN));
        let _text: Font8x16<Rgb565> = text_8x16!("Hello!", text_color = Some(Rgb565::BLUE));
        let _text: Font12x16<Rgb565> = text_12x16!("Hello!", text_color = Some(Rgb565::YELLOW));
        let _text: Font24x32<Rgb565> = text_24x32!("Hello!", text_color = Some(Rgb565::MAGENTA));
    }
}
