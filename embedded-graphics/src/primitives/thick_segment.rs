//! A line segment constructed from two line joints.

use crate::{
    geometry::{Dimensions, Point},
    primitives::{line_join::LineJoin, ContainsPoint, Line, Primitive, Rectangle},
};

#[derive(Debug, Clone, Copy)]
pub(in crate::primitives) struct ThickSegment {
    start_join: LineJoin,
    end_join: LineJoin,
}

impl ThickSegment {
    /// Create a new thick segment from two joints.
    pub fn new(start_join: LineJoin, end_join: LineJoin) -> Self {
        Self {
            start_join,
            end_join,
        }
    }

    /// Get the right/left edges of this line segment.
    fn edges(&self) -> (Line, Line) {
        (
            Line::new(
                self.start_join.second_edge_start.right,
                self.end_join.first_edge_end.right,
            ),
            Line::new(
                self.end_join.first_edge_end.left,
                self.start_join.second_edge_start.left,
            ),
        )
    }

    /// Get the bounding box containing the left/right edges of the segment.
    ///
    /// Note that this does not include any bevel/cap lines as returned by `perimeter` which is why
    /// this is not `impl Dimensions`. These lines don't need to be included as other segments
    /// in the polyline will expand the bounding box to the right place anyway.
    pub fn edges_bounding_box(&self) -> Rectangle {
        let (right, left) = self.edges();

        let left = left.bounding_box();
        let right = right.bounding_box();

        let tl = left.top_left.component_min(right.top_left);

        let left_br = left.bottom_right().unwrap_or(tl);
        let right_br = right.bottom_right().unwrap_or(tl);

        let br = left_br.component_max(right_br);

        Rectangle::with_corners(tl, br)
    }

    pub(in crate::primitives) fn intersection(&self, scanline_y: i32) -> Option<Line> {
        // Loop through perimeter and get any intersections
        let it = PerimeterLineIterator::new(&self)
            .filter_map(|l| bresenham_scanline_intersection(&l, scanline_y));

        // Loop through intersections and collect min/max bounds of all of them into a single line
        let line = it.fold(None, |acc: Option<Line>, line| {
            if let Some(acc) = acc {
                Some(Line::new(
                    acc.start.component_min(line.start),
                    acc.end.component_max(line.end),
                ))
            } else {
                Some(line)
            }
        })?;

        Some(line)
    }
}

/// Intersect a horizontal scan line with the Bresenham representation of this line segment.
fn bresenham_scanline_intersection(line: &Line, scan_y: i32) -> Option<Line> {
    if !line
        .bounding_box()
        .contains(Point::new(line.start.x, scan_y))
    {
        return None;
    }

    let mut points = line.points().filter(|p| p.y == scan_y);

    let first = points.next()?;

    points
        .last()
        .filter(|last| *last != first)
        .map(|last| Line::new(first, last).sorted_x())
        .or_else(|| Some(Line::new(first, first)))
}

enum State {
    StartCap,
    EndCap,
    Edges,
    Done,
}

struct PerimeterLineIterator<'a> {
    segment: &'a ThickSegment,
    next_line: Option<Line>,
    state: State,
}

impl<'a> PerimeterLineIterator<'a> {
    fn new(segment: &'a ThickSegment) -> Self {
        Self {
            segment,
            state: State::StartCap,
            next_line: None,
        }
    }
}

impl<'a> Iterator for PerimeterLineIterator<'a> {
    type Item = Line;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.next_line.take() {
            return Some(next);
        }

        match self.state {
            State::StartCap => {
                self.state = State::EndCap;
                let (line, next) = self.segment.start_join.start_cap_lines();
                self.next_line = next;
                Some(line)
            }
            State::EndCap => {
                self.state = State::Edges;
                let (line, next) = self.segment.end_join.end_cap_lines();
                self.next_line = next;
                Some(line)
            }
            State::Edges => {
                self.state = State::Done;
                let (line, next) = self.segment.edges();
                self.next_line = Some(next);
                Some(line)
            }
            State::Done => None,
        }
    }
}
