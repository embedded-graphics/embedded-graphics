//! Geometry module.

mod point;
mod size;

pub use point::Point;
pub use size::Size;

use crate::primitives::Rectangle;

/// Adds the ability to get the dimensions/position of an item.
///
/// This trait is implemented for many builtin items in embedded-graphics:
///
/// * Primitives - returns a rectangle that contains the mathematical representation of the shape.
/// * Styled primitives - returns a rectangle encompassing all drawn pixels, including stroke
///   thickness/offset.
/// * [`DrawTarget`]s (displays, simulator, etc) - returns a rectangle encompassing the drawable
///   display area with the origin at the top left (`0, 0`).
/// * [`Image`]s - returns a rectangle with the exact width and height of the image.
/// * [`Text`] - returns a rectangle containing all pixels required to draw the given text with the
///   chosen font.
///
/// Dimensions are defined as [`Rectangle`]s. This allows, for example, iterating over all points
/// in an item's bounding box with the [`points`] method.
///
/// # Implementation notes
///
/// An implementation of `Dimensions` must produce a rectangle encompassing the entire item. For
/// shapes, images and other drawable items, this means the rectangle must contain all pixels
/// required to draw the item.
///
/// [`DrawTarget`]s  (display drivers, etc) require that `Dimensions` is also implemented. The
/// implementation must return the a rectangle representing the entire visible area of the display
/// with the top left coordinate at the origin (`0, 0`). It is recommended that [`OriginDimensions`]
/// is also implemented for draw targets.
///
/// Some traits in embedded-graphics-core require `Dimensions` to be implemented as well:
///
/// * [`DrawTarget`]: The implementation should return the drawable display area with the top left
///   coordinate at the origin (`0, 0`).
///
/// When testing `Dimensions` implementations for [`Drawable`] items, [`MockDisplay::affected_area`]
/// can be a useful method of checking that the bounding box is the correct size.
///
/// [`Drawable`]: ../drawable/trait.Drawable.html
/// [`DrawTarget`]: ../draw_target/trait.DrawTarget.html
/// [`OriginDimensions`]: trait.OriginDimensions.html
/// [`Rectangle`]: ../primitives/rectangle/struct.Rectangle.html
/// [`points`]: ../primitives/trait.PointsIter.html
/// [`MockDisplay::affected_area`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/mock_display/struct.MockDisplay.html#method.affected_area
/// [`Image`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/image/struct.Image.html
/// [`Text`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/fonts/struct.Text.html
pub trait Dimensions {
    /// Returns the bounding box.
    fn bounding_box(&self) -> Rectangle;
}

/// Dimensions with `top_left` of the bounding box at `(0, 0)`.
///
/// A blanket implementation of `Dimensions` is provided for all types that implement this trait.
///
/// # Implementation notes
///
/// `OriginDimensions` must be implemented for [`ImageDrawable`]s.
///
/// [`ImageDrawable`]: ../image/trait.ImageDrawable.html
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
