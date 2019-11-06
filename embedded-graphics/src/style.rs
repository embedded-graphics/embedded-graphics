//! Styling struct to customise the look of objects.

use crate::pixelcolor::PixelColor;
use core::convert::TryFrom;

/// Style properties for an object
#[derive(Debug, Copy, Clone)]
pub struct Style<P: PixelColor> {
    /// Fill colour of the object
    ///
    /// For fonts, this is the background colour of the text
    pub fill_color: Option<P>,

    /// Stroke (border/line) color of the object
    ///
    /// For fonts, this is the foreground colour of the text
    pub stroke_color: Option<P>,

    /// Stroke width
    ///
    /// Set the stroke width for an object. Has no effect on fonts.
    pub stroke_width: u32,
}

impl<P> Style<P>
where
    P: PixelColor,
{
    /// Create a new style with a given stroke value and defaults for everything else
    pub fn stroke_color(stroke_color: P) -> Self {
        Self {
            stroke_color: Some(stroke_color),
            ..Style::default()
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

impl<P> Default for Style<P>
where
    P: PixelColor,
{
    fn default() -> Self {
        Self {
            fill_color: None,
            stroke_color: None,
            stroke_width: 1,
        }
    }
}

/// Add a style to an object
pub trait WithStyle<C>
where
    C: PixelColor,
{
    /// Add a complete style to the object
    fn style(self, style: Style<C>) -> Self;

    /// Set the stroke colour for the object
    ///
    /// This can be a noop
    fn stroke_color(self, color: Option<C>) -> Self;

    /// Set the stroke width for the object
    ///
    /// A stroke with a width of zero will not be rendered
    fn stroke_width(self, width: u32) -> Self;

    /// Set the fill property of the object's style
    ///
    /// This can be a noop
    fn fill_color(self, color: Option<C>) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelcolor::BinaryColor;

    #[test]
    fn stroke_width_i32() {
        let mut style: Style<BinaryColor> = Style::default();
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
