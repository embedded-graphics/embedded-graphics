//! Styling struct to customise the look of objects.

use pixelcolor::PixelColor;

/// Style properties for an object
#[derive(Debug, Copy, Clone)]
pub struct Style<P: PixelColor> {
    /// Fill colour of the object
    ///
    /// Has no effect for fonts
    pub fill_color: Option<P>,

    /// Stroke (border/line) color of the object
    ///
    /// For fonts, this is the foreground colour of the text
    pub stroke_color: Option<P>,
}

impl<P> Style<P>
where
    P: PixelColor,
{
    /// Create a new style with a given stroke value and defaults for everything else
    pub fn with_stroke(stroke_color: P) -> Self {
        Self {
            stroke_color: Some(stroke_color),
            ..Style::default()
        }
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
        }
    }
}

/// Add a style to an object
pub trait WithStyle<C>
where
    C: PixelColor,
{
    /// Add a complete style to the object
    fn with_style(self, style: Style<C>) -> Self;

    /// Set the stroke property of the object's style
    ///
    /// This can be a noop
    fn with_stroke(self, stroke: Option<C>) -> Self;

    /// Set the fill property of the object's style
    ///
    /// This can be a noop
    fn with_fill(self, stroke: Option<C>) -> Self;
}
