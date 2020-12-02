//! The line primitive

mod bresenham;
mod points;
mod styled;
mod thick_points;

use crate::{
    geometry::{Dimensions, Point, PointExt},
    primitives::{
        common::{LineSide, LinearEquation, StrokeOffset},
        line::thick_points::{ParallelLineType, ParallelsIterator},
        PointsIter, Primitive, Rectangle,
    },
    transform::Transform,
    SaturatingCast,
};
pub use points::Points;
pub use styled::StyledPixels;

/// Line primitive
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

/// Intersection test result.
#[derive(Copy, Clone, Debug)]
pub enum Intersection {
    /// Intersection at point
    Point {
        /// Intersection point.
        point: Point,

        /// The "outer" side of the intersection, i.e. the side that has the joint's reflex angle.
        ///
        /// For example:
        ///
        /// ```text
        /// # Left outer side:
        ///
        ///  ⎯
        /// ╱
        ///
        /// # Right outer side:
        ///  │
        /// ╱
        /// ```
        ///
        /// This is used to find the outside edge of a corner.
        outer_side: LineSide,
    },

    /// No intersection: lines are colinear or parallel.
    Colinear,
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
    pub fn extents(&self, thickness: u32, stroke_offset: StrokeOffset) -> (Line, Line) {
        let mut it = ParallelsIterator::new(self, thickness.saturating_cast(), stroke_offset);
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

    /// Integer-only line intersection
    ///
    /// Inspired from https://stackoverflow.com/a/61485959/383609, which links to
    /// https://webdocs.cs.ualberta.ca/~graphics/books/GraphicsGems/gemsii/xlines.c
    pub fn intersection(&self, other: &Line) -> Intersection {
        let line1 = LinearEquation::from_line(self);
        let line2 = LinearEquation::from_line(other);

        // Calculate the determinant to solve the system of linear equations using Cramer's rule.
        let denominator = line1.normal_vector.determinant(line2.normal_vector);

        // The system of linear equations has no solutions if the determinant is zero. In this case,
        // the lines must be colinear.
        if denominator == 0 {
            return Intersection::Colinear;
        }

        // If we got here, line segments intersect. Compute intersection point using method similar
        // to that described here: http://paulbourke.net/geometry/pointlineplane/#i2l

        // The denominator/2 is to get rounding instead of truncating.
        let offset = denominator.abs() / 2;

        let origin_distances = Point::new(line1.origin_distance, line2.origin_distance);

        let numerator =
            origin_distances.determinant(Point::new(line1.normal_vector.y, line2.normal_vector.y));
        let x_numerator = if numerator < 0 {
            numerator - offset
        } else {
            numerator + offset
        };

        let numerator =
            Point::new(line1.normal_vector.x, line2.normal_vector.x).determinant(origin_distances);
        let y_numerator = if numerator < 0 {
            numerator - offset
        } else {
            numerator + offset
        };

        Intersection::Point {
            point: Point::new(x_numerator, y_numerator) / denominator,
            outer_side: if denominator > 0 {
                LineSide::Right
            } else {
                LineSide::Left
            },
        }
    }

    pub fn new_intersection(l1: &Line, l2: &Line) -> (i32, i32) {
        let Line {
            start: Point { x: x1, y: y1 },
            end: Point { x: x2, y: y2 },
        } = *l1;
        let Line {
            start: Point { x: x3, y: y3 },
            end: Point { x: x4, y: y4 },
        } = *l2;

        let denom = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);

        let ua_top = (x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3);
        let ub_top = (x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3);

        // dbg!(ua_top, ub_top, denom);

        // if denom == 0 {
        //     return;
        // }

        // let ua = ua_top / denom;
        // let ub = ub_top / denom;

        // let x = x1 + ua * (x2 - x1);
        // let y = y1 + ua * (y2 - y1);

        // dbg!(x, y);

        // dbg!(ua, ub);

        (ua_top, ub_top)
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
        geometry::Size, iterator::IntoPixels, mock_display::MockDisplay, pixelcolor::BinaryColor,
        style::PrimitiveStyle, Drawable, Pixel,
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

        assert!(line.into_pixels().eq(core::iter::empty()));
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
            .into_pixels()
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
