//! Text drawable.

mod text;

pub use embedded_graphics_core::text::TextStyle;
pub use text::Text;

/// Vertical text alignment.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum VerticalAlignment {
    /// Top.
    Top,
    /// Bottom.
    Bottom,
    /// Center.
    Center,
    /// Baseline.
    Baseline,
}

/// Horizontal text alignment.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum HorizontalAlignment {
    /// Left.
    Left,
    /// Center.
    Center,
    /// Right.
    Right,
}
