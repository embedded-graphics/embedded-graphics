use crate::{
    prelude::Point,
    primitives::{
        line::Side,
        line_joint::JointKind,
        polyline::joint_iterator::{JointTriangleIterator, State},
        triangle::{FillScanlineIterator, Triangle},
        ContainsPoint,
    },
    style::StrokeAlignment,
};

/// Remembers the previous 6 points to avoid overdraw.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct LookbackBuffer {
    points: [Point; 6],
}

impl LookbackBuffer {
    /// Creates a new buffer.
    pub const fn new() -> Self {
        Self {
            points: [Point::zero(); 6],
        }
    }

    /// Shifts a point to the front of the buffer.
    pub fn add(&mut self, p: Point) {
        self.points[5] = self.points[4];
        self.points[4] = self.points[3];
        self.points[3] = self.points[2];
        self.points[2] = self.points[1];
        self.points[1] = self.points[0];
        self.points[0] = p;
    }

    /// Returns the most recent triangle.
    pub const fn prev1(&self) -> Triangle {
        Triangle::new(self.points[1], self.points[2], self.points[3])
    }

    /// Returns the second tracked triangle.
    pub const fn prev2(&self) -> Triangle {
        Triangle::new(self.points[2], self.points[3], self.points[4])
    }

    /// Returns the oldest tracked triangle.
    pub const fn prev3(&self) -> Triangle {
        Triangle::new(self.points[3], self.points[4], self.points[5])
    }
}

// TODO: Generalise name, move into more common folder path
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(in crate::primitives) struct ThickPoints<'a> {
    prev_points: LookbackBuffer,
    triangle_iter: JointTriangleIterator<'a>,
    points_iter: FillScanlineIterator,
    prev_joint_kind: JointKind,
}

impl<'a> ThickPoints<'a> {
    pub fn new(points: &'a [Point], width: u32, alignment: StrokeAlignment) -> Self {
        let mut triangle_iter = JointTriangleIterator::new(points, width, alignment);

        let triangle = triangle_iter.next().unwrap_or_else(Triangle::empty);
        let points_iter = FillScanlineIterator::new(&triangle, None);

        let mut prev_points = LookbackBuffer::new();
        prev_points.add(triangle.p1);
        prev_points.add(triangle.p2);
        prev_points.add(triangle.p3);

        Self {
            prev_points,
            triangle_iter,
            points_iter,
            prev_joint_kind: JointKind::Start,
        }
    }
}

impl<'a> Iterator for ThickPoints<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(point) = self.points_iter.next() {
                let return_point = match self.prev_joint_kind {
                    // List here the possible overdraw cases
                    // Triangle iterator states are the states after the current triangle, e.g.:
                    // - State::SecondTriangle: first triangle of a joint
                    // - State::ExtraTriangle: second triangle of a joint
                    // - State::NextJoint: extra triangle of a joint

                    // Bevelled(left) joints generate 4 triangles that share a single point:
                    //  - The second and filler joint triangle
                    //  - The "normal" triangles of the next joint
                    JointKind::Bevel(Side::Left) => match self.triangle_iter.state() {
                        State::SecondTriangle => !self.prev_points.prev2().contains(point),
                        State::ExtraTriangle => !self.prev_points.prev3().contains(point),
                        _ => true,
                    },

                    // Bevelled(right) joints are a bit nicer
                    JointKind::Bevel(Side::Right) => {
                        if self.triangle_iter.state() == State::SecondTriangle {
                            !self.prev_points.prev3().contains(point)
                        } else {
                            true
                        }
                    }

                    JointKind::Colinear => {
                        if self.triangle_iter.state() == State::ExtraTriangle {
                            !self.prev_points.prev3().contains(point)
                        } else {
                            !self.prev_points.prev2().contains(point)
                                && !self.prev_points.prev3().contains(point)
                        }
                    }

                    // For anything else, don't check
                    _ => true,
                };
                if return_point {
                    return Some(point);
                }
            } else {
                if self.triangle_iter.state() == State::NextJoint {
                    self.prev_joint_kind = self.triangle_iter.current_joint_kind();
                }

                // Calculate the next triangle
                let triangle = self.triangle_iter.next()?;

                // Remember the new point - iterator ensures it's always p3
                self.prev_points.add(triangle.p3);

                // Calculate the shared edge so that it can be ignored while drawing
                let edge = self.prev_points.prev1().shared_edge(triangle);

                // Create the pixel iterator for the current triangle
                self.points_iter = FillScanlineIterator::new(&triangle, edge);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        drawable::Drawable,
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        primitives::{Polyline, Primitive},
        style::PrimitiveStyle,
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
