use crate::{
    geometry::{Dimensions, Point},
    primitives::Rectangle,
    transform::Transform,
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

impl<T, S> Dimensions for Styled<T, S>
where
    T: Dimensions,
{
    //FIXME: The bounding box for a styled primitive should use the stroke width and alignment.
    fn bounding_box(&self) -> Rectangle {
        self.primitive.bounding_box()
    }
}
