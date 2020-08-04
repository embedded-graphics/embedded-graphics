use crate::{
    prelude::Point,
    primitives::{
        line_joint::{EdgeCorners, LineJoint},
        triangle::{scanline_iterator::PointType, Triangle},
        Line,
    },
    style::StrokeAlignment,
};

/// Calculate squared distance from midpoint of an outside (left) edge to the center of the triangle
fn calc_dist(center: Point, start: LineJoint, end: LineJoint) -> u32 {
    let start = start.second_edge_start.left;
    let end = end.first_edge_end.left;

    Line::new(start, end).distance_to_point_squared(center)
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(in crate::primitives) struct TriangleIterator {
    is_collapsed: bool,

    /// Whether the triangle center is filled or not.
    has_fill: bool,
    joints: [LineJoint; 3],
    at_joint: usize,
    t1: Option<Triangle>,
    t2: Option<Triangle>,
    filler: Option<Triangle>,
    inner_fill: Option<Triangle>,
    center: Point,
}

impl TriangleIterator {
    pub fn new(
        triangle: &Triangle,
        stroke_width: u32,
        stroke_alignment: StrokeAlignment,
        has_fill: bool,
    ) -> Self {
        let triangle = triangle.sorted_clockwise();
        let Triangle { p1, p2, p3 } = triangle;

        // Joint centered around P1
        let joint1 = LineJoint::from_points(p3, p1, p2, stroke_width, stroke_alignment);
        // Joint centered around P2
        let joint2 = LineJoint::from_points(p1, p2, p3, stroke_width, stroke_alignment);
        // Joint centered around P3
        let joint3 = LineJoint::from_points(p2, p3, p1, stroke_width, stroke_alignment);

        let centroid = triangle.centroid();

        let dist1 = calc_dist(centroid, joint1, joint2);
        let dist2 = calc_dist(centroid, joint2, joint3);
        let dist3 = calc_dist(centroid, joint3, joint1);

        // Flag denoting whether the inside of the triangle is completely filled by the edge strokes
        // or not.
        let is_collapsed = dist1 < stroke_width.pow(2)
            || dist2 < stroke_width.pow(2)
            || dist3 < stroke_width.pow(2);

        let joints = [joint1, joint2, joint3];

        let inner_fill = if has_fill && !is_collapsed {
            Some(Triangle::new(
                joint1.first_edge_end.right,
                joint2.first_edge_end.right,
                joint3.first_edge_end.right,
            ))
        } else {
            None
        };

        let mut self_ = Self {
            is_collapsed,
            at_joint: 0,
            joints,
            t1: None,
            t2: None,
            filler: None,
            center: centroid,
            has_fill,
            inner_fill,
        };

        self_.update_triangles();

        self_
    }

    pub fn empty() -> Self {
        Self {
            is_collapsed: false,
            at_joint: 0,
            joints: [LineJoint::empty(); 3],
            t1: None,
            t2: None,
            filler: None,
            center: Point::zero(),
            has_fill: false,
            inner_fill: None,
        }
    }

    fn update_triangles(&mut self) -> Option<()> {
        let s = self.at_joint % 3;
        let e = (self.at_joint + 1) % 3;

        let start_joint = self.joints.get(s)?;
        let end_joint = self.joints.get(e)?;

        if !self.is_collapsed {
            let (t1, t2) = Self::edge_triangles(*start_joint, *end_joint);

            self.t1 = Some(t1);
            self.t2 = Some(t2);
            self.filler = start_joint.filler();
        } else {
            let filler = start_joint.filler().map(|filler| {
                // Rework filler triangle so it touches the triangle center
                Triangle::new(filler.p1, filler.p2, self.center)
            });

            self.t1 = Self::collapsed_edge_triangle(*start_joint, *end_joint, self.center);
            self.t2 = None;
            self.filler = filler;
        }

        Some(())
    }

    // TODO: Dedupe with polyline method of same name
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

    fn collapsed_edge_triangle(
        start_joint: LineJoint,
        end_joint: LineJoint,
        center: Point,
    ) -> Option<Triangle> {
        let LineJoint {
            second_edge_start:
                EdgeCorners {
                    left: left_start,
                    // right: right_start,
                    ..
                },
            ..
        } = start_joint;
        let LineJoint {
            first_edge_end:
                EdgeCorners {
                    left: left_end,
                    // right: right_end,
                    ..
                },
            ..
        } = end_joint;

        let tri = Triangle::new(left_start, left_end, center);

        if tri.area_doubled() > 0 {
            Some(tri)
        } else {
            None
        }
    }
}

impl Iterator for TriangleIterator {
    type Item = (Triangle, PointType);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_fill
            .take()
            .map(|t| (t, PointType::Inside))
            .or_else(|| self.t1.take().map(|t| (t, PointType::Border)))
            .or_else(|| self.t2.take().map(|t| (t, PointType::Border)))
            .or_else(|| self.filler.take().map(|t| (t, PointType::Border)))
            .or_else(|| {
                self.at_joint += 1;

                // Triangles for all 3 joints and edges have been returned. We're done.
                if self.at_joint == 3 {
                    return None;
                }

                self.update_triangles();

                self.next()
            })
    }
}
