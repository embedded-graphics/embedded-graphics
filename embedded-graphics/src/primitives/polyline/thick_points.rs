use crate::primitives::polyline::triangle_iterator::TriangleIteratorState;
use crate::{
    prelude::Point,
    primitives::{
        line::Side,
        line_joint::JointKind,
        polyline::triangle_iterator::TriangleIterator,
        triangle::{Points, Triangle},
        ContainsPoint, Primitive,
    },
    style::StrokeAlignment,
};

/// Remembers the previous 5 points to avoid overdraw.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct LookbackBuffer {
    points: [Point; 5],
    count: u8,
}

impl LookbackBuffer {
    /// Creates a new buffer.
    pub const fn new() -> Self {
        Self {
            points: [Point::zero(); 5],
            count: 0,
        }
    }

    /// Shifts a point to the front of the buffer.
    pub fn add(&mut self, p: Point) {
        self.points[4] = self.points[3];
        self.points[3] = self.points[2];
        self.points[2] = self.points[1];
        self.points[1] = self.points[0];
        self.points[0] = p;
        if self.count < 5 {
            self.count += 1;
        }
    }

    /// Returns whether the given point is inside the newest tracked triangle.
    pub fn prev1_contains(&self, p: Point) -> bool {
        self.count >= 3 && Triangle::new(self.points[0], self.points[1], self.points[2]).contains(p)
    }

    /// Returns whether the given point is inside the second tracked triangle.
    pub fn prev2_contains(&self, p: Point) -> bool {
        self.count >= 4 && Triangle::new(self.points[1], self.points[2], self.points[3]).contains(p)
    }

    /// Returns whether the given point is inside the oldest tracked.
    pub fn prev3_contains(&self, p: Point) -> bool {
        self.count == 5 && Triangle::new(self.points[2], self.points[3], self.points[4]).contains(p)
    }
}

// TODO: Generalise name, move into more common folder path
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(in crate::primitives) struct ThickPoints<'a> {
    prev_points: LookbackBuffer,
    triangle_iter: TriangleIterator<'a>,
    new_point: Point,
    points_iter: Points,
}

impl<'a> ThickPoints<'a> {
    pub fn new(points: &'a [Point], width: u32, alignment: StrokeAlignment) -> Self {
        let mut triangle_iter = TriangleIterator::new(points, width, alignment);

        let triangle = triangle_iter.next().unwrap_or_else(Triangle::empty);
        let points_iter = triangle.points();

        let mut prev_points = LookbackBuffer::new();
        prev_points.add(triangle.p1);
        prev_points.add(triangle.p2);

        Self {
            prev_points,
            new_point: triangle.p3,
            triangle_iter,
            points_iter,
        }
    }

    pub fn empty() -> Self {
        Self {
            prev_points: LookbackBuffer::new(),
            new_point: Point::zero(),
            triangle_iter: TriangleIterator::empty(),
            points_iter: Triangle::empty().points(),
        }
    }
}

impl<'a> Iterator for ThickPoints<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(point) = self.points_iter.next() {
                // We need to check previous triangles so we don't overdraw them
                if !self.prev_points.prev1_contains(point) {
                    // Not every previous triangle must be checked
                    let return_point = match self.triangle_iter.joint_kind() {
                        JointKind::Bevel(Side::Left) => match self.triangle_iter.state {
                            TriangleIteratorState::Second => {
                                !self.prev_points.prev2_contains(point)
                            }
                            TriangleIteratorState::Filler => {
                                !self.prev_points.prev3_contains(point)
                            }
                            _ => true,
                        },
                        JointKind::Bevel(Side::Right) => match self.triangle_iter.state {
                            TriangleIteratorState::Second => {
                                !self.prev_points.prev3_contains(point)
                            }
                            _ => true,
                        },
                        JointKind::Miter(true) => !self.prev_points.prev2_contains(point),
                        _ => true,
                    };

                    if return_point {
                        return Some(point);
                    }
                }
            } else {
                self.prev_points.add(self.new_point);
                let triangle = self.triangle_iter.next()?;
                self.points_iter = triangle.points();
                self.new_point = triangle.p3;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        drawable::Drawable, mock_display::MockDisplay, pixelcolor::BinaryColor,
        primitives::Polyline, style::PrimitiveStyle,
    };

    #[test]
    fn test_no_overwrite_on_sharp_corner() {
        let mut mock_display = MockDisplay::new();

        Polyline::new(&[Point::new(35, 5), Point::new(25, 35), Point::new(15, 5)])
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 10))
            .draw(&mut mock_display)
            .unwrap();

        let mut mock_display = MockDisplay::new();

        Polyline::new(&[Point::new(15, 5), Point::new(25, 35), Point::new(35, 5)])
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 10))
            .draw(&mut mock_display)
            .unwrap();
    }
}
