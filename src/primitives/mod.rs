//! Graphics primitives

pub mod arc;
pub mod circle;
mod common;
pub mod ellipse;
pub mod line;
pub mod polyline;
mod primitive_style;
pub mod rectangle;
pub mod rounded_rectangle;
pub mod sector;
mod styled;
pub mod triangle;

#[doc(no_inline)]
pub use self::rectangle::Rectangle;
pub use self::{
    arc::Arc,
    circle::Circle,
    ellipse::Ellipse,
    line::Line,
    polyline::Polyline,
    primitive_style::{PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment},
    rounded_rectangle::{CornerRadii, CornerRadiiBuilder, RoundedRectangle},
    sector::Sector,
    triangle::Triangle,
};
use crate::geometry::{Dimensions, Point};
pub use embedded_graphics_core::primitives::PointsIter;

#[cfg(feature = "async_draw")]
pub use styled::AsyncStyledDrawable;

pub use styled::{Styled, StyledDimensions, StyledDrawable};

/// Primitive trait
pub trait Primitive: Dimensions {
    /// Converts this primitive into a `Styled`.
    fn into_styled<S>(self, style: S) -> Styled<Self, S>
    where
        Self: Sized,
    {
        Styled::new(self, style)
    }
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
