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
///
/// ```rust
/// use embedded_graphics::{primitivestyle, pixelcolor::{RgbColor, Rgb565}};
///
/// let style = primitivestyle!(
///     fill_color = Some(Rgb565::RED),
/// );
/// ```
///
/// [`PrimitiveStyle`]: ./struct.PrimitiveStyle.html

#[macro_export]
macro_rules! primitivestyle {
    ($($style_key:ident = $style_value:expr ),* $(,)?) => {{
    	#[allow(unused_mut)]
        let mut style = $crate::style::PrimitiveStyle::default();

        $( style.$style_key = $style_value; )*

        style

    }};
}
