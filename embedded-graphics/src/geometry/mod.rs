//! Geometry module.

mod point;
mod size;

pub use point::Point;
pub use size::Size;

/// Adds the ability to get the dimensions/position of a graphics object
///
/// This **should** be implemented for all builtin embedded-graphics primitives and fonts. Third party
/// implementations do not have to implement this trait as an object may not have a known size. If
/// the object _does_ have a known size, this trait **should** be implemented.
pub trait Dimensions {
    /// Get the top left corner of the bounding box for an object
    fn top_left(&self) -> Point;

    /// Get the bottom right corner of the bounding box for an object
    fn bottom_right(&self) -> Point;

    /// Get the width and height for an object
    fn size(&self) -> Size;
}
