//! The line primitive

// TODO: Group imports
use crate::draw_target::DrawTarget;
use crate::drawable::Drawable;
use crate::drawable::Pixel;
use crate::geometry::Dimensions;
use crate::geometry::Size;
use crate::pixelcolor::PixelColor;
use crate::primitives::Primitive;
use crate::style::PrimitiveStyle;
use crate::style::Styled;
use crate::{
    geometry::Point,
    primitives::{line::Line, thick_line_iterator::ThickLineIterator},
};

/// Polyline primitive
///
/// Creates an unfilled chained line shape
///
/// # Examples
///
/// ## Create some lines with different styles
///
/// ```rust
/// use embedded_graphics::{
///     pixelcolor::Rgb565, prelude::*, primitives::Line, style::PrimitiveStyle,
/// };
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::default();
///
/// // TODO: Example
/// # Ok::<(), core::convert::Infallible>(())
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Polyline<'a> {
    /// All vertices in the line
    pub vertices: &'a [Point],
}

impl<'a> Polyline<'a> {
    /// Create a new polyline from a list of vertices
    ///
    /// # Panics
    ///
    /// This method will panic if the number of vertices is less than 2
    pub fn new(vertices: &'a [Point]) -> Self {
        if vertices.len() < 2 {
            panic!("Polyline must contain at least two vertices");
        }

        Self { vertices }
    }
}

impl<'a> Primitive for Polyline<'a> {}

impl<'a> Dimensions for Polyline<'a> {
    fn top_left(&self) -> Point {
        self.vertices
            .iter()
            .fold(Point::new(core::i32::MAX, core::i32::MAX), |accum, v| {
                Point::new(accum.x.min(v.x), accum.y.min(v.y))
            })
    }

    fn bottom_right(&self) -> Point {
        self.vertices
            .iter()
            .fold(Point::new(core::i32::MIN, core::i32::MIN), |accum, v| {
                Point::new(accum.x.max(v.x), accum.y.max(v.y))
            })
    }

    fn size(&self) -> Size {
        Size::from_bounding_box(self.top_left(), self.bottom_right())
    }
}

/// TODO: Docs
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PolylineIterator<'a> {
    stop: bool,
    vertices: &'a [Point],
    segment_iter: ThickLineIterator,
}

impl<'a> Iterator for PolylineIterator<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stop {
            return None;
        }

        if let Some(p) = self.segment_iter.next() {
            Some(p)
        } else {
            let (start, rest) = self.vertices.split_first()?;
            let end = rest.get(0)?;

            self.vertices = rest;

            self.segment_iter = ThickLineIterator::new(&Line::new(*start, *end), 1);

            Self::next(self)
        }
    }
}

impl<'a> IntoIterator for Polyline<'a> {
    type Item = Point;
    type IntoIter = PolylineIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.vertices
            .split_first()
            .and_then(|(start, rest)| {
                // Polyline is 2 or more vertices long, return an iterator for it
                rest.get(0).map(|end| Self::IntoIter {
                    stop: false,
                    vertices: rest,
                    segment_iter: ThickLineIterator::new(&Line::new(*start, *end), 1),
                })
            })
            .unwrap_or_else(||
                // Polyline is less than 2 vertices long. Return a dummy iterator that will short
                // circuit due to `stop: true`
                Self::IntoIter {
                    stop: true,
                    vertices: &[],
                    segment_iter: ThickLineIterator::new(&Line::new(Point::zero(), Point::zero()), 1),
                })
    }
}

impl<'a, C> IntoIterator for &'a Styled<Polyline<'a>, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = StyledPolylineIterator<'a, C>;

    fn into_iter(self) -> Self::IntoIter {
        StyledPolylineIterator {
            style: self.style,

            line_iter: self.primitive.into_iter(),
        }
    }
}

/// Pixel iterator for each pixel in the line
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct StyledPolylineIterator<'a, C>
where
    C: PixelColor,
{
    style: PrimitiveStyle<C>,

    line_iter: PolylineIterator<'a>,
}

// [Bresenham's line algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm)
impl<'a, C: PixelColor> Iterator for StyledPolylineIterator<'a, C> {
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        // Break if stroke width is zero
        if self.style.stroke_width == 0 {
            return None;
        }

        // Return none if stroke color is none
        let stroke_color = self.style.stroke_color?;

        self.line_iter
            .next()
            .map(|point| Pixel(point, stroke_color))
    }
}

impl<'a, C: 'a> Drawable<C> for &Styled<Polyline<'a>, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        display.draw_iter(self.into_iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{drawable::Pixel, pixelcolor::BinaryColor};

    // A "heartbeat" shaped polyline
    const HEARTBEAT: [Point; 10] = [
        Point::new(10, 64),
        Point::new(50, 64),
        Point::new(60, 44),
        Point::new(70, 64),
        Point::new(80, 64),
        Point::new(90, 74),
        Point::new(100, 10),
        Point::new(110, 84),
        Point::new(120, 64),
        Point::new(300, 64),
    ];

    #[test]
    fn positive_dimensions() {
        let polyline = Polyline::new(&HEARTBEAT);

        assert_eq!(polyline.top_left(), Point::new(10, 10));
        assert_eq!(polyline.bottom_right(), Point::new(300, 84));
        assert_eq!(polyline.size(), Size::new(290, 74));
    }

    #[test]
    fn negative_dimensions() {
        let mut negative: [Point; 10] = [Point::zero(); 10];

        for (i, v) in HEARTBEAT.iter().enumerate() {
            negative[i] = *v - Point::new(100, 100);
        }

        let polyline = Polyline::new(&negative);

        assert_eq!(polyline.top_left(), Point::new(-90, -90));
        assert_eq!(polyline.bottom_right(), Point::new(200, -16));
        assert_eq!(polyline.size(), Size::new(290, 74));
    }
}
