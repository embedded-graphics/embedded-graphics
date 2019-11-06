use crate::geometry::{Dimensions, Point, Size};
use crate::pixelcolor::PixelColor;
use crate::style::{Style, WithStyle};
use crate::transform::Transform;

/// Styled.
#[derive(Debug, Clone)]
pub struct Styled<T, C>
where
    C: PixelColor,
{
    /// Primitive.
    pub primitive: T,
    /// Style.
    pub style: Style<C>,
}

impl<T, C> Styled<T, C>
where
    C: PixelColor,
{
    /// Creates a styled.
    pub fn new(primitive: T) -> Self {
        Self {
            primitive,
            style: Style::default(),
        }
    }
}

impl<T, C> WithStyle<C> for Styled<T, C>
where
    C: PixelColor,
{
    fn style(mut self, new_style: Style<C>) -> Self {
        self.style = new_style;

        self
    }

    fn fill_color(mut self, color: Option<C>) -> Self {
        self.style.fill_color = color;

        self
    }

    fn stroke_color(mut self, color: Option<C>) -> Self {
        self.style.stroke_color = color;

        self
    }

    fn stroke_width(mut self, width: u32) -> Self {
        self.style.stroke_width = width;

        self
    }
}

impl<T, C> Transform for Styled<T, C>
where
    C: PixelColor,
    T: Transform,
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

impl<T, C> Dimensions for Styled<T, C>
where
    C: PixelColor,
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
