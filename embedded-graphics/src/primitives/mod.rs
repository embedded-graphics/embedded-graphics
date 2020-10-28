//! Graphics primitives

pub mod arc;
pub mod circle;
pub mod ellipse;
pub mod line;
pub mod line_joint;
pub mod polyline;
pub mod polyline_outline_iter;
pub mod rectangle;
pub mod rounded_rectangle;
pub mod sector;
pub mod thick_segment;
pub mod thick_segment_iter;
pub mod triangle;

pub use self::{
    arc::Arc,
    circle::Circle,
    ellipse::Ellipse,
    line::Line,
    polyline::Polyline,
    rectangle::Rectangle,
    rounded_rectangle::{CornerRadii, CornerRadiiBuilder, RoundedRectangle},
    sector::Sector,
    triangle::Triangle,
};
use crate::{
    geometry::{Dimensions, Point},
    pixelcolor::PixelColor,
    style::{PrimitiveStyle, Styled},
};

/// Primitive trait
pub trait Primitive: Dimensions {
    /// Iterator over all points inside the primitive.
    type PointsIter: Iterator<Item = Point>;

    /// Converts this primitive into a `Styled`.
    fn into_styled<C>(self, style: PrimitiveStyle<C>) -> Styled<Self, PrimitiveStyle<C>>
    where
        C: PixelColor,
        Self: Sized,
    {
        Styled::new(self, style)
    }

    /// Returns an iterator over all points inside the primitive.
    fn points(&self) -> Self::PointsIter;
}

/// Trait to check if a point is inside a closed shape.
pub trait ContainsPoint {
    /// Returns `true` if the given point is inside the shape.
    fn contains(&self, point: Point) -> bool;
}

/// Offset outline trait.
pub trait OffsetOutline {
    /// Offsets the outline of the shape.
    ///
    /// The offset is applied perpendicular to each element of the outline.
    /// Offset values greater than zero will expand the shape and values less
    /// than zero will shrink the shape.
    fn offset(&self, offset: i32) -> Self;
}
