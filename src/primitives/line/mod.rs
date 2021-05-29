//! The line primitive

use crate::{
    geometry::{Dimensions, Point},
    primitives::{
        common::StrokeOffset,
        line::thick_points::{ParallelLineType, ParallelsIterator},
        PointsIter, Primitive, Rectangle,
    },
    transform::Transform,
};
use az::SaturatingAs;

mod bresenham;
pub(in crate::primitives) mod intersection_params;
mod points;
mod styled;
mod thick_points;

pub use points::Points;
pub use styled::StyledPixelsIterator;

/// Line primitive
///
/// # Examples
///
/// ## Create some lines with different styles
///
/// ```rust
/// use embedded_graphics::{
///     pixelcolor::Rgb565, prelude::*, primitives::{Line, PrimitiveStyle},
/// };
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::default();
///
/// // Red 1 pixel wide line from (50, 20) to (60, 35)
/// Line::new(Point::new(50, 20), Point::new(60, 35))
///     .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 1))
///     .draw(&mut display)?;
///
/// // Green 10 pixel wide line with translation applied
/// Line::new(Point::new(50, 20), Point::new(60, 35))
///     .translate(Point::new(-30, 10))
///     .into_styled(PrimitiveStyle::with_stroke(Rgb565::GREEN, 10))
///     .draw(&mut display)?;
/// # Ok::<(), core::convert::Infallible>(())
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Line {
    /// Start point
    pub start: Point,

    /// End point
    pub end: Point,
}

impl Primitive for Line {}

impl PointsIter for Line {
    type Iter = Points;

    fn points(&self) -> Self::Iter {
        Points::new(self)
    }
}

impl Dimensions for Line {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::with_corners(self.start, self.end)
    }
}

impl Line {
    /// Create a new line
    pub const fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    /// Returns a perpendicular line.
    ///
    /// The returned line is rotated 90 degree counter clockwise and shares the start point with the
    /// original line.
    fn perpendicular(&self) -> Self {
        let delta = self.end - self.start;
        let delta = Point::new(delta.y, -delta.x);

        Line::new(self.start, self.start + delta)
    }

    /// Get two lines representing the left and right edges of the thick line.
    ///
    /// If a thickness of `0` is given, the lines returned will lie on the same points as `self`.
    pub(in crate::primitives) fn extents(
        &self,
        thickness: u32,
        stroke_offset: StrokeOffset,
    ) -> (Line, Line) {
        let mut it = ParallelsIterator::new(self, thickness.saturating_as(), stroke_offset);
        let reduce =
            it.parallel_parameters.position_step.major + it.parallel_parameters.position_step.minor;

        let mut left = (self.start, ParallelLineType::Normal);
        let mut right = (self.start, ParallelLineType::Normal);

        match stroke_offset {
            StrokeOffset::None => loop {
                if let Some((bresenham, reduce)) = it.next() {
                    right = (bresenham.point, reduce);
                } else {
                    break;
                }

                if let Some((bresenham, reduce)) = it.next() {
                    left = (bresenham.point, reduce);
                } else {
                    break;
                }
            },
            StrokeOffset::Left => {
                if let Some((bresenham, reduce)) = it.last() {
                    left = (bresenham.point, reduce);
                }
            }
            StrokeOffset::Right => {
                if let Some((bresenham, reduce)) = it.last() {
                    right = (bresenham.point, reduce);
                }
            }
        };

        let left_start = left.0;
        let right_start = right.0;

        let delta = self.end - self.start;

        let left_line = Line::new(
            left_start,
            left_start + delta
                - match left.1 {
                    ParallelLineType::Normal => Point::zero(),
                    ParallelLineType::Extra => reduce,
                },
        );

        let right_line = Line::new(
            right_start,
            right_start + delta
                - match right.1 {
                    ParallelLineType::Normal => Point::zero(),
                    ParallelLineType::Extra => reduce,
                },
        );
        (left_line, right_line)
    }

    /// Compute the midpoint of the line.
    pub fn midpoint(&self) -> Point {
        self.start + (self.end - self.start) / 2
    }

    /// Compute the delta (`end - start`) of the line.
    pub fn delta(&self) -> Point {
        self.end - self.start
    }
}

impl Transform for Line {
    /// Translate the line from its current position to a new position by (x, y) pixels, returning
    /// a new `Line`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Line;
    /// # use embedded_graphics::prelude::*;
    /// let line = Line::new(Point::new(5, 10), Point::new(15, 20));
    /// let moved = line.translate(Point::new(10, 10));
    ///
    /// assert_eq!(moved.start, Point::new(15, 20));
    /// assert_eq!(moved.end, Point::new(25, 30));
    /// ```
    fn translate(&self, by: Point) -> Self {
        Self {
            start: self.start + by,
            end: self.end + by,
        }
    }

    /// Translate the line from its current position to a new position by (x, y) pixels.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Line;
    /// # use embedded_graphics::prelude::*;
    /// let mut line = Line::new(Point::new(5, 10), Point::new(15, 20));
    /// line.translate_mut(Point::new(10, 10));
    ///
    /// assert_eq!(line.start, Point::new(15, 20));
    /// assert_eq!(line.end, Point::new(25, 30));
    /// ```
    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.start += by;
        self.end += by;

