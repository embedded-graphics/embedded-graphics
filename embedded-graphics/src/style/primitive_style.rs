use crate::pixelcolor::PixelColor;
use core::convert::TryFrom;

/// Style properties for primitives.
///
/// `PrimitiveStyle` can be applied to a [primitive] to define how the primitive
/// is drawn.
///
/// Because `PrimitiveStyle` has the [`non_exhaustive`] attribute, it cannot be created using a
/// struct literal. To create a `PrimitiveStyle`, use the [`primitive_style!`] macro or
/// [`PrimitiveStyleBuilder`].
///
/// [primitive]: ../primitives/index.html
/// [`PrimitiveStyleBuilder`]: ../style/struct.PrimitiveStyleBuilder.html
/// [`non_exhaustive`]: https://blog.rust-lang.org/2019/12/19/Rust-1.40.0.html#[non_exhaustive]-structs,-enums,-and-variants
/// [`primitive_style!`]: ../macro.primitive_style.html
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
}

impl<C> PrimitiveStyle<C>
where
    C: PixelColor,
{
    /// Creates a primitive style without fill and stroke.
    pub fn new() -> Self {
        Self {
            fill_color: None,
            stroke_color: None,
            stroke_width: 0,
        }
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

    /// Returns the stroke width as an `i32`.
    ///
    /// If the stroke width is too large to fit into an `i32` the maximum value
    /// for an `i32` is returned instead.
    pub(crate) fn stroke_width_i32(&self) -> i32 {
        i32::try_from(self.stroke_width).unwrap_or(i32::max_value())
    }
}

impl<C> Default for PrimitiveStyle<C>
where
    C: PixelColor,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Primitive style builder.
///
/// Use this builder to create [`PrimitiveStyle`]s. If any properties on the builder are omitted,
/// the value will remain at its default value.
///
/// The [`primitive_style!`] macro can also be used to create [`PrimitiveStyle`]s, but with a
/// shorter syntax. See the [`primitive_style!`] documentation for examples.
///
/// # Examples
///
/// ## Build a style with configured stroke and fill
///
/// This example builds a style for a circle with a 3px red stroke and a solid green fill. The
/// circle is centered at (20, 20) with a radius of 10px.
///
/// ```rust
/// use embedded_graphics::{
///     egtext,
///     pixelcolor::Rgb565,
///     primitives::Circle,
///     prelude::*,
///     style::{PrimitiveStyle, PrimitiveStyleBuilder},
/// };
///
/// let style: PrimitiveStyle<Rgb565> = PrimitiveStyleBuilder::new()
///     .stroke_color(Rgb565::RED)
///     .stroke_width(3)
///     .fill_color(Rgb565::GREEN)
///     .build();
///
/// let circle = Circle::new(Point::new(20, 20), 10)
///     .into_styled(style);
/// ```
///
/// ## Build a style with stroke and no fill
///
/// This example builds a style for a rectangle with a 1px red stroke. Because `.fill_color()` is
/// not called, the fill color remains the default value of `None` (i.e. transparent).
///
/// ```rust
/// use embedded_graphics::{
///     egtext,
///     pixelcolor::Rgb565,
///     primitives::Rectangle,
///     prelude::*,
///     style::{PrimitiveStyle, PrimitiveStyleBuilder},
/// };
///
/// let style: PrimitiveStyle<Rgb565> = PrimitiveStyleBuilder::new()
///     .stroke_color(Rgb565::RED)
///     .stroke_width(1)
///     .build();
///
/// let rectangle = Rectangle::new(Point::new(20, 20), Point::new(40, 30))
///     .into_styled(style);
/// ```
///
/// [`PrimitiveStyle`]: ./struct.PrimitiveStyle.html
/// [`primitive_style!`]: ../macro.primitive_style.html
#[derive(Debug, PartialEq, Eq)]
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
    pub fn fill_color(&mut self, fill_color: C) -> &mut Self {
        self.style.fill_color = Some(fill_color);

        self
    }

    /// Sets the stroke color.
    pub fn stroke_color(&mut self, stroke_color: C) -> &mut Self {
        self.style.stroke_color = Some(stroke_color);

        self
    }

    /// Sets the stroke width.
    pub fn stroke_width(&mut self, stroke_width: u32) -> &mut Self {
        self.style.stroke_width = stroke_width;

        self
    }

    /// Builds the primitive style.
    pub fn build(&self) -> PrimitiveStyle<C> {
        self.style
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelcolor::{BinaryColor, Rgb888, RgbColor};

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
    fn stroke_width_i32() {
        let mut style: PrimitiveStyle<BinaryColor> = PrimitiveStyle::default();
        style.stroke_width = 1;
        assert_eq!(style.stroke_width_i32(), 1);

        style.stroke_width = 0x7FFFFFFF;
        assert_eq!(style.stroke_width_i32(), 0x7FFFFFFF);

        style.stroke_width = 0x80000000;
        assert_eq!(style.stroke_width_i32(), 0x7FFFFFFF);

        style.stroke_width = 0xFFFFFFFF;
        assert_eq!(style.stroke_width_i32(), 0x7FFFFFFF);
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
    fn builder_fill() {
        assert_eq!(
            PrimitiveStyleBuilder::new()
                .fill_color(BinaryColor::On)
                .build(),
            PrimitiveStyle::with_fill(BinaryColor::On)
        );
    }
}
