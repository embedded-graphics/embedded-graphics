//! The triangle primitive.

mod points;
mod scanline_intersections;
mod scanline_iterator;
mod styled;

use crate::{
    geometry::{Dimensions, Point},
    primitives::{
        common::{bresenham_scanline_intersection, LineJoin, LineSide, StrokeOffset},
        ContainsPoint, Line, Primitive, Rectangle,
    },
    transform::Transform,
};
use core::{
    borrow::Borrow,
    cmp::{max, min, Ordering},
};
pub use points::Points;
pub use styled::StyledPixels;

/// Triangle primitive
///
/// # Examples
///
/// ## Create some triangles with different styles
///
/// ```rust
/// use embedded_graphics::{
///     pixelcolor::Rgb565, prelude::*, primitives::Triangle, style::PrimitiveStyle,
/// };
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::default();
/// # display.set_allow_overdraw(true);
///
/// // Triangle with red 1 px wide stroke
/// Triangle::new(Point::new(40, 20), Point::new(50, 25), Point::new(60, 60))
///     .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 1))
///     .draw(&mut display)?;
///
/// // Triangle with translation applied
/// Triangle::new(Point::new(40, 20), Point::new(50, 25), Point::new(60, 60))
///     .translate(Point::new(-10, -20))
///     .into_styled(PrimitiveStyle::with_stroke(Rgb565::GREEN, 1))
///     .draw(&mut display)?;
/// # Ok::<(), core::convert::Infallible>(())
/// ```
///
/// ## Create a triangle from an array of points
///
/// ```rust
/// use embedded_graphics::{geometry::Point, primitives::Triangle};
///
/// let p1 = Point::new(5, 10);
/// let p2 = Point::new(15, 25);
/// let p3 = Point::new(5, 25);
///
/// // Owned
/// let tri = Triangle::from_points([p1, p2, p3]);
///
/// // Or borrowed
/// let tri_ref = Triangle::from_points(&[p1, p2, p3]);
/// #
/// # assert_eq!(tri, Triangle::new(p1, p2, p3));
/// # assert_eq!(tri_ref, Triangle::new(p1, p2, p3));
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Triangle {
    /// First point of the triangle
    pub p1: Point,

    /// Second point of the triangle
    pub p2: Point,

    /// Third point of the triangle
    pub p3: Point,
}

impl Primitive for Triangle {
    type PointsIter = Points;

    fn points(&self) -> Self::PointsIter {
        Points::new(self)
    }
}

impl ContainsPoint for Triangle {
    fn contains(&self, point: Point) -> bool {
        // Skip expensive calculations below if point is outside the bounding box
        if !self.bounding_box().contains(point) {
            return false;
        }

        let p = point;
        let Self { p1, p2, p3 } = *self;

        // Check if point is inside triangle using https://stackoverflow.com/a/20861130/383609.
        // Works for any point ordering.
        let is_inside = {
            let s = p1.y * p3.x - p1.x * p3.y + (p3.y - p1.y) * p.x + (p1.x - p3.x) * p.y;
            let t = p1.x * p2.y - p1.y * p2.x + (p1.y - p2.y) * p.x + (p2.x - p1.x) * p.y;

            if (s < 0) != (t < 0) {
                false
            } else {
                // Determinant
                let a = self.area_doubled();

                // If determinant is zero, triangle is colinear and can never contain a point.
                if a == 0 {
                    return false;
                }

                // This check allows this algorithm to work with clockwise or counterclockwise
                // triangles.
                if a < 0 {
                    s <= 0 && s + t >= a
                } else {
                    s >= 0 && s + t <= a
                }
            }
        };

        // Skip expensive point-on-line check below if point is definitely inside triangle
        if is_inside {
            return true;
        }

        // Sort points into same order as `ScanlineIterator` so this check produces the same results
        // as a rendered triangle would.
        let Triangle { p1, p2, p3 } = self.sorted_yx();

        // Special case: due to the Bresenham algorithm being used to render triangles, some pixel
        // centers on a Styled<Triangle> lie outside the mathematical triangle. This check
        // inefficiently checks to see if the point lies on any of the border edges.
        Line::new(p1, p2)
            .points()
            .chain(Line::new(p1, p3).points())
            .chain(Line::new(p2, p3).points())
            .any(|line_point| line_point == p)
    }
}

impl Dimensions for Triangle {
    fn bounding_box(&self) -> Rectangle {
        let x_min = min(min(self.p1.x, self.p2.x), self.p3.x);
        let y_min = min(min(self.p1.y, self.p2.y), self.p3.y);

        let x_max = max(max(self.p1.x, self.p2.x), self.p3.x);
        let y_max = max(max(self.p1.y, self.p2.y), self.p3.y);

        Rectangle::with_corners(Point::new(x_min, y_min), Point::new(x_max, y_max))
    }
}

impl Triangle {
    /// Create a new triangle with a given style
    pub const fn new(p1: Point, p2: Point, p3: Point) -> Self {
        Triangle { p1, p2, p3 }
    }

