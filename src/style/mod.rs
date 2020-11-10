//! Styling struct to customize the look of objects.

mod monospaced_text_style;
mod primitive_style;
mod styled;
mod text_style;

pub use monospaced_text_style::{MonospacedTextStyle, MonospacedTextStyleBuilder};
pub use primitive_style::{PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment};
pub use styled::{Styled, StyledPrimitiveAreas};
pub use text_style::{TextStyle, TextStylePixels};
