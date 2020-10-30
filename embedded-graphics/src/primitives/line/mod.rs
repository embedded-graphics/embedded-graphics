//! The line primitive

mod bresenham;
mod points;
mod styled;
mod thick_points;

use crate::{
    geometry::{Dimensions, Point},
    primitives::{
        line::thick_points::{ParallelLineType, ParallelsIterator},
        ContainsPoint, Primitive, Rectangle,
    },
    style::StrokeAlignment,
    transform::Transform,
    SaturatingCast,
};
pub use points::Points;
pub use styled::StyledPixels;
pub(in crate::primitives) use thick_points::{Side, ThickPoints};

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

impl Primitive for Line {
    type PointsIter = Points;

    fn points(&self) -> Self::PointsIter {
        Points::new(self)
    }
}

impl Dimensions for Line {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::with_corners(self.start, self.end)
    }
}

/// Check signs of two signed numbers
///
/// Fastest ASM output compared to other methods. See: https://godbolt.org/z/zVx9cD
fn same_signs(a: i32, b: i32) -> bool {
    (a >= 0) == (b >= 0)
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
        outer_side: Side,
    },

    /// No intersection: lines are colinear or parallel.
    Colinear,
}

/// Bresenham intersection result.
#[derive(Debug, Copy, Clone)]
pub(in crate::primitives) enum BresenhamIntersection {
    /// Point
    Point(Point),

    /// Multiple intersection points.
    ///
    /// This may be caused by e.g. a shallow line where multiple points lie on the same Y
    /// coordinate.
    Line(Line),
}

