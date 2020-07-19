use crate::{
    geometry::{Dimensions, Point},
    pixelcolor::PixelColor,
    prelude::Primitive,
    primitives::{OffsetOutline, Rectangle},
    style::PrimitiveStyle,
    transform::Transform,
};
use core::convert::TryFrom;

/// Styled.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Styled<T, S> {
    /// Primitive.
    pub primitive: T,
    /// Style.
    pub style: S,
}

impl<T, S> Styled<T, S> {
    /// Creates a styled.
    pub fn new(primitive: T, style: S) -> Self {
        Self { primitive, style }
    }
}

impl<T, S> Transform for Styled<T, S>
where
    T: Transform,
    S: Clone,
{
    fn translate(&self, by: Point) -> Self {
        Self {
            primitive: self.primitive.translate(by),
            style: self.style.clone(),
        }
    }

    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.primitive.translate_mut(by);

        self
    }
}

impl<T, S> Dimensions for Styled<T, S>
where
    T: Dimensions,
{
    //FIXME: The bounding box for a styled primitive should use the stroke width and alignment.
    fn bounding_box(&self) -> Rectangle {
        self.primitive.bounding_box()
    }
}

/// Stroke and fill area trait.
pub trait StyledPrimitiveAreas {
    /// Type of primitive shape used for the stroke and fill areas.
    type Primitive;

    /// Returns the stroke area.
    fn stroke_area(&self) -> Self::Primitive;

    /// Returns the fill area.
    fn fill_area(&self) -> Self::Primitive;
}

impl<T, C> StyledPrimitiveAreas for Styled<T, PrimitiveStyle<C>>
where
    T: Primitive + OffsetOutline,
    C: PixelColor,
{
    type Primitive = T;

    fn stroke_area(&self) -> Self::Primitive {
        // saturate offset at i32::max_value() if stroke width is to large
        let offset = i32::try_from(self.style.outside_stroke_width()).unwrap_or(i32::max_value());

        self.primitive.offset(offset)
    }

    fn fill_area(&self) -> Self::Primitive {
        // saturate offset at i32::max_value() if stroke width is to large
        let offset = i32::try_from(self.style.inside_stroke_width()).unwrap_or(i32::max_value());

        self.primitive.offset(-offset)
    }
}
