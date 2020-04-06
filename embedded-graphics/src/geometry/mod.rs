//! Geometry module.

mod point;
mod size;

pub use point::Point;
pub use size::Size;

use crate::primitives::Rectangle;

/// Adds the ability to get the dimensions/position of a graphics object
///
/// This **should** be implemented for all builtin embedded-graphics primitives and fonts. Third party
/// implementations do not have to implement this trait as an object may not have a known size. If
/// the object _does_ have a known size, this trait **should** be implemented.
pub trait Dimensions {
    /// Returns the bounding box.
    fn bounding_box(&self) -> Rectangle;
}