    /// Creates a new triangle from an array of points.
    ///
    /// This supports both [`Point`]s, as well as anything that implements `Into<Point>` like
    /// `(i32, i32)`.
    ///
    /// [`Point`]: ../../geometry/struct.Point.html
    pub fn from_points<P, I>(points: P) -> Self
    where
        I: Into<Point> + Copy,
        P: Borrow<[I; 3]>,
    {
        let points = points.borrow();

        Triangle {
            p1: points[0].into(),
            p2: points[1].into(),
            p3: points[2].into(),
        }
    }

    /// Return the area of the triangle, doubled.
    ///
    /// This method can be used to determine if the triangle is colinear by checking if the returned
    /// value is equal to zero.
    pub(in crate::primitives) fn area_doubled(&self) -> i32 {
        let Self { p1, p2, p3 } = self;

        -p2.y * p3.x + p1.y * (p3.x - p2.x) + p1.x * (p2.y - p3.y) + p2.x * p3.y
    }

    /// Create a new triangle with points sorted in a clockwise direction.
    pub(in crate::primitives::triangle) fn sorted_clockwise(&self) -> Self {
        let Self { p1, p2, p3 } = self;

        let determinant = -p2.y * p3.x + p1.y * (p3.x - p2.x) + p1.x * (p2.y - p3.y) + p2.x * p3.y;

        match determinant.cmp(&0) {
            // Triangle is wound CCW. Swap two points to make it CW.
            Ordering::Less => Self::new(*p2, *p1, *p3),
            // Triangle is already CW, do nothing.
            Ordering::Greater => *self,
            // Triangle is colinear. Sort points so they lie sequentially along the line.
            Ordering::Equal => {
                let (p1, p2, p3) = sort_yx(*p1, *p2, *p3);

                Self::new(p1, p2, p3)
            }
        }
    }

    fn sorted_yx(&self) -> Self {
        let (p1, p2, p3) = sort_yx(self.p1, self.p2, self.p3);

        Self::new(p1, p2, p3)
    }

    pub(in crate::primitives::triangle) fn scanline_intersection(
        &self,
        scanline_y: i32,
    ) -> Option<Line> {
        let Triangle { p1, p2, p3 } = self.sorted_yx();

        // Triangle is colinear. We can get away with only intersecting the single line.
        if self.area_doubled() == 0 {
            return bresenham_scanline_intersection(&Line::new(p1, p3), scanline_y);
        }

        let line_a = Line::new(p1, p2);
        let line_b = Line::new(p1, p3);
        let line_c = Line::new(p2, p3);

        let first = bresenham_scanline_intersection(&line_b, scanline_y)?;
        let second = bresenham_scanline_intersection(&line_a, scanline_y)
            .or_else(|| bresenham_scanline_intersection(&line_c, scanline_y))?;

        Some(Line::new(
            first.start.component_min(second.start),
            first.end.component_max(second.end),
        ))
    }

    /// Generate a line join for each corner of the triangle.
    fn joins(&self, stroke_width: u32, stroke_offset: StrokeOffset) -> [LineJoin; 3] {
        [
            LineJoin::from_points(self.p3, self.p1, self.p2, stroke_width, stroke_offset),
            LineJoin::from_points(self.p1, self.p2, self.p3, stroke_width, stroke_offset),
            LineJoin::from_points(self.p2, self.p3, self.p1, stroke_width, stroke_offset),
        ]
    }

    /// Compute whether a triangle with thick stroke has a hole in its center or is completely
    /// filled by stroke.
    // PERF: This doesn't need to compute the entire join, much like how `thick_stroke_inset`
    // doesn't
    pub(in crate::primitives::triangle) fn is_collapsed(
        &self,
        stroke_width: u32,
        stroke_offset: StrokeOffset,
    ) -> bool {
        let joins = self.joins(stroke_width, stroke_offset);

        joins.iter().enumerate().any(|(i, join)| {
            // Quick check: if the join is degenerate, no hole can occur.
            if join.is_degenerate() {
                return true;
            }

            // Compute inner-most points of each join. The triangle is sorted clockwise, so that's
            // the right-side point. The `first_edge_end` and `second_edge_start` points are always
            // the same in this case, as this is the "pinched" side of the join, so we'll
            // arbitrarily pick `first_edge_end`.
            let inner_point = join.first_edge_end.right;

            // Find opposite edge to the given point.
            let opposite = {
                let start = self.vertex(i + 1);
                let end = self.vertex(i + 2);

                // Get right side extent (triangle is sorted clockwise, remember)
                Line::new(start, end).extents(stroke_width, stroke_offset).1
            };

            // If the inner point is to the left of the opposite side line, the triangle edges self-
            // intersect, so the triangle is collapsed.
            opposite.check_side(inner_point, LineSide::Left)
        })
    }

    /// Get a vertex at a given index.
    ///
    /// The given index will always wrap in the range 0..=2.
    pub(in crate::primitives::triangle) fn vertex(&self, idx: usize) -> Point {
        let idx = idx % 3;

        match idx {
            0 => self.p1,
            1 => self.p2,
            2 => self.p3,
            _ => unreachable!(),
        }
    }
}

