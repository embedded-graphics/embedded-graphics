//! An iterator over all line intersections with a given scanline.

use crate::{
    geometry::Point,
    primitives::{
        line::BresenhamIntersection, line_joint::JointKind, line_joint::LineJoint,
        thick_segment::ThickSegment, Line,
    },
    style::StrokeAlignment,
};

/// Scanline intersections iterator.
///
/// This iterator returns multiple `Line`s corresponding to the filled in areas of a polyline
/// defined by the `points` parameter.
///
/// The result is one line of a filled polygon.
#[derive(Clone, Debug)]
pub struct ScanlineIntersections<'a> {
    windows: core::slice::Windows<'a, Point>,
    points: &'a [Point],
    scanline_y: i32,
    start_joint: LineJoint,
    end_joint: LineJoint,
    width: u32,
    alignment: StrokeAlignment,
    stop: bool,
}

// type InternalItem = (Option<BresenhamIntersection>, LineType, Line);
// type InternalItem = (BresenhamIntersection, Line, LineType);

static EMPTY: &[Point; 0] = &[];

impl<'a> ScanlineIntersections<'a> {
    /// New
    pub fn new(
        points: &'a [Point],
        width: u32,
        alignment: StrokeAlignment,
        scanline_y: i32,
    ) -> Self {
        // let lines = LineJointsIter::new(points, width, alignment);

        println!("\n\n\n---\n\n\n");

        // Self {
        //     lines,
        //     scanline_y,
        //     prev_intersection: None,
        //     prev_line: None,
        // }

        let mut windows = points.windows(3);

        if let Some([start, mid, end]) = windows.next() {
            let start_joint = LineJoint::start(*start, *mid, width, alignment);
            let end_joint = LineJoint::from_points(*start, *mid, *end, width, alignment);

            Self {
                windows,
                start_joint,
                end_joint,
                width,
                alignment,
                points,
                scanline_y,
                stop: false,
            }
        } else if let [start, end] = points {
            // Single line segment.
            let start_joint = LineJoint::start(*start, *end, width, alignment);
            let end_joint = LineJoint::end(*start, *end, width, alignment);

            Self {
                windows: EMPTY.windows(3),
                start_joint,
                end_joint,
                width,
                alignment,
                points,
                scanline_y,
                stop: false,
            }
        } else {
            // Points must be at least 2 in length to make a polyline iterator out of.
            Self::empty()
        }
    }

    /// Empty
    fn empty() -> Self {
        Self {
            windows: EMPTY.windows(3),
            start_joint: LineJoint::empty(),
            end_joint: LineJoint::empty(),
            width: 0,
            alignment: StrokeAlignment::Center,
            points: EMPTY,
            scanline_y: 0,
            stop: true,
        }
    }

    /// TODO: DOCS. SHould probably un-pub this?
    pub fn reset_with_new_scanline(&mut self, scanline_y: i32) {
        *self = Self::new(self.points, self.width, self.alignment, scanline_y);
    }

    // fn next_intersection(&mut self) -> Option<InternalItem> {
    //     let scanline_y = self.scanline_y;

    //     self.lines.find_map(|(l, line_type)| {
    //         l.bresenham_scanline_intersection(scanline_y)
    //             .map(|(intersection, _line)| (intersection, l, line_type))
    //     })
    // }

    // fn next_line(&mut self) -> Option<Line> {
    //     self.prev_line.take().or_else(|| {
    //         let first = self
    //             .prev_intersection
    //             .take()
    //             .or_else(|| self.next_intersection())?;

    //         let second = self.next_intersection()?;

    //         let l1 = first.0.as_line();
    //         let l2 = second.0.as_line();

    //         self.prev_intersection = Some(second);

    //         let line = if l2.end.x < l1.start.x {
    //             Line::new(l2.end, l1.start)
    //         } else {
    //             Line::new(l1.start, l2.end)
    //         };

    //         Some(line)
    //     })
    // }

    fn update(&mut self) -> Option<()> {
        self.start_joint = self.end_joint;

        if let Some([start, mid, end]) = self.windows.next() {
            self.end_joint = LineJoint::from_points(*start, *mid, *end, self.width, self.alignment);
        } else {
            // self.stop = true;
            let start = *self.points.get(self.points.len() - 2)?;
            let end = *self.points.last()?;

            self.end_joint = LineJoint::end(start, end, self.width, self.alignment);
        }

        Some(())
    }

    // fn next_joint(&mut self) -> Option<LineJoint> {
    //     if self.stop {
    //         return None;
    //     }

    //     if let Some([start, mid, end]) = self.windows.next() {
    //         Some(LineJoint::from_points(
    //             *start,
    //             *mid,
    //             *end,
    //             self.width,
    //             self.alignment,
    //         ))
    //     } else {
    //         self.stop = true;
    //         let start = *self.points.get(self.points.len() - 2)?;
    //         let end = *self.points.last()?;

    //         Some(LineJoint::end(start, end, self.width, self.alignment))
    //     }
    // }
}

impl<'a> Iterator for ScanlineIntersections<'a> {
    type Item = Line;

    #[allow(unused)]
    fn next(&mut self) -> Option<Self::Item> {
        println!("xxxx");

        if self.stop {
            return None;
        }

        if self.end_joint.kind == JointKind::End {
            self.stop = true;
        }

        let line =
            ThickSegment::new(self.start_joint, self.end_joint).intersection(self.scanline_y);

        self.update()?;

        if let Some(line) = line {
            Some(line)
        } else {
            self.next()
        }

        // self.prev_line = Some(line);

        // Various reasons to _not_ skip the currently computed line, i.e. these pick any line
        // that's inside the shape. Lines outside the shape are skipped with the fallthrough
        // branch.
        // match dbg!((
        //     first.0,
        //     second.0,
        //     first.1.sign_y(),
        //     second.1.sign_y(),
        //     first.2,
        //     second.2,
        // )) {
        //     // Colinear lines are always inside
        //     (BresenhamIntersection::Colinear(_), _, _, _, _, _) => {}
        //     // Colinear lines are always inside
        //     (_, BresenhamIntersection::Colinear(_), _, _, _, _) => {}
        //     // Start cap to any intersection is always inside
        //     (_, _, _, _, LineType::StartCap, _) => {}
        //     // Final line is always inside shape if it hits end cap
        //     (_, _, _, _, _, LineType::EndCap) => {}
        //     // Left side -> right side line always results in intersection
        //     (_, _, _, _, LineType::LeftSide, LineType::RightSide) => {}
        //     // Likewise with the opposite right -> left
        //     (_, _, _, _, LineType::RightSide, LineType::LeftSide) => {}
        //     // Left side but pyramid shape is inside
        //     (_, _, s1, s2, LineType::LeftSide, LineType::LeftSide) if s1 == -1 && s2 == 1 => {}
        //     // Right side but pyramid shape is inside
        //     (_, _, s1, s2, LineType::RightSide, LineType::RightSide) if s1 == 1 && s2 == -1 => {}
        //     _ => {
        //         return self.next();
        //     }
        // }

        // Some(self.prev_line)
        // Some(line)
    }
}