impl BresenhamIntersection {
    /// As a line
    pub fn into_line(self) -> Line {
        match self {
            Self::Line(l) => l,
            Self::Point(p) => Line::new(p, p),
        }
        .sorted_x()
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
    /// Outside stroke alignment is on the left side of the line, making this compatible with
    /// clockwise triangles, polygons, etc.
    pub(in crate::primitives) fn extents(
        &self,
        thickness: u32,
        alignment: StrokeAlignment,
    ) -> (Line, Line) {
        let mut it = ParallelsIterator::new(self, thickness.saturating_cast(), alignment);
        let reduce =
            it.parallel_parameters.position_step.major + it.parallel_parameters.position_step.minor;

        let mut left = (self.start, ParallelLineType::Normal);
        let mut right = (self.start, ParallelLineType::Normal);

        match alignment {
            StrokeAlignment::Center => loop {
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
            // Left side
            StrokeAlignment::Outside => {
                if let Some((bresenham, reduce)) = it.last() {
                    left = (bresenham.point, reduce);
                }
            }
            // Right side
            StrokeAlignment::Inside => {
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

    /// Sort line so start point is to the left of the end point.
    fn sorted_x(&self) -> Self {
        let (start, end) = if self.start.x > self.end.x {
            (self.end, self.start)
        } else {
            (self.start, self.end)
        };

        Self::new(start, end)
    }

    /// Compute the midpoint of the line.
    pub fn midpoint(&self) -> Point {
        self.start + (self.end - self.start) / 2
    }

    const fn coefficients(&self, other: &Self) -> (i32, i32, i32, i32, i32, i32, i32) {
        let Point { x: x1, y: y1 } = self.start;
        let Point { x: x2, y: y2 } = self.end;
        let Point { x: x3, y: y3 } = other.start;
        let Point { x: x4, y: y4 } = other.end;

        // First line coefficients where "a1 x  +  b1 y  +  c1  =  0"
        let a1 = y2 - y1;
        let b1 = x1 - x2;
        let c1 = x2 * y1 - x1 * y2;

        // Second line coefficients
        let a2 = y4 - y3;
        let b2 = x3 - x4;
        let c2 = x4 * y3 - x3 * y4;

        let denom = a1 * b2 - a2 * b1;

        (a1, b1, c1, a2, b2, c2, denom)
    }

    /// Check if two line segments intersect.
    pub(in crate::primitives) fn segment_intersection(&self, other: &Self) -> bool {
        // Note: a bounding box check here causes render time regression for thick polylines
        let (a1, b1, c1, a2, b2, c2, denom) = self.coefficients(other);

        // Lines are colinear or parallel
        if denom == 0 {
            return false;
        }

        let Point { x: x1, y: y1 } = self.start;
        let Point { x: x2, y: y2 } = self.end;
        let Point { x: x3, y: y3 } = other.start;
        let Point { x: x4, y: y4 } = other.end;

        // Sign values for first line
        let r1 = a2 * x1 + b2 * y1 + c2;
        let r2 = a2 * x2 + b2 * y2 + c2;

        // Sign values for second line
        let r3 = a1 * x3 + b1 * y3 + c1;
        let r4 = a1 * x4 + b1 * y4 + c1;

        // If r1 is 0, the intersection is at the beginning of the first line
        // If r2 is 0, the intersection is at the end of the first line
        // If r3 is 0, the intersection is at the beginning of the second line
        // If r4 is 0, the intersection is at the end of the second line

        // Flag denoting whether intersection point is on given line segments. If this is false,
        // the intersection occurs somewhere along the two mathematical, infinite lines instead.
        //
        // Check signs of r3 and r4.  If both point 3 and point 4 lie on same side of line 1, the
        // line segments do not intersect.
        //
        // Check signs of r1 and r2.  If both point 1 and point 2 lie on same side of second line
        // segment, the line segments do not intersect.
        (r3 == 0 || r4 == 0 || !same_signs(r3, r4)) && (r1 == 0 || r2 == 0 || !same_signs(r1, r2))
    }

    /// Integer-only line intersection
    ///
    /// Inspired from https://stackoverflow.com/a/61485959/383609, which links to
    /// https://webdocs.cs.ualberta.ca/~graphics/books/GraphicsGems/gemsii/xlines.c
    pub(in crate::primitives) fn line_intersection(&self, other: &Self) -> Intersection {
        let (a1, b1, c1, a2, b2, c2, denom) = self.coefficients(other);

        // Lines are colinear or parallel
        if denom == 0 {
            return Intersection::Colinear;
        }

        // If we got here, line segments intersect. Compute intersection point using method similar
        // to that described here: http://paulbourke.net/geometry/pointlineplane/#i2l

        // The denom/2 is to get rounding instead of truncating.
        let offset = denom.abs() / 2;

        let num = b1 * c2 - b2 * c1;
        let x = if num < 0 { num - offset } else { num + offset } / denom;

        let num = a2 * c1 - a1 * c2;
        let y = if num < 0 { num - offset } else { num + offset } / denom;

        Intersection::Point {
            point: Point::new(x, y),
            outer_side: if denom > 0 { Side::Right } else { Side::Left },
        }
    }

    /// Intersect a horizontal scan line with the Bresenham representation of this line segment.
    pub(in crate::primitives) fn bresenham_scanline_intersection(
        &self,
        scan_y: i32,
    ) -> Option<BresenhamIntersection> {
        if !self
            .bounding_box()
            .contains(Point::new(self.start.x, scan_y))
        {
            return None;
        }

        let mut points = self.points().filter(|p| p.y == scan_y);

        let first = points.next()?;

        let intersection = if let Some(last) = points.last() {
            if last != first {
                BresenhamIntersection::Line(Line::new(first, last))
            } else {
                BresenhamIntersection::Point(first)
            }
        } else {
            BresenhamIntersection::Point(first)
        };

        Some(intersection)
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
        drawable::{Drawable, Pixel},
        geometry::Size,
        iterator::IntoPixels,
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        style::PrimitiveStyle,
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

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
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
            ])
        );
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

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
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
            ])
        );
    }

    // Check that 45 degree lines don't draw their right side 1px too long
    #[test]
    fn diagonal() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();

        Line::new(Point::new(3, 2), Point::new(10, 9))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 7))
            .draw(&mut display)
            .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
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
            ])
        );
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

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
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
            ])
        );
    }

    #[test]
    fn thick_line_0px() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();

        Line::new(Point::new(2, 2), Point::new(2, 2))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 3))
            .draw(&mut display)
            .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "   ", //
                "  #", //
                "  #", //
                "  #", //
            ])
        );
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

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
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
            ])
        );
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

        let (l, r) = line.extents(0, StrokeAlignment::Center);

        assert_eq!(l, line);
        assert_eq!(r, line);
    }
}
