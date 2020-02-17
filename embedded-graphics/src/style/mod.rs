//! Styling struct to customize the look of objects.

mod primitive_style;
mod styled;
mod text_style;

pub use primitive_style::{PrimitiveStyle, PrimitiveStyleBuilder};
pub use styled::Styled;
pub use text_style::{TextStyle, TextStyleBuilder};

/// Create a [`PrimitiveStyle`]
///
/// All properties on [`PrimitiveStyle`] are supported. Any properties not specified in the macro
/// call will use the values provided by `PrimitiveStyle::default()`.
///
/// ```rust
/// use embedded_graphics::{
///     pixelcolor::{Rgb565, RgbColor},
///     primitive_style,
/// };
///
/// let style = primitive_style!(fill_color = Rgb565::RED);
/// ```
///
/// [`PrimitiveStyle`]: ./style/struct.PrimitiveStyle.html

#[macro_export]
macro_rules! primitive_style {
    ($($style_key:ident = $style_value:expr ),* $(,)?) => {{
        #[allow(unused_mut)]
        let builder = $crate::style::PrimitiveStyleBuilder::new();

        $( let builder = builder.$style_key($style_value); )*

        builder.build()

    }};
}

/// Create a [`TextStyle`]
///
/// All properties on [`TextStyle`] are supported. At least `font` is required, and must be the
/// first property passed to the macro.
///
/// ## Examples
///
/// ### Create a default text style
///
/// This example uses [`Font8x16`] and the [`Rgb565`] color space to create a default text style.
/// This will result in a white font with transparent background.
///
/// ```rust
/// use embedded_graphics::{
///     fonts::Font8x16,
///     pixelcolor::{Rgb565, RgbColor},
///     style::TextStyle,
///     text_style,
/// };
///
/// let style: TextStyle<Rgb565, _> = text_style!(font = Font8x16);
/// #
/// # assert_eq!(
/// #     style,
/// #     embedded_graphics::style::TextStyleBuilder::new(Font8x16).build()
/// # );
/// ```
///
/// ### Create colored text with background
///
/// This example uses [`Font6x8`] and the [`Rgb565`] color space to create a text style with red
/// text on a green background.
///
/// ```rust
/// use embedded_graphics::{
///     fonts::Font6x8,
///     pixelcolor::{Rgb565, RgbColor},
///     style::TextStyle,
///     text_style,
/// };
///
/// let style = text_style!(
///     font = Font6x8,
///     text_color = Rgb565::RED,
///     background_color = Rgb565::GREEN
/// );
/// #
/// # assert_eq!(
/// #     style,
/// #     embedded_graphics::style::TextStyleBuilder::new(Font6x8)
/// #         .text_color(Rgb565::RED)
/// #         .background_color(Rgb565::GREEN)
/// #         .build()
/// # );
/// ```
///
/// [`TextStyle`]: ./style/struct.TextStyle.html
#[macro_export]
macro_rules! text_style {
    (font = $font:expr, $( $style_key:ident = $style_value:expr ),* $(,)?) => {{
        let builder = $crate::style::TextStyleBuilder::new($font);

        $( let builder = builder.$style_key($style_value); )*

        builder.build()
    }};
    (font = $font:expr $(,)?) => {{
        $crate::style::TextStyleBuilder::new($font).build()
    }};

}
