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

/// Offset outline trait.
pub trait OffsetOutline {
    /// Offsets the outline of the shape.
    ///
    /// The offset is applied perpendicular to each element of the outline.
    /// Offset values greater than zero will expand the shape and values less
    /// than zero will shrink the shape.
    fn offset(&self, offset: i32) -> Self;
}

/// Trait to check if a point is inside a closed shape.
pub trait ContainsPoint {
    /// Returns `true` if the given point is inside the shape.
    fn contains(&self, point: Point) -> bool;
}

impl<T: ContainsPoint, const N: usize> ContainsPoint for [T; N] {
    fn contains(&self, point: Point) -> bool {
        // Proxy to the [T] implementation
        ContainsPoint::contains(&self[..], point)
    }
}

impl<T: ContainsPoint> ContainsPoint for [T] {
    fn contains(&self, point: Point) -> bool {
        self.iter().any(|t| t.contains(point))
    }
}

macro_rules! tuple {
    ($a:ident,) => {
        #[doc = "This trait is implemented for tuples up to twelve items long."]
        impl<$a:ContainsPoint,> ContainsPoint for ($a,)
        {
            fn contains(&self, point: Point) -> bool {
                self.0.contains(point)
            }
        }
    };
    ($a:ident, $($rest:ident,)+) => {
        #[doc(hidden)]
        impl<$a:ContainsPoint, $($rest:ContainsPoint),+> ContainsPoint for ($a, $($rest,)+)
        {
            #[allow(non_snake_case)]
            fn contains(&self, point: Point) -> bool {
                let (a, $($rest),+) = self;
                a.contains(point) $(|| $rest.contains(point))+
            }
        }
        tuple! { $($rest,)+ }
    }
}

tuple!(L, K, J, I, H, G, F, E, D, C, B, A,);

#[cfg(test)]
mod tests {
    use embedded_graphics_core::prelude::Size;
    use embedded_graphics_core::primitives::Rectangle;

    use super::*;

    #[test]
    fn slice_contains_point() {
        let rects = &[
            Rectangle::new(Point::zero(), Size::new_equal(1)),
            Rectangle::new(Point::new(3, 6), Size::new_equal(2)),
        ];

        assert!(ContainsPoint::contains(rects, Point::zero()));
        assert!(ContainsPoint::contains(rects, Point::new(4, 7)));
        assert!(!ContainsPoint::contains(rects, Point::new(10, 10)));
    }

    #[test]
    fn tuple_contains_point() {
        let rects = (
            Rectangle::new(Point::zero(), Size::new_equal(1)),
            Rectangle::new(Point::new(3, 6), Size::new_equal(2)),
        );

        assert!(rects.contains(Point::zero()));
        assert!(rects.contains(Point::new(4, 7)));
        assert!(!rects.contains(Point::new(10, 10)));
    }
}
