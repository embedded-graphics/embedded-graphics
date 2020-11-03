use super::TextStyle;
use crate::{
    fonts::Font,
    geometry::{Dimensions, Point},
    pixelcolor::PixelColor,
    primitives::{OffsetOutline, Primitive, Rectangle},
    style::PrimitiveStyle,
    transform::Transform,
    SaturatingCast,
};

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

impl<T, C, F> Dimensions for Styled<T, TextStyle<C, F>>
where
    T: Dimensions,
    C: PixelColor,
    F: Font,
{
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
        let offset = self.style.outside_stroke_width().saturating_cast();

        self.primitive.offset(offset)
    }

    fn fill_area(&self) -> Self::Primitive {
        // saturate offset at i32::min_value() if stroke width is to large
        let offset = self.style.inside_stroke_width().saturating_cast_neg();

        self.primitive.offset(offset)
    }
}
