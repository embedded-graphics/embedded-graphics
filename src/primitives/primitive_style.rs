use crate::{
    pixelcolor::PixelColor,
    primitives::{OffsetOutline, Primitive, Styled},
    SaturatingCast,
};

/// Style properties for primitives.
///
/// `PrimitiveStyle` can be applied to a [primitive] to define how the primitive
/// is drawn.
///
/// Because `PrimitiveStyle` has the [`non_exhaustive`] attribute, it cannot be created using a
/// struct literal. To create a `PrimitiveStyle`, the [`with_stroke`](#method.with_stroke) and
/// [`with_fill`](#method.with_fill) methods can be used for styles that only require a stroke or
/// fill respectively. For more complex styles, use the [`PrimitiveStyleBuilder`].
///
/// [primitive]: ../primitives/index.html
/// [`PrimitiveStyleBuilder`]: struct.PrimitiveStyleBuilder.html
/// [`non_exhaustive`]: https://blog.rust-lang.org/2019/12/19/Rust-1.40.0.html#[non_exhaustive]-structs,-enums,-and-variants
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[non_exhaustive]
pub struct PrimitiveStyle<C>
where
    C: PixelColor,
{
    /// Fill color of the primitive.
    ///
    /// If `fill_color` is set to `None` no fill will be drawn.
    pub fill_color: Option<C>,

    /// Stroke color of the primitive.
    ///
    /// If `stroke_color` is set to `None` or the `stroke_width` is set to `0` no stroke will be
    /// drawn.
    pub stroke_color: Option<C>,

    /// Stroke width in pixels.
    pub stroke_width: u32,

    /// Stroke alignment.
    ///
    /// The stroke alignment sets if the stroke is drawn inside, outside or centered
    /// on the outline of a shape.
    ///
    /// This property only applies to closed shapes (rectangle, circle, ...) and is
    /// ignored for open shapes (line, ...).
    pub stroke_alignment: StrokeAlignment,
}

impl<C> PrimitiveStyle<C>
where
    C: PixelColor,
{
    /// Creates a primitive style without fill and stroke.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a stroke primitive style.
    ///
    /// If the `stroke_width` is `0` the resulting style won't draw a stroke.
    pub fn with_stroke(stroke_color: C, stroke_width: u32) -> Self {
        Self {
            stroke_color: Some(stroke_color),
            stroke_width,
            ..PrimitiveStyle::default()
        }
    }

    /// Creates a fill primitive style.
    pub fn with_fill(fill_color: C) -> Self {
        Self {
            fill_color: Some(fill_color),
            ..PrimitiveStyle::default()
        }
    }

    /// Returns the stroke width on the outside of the shape.
    ///
    /// The outside stroke width is determined by `stroke_width` and `stroke_alignment`.
    pub(crate) fn outside_stroke_width(&self) -> u32 {
        match self.stroke_alignment {
            StrokeAlignment::Inside => 0,
            StrokeAlignment::Center => self.stroke_width / 2,
            StrokeAlignment::Outside => self.stroke_width,
        }
    }

    /// Returns the stroke width on the inside of the shape.
    ///
    /// The inside stroke width is determined by `stroke_width` and `stroke_alignment`.
    pub(crate) fn inside_stroke_width(&self) -> u32 {
        match self.stroke_alignment {
            StrokeAlignment::Inside => self.stroke_width,
            StrokeAlignment::Center => self.stroke_width.saturating_add(1) / 2,
            StrokeAlignment::Outside => 0,
        }
    }

    /// Returns if a primitive drawn with this style is completely transparent.
    pub fn is_transparent(&self) -> bool {
        (self.stroke_color.is_none() || self.stroke_width == 0) && self.fill_color.is_none()
    }

    /// Returns the effective stroke color of the style.
    ///
    /// If the stroke width is 0, this method will return `None` regardless of the value in
    /// `stroke_color`.
    pub(crate) fn effective_stroke_color(&self) -> Option<C> {
        self.stroke_color.filter(|_| self.stroke_width > 0)
    }
}

impl<C> Default for PrimitiveStyle<C>
where
    C: PixelColor,
{
    fn default() -> Self {
        Self {
            fill_color: None,
            stroke_color: None,
            stroke_width: 0,
            stroke_alignment: StrokeAlignment::Center,
        }
    }
}

