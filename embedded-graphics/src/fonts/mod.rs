//! Monospaced bitmap fonts.
//!
//! This module contains support for drawing monospaced bitmap fonts and provides
//! several [built-in fonts].
//!
//! Additional custom fonts can be added by the application or other crates. This
//! is demonstrated in the `text-custom-font` example in the simulator crate.
//!
//! # Examples
//!
//! The examples below use the [`Font6x8`] font, however any of the [built-in fonts]
//! in this module or custom fonts can be substituted.
//!
//! ## Print styled "Hello Rust!"
//!
//! Text can be drawn to a display by creating a [`Text`] object and attaching a
//! text style to it by using a [`Styled`] object. This example prints
//! "Hello Rust" with a yellow text on a blue background.
//!
//! ```rust
//! use embedded_graphics::{
//!     fonts::{Font6x8, Text},
//!     pixelcolor::Rgb565,
//!     prelude::*,
//!     style::{TextStyle, TextStyleBuilder},
//! };
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display: MockDisplay<Rgb565> = MockDisplay::default();
//! # display.set_allow_out_of_bounds_drawing(true);
//!
//! // Create a new text style
//! let style = TextStyleBuilder::new(Font6x8)
//!     .text_color(Rgb565::YELLOW)
//!     .background_color(Rgb565::BLUE)
//!     .build();
//!
//! // Create a text at position (20, 30) and draw it using the previously defined style
//! Text::new("Hello Rust!", Point::new(20, 30))
//!     .into_styled(style)
//!     .draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//!
//! ## Translate text by (20px, 30px)
//!
//! ```rust
//! use embedded_graphics::{
//!     fonts::{Font6x8, Text},
//!     pixelcolor::BinaryColor,
//!     prelude::*,
//!     style::TextStyle,
//! };
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display: MockDisplay<BinaryColor> = MockDisplay::default();
//! # display.set_allow_out_of_bounds_drawing(true);
//!
//! Text::new("Hello Rust!", Point::zero())
//!     .into_styled(TextStyle::new(Font6x8, BinaryColor::On))
//!     .translate(Point::new(20, 30))
//!     .draw(&mut display)?;
//!
//! // this is equivalent to:
//!
//! # let mut display: MockDisplay<BinaryColor> = MockDisplay::default();
//! # display.set_allow_out_of_bounds_drawing(true);
//! Text::new("Hello Rust!", Point::new(20, 30))
//!     .into_styled(TextStyle::new(Font6x8, BinaryColor::On))
//!     .draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(())
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
//! use embedded_graphics::{
//!     fonts::{Font6x8, Text},
//!     pixelcolor::Rgb565,
//!     prelude::*,
//!     style::TextStyleBuilder,
//! };
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display = MockDisplay::default();
//! # display.set_allow_out_of_bounds_drawing(true);
//!
//! let value = 12.34567;
//!
//! // Create a fixed buffer of length 12
//! let mut buf = ArrayString::<[_; 12]>::new();
//!
//! // Output `Value: 12.35`
//! write!(&mut buf, "Value: {:.2}", value).expect("Failed to write to buffer");
//!
//! Text::new(&buf, Point::zero())
//!     .into_styled(
//!         TextStyleBuilder::new(Font6x8)
//!             .text_color(Rgb565::YELLOW)
//!             .background_color(Rgb565::BLUE)
//!             .build(),
//!     )
//!     .draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//!
//! # Built-in fonts
//!
//! | Type | Screenshot |
//! |------|------------|
//! | [`Font6x6`] | ![6x6 font spritemap screenshot](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/embedded-graphics/data/font6x6.png) |
//! | [`Font6x8`] | ![6x8 font spritemap screenshot](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/embedded-graphics/data/font6x8.png) |
//! | [`Font6x12`] | ![6x12 font spritemap screenshot](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/embedded-graphics/data/font6x12.png) |
//! | [`Font8x16`] | ![8x16 font spritemap screenshot](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/embedded-graphics/data/font8x16.png) |
//! | [`Font12x16`] | ![12x16 font spritemap screenshot](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/embedded-graphics/data/font12x16.png) |
//! | [`Font24x32`] | The 24x32 font is a pixel doubled version of the 12x16 font. |
//!
//! [built-in fonts]: #built-in-fonts
//! [`Font6x6`]: struct.Font6x6.html
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
mod font6x6;
mod font6x8;
mod font8x16;
mod text;

pub use text::{StyledTextIterator, Text};

pub use font12x16::Font12x16;
pub use font24x32::Font24x32;
pub use font6x12::Font6x12;
pub use font6x6::Font6x6;
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

    /// Whether characters have a variable width or not.
    ///
    /// Variable width characters have a maximum width of CHARACTER_SIZE.x, but the empty columns at
    /// the right of each characters are ignored, allowing some characters to be smaller than others.
    const VARIABLE_WIDTH: bool = false;

    /// Returns the position a character in the font.
    fn char_offset(_: char) -> u32;

    /// Returns the actual width of a character in the font.
    fn char_width(c: char) -> u32 {
        if Self::VARIABLE_WIDTH {
            let mut x_max = 0;
            for y in 0..Self::CHARACTER_SIZE.height {
                for x in (x_max..Self::CHARACTER_SIZE.width).rev() {
                    if Self::character_pixel(c, x, y) {
                        x_max = x;
                        break;
                    }
                }
            }
            x_max + 1
        } else {
            Self::CHARACTER_SIZE.width
        }
    }

    /// Returns the value of a pixel in a character in the font.
    fn character_pixel(c: char, x: u32, y: u32) -> bool {
        let char_per_row = Self::FONT_IMAGE_WIDTH / Self::CHARACTER_SIZE.width;

        // Char _code_ offset from first char, most often a space
        // E.g. first char = ' ' (32), target char = '!' (33), offset = 33 - 32 = 1
        let char_offset = Self::char_offset(c);
        let row = char_offset / char_per_row;

        // Top left corner of character, in pixels
        let char_x = (char_offset - (row * char_per_row)) * Self::CHARACTER_SIZE.width;
        let char_y = row * Self::CHARACTER_SIZE.height;

        // Bit index
        // = X pixel offset for char
        // + Character row offset (row 0 = 0, row 1 = (192 * 8) = 1536)
        // + X offset for the pixel block that comprises this char
        // + Y offset for pixel block
        let bitmap_bit_index = char_x + x + ((char_y + y) * Self::FONT_IMAGE_WIDTH);

        let bitmap_byte = bitmap_bit_index / 8;
        let bitmap_bit = 7 - (bitmap_bit_index % 8);

        Self::FONT_IMAGE[bitmap_byte as usize] & (1 << bitmap_bit) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        drawable::Drawable, geometry::Point, mock_display::MockDisplay, pixelcolor::BinaryColor,
        style::TextStyle,
    };

    /// Draws a text using the given font and checks it against the expected pattern.
    pub(super) fn assert_text_from_pattern<F>(text: &str, font: F, pattern: &[&str])
    where
        F: Font + Copy,
    {
        let mut display = MockDisplay::new();
        Text::new(text, Point::zero())
            .into_styled(TextStyle::new(font, BinaryColor::On))
            .draw(&mut display)
            .unwrap();

        assert_eq!(display, MockDisplay::from_pattern(pattern));
    }
}
