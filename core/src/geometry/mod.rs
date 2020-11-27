//! Geometry module.

mod point;
mod size;

pub use point::Point;
pub use size::Size;

use crate::primitives::Rectangle;

/// Adds the ability to get the dimensions/position of an item.
///
/// TODO
///
/// # Implementation notes
///
/// This trait must be implemented for any item that has a known size and position. For items where
/// only the size is known, see the [`OriginDimensions`] trait.
///
/// [`OriginDimensions`]: trait.OriginDimensions.html
pub trait Dimensions {
    /// Returns the bounding box.
    fn bounding_box(&self) -> Rectangle;
}

/// Dimensions with `top_left` of the bounding box at `(0, 0)`.
///
/// A blanket implementation of `Dimensions` is provided for all types that implement this trait.
pub trait OriginDimensions {
    /// Returns the size of the bounding box.
    fn size(&self) -> Size;
}

impl<T> Dimensions for T
where
    T: OriginDimensions,
{
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(Point::zero(), self.size())
    }
}

/// Anchor point.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
pub enum AnchorPoint {
    /// Top left.
    TopLeft,
    /// Top center.
    TopCenter,
    /// Top right.
    TopRight,
    /// Center left.
    CenterLeft,
    /// Center.
    Center,
    /// Center right.
    CenterRight,
    /// Bottom left.
    BottomLeft,
    /// Bottom center.
    BottomCenter,
    /// Bottom right.
    BottomRight,
}
