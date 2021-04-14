use crate::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point},
    pixelcolor::PixelColor,
    primitives::OffsetOutline,
    primitives::{PrimitiveStyle, Rectangle},
    transform::Transform,
    Drawable,
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

impl<T: OffsetOutline, C: PixelColor> Styled<T, PrimitiveStyle<C>> {
    /// TODO: docs
    pub fn fill_area(&self) -> T {
        self.style.fill_area(&self.primitive)
    }

    /// TODO: docs
    pub fn stroke_area(&self) -> T {
        self.style.stroke_area(&self.primitive)
    }
}

impl<T: StyledDimensions<S>, S> Dimensions for Styled<T, S> {
    fn bounding_box(&self) -> Rectangle {
        self.primitive.styled_bounding_box(&self.style)
    }
}

impl<T: StyledDrawable<S>, S> Drawable for Styled<T, S> {
    type Color = T::Color;
    type Output = T::Output;

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        self.primitive.draw_styled(&self.style, target)
    }
}

impl<T: Transform, S: Clone> Transform for Styled<T, S> {
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

/// Styled drawable.
pub trait StyledDrawable<S> {
    /// Color type.
    type Color: PixelColor;
    /// Output type.
    type Output;

    /// Draws the primitive using the given style.
    fn draw_styled<D>(&self, style: &S, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>;
}

/// Styled dimensions.
pub trait StyledDimensions<S> {
    /// Returns the bounding box using the given style.
    fn styled_bounding_box(&self, style: &S) -> Rectangle;
}
