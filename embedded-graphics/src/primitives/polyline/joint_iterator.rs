//! Generate polyline joints from a series of points.

use crate::{
    geometry::Point,
    primitives::{
        line::{Intersection, Side},
        line_joint::{EdgeCorners, JointKind},
        Line, Triangle,
    },
    style::StrokeAlignment,
};

/// Processing state
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum State {
    /// Processing has finished
    End,

    /// Advance the processing so the next triangle can be generated
    NextJoint,
    FirstTriangle,
    SecondTriangle,
    ExtraTriangle,
}

/// Generate triangles of a polyline
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct JointTriangleIterator<'a> {
    /// Current state.
    pub state: State,

    /// The previous joint kind.
    pub prev_joint_kind: JointKind,

    points: &'a [Point],
    width: i32,
    alignment: StrokeAlignment,
    left: Line,
    right_start: Point, // right line can be calculated easily enough, don't waste memory
    previous_points: EdgeCorners,
    next_points: EdgeCorners,
    extra_point: Point,
    joint_kind: JointKind,
}

impl<'a> JointTriangleIterator<'a> {
    /// return a parallel line starting from the given point
    fn offset_line(line: Line, start: Point) -> Line {
        let delta = line.start - start;

        Line::new(start, line.end - delta)
    }

    /// return the right line
    fn right(&self) -> Line {
        Self::offset_line(self.left, self.right_start)
    }

    /// Returns a new iterator to process a number of points into a number of triangles.
    ///
    /// TODO: support closed shapes
    /// TODO: maybe simplify degenerate / colinear cases into a single joint kind
    pub fn new(points: &'a [Point], width: u32, alignment: StrokeAlignment) -> Self {
        if points.len() < 2 {
            return Self::empty();
        }

        let mut self_ = Self {
            points,
            state: State::NextJoint,
            width: width as i32,
            alignment,
            left: Line::new(Point::zero(), Point::zero()),
            right_start: Point::zero(),
            previous_points: EdgeCorners::default(),
            next_points: EdgeCorners::default(),
            extra_point: Point::zero(),
            joint_kind: JointKind::Start,
            prev_joint_kind: JointKind::Start,
        };

        self_.start();

        self_
    }

    /// Returns a new iterator that yields no triangles.
    pub fn empty() -> Self {
        Self {
            points: &[],
            state: State::End,
            width: 0,
            alignment: StrokeAlignment::Center,
            left: Line::new(Point::zero(), Point::zero()),
            right_start: Point::zero(),
            previous_points: EdgeCorners::default(),
            next_points: EdgeCorners::default(),
            extra_point: Point::zero(),
            joint_kind: JointKind::End,
            prev_joint_kind: JointKind::End,
        }
    }

    /// Initializes the internal state with the first segment.
    fn start(&mut self) {
        let (l, r) = self.next_line().extents(self.width, self.alignment);

        self.next_points = EdgeCorners {
            left: l.start,
            right: r.start,
        };

        self.left = l;
        self.right_start = r.start;

        self.state = State::NextJoint;
    }

    /// Returns the next, unprocessed line.
    fn next_line(&mut self) -> Line {
        let mid = self.points[0];
        let end = self.points[1];
        self.points = &self.points[1..];

        Line::new(mid, end)
    }

    /// Updates the state of the current joint.
    fn new_joint(&mut self, kind: JointKind, left: Point, right: Point) {
        self.prev_joint_kind = self.joint_kind;
        self.joint_kind = kind;
        self.previous_points = self.next_points;
        self.next_points = EdgeCorners { left, right };
    }

