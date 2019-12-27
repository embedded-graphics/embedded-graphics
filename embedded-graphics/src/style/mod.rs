//! Styling struct to customize the look of objects.

mod primitive_style;
mod styled;
mod text_style;

pub use primitive_style::{PrimitiveStyle, PrimitiveStyleBuilder};
pub use styled::Styled;
pub use text_style::TextStyle;

/// Create a [`PrimitiveStyle`]
///
/// Any properties not specified in the macro call will use the values provided by
/// `PrimitiveStyle::default()`.
///
/// ```rust
/// use embedded_graphics::{
///     pixelcolor::{Rgb565, RgbColor},
///     primitive_style,
/// };
///
/// let style = primitive_style!(fill_color = Some(Rgb565::RED),);
/// ```
///
/// [`PrimitiveStyle`]: ./style/struct.PrimitiveStyle.html

#[macro_export]
macro_rules! primitive_style {
    ($($style_key:ident = $style_value:expr ),* $(,)?) => {{
        #[allow(unused_mut)]
        let mut style = $crate::style::PrimitiveStyle::default();

        $( style.$style_key = $style_value; )*

        style

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
/// This example uses [`Font8x16`] and the [`Rgb565`] colour space to create a default text style.
/// This will result in a white font with transparent background.
///
/// ```rust
/// use embedded_graphics::fonts::Font6x12;
/// use embedded_graphics::pixelcolor::{Rgb565, RgbColor};
/// use embedded_graphics::style::TextStyle;
/// use embedded_graphics::text_style;
///
/// let style: TextStyle<Rgb565, _> = text_style!(font = Font6x12);
/// #
/// # assert_eq!(
/// #     style,
/// #     TextStyle {
/// #         font: Font6x12,
/// #         text_color: None,
/// #         background_color: None
/// #     }
/// # );
/// ```
///
/// ### Create coloured text with background
///
/// This example uses [`Font6x8`] and the [`Rgb565`] colour space to create a text style with red
/// text on a green background.
///
/// ```rust
/// use embedded_graphics::fonts::Font6x8;
/// use embedded_graphics::pixelcolor::{Rgb565, RgbColor};
/// use embedded_graphics::style::{TextStyle};
/// use embedded_graphics::text_style;
///
/// let style = text_style!(font = Font6x8, text_color = Some(Rgb565::RED), background_color = Some(Rgb565::GREEN));
/// #
/// # assert_eq!(
/// #     style,
/// #     TextStyle {
/// #         font: Font6x8,
/// #         text_color: Some(Rgb565::RED),
/// #         background_color: Some(Rgb565::GREEN),
/// #     }
/// # );
/// ```
///
/// [`TextStyle`]: ./struct.TextStyle.html
#[macro_export]
macro_rules! text_style {

    (font = $font:expr, $( $style_key:ident = $style_value:expr ),* $(,)?) => {{
        #[allow(unused_mut)]
        let mut style = $crate::style::TextStyle::with_font($font);

        $( style.$style_key = $style_value; )*

        style
    }};
    (font = $font:expr $(,)?) => {{
        $crate::style::TextStyle::with_font($font)
    }};

}
