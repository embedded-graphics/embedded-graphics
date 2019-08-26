//! Pixel based fonts
//!
//! # Examples
//!
//! The examples below use the [`Font6x8`] font and the [`text_6x8`] macro, however any of the [font
//! types in this module](#types) or [`text_*` macros](../index.html#macros) can be substituted.
//!
//! ## Write some text to the screen at the default `(0, 0)` position
//!
//! ```rust
//! use embedded_graphics::prelude::*;
//! use embedded_graphics::fonts::Font6x8;
//! use embedded_graphics::text_6x8;
//! # use embedded_graphics::mock_display::MockDisplay;
//! # use embedded_graphics::pixelcolor::BinaryColor;
//! # let mut display: MockDisplay<BinaryColor> = MockDisplay::default();
//!
//! // Use struct methods directly
//! display.draw(Font6x8::render_str("Hello Rust!"));
//!
//! // Use a macro instead
//! display.draw(text_6x8!("Hello Rust!"));
//! ```
//!
//! ## Translate text by (20px, 30px)
//!
//! ```rust
//! use embedded_graphics::prelude::*;
//! use embedded_graphics::fonts::Font6x8;
//! # use embedded_graphics::mock_display::MockDisplay;
//! # use embedded_graphics::pixelcolor::BinaryColor;
//! # let mut display: MockDisplay<BinaryColor> = MockDisplay::default();
//!
//! display.draw(
//!     Font6x8::render_str("Hello Rust!").translate(Point::new(20, 30))
//! );
//! ```
//!
//! ## Add some styling to the text
//!
//! Use [any method provided by the `WithStyle` trait](../style/trait.WithStyle.html#required-methods).
//! Properties like `fill_color` or `stroke_color` passed to the `text_6x8` macro are converted into method
//! calls verbatim.
//!
//! ```rust
//! use embedded_graphics::prelude::*;
//! use embedded_graphics::text_6x8;
//! use embedded_graphics::fonts::Font6x8;
//! use embedded_graphics::pixelcolor::Rgb565;
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display = MockDisplay::default();
//!
//! display.draw(text_6x8!(
//!     "Hello Rust!",
//!     fill_color = Some(Rgb565::BLUE),
//!     stroke_color = Some(Rgb565::YELLOW)
//! ));
//!
//! display.draw(
//!     Font6x8::render_str("Hello Rust!")
//!         .translate(Point::new(20, 30))
//!         .fill_color(Some(Rgb565::BLUE))
//!         .stroke_color(Some(Rgb565::YELLOW)),
//! );
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
//! display.draw(text_6x8!(
//!     &buf,
//!     fill_color = Some(Rgb565::BLUE),
//!     stroke_color = Some(Rgb565::YELLOW)
//! ));
//! ```
//!
//! [`text_6x8`]: ../macro.text_6x8.html
//! [`Font6x8`]: ./type.Font6x8.html
//! [`ArrayString`]: https://docs.rs/arrayvec/0.4.11/arrayvec/struct.ArrayString.html
//! [`write!()`]: https://doc.rust-lang.org/nightly/std/macro.write.html

mod font24x32;
mod font12x16;
mod font6x12;
mod font6x8;
mod font8x16;
pub mod font_builder;

pub use self::font24x32::Font24x32;
pub use self::font12x16::Font12x16;
pub use self::font6x12::Font6x12;
pub use self::font6x8::Font6x8;
pub use self::font8x16::Font8x16;
use crate::geometry::Dimensions;
use crate::pixelcolor::PixelColor;
use crate::style::WithStyle;

/// Common methods for all fonts
pub trait Font<'a, C>: WithStyle<C> + Dimensions
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
    /// # use embedded_graphics::mock_display::MockDisplay as Display;
    ///
    /// fn main() {
    ///     let mut disp = Display::default();
    ///     // Render a string with a red stroke
    ///     let text = Font6x8::render_str("Hello world")
    ///         .style(Style::stroke_color(Rgb565::RED));
    ///
    ///     disp.draw(text);
    /// }
    /// ```
    fn render_str(chars: &'a str) -> Self;
}

/// Internal macro used to implement `text_*` on fonts. Do not use directly!
#[doc(hidden)]
#[macro_export]
macro_rules! impl_text {
    ($Font:ident, $text:expr $(, $style_key:ident = $style_value:expr )* $(,)?) => {{
        #[allow(unused_imports)]
        use $crate::style::WithStyle;
        $crate::fonts::$Font::render_str($text)
            $( .$style_key($style_value) )*
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
///     stroke_color = Some(Rgb565::RED),
///     fill_color = Some(Rgb565::GREEN)
/// );
/// ```
///
/// Style properties like `stroke` map to the method calls on the
/// [`WithStyle`](./style/trait.WithStyle.html) trait.
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
///     stroke_color = Some(Rgb565::RED),
///     fill_color = Some(Rgb565::GREEN)
/// );
/// ```
///
/// Style properties like `stroke` map to the method calls on the
/// [`WithStyle`](./style/trait.WithStyle.html) trait.
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
///     stroke_color = Some(Rgb565::RED),
///     fill_color = Some(Rgb565::GREEN)
/// );
/// ```
///
/// Style properties like `stroke` map to the method calls on the
/// [`WithStyle`](./style/trait.WithStyle.html) trait.
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
///     stroke_color = Some(Rgb565::RED),
///     fill_color = Some(Rgb565::GREEN)
/// );
/// ```
///
/// Style properties like `stroke` map to the method calls on the
/// [`WithStyle`](./style/trait.WithStyle.html) trait.
#[macro_export]
macro_rules! text_12x16 {
    ($text:expr $(, $style_key:ident = $style_value:expr )* $(,)?) => {
        $crate::impl_text!(Font12x16, $text $(, $style_key = $style_value )*)
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
    }

    #[test]
    fn styled_text() {
        let _text: Font6x8<Rgb565> = text_6x8!("Hello!", stroke_color = Some(Rgb565::RED));
        let _text: Font6x12<Rgb565> = text_6x12!("Hello!", stroke_color = Some(Rgb565::GREEN));
        let _text: Font8x16<Rgb565> = text_8x16!("Hello!", stroke_color = Some(Rgb565::BLUE));
        let _text: Font12x16<Rgb565> = text_12x16!("Hello!", stroke_color = Some(Rgb565::YELLOW));
    }
}
