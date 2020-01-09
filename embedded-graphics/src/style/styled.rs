use crate::geometry::{Dimensions, Point, Size};
use crate::transform::Transform;

/// Styled.
#[derive(Debug, Clone, PartialEq)]
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
    fn top_left(&self) -> Point {
        self.primitive.top_left()
    }

    fn bottom_right(&self) -> Point {
        self.primitive.bottom_right()
    }

    fn size(&self) -> Size {
        self.primitive.size()
    }
}
