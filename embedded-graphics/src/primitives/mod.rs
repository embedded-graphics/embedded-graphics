//! Graphics primitives

pub mod arc;
pub mod circle;
pub mod ellipse;
pub mod line;
pub mod polyline;
pub mod rectangle;
pub mod rounded_rectangle;
pub mod sector;
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
