//! Pixel based fonts

mod font12x16;
mod font6x12;
mod font6x8;
mod font8x16;
pub mod font_builder;

pub use self::font12x16::Font12x16;
pub use self::font6x12::Font6x12;
pub use self::font6x8::Font6x8;
pub use self::font8x16::Font8x16;
use crate::drawable::Dimensions;
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
    ///
    /// # struct Display {}
    /// # impl Display {
    /// #     pub fn draw<T>(&self, item_pixels: T) -> Result<(), ()>
    /// #     where
    /// #         T: IntoIterator<Item = Pixel<u8>>,
    /// #     {
    /// #         Ok(())
    /// #     }
    /// # }
    /// #
    /// fn main() {
    ///     let disp: Display = Display {};
    ///     // Render a string with a 8bit color
    ///     let text = Font6x8::render_str("Hello world")
    ///         .style(Style::stroke(1u8));
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
/// use embedded_graphics::{text_6x8, prelude::*, fonts::Font6x8};
///
/// let text: Font6x8<u8> = text_6x8!("Hello world!");
/// let styled_text: Font6x8<u8> = text_6x8!(
///     "Hello world!",
///     stroke = Some(10u8),
///     fill = Some(20u8)
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
/// use embedded_graphics::{text_6x12, prelude::*, fonts::Font6x12};
///
/// let text: Font6x12<u8> = text_6x12!("Hello world!");
/// let styled_text: Font6x12<u8> = text_6x12!(
///     "Hello world!",
///     stroke = Some(10u8),
///     fill = Some(20u8)
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
/// use embedded_graphics::{text_8x16, prelude::*, fonts::Font8x16};
///
/// let text: Font8x16<u8> = text_8x16!("Hello world!");
/// let styled_text: Font8x16<u8> = text_8x16!(
///     "Hello world!",
///     stroke = Some(10u8),
///     fill = Some(20u8)
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
/// use embedded_graphics::{text_12x16, prelude::*, fonts::Font12x16};
///
/// let text: Font12x16<u8> = text_12x16!("Hello world!");
/// let styled_text: Font12x16<u8> = text_12x16!(
///     "Hello world!",
///     stroke = Some(10u8),
///     fill = Some(20u8)
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

    #[test]
    fn font_macros() {
        let _text: Font6x8<u8> = text_6x8!("Hello!");
        let _text: Font6x12<u8> = text_6x12!("Hello!");
        let _text: Font8x16<u8> = text_8x16!("Hello!");
        let _text: Font12x16<u8> = text_12x16!("Hello!");
    }

    #[test]
    fn styled_text() {
        let _text: Font6x8<u8> = text_6x8!("Hello!", stroke = Some(10u8));
        let _text: Font6x12<u8> = text_6x12!("Hello!", stroke = Some(10u8));
        let _text: Font8x16<u8> = text_8x16!("Hello!", stroke = Some(10u8));
        let _text: Font12x16<u8> = text_12x16!("Hello!", stroke = Some(10u8));
    }
}
