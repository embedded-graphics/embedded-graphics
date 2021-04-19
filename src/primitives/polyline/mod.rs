//! The polyline primitive

use crate::{
    geometry::{Dimensions, Point, Size},
    primitives::{PointsIter, Primitive, Rectangle},
    transform::Transform,
};

mod points;
pub(in crate::primitives) mod scanline_intersections;
mod scanline_iterator;
mod styled;

pub use points::Points;
pub use styled::StyledPixelsIterator;

/// Polyline primitive
///
/// Creates an unfilled chained line shape.
///
/// # Examples
///
/// ## Draw a "heartbeat" shaped polyline
///
/// This example draws a stylized cardiogram in a 5px green stroke.
///
/// ```rust
/// use embedded_graphics::{
///     pixelcolor::Rgb565, prelude::*, primitives::{Polyline, PrimitiveStyle},
/// };
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::default();
/// # display.set_allow_out_of_bounds_drawing(true);
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
/// let line_style = PrimitiveStyle::with_stroke(Rgb565::GREEN, 5);
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
    pub const fn new(vertices: &'a [Point]) -> Self {
        Self {
            vertices,
            translate: Point::zero(),
        }
    }
}

impl<'a> Primitive for Polyline<'a> {}

impl<'a> PointsIter for Polyline<'a> {
    type Iter = Points<'a>;

    fn points(&self) -> Self::Iter {
        Points::new(self)
    }
}

impl<'a> Dimensions for Polyline<'a> {
    fn bounding_box(&self) -> Rectangle {
        match self.vertices {
            [] => Rectangle::zero(),
            [v] => Rectangle::new(*v, Size::zero()),
            vertices => {
                let top_left = vertices
                    .iter()
                    .map(|v| *v + self.translate)
                    .fold(Point::new(core::i32::MAX, core::i32::MAX), |accum, v| {
                        Point::new(accum.x.min(v.x), accum.y.min(v.y))
                    });

                let bottom_right = vertices
                    .iter()
                    .map(|v| *v + self.translate)
                    .fold(Point::new(core::i32::MIN, core::i32::MIN), |accum, v| {
                        Point::new(accum.x.max(v.x), accum.y.max(v.y))
                    });

                Rectangle::with_corners(top_left, bottom_right)
            }
        }
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
            translate: self.translate + by,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{Point, Size};

    // A "heartbeat" shaped polyline
    pub(in crate::primitives::polyline) const HEARTBEAT: [Point; 10] = [
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
    pub(in crate::primitives::polyline) const SMALL: [Point; 4] = [
        Point::new(2, 5),
        Point::new(5, 2),
        Point::new(10, 5),
        Point::new(15, 2),
    ];

    #[test]
    fn special_case_dimensions() {
        assert_eq!(Polyline::new(&[]).bounding_box(), Rectangle::zero(),);

        assert_eq!(
            Polyline::new(&[Point::new(15, 17)]).bounding_box(),
            Rectangle::new(Point::new(15, 17), Size::zero())
        );
    }

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
    fn translate_does_not_modify_size() {
        let points = [
            Point::new(5, 10),
            Point::new(7, 7),
            Point::new(5, 8),
            Point::new(10, 10),
        ];

        let polyline = Polyline::new(&points);
        let moved = polyline.translate(Point::new(10, 12));

        assert_eq!(moved.bounding_box().size, polyline.bounding_box().size);
    }

    #[test]
    fn translate_translated() {
        let points = [
            Point::new(5, 10),
            Point::new(7, 7),
            Point::new(5, 8),
            Point::new(10, 10),
        ];

        let polyline = Polyline::new(&points);
        let moved = polyline.translate(Point::new(10, 12));
        let moved2 = moved.translate(Point::new(10, 12));

        assert_eq!(
            moved.bounding_box(),
            polyline.bounding_box().translate(Point::new(10, 12))
        );
        assert_eq!(
            moved2.bounding_box(),
            polyline.bounding_box().translate(Point::new(20, 24))
        );
    }
}
