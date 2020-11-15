//! Styling struct to customize the look of objects.

mod mono_text_style;
mod primitive_style;
mod styled;
mod text_style;

pub use mono_text_style::{MonoTextStyle, MonoTextStyleBuilder};
pub use primitive_style::{PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment};
pub use styled::{Styled, StyledPrimitiveAreas};
pub use text_style::{TextStyle, TextStylePixels};
