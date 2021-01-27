//! Text drawable.

mod text;
mod text_style;

pub use embedded_graphics_core::text::{
    CharacterStyle, DecorationColor, HorizontalAlignment, TextMetrics, TextRenderer,
    VerticalAlignment,
};
pub use text::Text;
pub use text_style::{TextStyle, TextStyleBuilder};
