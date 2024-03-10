use crate::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point},
    pixelcolor::PixelColor,
    primitives::OffsetOutline,
    primitives::{PrimitiveStyle, Rectangle},
    transform::Transform,
    Drawable,
};

#[cfg(feature = "async_draw")]
use crate::draw_target::AsyncDrawTarget;

/// Styled.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct Styled<T, S> {
    /// Primitive.
    pub primitive: T,
    /// Style.
    pub style: S,
}

impl<T, S> Styled<T, S> {
    /// Creates a styled.
    pub const fn new(primitive: T, style: S) -> Self {
        Self { primitive, style }
    }
}

impl<T: OffsetOutline, C: PixelColor> Styled<T, PrimitiveStyle<C>> {
    /// Returns the fill area.
    ///
    /// The returned primitive coincides with the area that would be filled by calling [`draw`]
    /// on this styled primitive.
    ///
    /// # Examples
    ///
    /// ```
    /// use embedded_graphics::{
    ///     pixelcolor::Rgb888,
    ///     prelude::*,
    ///     primitives::{Circle, PrimitiveStyleBuilder},
    /// };
    ///
    /// let style = PrimitiveStyleBuilder::new()
    ///     .fill_color(Rgb888::RED)
    ///     .stroke_color(Rgb888::GREEN)
    ///     .stroke_width(6)
    ///     .build();
    ///
    /// let center = Point::new(10, 20);
    /// let diameter = 10;
    ///
    /// let circle = Circle::with_center(center, diameter).into_styled(style);
    ///
    /// assert_eq!(circle.fill_area(), Circle::with_center(center, diameter - style.stroke_width));
    /// ```
    ///
    /// [`draw`]: crate::Drawable::draw
    pub fn fill_area(&self) -> T {
        self.style.fill_area(&self.primitive)
    }

    /// Returns the stroke area.
    ///
    /// The outer edge of the returned primitive coincides with the outer edge of the stroke that
    /// would be drawn by calling [`draw`] on this styled primitive.
    ///
    /// # Examples
    ///
    /// This example tests if a point lies on the stroke. Because the stoke area surrounds the
    /// stoke and fill an additional test is required to check that the point is not inside the fill
    /// area.
    ///
    /// ```
    /// use embedded_graphics::{
    ///     pixelcolor::Rgb888,
    ///     prelude::*,
    ///     primitives::{Circle, PrimitiveStyle},
    /// };
    ///
    /// let style = PrimitiveStyle::with_stroke(Rgb888::RED, 6);
    ///
    /// let center = Point::new(10, 20);
    /// let diameter = 10;
    ///
    /// let circle = Circle::with_center(center, diameter).into_styled(style);
    ///
    /// // A point lies on the stroke if it is inside the stroke area but not in the fill area.
    /// let is_on_stroke = |p| circle.stroke_area().contains(p) && !circle.fill_area().contains(p);
    ///
    /// assert!(is_on_stroke(center + Size::new(0, diameter / 2)));
    /// assert!(!is_on_stroke(center));
    /// ```
    ///
    /// [`draw`]: crate::Drawable::draw
    pub fn stroke_area(&self) -> T {
        self.style.stroke_area(&self.primitive)
    }
}

impl<T: StyledPixels<S>, S> Styled<T, S> {
    /// Returns an iterator over the pixels in this styled primitive.
    pub fn pixels(&self) -> T::Iter {
        self.primitive.pixels(&self.style)
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

#[cfg(feature = "async_draw")]
/// Async version of `StyledDrawable`.
pub trait AsyncStyledDrawable<S> {
    /// Color type.
    type Color: PixelColor;
    /// Output type.
    type Output;
    /// Draws the primitive using the given style.
    async fn draw_styled_async<D>(
        &self,
        style: &S,
        target: &mut D,
    ) -> Result<Self::Output, D::Error>
    where
        D: AsyncDrawTarget<Color = Self::Color>;
}

/// Styled dimensions.
pub trait StyledDimensions<S> {
    /// Returns the bounding box using the given style.
    fn styled_bounding_box(&self, style: &S) -> Rectangle;
}

/// Styled drawable.
pub trait StyledPixels<S> {
    /// Iterator type.
    type Iter;

    /// Returns an iterator over all pixels in this styled primitive.
    fn pixels(&self, style: &S) -> Self::Iter;
}
