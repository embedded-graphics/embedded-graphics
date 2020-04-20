//! The polyline primitive

use crate::{
    draw_target::DrawTarget,
    drawable::{Drawable, Pixel},
    geometry::{Dimensions, Point},
    pixelcolor::PixelColor,
    primitives::{line::Line, thick_line_iterator::ThickLineIterator, Primitive, Rectangle},
    style::{PrimitiveStyle, Styled},
    transform::Transform,
};

/// Polyline primitive
///
/// Creates an unfilled chained line shape. Styles with a width greater than 1px are not currently
/// supported.
///
/// # Examples
///
/// ## Draw a "heartbeat" shaped polyline
///
/// This example draws a stylized cardiogram in
///
/// ```rust
/// use embedded_graphics::{
///     pixelcolor::Rgb565, prelude::*, primitives::Polyline, style::PrimitiveStyle,
/// };
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::default();
///
/// // A "heartbeat" shaped polyline
/// let points: [Point; 10] = [
///     Point::new(10, 64),
///     Point::new(50, 64),
///     Point::new(60, 44),
///     Point::new(70, 64),
///     Point::new(80, 64),
///     Point::new(90, 74),
///     Point::new(100, 10),
///     Point::new(110, 84),
///     Point::new(120, 64),
///     Point::new(300, 64),
/// ];
///
/// let line_style = PrimitiveStyle::with_stroke(Rgb565::GREEN, 1);
///
/// Polyline::new(&points)
///     .into_styled(line_style)
///     .draw(&mut display)?;
/// # Ok::<(), core::convert::Infallible>(())
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Polyline<'a> {
    translate: Point,

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

        Self {
            vertices,
            translate: Point::zero(),
        }
    }
}

impl<'a> Primitive for Polyline<'a> {
    type PointsIter = PolylineIterator<'a>;

    fn points(&self) -> Self::PointsIter {
        self.into_iter()
    }
}

impl<'a> Dimensions for Polyline<'a> {
    fn bounding_box(&self) -> Rectangle {
        let top_left = self
            .vertices
            .iter()
            .map(|v| *v + self.translate)
            .fold(Point::new(core::i32::MAX, core::i32::MAX), |accum, v| {
                Point::new(accum.x.min(v.x), accum.y.min(v.y))
            });

        let bottom_right = self
            .vertices
            .iter()
            .map(|v| *v + self.translate)
            .fold(Point::new(core::i32::MIN, core::i32::MIN), |accum, v| {
                Point::new(accum.x.max(v.x), accum.y.max(v.y))
            });

        Rectangle::with_corners(top_left, bottom_right)
    }
}

impl<'a> Transform for Polyline<'a> {
    /// Translate the polyline from its current position to a new position by (x, y) pixels, returning
    /// a new `Polyline`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Polyline;
    /// # use embedded_graphics::prelude::*;
    /// let points = [
    ///     Point::new(5, 10),
    ///     Point::new(7, 7),
    ///     Point::new(5, 8),
    ///     Point::new(10, 10),
    /// ];
    ///
    /// let polyline = Polyline::new(&points);
    /// let moved = polyline.translate(Point::new(10, 12));
    ///
    /// assert_eq!(polyline.bounding_box().top_left, Point::new(5, 7));
    /// assert_eq!(moved.bounding_box().top_left, Point::new(15, 19));
    /// ```
    fn translate(&self, by: Point) -> Self {
        Self {
            translate: by,
            ..*self
        }
    }

    /// Translate the polyline from its current position to a new position by (x, y) pixels.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Polyline;
    /// # use embedded_graphics::prelude::*;
    /// let points = [
    ///     Point::new(5, 10),
    ///     Point::new(7, 7),
    ///     Point::new(5, 8),
    ///     Point::new(10, 10),
    /// ];
    ///
    /// let mut polyline = Polyline::new(&points);
    ///
    /// polyline.translate_mut(Point::new(10, 12));
    ///
    /// assert_eq!(polyline.bounding_box().top_left, Point::new(15, 19));
    /// ```
    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.translate += by;

        self
    }
}

/// An iterator over all pixel positions on the polyline
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PolylineIterator<'a> {
    stop: bool,
    vertices: &'a [Point],
    translate: Point,
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

            self.segment_iter = ThickLineIterator::new(
                &Line::new(*start + self.translate, *end + self.translate),
                1,
            );

            // Skip first point of next line, otherwise we overlap with the previous line
            Self::next(self);

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
                    translate: self.translate,
                    segment_iter:ThickLineIterator::new(
                        &Line::new(*start + self.translate, *end + self.translate),
                        1,
                    ),
                })
            })
            .unwrap_or_else(||
                // Polyline is less than 2 vertices long. Return a dummy iterator that will short
                // circuit due to `stop: true`
                Self::IntoIter {
                    stop: true,
                    vertices: &[],
                    translate: Point::zero(),
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
    use crate::{geometry::Size, pixelcolor::Rgb565, prelude::*};

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

        let bb = polyline.bounding_box();

        assert_eq!(bb.top_left, Point::new(10, 10));
        assert_eq!(bb.top_left + bb.size, Point::new(301, 85));
        assert_eq!(bb.size, Size::new(291, 75));
    }

    #[test]
    fn negative_dimensions() {
        let mut negative: [Point; 10] = [Point::zero(); 10];

        for (i, v) in HEARTBEAT.iter().enumerate() {
            negative[i] = *v - Point::new(100, 100);
        }

        let polyline = Polyline::new(&negative);

        let bb = polyline.bounding_box();

        assert_eq!(bb.top_left, Point::new(-90, -90));
        assert_eq!(bb.top_left + bb.size, Point::new(201, -15));
        assert_eq!(bb.size, Size::new(291, 75));
    }

    #[test]
    fn transformed_dimensions() {
        let polyline = Polyline::new(&HEARTBEAT).translate(Point::new(-100, -100));

        let bb = polyline.bounding_box();

        assert_eq!(bb.top_left, Point::new(-90, -90));
        assert_eq!(bb.top_left + bb.size, Point::new(201, -15));
        assert_eq!(bb.size, Size::new(291, 75));
    }

    #[test]
    #[should_panic]
    fn one_point() {
        let _polyline = Polyline::new(&[Point::zero()]);
    }

    // Ensure that polylines only draw 1px wide due to lack of support for line joiners. This test
    // should fail when joiners are supported and should be removed then.
    #[test]
    fn one_px_wide_only() {
        let polyline = Polyline::new(&HEARTBEAT);

        let thick = polyline.into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 10));
        let thin = polyline.into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 1));

        assert!(thick.into_iter().eq(thin.into_iter()));
    }
}