impl Transform for Triangle {
    /// Translate the triangle from its current position to a new position by (x, y) pixels,
    /// returning a new `Triangle`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Triangle;
    /// # use embedded_graphics::prelude::*;
    /// let tri = Triangle::new(Point::new(5, 10), Point::new(15, 20), Point::new(8, 15));
    /// let moved = tri.translate(Point::new(10, 10));
    ///
    /// assert_eq!(moved.p1, Point::new(15, 20));
    /// assert_eq!(moved.p2, Point::new(25, 30));
    /// assert_eq!(moved.p3, Point::new(18, 25));
    /// ```
    fn translate(&self, by: Point) -> Self {
        Self {
            p1: self.p1 + by,
            p2: self.p2 + by,
            p3: self.p3 + by,
        }
    }

    /// Translate the triangle from its current position to a new position by (x, y) pixels.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Triangle;
    /// # use embedded_graphics::prelude::*;
    /// let mut tri = Triangle::new(Point::new(5, 10), Point::new(15, 20), Point::new(10, 15));
    /// tri.translate_mut(Point::new(10, 10));
    ///
    /// assert_eq!(tri.p1, Point::new(15, 20));
    /// assert_eq!(tri.p2, Point::new(25, 30));
    /// assert_eq!(tri.p3, Point::new(20, 25));
    /// ```
    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.p1 += by;
        self.p2 += by;
        self.p3 += by;

        self
    }
}

fn sort_two_yx(p1: Point, p2: Point) -> (Point, Point) {
    // If p1.y is less than p2.y, return it first. Otherwise, if they have the same Y coordinate,
    // the first point becomes the one with the lesser X coordinate.
    if p1.y < p2.y || (p1.y == p2.y && p1.x < p2.x) {
        (p1, p2)
    } else {
        (p2, p1)
    }
}

/// Sort 3 points in order of increasing Y value. If two points have the same Y value, the one with
/// the lesser X value is put before.
fn sort_yx(p1: Point, p2: Point, p3: Point) -> (Point, Point, Point) {
    let (y1, y2) = sort_two_yx(p1, p2);
    let (y1, y3) = sort_two_yx(p3, y1);
    let (y2, y3) = sort_two_yx(y3, y2);

    (y1, y2, y3)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Size;

    #[test]
    fn dimensions() {
        let tri = Triangle::new(Point::new(5, 10), Point::new(15, 25), Point::new(5, 25));
        let moved = tri.translate(Point::new(-10, -11));

        assert_eq!(tri.p1, Point::new(5, 10));
        assert_eq!(tri.p2, Point::new(15, 25));
        assert_eq!(tri.p3, Point::new(5, 25));
        assert_eq!(tri.bounding_box().size, Size::new(11, 16));

        assert_eq!(moved.p1, Point::new(-5, -1));
        assert_eq!(moved.p2, Point::new(5, 14));
        assert_eq!(moved.p3, Point::new(-5, 14));
        assert_eq!(moved.bounding_box().size, Size::new(11, 16));
    }

    #[test]
    fn it_can_be_translated() {
        let tri = Triangle::new(Point::new(5, 10), Point::new(15, 20), Point::new(10, 15));
        let moved = tri.translate(Point::new(5, 10));

        assert_eq!(
            moved,
            Triangle::new(Point::new(10, 20), Point::new(20, 30), Point::new(15, 25))
        );
    }

    #[test]
    fn contains() {
        let triangles = [
            Triangle::new(Point::new(0, 0), Point::new(64, 10), Point::new(15, 64)),
            Triangle::new(Point::new(5, 0), Point::new(30, 64), Point::new(64, 0)),
            Triangle::new(Point::new(0, 0), Point::new(0, 64), Point::new(64, 30)),
            Triangle::new(Point::new(22, 0), Point::new(0, 64), Point::new(64, 64)),
            Triangle::new(Point::new(0, 22), Point::new(64, 0), Point::new(64, 64)),
        ];

        for triangle in triangles.iter() {
            for point in Rectangle::new(Point::new(-5, -5), Size::new(70, 70)).points() {
                let expected = triangle.points().any(|p| p == point);

                assert_eq!(
                    triangle.contains(point),
                    expected,
                    "{:?}, {:?}",
                    point,
                    triangle
                );
            }
        }
    }

    #[test]
    fn colinear_never_contains() {
        let triangles = [
            Triangle::new(Point::new(5, 10), Point::new(15, 20), Point::new(10, 15)),
            Triangle::new(Point::new(2, 2), Point::new(2, 4), Point::new(2, 4)),
            Triangle::new(Point::new(2, 2), Point::new(4, 2), Point::new(4, 2)),
        ];

        for triangle in triangles.iter() {
            for point in Rectangle::new(Point::new(-5, -5), Size::new(70, 70)).points() {
                assert_eq!(triangle.contains(point), false);
            }
        }
    }
}
