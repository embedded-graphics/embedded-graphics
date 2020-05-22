//! Styling struct to customize the look of objects.

mod primitive_style;
mod styled;
mod text_style;

pub use primitive_style::{PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment};
pub use styled::Styled;
pub use text_style::{TextStyle, TextStyleBuilder};