/// Primitive style builder.
///
/// Use this builder to create [`PrimitiveStyle`]s. If any properties on the builder are omitted,
/// the value will remain at its default value.
///
/// # Examples
///
/// ## Build a style with configured stroke and fill
///
/// This example builds a style for a circle with a 3px red stroke and a solid green fill. The
/// circle has its top-left at (10, 10) with a diameter of 20px.
///
/// ```rust
/// use embedded_graphics::{
///     pixelcolor::Rgb565,
///     prelude::*,
///     primitives::{Circle, PrimitiveStyle, PrimitiveStyleBuilder},
/// };
///
/// let style: PrimitiveStyle<Rgb565> = PrimitiveStyleBuilder::new()
///     .stroke_color(Rgb565::RED)
///     .stroke_width(3)
///     .fill_color(Rgb565::GREEN)
///     .build();
///
/// let circle = Circle::new(Point::new(10, 10), 20).into_styled(style);
/// ```
///
/// ## Build a style with stroke and no fill
///
/// This example builds a style for a rectangle with a 1px red stroke. Because `.fill_color()` is
/// not called, the fill color remains the default value of `None` (i.e. transparent).
///
/// ```rust
/// use embedded_graphics::{
///     pixelcolor::Rgb565,
///     prelude::*,
///     primitives::{Rectangle, PrimitiveStyle, PrimitiveStyleBuilder},
/// };
///
/// let style: PrimitiveStyle<Rgb565> = PrimitiveStyleBuilder::new()
///     .stroke_color(Rgb565::RED)
///     .stroke_width(1)
///     .build();
///
/// let rectangle = Rectangle::new(Point::new(20, 20), Size::new(20, 10)).into_styled(style);
/// ```
///
/// [`PrimitiveStyle`]: ./struct.PrimitiveStyle.html
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct PrimitiveStyleBuilder<C>
where
    C: PixelColor,
{
    style: PrimitiveStyle<C>,
}

impl<C> PrimitiveStyleBuilder<C>
where
    C: PixelColor,
{
    /// Creates a new primitive style builder.
    pub fn new() -> Self {
        Self {
            style: PrimitiveStyle::default(),
        }
    }

    /// Sets the fill color.
    pub fn fill_color(mut self, fill_color: C) -> Self {
        self.style.fill_color = Some(fill_color);

        self
    }

    /// Resets the fill color to transparent.
    pub fn reset_fill_color(mut self) -> Self {
        self.style.fill_color = None;

        self
    }

    /// Sets the stroke color.
    pub fn stroke_color(mut self, stroke_color: C) -> Self {
        self.style.stroke_color = Some(stroke_color);

        self
    }

    /// Resets the stroke color to transparent.
    pub fn reset_stroke_color(mut self) -> Self {
        self.style.stroke_color = None;

        self
    }

    /// Sets the stroke width.
    pub fn stroke_width(mut self, stroke_width: u32) -> Self {
        self.style.stroke_width = stroke_width;

        self
    }

    /// Sets the stroke alignment.
    pub fn stroke_alignment(mut self, stroke_alignment: StrokeAlignment) -> Self {
        self.style.stroke_alignment = stroke_alignment;

        self
    }

    /// Builds the primitive style.
    pub fn build(self) -> PrimitiveStyle<C> {
        self.style
    }
}

impl<C> From<&PrimitiveStyle<C>> for PrimitiveStyleBuilder<C>
where
    C: PixelColor,
{
    fn from(style: &PrimitiveStyle<C>) -> Self {
        Self { style: *style }
    }
}

/// Stroke alignment.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum StrokeAlignment {
    /// Inside.
    Inside,
    /// Center.
    Center,
    /// Outside.
    Outside,
}

