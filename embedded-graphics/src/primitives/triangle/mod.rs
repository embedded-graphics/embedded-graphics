//! The triangle primitive.

mod fill_scanline_iterator;
mod mathematical_points;
mod points;
mod scanline_iterator;
mod styled;
mod thick_points;
mod triangle_iterator;

use crate::{
    geometry::{Dimensions, Point},
    primitives::{ContainsPoint, Line, Primitive, Rectangle},
    transform::Transform,
};
use core::cmp::Ordering;
use core::{
    borrow::Borrow,
    cmp::{max, min},
};
pub use fill_scanline_iterator::FillScanlineIterator;
pub use mathematical_points::MathematicalPoints;
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

fn is_inside(a: i32, b: i32) -> bool {
    (a > 0) == (b > 0)
}

fn point_on_line(point: Point, p1: Point, p2: Point) -> i32 {
    (p2.x - p1.x) * (point.y - p1.y) - (p2.y - p1.y) * (point.x - p1.x)
}

impl ContainsPoint for Triangle {
    fn contains(&self, point: Point) -> bool {
        // Skip expensive calculations below if point is outside the bounding box
        if !self.bounding_box().contains(point) {
            return false;
        }

        let Self { p1, p2, p3 } = *self;

        // Sort points into same order as `ScanlineIterator` so this check produces the same results
        // as a rendered triangle would.
        let (p1, p2, p3) = sort_yx(p1, p2, p3);
        let cw = Triangle::new(p1, p2, p3).area_doubled();

        let edge1 = point_on_line(point, p1, p2);
        if !is_inside(edge1, cw) {
            if Line::new(p1, p2)
                .points()
                .any(|line_point| line_point == point)
            {
                return true;
            }
        }

        let edge2 = point_on_line(point, p2, p3);
        if !is_inside(edge2, cw) {
            if Line::new(p2, p3)
                .points()
                .any(|line_point| line_point == point)
            {
                return true;
            }
        }

        let edge3 = point_on_line(point, p3, p1);
        if !is_inside(edge3, cw) {
            if Line::new(p1, p3)
                .points()
                .any(|line_point| line_point == point)
            {
                return true;
            }
        }

        is_inside(edge1, cw) && is_inside(edge2, cw) && is_inside(edge3, cw)
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
    /// If the triangle's winding order is counter-clockwise, this method will return a negative
    /// value.
    ///
    /// This method can be used to determine if the triangle is colinear by checking if the returned
    /// value is equal to zero.
    pub fn area_doubled(&self) -> i32 {
        let Self { p1, p2, p3 } = self;

        -p2.y * p3.x + p1.y * (p3.x - p2.x) + p1.x * (p2.y - p3.y) + p2.x * p3.y
    }

    /// Create a new triangle with points sorted in a clockwise direction
    pub fn sorted_clockwise(&self) -> Self {
        match self.area_doubled().cmp(&0) {
            // Triangle is wound CCW. Swap two points to make it CW.
            Ordering::Less => Self::new(self.p2, self.p1, self.p3),
            // Triangle is already CW, do nothing.
            Ordering::Greater => *self,
            // Triangle is colinear. Sort points so they lie sequentially along the line.
            Ordering::Equal => {
                let (p1, p2, p3) = sort_yx(self.p1, self.p2, self.p3);

                Self::new(p1, p2, p3)
            }
        }
    }

    /// Find the center of gravity/centroid of the triangle
    pub fn centroid(&self) -> Point {
        (self.p1 + self.p2 + self.p3) / 3
    }

    /// Point inside triangle, ignoring pixels that partially lie outside triangle lines.
    pub(self) fn mathematical_contains(&self, point: &Point) -> bool {
        // Skip expensive calculations if point is outside the bounding box
        self.bounding_box().contains(*point) && self.in_mathematical_triangle(point)
    }

    fn in_mathematical_triangle(&self, point: &Point) -> bool {
        let Self { p1, p2, p3 } = self;
        let p = point;

        // Method as described in https://stackoverflow.com/a/9755252, but with `>` changed to `>=`
        // to match points that lie _on_ the edge.
        let as_x = p.x - p1.x;
        let as_y = p.y - p1.y;

        let s_ab = (p2.x - p1.x) * as_y - (p2.y - p1.y) * as_x >= 0;

        if ((p3.x - p1.x) * as_y - (p3.y - p1.y) * as_x >= 0) == s_ab {
            return false;
        }

        if ((p3.x - p2.x) * (p.y - p2.y) - (p3.y - p2.y) * (p.x - p2.x) >= 0) != s_ab {
            return false;
        }

        true
    }

    /// Maths points yeahahhhhh
    pub fn mathematical_points(&self) -> MathematicalPoints {
        MathematicalPoints::new(self)
    }

    /// Empty triangle
    pub(in crate::primitives) const fn empty() -> Self {
        Self::new(Point::zero(), Point::zero(), Point::zero())
    }
}

// https://stackoverflow.com/a/6989383/383609
// NOTE: This is unused, but kept around as it took a while to find, and may be useful for polygon
// calculations.
/// Used by sorting functions to sort points in clockwise order.
#[allow(unused)]
pub fn sort_clockwise(a: &Point, b: &Point, center: Point) -> Ordering {
    if a.x - center.x >= 0 && b.x - center.x < 0 {
        return Ordering::Greater;
    }
    if a.x - center.x < 0 && b.x - center.x >= 0 {
        return Ordering::Less;
    }
    if a.x - center.x == 0 && b.x - center.x == 0 {
        if a.y - center.y >= 0 || b.y - center.y >= 0 {
            return a.y.cmp(&b.y);
        }
        return b.y.cmp(&a.y);
    }

    // Compute the cross product of vectors (center -> a) x (center -> b)
    let det = (a.x - center.x) * (b.y - center.y) - (b.x - center.x) * (a.y - center.y);

    match det.cmp(&0) {
        Ordering::Less => Ordering::Greater,
        Ordering::Greater => Ordering::Less,
        Ordering::Equal => {
            // Points a and b are on the same line from the center. Check which point is closer to
            // the center.
            let d1 = (a.x - center.x) * (a.x - center.x) + (a.y - center.y) * (a.y - center.y);
            let d2 = (b.x - center.x) * (b.x - center.x) + (b.y - center.y) * (b.y - center.y);

            d1.cmp(&d2)
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

fn sort_two_x(p1: Point, p2: Point) -> (Point, Point) {
    if p1.x < p2.x {
        (p1, p2)
    } else {
        (p2, p1)
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

enum IterState {
    Border(Point),
    LeftRight(Point, Point),
    None,
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
    fn triangle_contains_edge_point_regression() {
        // This test is a regression test case found while optimizing Triangle::contains()
        assert!(
            Triangle::new(Point::new(30, 30), Point::new(0, 0), Point::new(32, 33))
                .contains(Point::new(31, 31))
        );
    }

    // FIXME: Colinear triangles are rendered as a line, so this should also return true. Why not?
    // #[test]
    // fn colinear_never_contains() {
    //     let triangles = [
    //         Triangle::new(Point::new(5, 10), Point::new(15, 20), Point::new(10, 15)),
    //         Triangle::new(Point::new(2, 2), Point::new(2, 4), Point::new(2, 4)),
    //         Triangle::new(Point::new(2, 2), Point::new(4, 2), Point::new(4, 2)),
    //     ];

    //     for (idx, triangle) in triangles.iter().enumerate() {
    //         for point in Rectangle::new(Point::new(-5, -5), Size::new(25, 25)).points() {
    //             assert_eq!(
    //                 triangle.contains(point),
    //                 false,
    //                 "Triangle #{}, point {:?}",
    //                 idx,
    //                 point
    //             );
    //         }
    //     }
    // }
}
