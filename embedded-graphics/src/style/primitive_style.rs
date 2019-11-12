use crate::pixelcolor::PixelColor;
use core::convert::TryFrom;

/// Style properties for primitives.
///
/// `PrimitiveStyle` can be applied to a [primitive] to define how the primitive
/// is drawn.
///
/// [primitive]: ../primitives/index.html
#[derive(Debug, Copy, Clone)]
pub struct PrimitiveStyle<C>
where
    C: PixelColor,
{
    /// Fill color of the primitive.
    pub fill_color: Option<C>,

    /// Stroke color of the primitive.
    pub stroke_color: Option<C>,

    /// Stroke width in pixels.
    pub stroke_width: u32,
}

impl<C> PrimitiveStyle<C>
where
    C: PixelColor,
{
    /// Creates a stroke primitive style.
    pub fn stroke(stroke_color: C, stroke_width: u32) -> Self {
        Self {
            stroke_color: Some(stroke_color),
            stroke_width,
            ..PrimitiveStyle::default()
        }
    }

    /// Creates a fill primitive style.
    pub fn fill(fill_color: C) -> Self {
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
        Self {
            fill_color: None,
            stroke_color: None,
            stroke_width: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelcolor::{BinaryColor, Rgb888, RgbColor};

    #[test]
    fn constructors() {
        let style = PrimitiveStyle::fill(Rgb888::RED);
        assert_eq!(style.fill_color, Some(Rgb888::RED));
        assert_eq!(style.stroke_color, None);

        let style = PrimitiveStyle::stroke(Rgb888::GREEN, 123);
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
}
