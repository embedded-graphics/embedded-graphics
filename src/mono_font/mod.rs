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
//! The examples below use the `Font6x8` font, however any of the [built-in fonts]
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
//!     mono_font::{ascii::Font6x9, MonoTextStyle, MonoTextStyleBuilder},
//!     pixelcolor::Rgb565,
//!     prelude::*,
//!     text::Text,
//! };
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display: MockDisplay<Rgb565> = MockDisplay::default();
//! # display.set_allow_out_of_bounds_drawing(true);
//!
//! // Create a new text style
//! let style = MonoTextStyleBuilder::new()
//!     .font(Font6x9)
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
//!     mono_font::{ascii::Font6x9, MonoTextStyle},
//!     pixelcolor::BinaryColor,
//!     prelude::*,
//!     text::Text,
//! };
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display: MockDisplay<BinaryColor> = MockDisplay::default();
//! # display.set_allow_out_of_bounds_drawing(true);
//!
//! Text::new("Hello Rust!", Point::zero())
//!     .into_styled(MonoTextStyle::new(Font6x9, BinaryColor::On))
//!     .translate(Point::new(20, 30))
//!     .draw(&mut display)?;
//!
//! // this is equivalent to:
//!
//! # let mut display: MockDisplay<BinaryColor> = MockDisplay::default();
//! # display.set_allow_out_of_bounds_drawing(true);
//! Text::new("Hello Rust!", Point::new(20, 30))
//!     .into_styled(MonoTextStyle::new(Font6x9, BinaryColor::On))
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
//!     mono_font::{ascii::Font6x9, MonoTextStyleBuilder},
//!     pixelcolor::Rgb565,
//!     prelude::*,
//!     text::Text,
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
//!         MonoTextStyleBuilder::new()
//!             .font(Font6x9)
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
//! TODO: Replace with generated fonts table
//!
//! | Type | Screenshot |
//! |------|------------|
//! | `Font6x8` | ![6x8 font spritemap screenshot](https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/master/data/font6x8.png) |
//! | `Font6x12` | ![6x12 font spritemap screenshot](https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/master/data/font6x12.png) |
//! | `Font8x16` | ![8x16 font spritemap screenshot](https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/master/data/font8x16.png) |
//! | `Font12x16` | ![12x16 font spritemap screenshot](https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/master/data/font12x16.png) |
//! | `Font24x32` | The 24x32 font is a pixel doubled version of the 12x16 font. |
//!
//! [built-in fonts]: #built-in-fonts
//! [`Text`]: ../text/struct.Text.html
//! [`Styled`]: ../struct.Styled.html
//! [`MonoTextStyle`]: struct.MonoTextStyle.html
//! [`ArrayString`]: https://docs.rs/arrayvec/0.4.11/arrayvec/struct.ArrayString.html
//! [`write!()`]: https://doc.rust-lang.org/nightly/std/macro.write.html

pub mod ascii;
pub mod latin1;
mod mono_char_pixels;
mod mono_text_style;

pub(crate) use mono_char_pixels::MonoCharPixels;

pub use mono_text_style::{MonoTextStyle, MonoTextStyleBuilder};

use crate::geometry::Size;

/// Monospaced bitmap font.
pub trait MonoFont: Copy {
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

    /// The baseline.
    ///
    /// TODO: add description how this value is used and what the default value is
    const BASELINE: Option<i32> = None;

    /// Offset from top of a character to the top of the strikethrough.
    const STRIKETHROUGH_OFFSET: i32 = Self::CHARACTER_SIZE.height as i32 / 2;

    /// Height of the strikethrough.
    const STRIKETHROUGH_HEIGHT: u32 = 1;

    /// Offset from top of a character to the top of the underline.
    const UNDERLINE_OFFSET: i32 = Self::CHARACTER_SIZE.height as i32;

    /// Height of the underline.
    const UNDERLINE_HEIGHT: u32 = 1;

    /// Returns the position of a character in the font.
    fn char_offset(_: char) -> u32;
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::{
        geometry::Point,
        mock_display::MockDisplay,
        mono_font::MonoTextStyleBuilder,
        pixelcolor::{BinaryColor, Rgb888, RgbColor},
        text::{Text, TextStyleBuilder, VerticalAlignment},
        Drawable,
    };

    /// Draws a text using the given font and checks it against the expected pattern.
    // MSRV: Add `track_caller` attribute for rust version >= 1.46.0
    // #[track_caller]
    pub fn assert_text_from_pattern<F>(text: &str, font: F, pattern: &[&str])
    where
        F: MonoFont,
    {
        let character_style = MonoTextStyleBuilder::new()
            .font(font)
            .text_color(BinaryColor::On)
            .build();

        let text_style = TextStyleBuilder::new()
            .character_style(character_style)
            .vertical_alignment(VerticalAlignment::Top)
            .build();

        let mut display = MockDisplay::new();
        Text::new(text, Point::zero())
            .into_styled(text_style)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(pattern);
    }

    /// Draws a white 'A' with green underline and red strikethrough.
    // MSRV: Add `track_caller` attribute for rust version >= 1.46.0
    // #[track_caller]
    pub fn test_text_decoration<F>(font: F, pattern: &[&str])
    where
        F: MonoFont,
    {
        let character_style = MonoTextStyleBuilder::new()
            .font(font)
            .text_color(Rgb888::WHITE)
            .underline_with_color(Rgb888::GREEN)
            .strikethrough_with_color(Rgb888::RED)
            .build();

        let text_style = TextStyleBuilder::new()
            .character_style(character_style)
            .vertical_alignment(VerticalAlignment::Top)
            .build();

        let mut display = MockDisplay::new();
        display.set_allow_overdraw(true);

        Text::new("A", Point::zero())
            .into_styled(text_style)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(pattern);
    }

    /// Test if the baseline constant is set correctly.
    ///
    /// This test assumes that the character `A` is on the baseline.
    pub fn test_baseline<F>(font: F)
    where
        F: MonoFont,
    {
        let character_style = MonoTextStyleBuilder::new()
            .font(font)
            .text_color(BinaryColor::On)
            .build();

        let text_style = TextStyleBuilder::new()
            .character_style(character_style)
            .vertical_alignment(VerticalAlignment::Top)
            .build();

        // Draw 'A' character to determine it's baseline
        let mut display = MockDisplay::new();
        Text::new("A", Point::zero())
            .into_styled(text_style)
            .draw(&mut display)
            .unwrap();

        let baseline = display.affected_area().bottom_right().unwrap().y;

        assert_eq!(F::BASELINE, Some(baseline));
    }
}
