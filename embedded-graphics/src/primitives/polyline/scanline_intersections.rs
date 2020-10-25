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
use core::iter::Peekable;

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
    prev_end: Point,
    i: u32,
    cache: Option<Line>,
    prev_was_colinear: bool,
    start: Option<Point>,
    winding_number: i32,
    prev_intersection: Option<InternalItem>,
    prev_line: Option<Line>,
    prev_line_type: LineType,
    prev_is_colinear: bool,
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
            prev_end: Point::zero(),
            i: 0,
            cache: None,
            prev_was_colinear: false,
            start: None,
            winding_number: 0,
            prev_intersection: None,
            prev_line: None,
            prev_line_type: LineType::StartCap,
            prev_is_colinear: false,
        }
    }

    /// DOCS. SHould probably un-pub this?
    pub fn reset_with_new_scanline(&mut self, scanline_y: i32) {
        self.lines.reset();
        self.scanline_y = scanline_y;
        self.prev_end = Point::zero();
    }

    fn next_intersection(&mut self) -> Option<InternalItem> {
        let scanline_y = self.scanline_y;

        // let (mut initial_intersection, initial_line, initial_type) =
        //     self.cache.take().or_else(|| {
        //         self.lines.find_map(|(l, line_type)| {
        //             l.bresenham_scanline_intersection(scanline_y)
        //                 .map(|(intersection, line)| (intersection, line, line_type))
        //         })
        //     })?;

        // while let Some((next_intersection, next_line, next_type)) =
        //     self.lines.find_map(|(l, line_type)| {
        //         l.bresenham_scanline_intersection(scanline_y)
        //             .map(|(intersection, line)| (intersection, line, line_type))
        //     })
        // {
        //     // let l1 = initial_intersection.as_line();
        //     // let l2 = next_intersection.as_line();

        //     // dbg!(initial_intersection, next_intersection);

        //     match next_intersection {
        //         BresenhamIntersection::Colinear(l2) | BresenhamIntersection::Line(l2)
        //             if l2.start == initial_intersection.as_line().end =>
        //         {
        //             match initial_intersection {
        //                 BresenhamIntersection::Colinear(mut l1)
        //                 | BresenhamIntersection::Line(mut l1) => {
        //                     l1.end = l2.end;
        //                 }
        //                 BresenhamIntersection::Point(p1) => {
        //                     initial_intersection =
        //                         BresenhamIntersection::Colinear(Line::new(p1, l2.end));
        //                 }
        //             }
        //         }
        //         _ => {
        //             self.cache = Some((next_intersection, next_line, next_type));

        //             break;
        //         }
        //     }
        // }

        // Some((initial_intersection, initial_line, initial_type))

        self.lines.find_map(|(l, line_type)| {
            l.bresenham_scanline_intersection(scanline_y)
                .map(|(intersection, line)| (intersection, line, line_type))
        })
    }

    // fn next_line(&mut self) -> Option<Line> {
    //     let (start_intersection, start_line, start_type) = self
    //         .prev_intersection
    //         .take()
    //         .or_else(|| self.next_intersection())?;

    //     let (end_intersection, end_line, end_type) = self.next_intersection()?;

    //     self.prev_intersection = Some((end_intersection, end_line, end_type));

    //     Some(Line::new(
    //         start_intersection.as_line().start,
    //         end_intersection.as_line().end,
    //     ))
    // }
}

impl<'a> Iterator for ScanlineIntersections<'a> {
    type Item = Line;

    #[allow(unused)]
    fn next(&mut self) -> Option<Self::Item> {
        println!("xxx");

        let first = self
            .prev_intersection
            .take()
            .or_else(|| self.next_intersection())?;

        let second = self.next_intersection()?;

        let l1 = first.0.as_line();
        let l2 = second.0.as_line();

        self.prev_intersection = Some(second);

        // dbg!(first, second);

        match (first.2, second.2, first.0, first.0, first.1, second.1) {
            // Colinear lines are always inside
            (_, _, BresenhamIntersection::Colinear(_), _) => {
                dbg!("Colinear first inside");
            }
            // Colinear lines are always inside
            (_, _, _, BresenhamIntersection::Colinear(_)) => {
                dbg!("Colinear second inside");
            }
            // Start cap to any intersection is always inside
            (LineType::StartCap, _, _, _) => {
                dbg!("Inside start cap -> any");
            }
            // Left side -> right side line always results in intersection
            (LineType::LeftSide, LineType::RightSide, _, _) => {
                dbg!("Inside L -> R");
            }
            // Likewise with the opposite right -> left
            (LineType::RightSide, LineType::LeftSide, _, _) => {
                dbg!("Inside R -> L");
            }
            // Final line is always inside shape if it hits end cap
            (_, LineType::EndCap, _, _) => {
                dbg!("Inside any -> end cap");
            }
            _ => {
                // dbg!("Not  dere");
                return self.next();
            }
        }

        Some(Line::new(l1.start, l2.end))

        // dbg!(l1, l2);

        // if l1 != l2 {
        //     self.prev_line = Some(l1);

        //     Some(l1)
        // }
        // // Ignores first line - the next one is longer and better
        // else if l1.start == l2.start && l2.end > l1.end {
        //     Some(l2)
        // }
        // // Also ignore first line? idk why
        // else {
        //     Some(l2)
        // }
    }
}