        self
    }
}

/// Pixel iterator for each pixel in the line
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::Size, mock_display::MockDisplay, pixelcolor::BinaryColor,
        primitives::PrimitiveStyle, Drawable, Pixel,
    };
    use arrayvec::ArrayVec;

    #[test]
    fn bounding_box() {
        let start = Point::new(10, 10);
        let end = Point::new(19, 29);

        let line: Line = Line::new(start, end);
        let backwards_line: Line = Line::new(end, start);

        assert_eq!(
            line.bounding_box(),
            Rectangle::new(start, Size::new(10, 20))
        );
        assert_eq!(
            backwards_line.bounding_box(),
            Rectangle::new(start, Size::new(10, 20))
        );
    }

    #[test]
    fn no_stroke_width_no_line() {
        let start = Point::new(2, 3);
        let end = Point::new(3, 2);

        let line =
            Line::new(start, end).into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 0));

        assert!(line.pixels().eq(core::iter::empty()));
    }

    #[test]
    fn thick_line_octant_1() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();

        Line::new(Point::new(2, 2), Point::new(20, 8))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 5))
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "   #                   ",
            "  #####                ",
            "  ########             ",
            "  ###########          ",
            " ###############       ",
            "    ###############    ",
            "       ############### ",
            "          ###########  ",
            "             ########  ",
            "                #####  ",
            "                   #   ",
        ]);
    }

    #[test]
    fn thick_line_2px() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();

        // Horizontal line
        Line::new(Point::new(2, 2), Point::new(10, 2))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2))
            .draw(&mut display)
            .unwrap();

        // Vertical line
        Line::new(Point::new(2, 5), Point::new(2, 10))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::Off, 2))
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "            ",
            "  ######### ",
            "  ######### ",
            "            ",
            "            ",
            "  ..        ",
            "  ..        ",
            "  ..        ",
            "  ..        ",
            "  ..        ",
            "  ..        ",
        ]);
    }

    // Check that 45 degree lines don't draw their right side 1px too long
    #[test]
    fn diagonal() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();

        Line::new(Point::new(3, 2), Point::new(10, 9))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 7))
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "     #        ",
            "    ###       ",
            "   #####      ",
            "  #######     ",
            " #########    ",
            "  #########   ",
            "   #########  ",
            "    ######### ",
            "     #######  ",
            "      #####   ",
            "       ###    ",
            "        #     ",
        ]);
    }

    #[test]
    fn thick_line_3px() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();

        // Horizontal line
        Line::new(Point::new(2, 2), Point::new(10, 2))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 3))
            .draw(&mut display)
            .unwrap();

        // Vertical line
        Line::new(Point::new(2, 5), Point::new(2, 10))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::Off, 3))
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "            ",
            "  ######### ",
            "  ######### ",
            "  ######### ",
            "            ",
            " ...        ",
            " ...        ",
            " ...        ",
            " ...        ",
            " ...        ",
            " ...        ",
        ]);
    }

    #[test]
    fn thick_line_0px() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();

        Line::new(Point::new(2, 2), Point::new(2, 2))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 3))
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "   ", //
            "  #", //
            "  #", //
            "  #", //
        ]);
    }

    #[test]
    fn event_width_offset() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();

        // Horizontal line
        Line::new(Point::new(2, 3), Point::new(10, 3))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 4))
            .draw(&mut display)
            .unwrap();

        // Vertical line
        Line::new(Point::new(2, 9), Point::new(10, 8))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 4))
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "            ",
            "  ######### ",
            "  ######### ",
            "  ######### ",
            "  ######### ",
            "            ",
            "       #### ",
            "  ######### ",
            "  ######### ",
            "  ######### ",
            "  #####     ",
        ]);
    }

    #[test]
    fn points_iter() {
        let line = Line::new(Point::new(10, 10), Point::new(20, 30));

        let styled_points: ArrayVec<[_; 32]> = line
            .clone()
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .pixels()
            .map(|Pixel(p, _)| p)
            .collect();

        let points: ArrayVec<[_; 32]> = line.points().collect();

        assert_eq!(points, styled_points);
    }

    #[test]
    fn perpendicular() {
        assert_eq!(
            Line::new(Point::zero(), Point::new(10, 0)).perpendicular(),
            Line::new(Point::zero(), Point::new(0, -10))
        );

        assert_eq!(
            Line::new(Point::new(10, 20), Point::new(20, 10)).perpendicular(),
            Line::new(Point::new(10, 20), Point::new(0, 10))
        );

        assert_eq!(
            Line::new(Point::zero(), Point::new(0, -10)).perpendicular(),
            Line::new(Point::zero(), Point::new(-10, 0))
        );
    }

    #[test]
    fn extents_zero_thickness() {
        let line = Line::new(Point::new(10, 20), Point::new(20, 10));

        let (l, r) = line.extents(0, StrokeOffset::None);

        assert_eq!(l, line);
        assert_eq!(r, line);
    }
}