    /// Processes the next point to calculate the next joint.
    fn process_next(&mut self) {
        if self.points.len() < 2 {
            // end
            if self.points.len() == 1 {
                self.points = &self.points[1..];

                let l = self.left;
                let r = self.right();

                self.new_joint(JointKind::End, l.end, r.end);
                self.state = State::FirstTriangle;
            } else {
                self.state = State::End;
            }
        } else {
            let current_line = self.next_line();
            let (second_edge_left, second_edge_right) =
                current_line.extents(self.width, self.alignment);

            let first_edge_left = self.left;
            let first_edge_right = self.right();

            self.left = second_edge_left;
            self.right_start = second_edge_right.start;

            self.state = State::FirstTriangle;

            if let (
                Intersection::Point {
                    point: l_intersection,
                    outer_side,
                },
                Intersection::Point {
                    point: r_intersection,
                    ..
                },
            ) = (
                second_edge_left.line_intersection(&first_edge_left),
                second_edge_right.line_intersection(&first_edge_right),
            ) {
                let first_segment_start_edge =
                    Line::new(self.next_points.left, self.next_points.right);
                let intersection_line = Line::new(l_intersection, r_intersection);

                let is_degenerate =
                    first_segment_start_edge.segment_intersection(&intersection_line);

                // Normal line: non-overlapping line end caps
                if !is_degenerate {
                    // Length of the intersection segment.
                    let miter_length_squared =
                        Line::new(l_intersection, r_intersection).length_squared();

                    // Miter length limit is double the line width (squared to avoid sqrt() costs)
                    // We consider a miter sharp if the angle between the lines is >90°, which is
                    // checked by the length of the intersection segment.
                    let miter_limit_sharp = self.width.pow(2) as u32;
                    let miter_limit = 8 * miter_limit_sharp;

                    if miter_length_squared <= miter_limit {
                        // Intersection is within limit at which it will be chopped off into a
                        // bevel, so return a miter.
                        self.new_joint(
                            JointKind::Miter(miter_length_squared > miter_limit_sharp),
                            l_intersection,
                            r_intersection,
                        );
                    } else {
                        // Miter is too long, chop it into bevel-style corner
                        let kind = JointKind::Bevel(outer_side);
                        match outer_side {
                            Side::Right => {
                                self.new_joint(kind, l_intersection, first_edge_right.end);
                                self.extra_point = second_edge_right.start;
                            }
                            Side::Left => {
                                self.new_joint(kind, first_edge_left.end, r_intersection);
                                self.extra_point = second_edge_left.start;
                            }
                        }
                    }
                } else {
                    // Line segments overlap (degenerate)
                    // should this be direction dependent?
                    self.new_joint(
                        JointKind::Degenerate,
                        first_edge_left.end,
                        first_edge_right.end,
                    );
                    self.extra_point = second_edge_right.start;
                }
            } else {
                // Lines are colinear
                self.new_joint(
                    JointKind::Colinear,
                    first_edge_left.end,
                    first_edge_right.end,
                );
                self.extra_point = second_edge_right.start;
            }
        }
    }

    fn first_joint_triangle(&mut self) -> Option<Triangle> {
        // return the first triangle of the section BEFORE the joint
        self.state = State::SecondTriangle;
        Some(Triangle::new(
            self.previous_points.left,
            self.previous_points.right,
            self.next_points.left,
        ))
    }

    fn second_joint_triangle(&mut self) -> Option<Triangle> {
        // return the second triangle of the section BEFORE the joint
        self.state = State::ExtraTriangle;
        Some(Triangle::new(
            self.previous_points.right,
            self.next_points.left,
            self.next_points.right,
        ))
    }

    fn extra_joint_triangle(&mut self) -> Option<Triangle> {
        // return the filler triangle of the current joint
        self.state = State::NextJoint;
        match self.joint_kind {
            JointKind::Bevel(Side::Right) => {
                let right = self.next_points.right;
                self.next_points.right = self.extra_point;

                Some(Triangle::new(
                    self.next_points.left,
                    right,
                    self.extra_point,
                ))
            }

            JointKind::Bevel(Side::Left) => {
                let left = self.next_points.left;
                self.next_points.left = self.extra_point;

                Some(Triangle::new(
                    left,
                    self.next_points.right,
                    self.extra_point,
                ))
            }

            JointKind::Colinear | JointKind::Degenerate => {
                let bevel_triangle = Triangle::new(
                    self.next_points.left,
                    self.next_points.right,
                    self.extra_point,
                );

                self.next_points = EdgeCorners {
                    left: self.left.start,
                    right: self.right_start,
                };

                Some(bevel_triangle)
            }

            _ => None,
        }
    }
}

impl Iterator for JointTriangleIterator<'_> {
    type Item = Triangle;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.state {
                State::NextJoint => self.process_next(),
                State::FirstTriangle => return self.first_joint_triangle(),
                State::SecondTriangle => return self.second_joint_triangle(),
                State::ExtraTriangle => {
                    if let Some(t) = self.extra_joint_triangle() {
                        return Some(t);
                    }
                }
                State::End => return None,
            }
        }
    }
}