impl Default for StrokeAlignment {
    fn default() -> Self {
        Self::Center
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelcolor::{BinaryColor, Rgb888, RgbColor};

    #[test]
    fn default_style() {
        assert_eq!(
            PrimitiveStyle::<BinaryColor>::default(),
            PrimitiveStyle {
                fill_color: None,
                stroke_color: None,
                stroke_width: 0,
                stroke_alignment: StrokeAlignment::Center,
            }
        );

        assert_eq!(
            PrimitiveStyle::<BinaryColor>::default(),
            PrimitiveStyle::new()
        );
    }

    #[test]
    fn constructors() {
        let style = PrimitiveStyle::with_fill(Rgb888::RED);
        assert_eq!(style.fill_color, Some(Rgb888::RED));
        assert_eq!(style.stroke_color, None);

        let style = PrimitiveStyle::with_stroke(Rgb888::GREEN, 123);
        assert_eq!(style.fill_color, None);
        assert_eq!(style.stroke_color, Some(Rgb888::GREEN));
        assert_eq!(style.stroke_width, 123);
    }

    #[test]
    fn stroke_alignment_1px() {
        let mut style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);

        style.stroke_alignment = StrokeAlignment::Inside;
        assert_eq!(style.inside_stroke_width(), 1);
        assert_eq!(style.outside_stroke_width(), 0);

        style.stroke_alignment = StrokeAlignment::Center;
        assert_eq!(style.inside_stroke_width(), 1);
        assert_eq!(style.outside_stroke_width(), 0);

        style.stroke_alignment = StrokeAlignment::Outside;
        assert_eq!(style.inside_stroke_width(), 0);
        assert_eq!(style.outside_stroke_width(), 1);
    }

    #[test]
    fn stroke_alignment_2px() {
        let mut style = PrimitiveStyle::with_stroke(BinaryColor::On, 2);

        style.stroke_alignment = StrokeAlignment::Inside;
        assert_eq!(style.inside_stroke_width(), 2);
        assert_eq!(style.outside_stroke_width(), 0);

        style.stroke_alignment = StrokeAlignment::Center;
        assert_eq!(style.inside_stroke_width(), 1);
        assert_eq!(style.outside_stroke_width(), 1);

        style.stroke_alignment = StrokeAlignment::Outside;
        assert_eq!(style.inside_stroke_width(), 0);
        assert_eq!(style.outside_stroke_width(), 2);
    }

    #[test]
    fn builder_default() {
        assert_eq!(
            PrimitiveStyleBuilder::<BinaryColor>::new().build(),
            PrimitiveStyle::<BinaryColor>::default()
        );
    }

    #[test]
    fn builder_stroke() {
        assert_eq!(
            PrimitiveStyleBuilder::new()
                .stroke_color(BinaryColor::On)
                .stroke_width(10)
                .build(),
            PrimitiveStyle::with_stroke(BinaryColor::On, 10)
        );
    }

    #[test]
    fn builder_reset_stroke_color() {
        assert_eq!(
            PrimitiveStyleBuilder::new()
                .stroke_color(BinaryColor::On)
                .stroke_width(10)
                .fill_color(BinaryColor::Off)
                .reset_stroke_color()
                .build(),
            PrimitiveStyleBuilder::new()
                .stroke_width(10)
                .fill_color(BinaryColor::Off)
                .build()
        );
    }

    #[test]
    fn builder_fill() {
        assert_eq!(
            PrimitiveStyleBuilder::new()
                .fill_color(BinaryColor::On)
                .build(),
            PrimitiveStyle::with_fill(BinaryColor::On)
        );
    }

    #[test]
    fn builder_reset_fill_color() {
        assert_eq!(
            PrimitiveStyleBuilder::new()
                .fill_color(BinaryColor::On)
                .stroke_color(BinaryColor::Off)
                .reset_fill_color()
                .build(),
            PrimitiveStyleBuilder::new()
                .stroke_color(BinaryColor::Off)
                .build(),
        );
    }

    #[test]
    fn effective_stroke_color() {
        assert_eq!(
            PrimitiveStyle::with_stroke(BinaryColor::On, 1).effective_stroke_color(),
            Some(BinaryColor::On)
        );

        assert_eq!(
            PrimitiveStyle::with_stroke(BinaryColor::On, 0).effective_stroke_color(),
            None
        );
    }

    #[test]
    fn stroke_width_max_value() {
        assert_eq!(
            PrimitiveStyleBuilder::from(&PrimitiveStyle::with_stroke(
                BinaryColor::On,
                core::u32::MAX
            ))
            .stroke_alignment(StrokeAlignment::Center)
            .build()
            .inside_stroke_width(),
            core::u32::MAX / 2
        );
    }
}
