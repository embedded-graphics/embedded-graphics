//! An iterator over all line intersections with a given scanline.

use crate::{
    geometry::Point,
    primitives::{
        line::BresenhamIntersection,
        line_joints_iter::{LineJointsIter, LineType},
        Line,
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
    lines: LineJointsIter<'a>,
    scanline_y: i32,
    prev_intersection: Option<InternalItem>,
}

// type InternalItem = (Option<BresenhamIntersection>, LineType, Line);
type InternalItem = (BresenhamIntersection, Line, LineType);

impl<'a> ScanlineIntersections<'a> {
    /// New
    pub fn new(
        points: &'a [Point],
        width: u32,
        alignment: StrokeAlignment,
        scanline_y: i32,
    ) -> Self {
        let lines = LineJointsIter::new(points, width, alignment);

        println!("\n\n\n---\n\n\n");

        Self {
            lines,
            scanline_y,
            prev_intersection: None,
        }
    }

    /// DOCS. SHould probably un-pub this?
    pub fn reset_with_new_scanline(&mut self, scanline_y: i32) {
        self.lines.reset();
        self.scanline_y = scanline_y;
    }

    fn next_intersection(&mut self) -> Option<InternalItem> {
        let scanline_y = self.scanline_y;

        self.lines.find_map(|(l, line_type)| {
            l.bresenham_scanline_intersection(scanline_y)
                .map(|(intersection, _line)| (intersection, l, line_type))
        })
    }
}

impl<'a> Iterator for ScanlineIntersections<'a> {
    type Item = Line;

    #[allow(unused)]
    fn next(&mut self) -> Option<Self::Item> {
        let first = self
            .prev_intersection
            .take()
            .or_else(|| self.next_intersection())?;

        let second = self.next_intersection()?;

        let l1 = first.0.as_line();
        let l2 = second.0.as_line();

        self.prev_intersection = Some(second);

        // Various reasons to _not_ skip the currently computed line, i.e. these pick any line
        // that's inside the shape. Lines outside the shape are skipped with the fallthrough
        // branch.
        match (
            first.0,
            first.0,
            first.1.sign_y(),
            second.1.sign_y(),
            first.2,
            second.2,
        ) {
            // Colinear lines are always inside
            (BresenhamIntersection::Colinear(_), _, _, _, _, _) => {}
            // Colinear lines are always inside
            (_, BresenhamIntersection::Colinear(_), _, _, _, _) => {}
            // Start cap to any intersection is always inside
            (_, _, _, _, LineType::StartCap, _) => {}
            // Final line is always inside shape if it hits end cap
            (_, _, _, _, _, LineType::EndCap) => {}
            // Left side -> right side line always results in intersection
            (_, _, _, _, LineType::LeftSide, LineType::RightSide) => {}
            // Likewise with the opposite right -> left
            (_, _, _, _, LineType::RightSide, LineType::LeftSide) => {}
            // Left side but pyramid shape is inside
            (_, _, l1, l2, LineType::LeftSide, LineType::LeftSide) if l1 == -1 && l2 == 1 => {}
            // Right side but pyramid shape is inside
            (_, _, l1, l2, LineType::RightSide, LineType::RightSide) if l1 == 1 && l2 == -1 => {}
            _ => {
                return self.next();
            }
        }

        Some(Line::new(l1.start, l2.end))
    }
}
