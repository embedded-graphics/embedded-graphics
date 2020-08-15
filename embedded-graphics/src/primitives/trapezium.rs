//! Trapezium (internal use only)

use crate::{
    geometry::Point,
    primitives::{Line, Rectangle},
    transform::Transform,
};

fn trapezium_bounding_box(points: [Point; 4]) -> Rectangle {
    let min = points[0]
        .component_min(points[1])
        .component_min(points[2])
        .component_min(points[3]);
    let max = points[0]
        .component_max(points[1])
        .component_max(points[2])
        .component_max(points[3]);

    Rectangle::with_corners(min, max)
}

/// Trapezium iterator
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct TrapeziumIterator {
    points: [Point; 4],
    pos: Point,
    right_limit: i32,
    y_limit: i32,
    scanline: Line,
}

impl TrapeziumIterator {
    // TODO: pub(in crate::primitives)
    /// Make a new one
    pub fn new(points: [Point; 4]) -> Self {
        let bb = trapezium_bounding_box(points);

        let scanline = Line::new(bb.top_left, bb.top_left + bb.size.x_axis());

        if let Some((left, right)) = Self::intersections(&scanline, &points) {
            Self {
                points,
                pos: left,
                right_limit: right.x,
                scanline,
                y_limit: bb.bottom_right().unwrap().y,
            }
        } else {
            Self::empty()
        }
    }

    fn empty() -> Self {
        Self {
            points: [Point::zero(); 4],
            pos: Point::zero(),
            right_limit: 0,
            scanline: Line::new(Point::zero(), Point::zero()),
            y_limit: 0,
        }
    }

    fn intersections(scanline: &Line, points: &[Point; 4]) -> Option<(Point, Point)> {
        let [p0, p1, p2, p3] = *points;

        let lines = [
            Line::new(p0, p1),
            Line::new(p1, p2),
            Line::new(p2, p3),
            Line::new(p3, p0),
        ];

        let intersections = lines
            .iter()
            .filter_map(|l| l.segment_intersection_point(&scanline));

        let (min, max): (Option<Point>, Option<Point>) =
            intersections.fold((None, None), |acc, intersection_point| {
                (
                    acc.0
                        .map(|min| min.component_min(intersection_point))
                        .or_else(|| Some(intersection_point)),
                    acc.1
                        .map(|max| max.component_max(intersection_point))
                        .or_else(|| Some(intersection_point)),
                )
            });

        if let (Some(min), Some(max)) = (min, max) {
            Some((min, max))
        } else {
            None
        }
    }
}

impl Iterator for TrapeziumIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let point = self.pos;

        self.pos.x += 1;

        // Reached end of scanline. Step down one.
        if self.pos.x > self.right_limit {
            // Step down a line
            self.scanline.translate_mut(Point::new(0, 1));

            // If scanline is off bottom of bounding box, we're finished
            if self.scanline.start.y > self.y_limit {
                return None;
            }

            let (min, max) = Self::intersections(&self.scanline, &self.points)?;

            self.pos = min;
            self.right_limit = max.x;

            self.next()
        } else {
            Some(point)
        }
    }
}
