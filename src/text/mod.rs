//! Text drawable.

mod text;
mod text_style;

pub use embedded_graphics_core::text::{
    Alignment, Baseline, CharacterStyle, DecorationColor, TextMetrics, TextRenderer,
};
pub use text::Text;
pub use text_style::{TextStyle, TextStyleBuilder};
