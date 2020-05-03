//! The polyline primitive

use crate::{
    draw_target::DrawTarget,
    drawable::{Drawable, Pixel},
    geometry::{Dimensions, Point},
    pixelcolor::PixelColor,
    primitives::{
        line::{self, Line},
        Primitive, Rectangle,
    },
    style::{PrimitiveStyle, Styled},
    transform::Transform,
};

/// Polyline primitive
///
/// Creates an unfilled chained line shape. Styles with a width greater than 1px are not currently
/// supported and will always render as a 1px wide line.
///
/// # Examples
///
/// ## Draw a "heartbeat" shaped polyline
///
/// This example draws a stylized cardiogram in a 1px green stroke.
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
    /// An offset to apply to the polyline as a whole
    pub translate: Point,

    /// All vertices in the line
    pub vertices: &'a [Point],
}

impl<'a> Polyline<'a> {
    /// Create a new polyline from a list of vertices
    ///
    /// If fewer than two vertices are provided, the line will not render anything when drawn.
    pub fn new(vertices: &'a [Point]) -> Self {
        Self {
            vertices,
            translate: Point::zero(),
        }
    }
}

impl<'a> Primitive for Polyline<'a> {
    type PointsIter = Points<'a>;

    fn points(&self) -> Self::PointsIter {
        Points::new(self)
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
pub struct Points<'a> {
    vertices: &'a [Point],
    translate: Point,
    segment_iter: line::Points,
}

impl<'a> Points<'a> {
    fn new<'b>(polyline: &'b Polyline<'a>) -> Self
    where
        'a: 'b,
    {
        Points {
            vertices: polyline.vertices,
            translate: polyline.translate,
            segment_iter: Line::new(Point::zero(), Point::zero()).points(),
        }
    }
}

impl<'a> Iterator for Points<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.vertices.len() < 2 {
            return None;
        }

        if let Some(p) = self.segment_iter.next() {
            Some(p)
        } else {
            let (start, rest) = self.vertices.split_first()?;
            let end = rest.get(0)?;

            self.vertices = rest;

            self.segment_iter = Line::new(*start + self.translate, *end + self.translate).points();

            // Skip first point of next line, otherwise we overlap with the previous line
            self.nth(1)
        }
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
            stroke_width: self.style.stroke_width,
            stroke_color: self.style.stroke_color,
            line_iter: self.primitive.points(),
        }
    }
}

/// Pixel iterator for each pixel in the line
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct StyledPolylineIterator<'a, C>
where
    C: PixelColor,
{
    stroke_width: u32,
    stroke_color: Option<C>,
    line_iter: Points<'a>,
}

impl<'a, C: PixelColor> Iterator for StyledPolylineIterator<'a, C> {
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        // Break if stroke width is zero
        if self.stroke_width == 0 {
            return None;
        }

        // Return none if stroke color is none
        let stroke_color = self.stroke_color?;

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
    use crate::{
        mock_display::MockDisplay,
        pixelcolor::{BinaryColor, Rgb565},
        prelude::*,
    };

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

    // Smaller test pattern for mock display
    const SMALL: [Point; 4] = [
        Point::new(2, 5),
        Point::new(5, 2),
        Point::new(10, 5),
        Point::new(15, 2),
    ];

    #[test]
    fn positive_dimensions() {
        let polyline = Polyline::new(&HEARTBEAT);

        let bb = polyline.bounding_box();

        assert_eq!(
            bb,
            Rectangle::with_corners(Point::new(10, 10), Point::new(300, 84))
        );
    }

    #[test]
    fn negative_dimensions() {
        let mut negative: [Point; 10] = [Point::zero(); 10];

        for (i, v) in HEARTBEAT.iter().enumerate() {
            negative[i] = *v - Point::new(100, 100);
        }

        let polyline = Polyline::new(&negative);

        let bb = polyline.bounding_box();

        assert_eq!(
            bb,
            Rectangle::with_corners(Point::new(-90, -90), Point::new(200, -16))
        );
    }

    #[test]
    fn transformed_dimensions() {
        let polyline = Polyline::new(&HEARTBEAT).translate(Point::new(-100, -100));

        let bb = polyline.bounding_box();

        assert_eq!(
            bb,
            Rectangle::with_corners(Point::new(-90, -90), Point::new(200, -16))
        );
    }

    #[test]
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

    #[test]
    fn mock_display() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();

        Polyline::new(&SMALL)
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut display)
            .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "                ",
                "                ",
                "     #         #",
                "    # ##     ## ",
                "   #    ## ##   ",
                "  #       #     ",
            ])
        );
    }

    // Ensure that consecutive points are always different
    #[test]
    fn no_duplicate_points() {
        let expected: [Point; 14] = [
            Point::new(2, 5),
            Point::new(3, 4),
            Point::new(4, 3),
            Point::new(5, 2),
            Point::new(6, 3),
            Point::new(7, 3),
            Point::new(8, 4),
            Point::new(9, 4),
            Point::new(10, 5),
            Point::new(11, 4),
            Point::new(12, 4),
            Point::new(13, 3),
            Point::new(14, 3),
            Point::new(15, 2),
        ];

        assert!(Polyline::new(&SMALL).points().eq(expected.iter().copied()))
    }
}
