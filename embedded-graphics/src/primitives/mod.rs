//! Graphics primitives

pub mod circle;
mod corner_radii;
pub mod ellipse;
mod ellipse_quadrant;
pub mod line;
pub mod polyline;
pub mod rectangle;
pub mod rounded_rectangle;
pub mod triangle;

pub use self::{
    circle::Circle,
    corner_radii::{CornerRadii, CornerRadiiBuilder},
    ellipse::Ellipse,
    line::Line,
    polyline::Polyline,
    rectangle::Rectangle,
    rounded_rectangle::RoundedRectangle,
    triangle::Triangle,
};
use crate::primitives::ellipse_quadrant::{EllipseQuadrant, Quadrant};
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
    /// Returns `true` is the given point is inside the shape.
    fn contains(&self, point: Point) -> bool;
}

/// TODO: Docs
pub trait CachedContainsPoint {
    /// TODO: Docs
    type Wrapper: ContainsPoint;

    /// TODO: Docs
    fn cached_contains_point(&self) -> Self::Wrapper;
}

impl CachedContainsPoint for RoundedRectangle {
    type Wrapper = RoundedRectangleHitTester;

    fn cached_contains_point(&self) -> Self::Wrapper {
        Self::Wrapper::new(self)
    }
}

/// TODO: Docs
#[derive(Debug, Copy, Clone)]
pub struct RoundedRectangleHitTester {
    rectangle: Rectangle,
    tl: EllipseQuadrant,
    tl_bounding_box: Rectangle,
    tr: EllipseQuadrant,
    tr_bounding_box: Rectangle,
    br: EllipseQuadrant,
    br_bounding_box: Rectangle,
    bl: EllipseQuadrant,
    bl_bounding_box: Rectangle,
}

impl RoundedRectangleHitTester {
    /// TODO: Docs
    pub fn new(primitive: &RoundedRectangle) -> Self {
        let rectangle = primitive.rectangle;

        let tl = primitive.get_confined_corner_quadrant(Quadrant::TopLeft);
        let tr = primitive.get_confined_corner_quadrant(Quadrant::TopRight);
        let br = primitive.get_confined_corner_quadrant(Quadrant::BottomRight);
        let bl = primitive.get_confined_corner_quadrant(Quadrant::BottomLeft);

        let tl_bounding_box = tl.bounding_box();
        let tr_bounding_box = tr.bounding_box();
        let br_bounding_box = br.bounding_box();
        let bl_bounding_box = bl.bounding_box();

        RoundedRectangleHitTester {
            rectangle,
            tl,
            tr,
            br,
            bl,
            tl_bounding_box,
            tr_bounding_box,
            br_bounding_box,
            bl_bounding_box,
        }
    }
}

impl ContainsPoint for RoundedRectangleHitTester {
    fn contains(&self, point: Point) -> bool {
        if !self.rectangle.contains(point) {
            return false;
        }

        if self.tl_bounding_box.contains(point) {
            return self.tl.contains(point);
        }

        if self.tr_bounding_box.contains(point) {
            return self.tr.contains(point);
        }

        if self.br_bounding_box.contains(point) {
            return self.br.contains(point);
        }

        if self.bl_bounding_box.contains(point) {
            return self.bl.contains(point);
        }

        // We're in the rest of the rectangle at this point
        true
    }
}
