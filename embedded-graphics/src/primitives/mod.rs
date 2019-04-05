//! Graphics primitives

use crate::drawable::Dimensions;

pub mod circle;
pub mod line;
pub mod rect;
pub mod triangle;

/// Primitive trait
pub trait Primitive: Dimensions {}

pub use self::circle::Circle;
pub use self::line::Line;
pub use self::rect::Rect;
pub use self::triangle::Triangle;
