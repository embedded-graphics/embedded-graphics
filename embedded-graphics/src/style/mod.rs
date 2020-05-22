//! Styling struct to customize the look of objects.

mod primitive_style;
mod styled;
mod text_style;

pub use primitive_style::{PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment};
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
