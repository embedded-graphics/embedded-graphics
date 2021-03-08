//! Text drawable.
//!
//! TODO: - Add an overview of the e-g text rendering, e.g. what a `Text` object is used for and
//!         how it can be used with different text styles/renderers.
//!       - Describe the difference between `TextStyle` and `MonoTextStyle`
//!       - How is `Text::position` used by text renders dependent on `TextStyle::baseline`
//!       - Add examples:
//!         - Basic text without `TextStyle`
//!         - Advanced text with `TextStyle`
//!         - Draw text with multiple styles by using the result of `Text.draw(...)`
//!       - Implementation notes for new `TextRenderer`s
//!         A text renderer example might be too complicated for the docs and the target audience
//!         is relatively small, so that a link to an external example could be a better idea.

mod text;
mod text_style;

pub use embedded_graphics_core::text::{
    Alignment, Baseline, CharacterStyle, DecorationColor, TextMetrics, TextRenderer,
};
pub use text::Text;
pub use text_style::{TextStyle, TextStyleBuilder};
