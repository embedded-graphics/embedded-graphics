use crate::{
    prelude::Point,
    primitives::{
        line_joint::{EdgeCorners, LineJoint},
        Triangle,
    },
    style::StrokeAlignment,
};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(in crate::primitives) struct TriangleIterator<'a> {
    points: &'a [Point],
    start_idx: usize,
    t1: Option<Triangle>,
    t2: Option<Triangle>,
    filler: Option<Triangle>,
    width: u32,
    alignment: StrokeAlignment,
    end_joint: LineJoint,
}

impl<'a> TriangleIterator<'a> {
    pub fn new(points: &'a [Point], width: u32, alignment: StrokeAlignment) -> Self {
        if points.len() < 2 {
            Self::empty()
        } else {
            let start_idx = 0;

            let start_joint =
                LineJoint::start(points[start_idx], points[start_idx + 1], width, alignment);

            // If there are enough points to compute first joint, do so. Otherwise the line is two
            // points long and should just be a straight segment.
            let end_joint = if points.len() >= 3 {
                LineJoint::from_points(
                    points[start_idx],
                    points[start_idx + 1],
                    points[start_idx + 2],
                    width,
                    alignment,
                )
            } else {
                LineJoint::end(points[start_idx], points[start_idx + 1], width, alignment)
            };

            // Initialise with line between p0 and p1
            let (t1, t2) = Self::edge_triangles(start_joint, end_joint);

            Self {
                points,
                t1: Some(t1),
                t2: Some(t2),
                start_idx,
                filler: end_joint.filler(),
                width,
                alignment,
                end_joint,
            }
        }
    }

    pub fn empty() -> Self {
        Self {
            points: &[],
            t1: None,
            t2: None,
            filler: None,
            start_idx: 0,
            width: 0,
            alignment: StrokeAlignment::Center,
            end_joint: LineJoint::empty(),
        }
    }

    fn edge_triangles(start_joint: LineJoint, end_joint: LineJoint) -> (Triangle, Triangle) {
        let LineJoint {
            second_edge_start:
                EdgeCorners {
                    left: left_start,
                    right: right_start,
                },
            ..
        } = start_joint;
        let LineJoint {
            first_edge_end:
                EdgeCorners {
                    left: left_end,
                    right: right_end,
                },
            ..
        } = end_joint;

        let t1 = Triangle::new(left_start, left_end, right_start);
        let t2 = Triangle::new(right_start, left_end, right_end);

        (t1, t2)
    }
}

impl<'a> Iterator for TriangleIterator<'a> {
    type Item = Triangle;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(triangle) = self
            .t1
            .take()
            .or_else(|| self.t2.take())
            .or_else(|| self.filler.take())
        {
            Some(triangle)
        }
        // We've gone through the list of triangles in this edge/joint. Reset state for next edge
        // and joint.
        else {
            self.start_idx += 1;

            let start_joint = self.end_joint;

            // Compute next end joint. The iterator will stop if the `points.get()` calls below
            // return `None`, denoting that we've gone past the end of the points array.
            self.end_joint = if let Some(third_point) = self.points.get(self.start_idx + 2) {
                LineJoint::from_points(
                    *self.points.get(self.start_idx)?,
                    *self.points.get(self.start_idx + 1)?,
                    *third_point,
                    self.width,
                    self.alignment,
                )
            } else {
                LineJoint::end(
                    *self.points.get(self.start_idx)?,
                    *self.points.get(self.start_idx + 1)?,
                    self.width,
                    self.alignment,
                )
            };

            let (t1, t2) = Self::edge_triangles(start_joint, self.end_joint);

            self.t1 = Some(t1);
            self.t2 = Some(t2);
            self.filler = self.end_joint.filler();

            self.next()
        }
    }
}
