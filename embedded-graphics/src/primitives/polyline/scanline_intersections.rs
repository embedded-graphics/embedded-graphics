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
}

// type InternalItem = (Option<BresenhamIntersection>, LineType, Line);
type InternalItem = (BresenhamIntersection, LineType, Line);

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
        }
    }

    /// DOCS. SHould probably un-pub this?
    pub fn reset_with_new_scanline(&mut self, scanline_y: i32) {
        self.lines.reset();
        self.scanline_y = scanline_y;
        self.prev_end = Point::zero();
    }

    fn next_intersection(&mut self) -> Option<Line> {
        let scanline_y = self.scanline_y;

        let mut line = self.cache.take().or_else(|| {
            self.lines
                .find_map(|(l, _line_type)| l.bresenham_scanline_intersection(scanline_y))
                .map(|intersection| intersection.as_line())
        })?;

        while let Some(next) = self
            .lines
            .find_map(|(l, _line_type)| l.bresenham_scanline_intersection(scanline_y))
            .map(|l| l.as_line())
        {
            // Extend line if they touch each other
            if line.end == next.start {
                line.end = next.end
            } else {
                // We've already read `next` out of the iterator, but we want to reuse it so store
                // it somewhere.
                self.cache = Some(next);

                break;
            }
        }

        // loop {
        //     let next = self
        //         .lines
        //         .find_map(|(l, _line_type)| l.bresenham_scanline_intersection(scanline_y))?
        //         .as_line();

        //     // Extend line if they touch each other
        //     if line.end == next.start {
        //         line.end = next.end
        //     } else {
        //         // We've already read `next` out of the iterator, but we want to reuse it so store
        //         // it somewhere.
        //         self.cache = Some(next);

        //         break;
        //     }
        // }

        Some(line)

        // self.lines.find_map(|(l, s)| {
        //     l.bresenham_scanline_intersection(scanline_y)
        //         .map(|intersection| (intersection, s, l))
        // })
    }

    // /// Join two colinear intersections into one longer line, if they both touch.
    // fn join_intersections(
    //     first: &BresenhamIntersection,
    //     second: &BresenhamIntersection,
    // ) -> Option<Line> {
    //     let first_line = match first {
    //         BresenhamIntersection::Colinear(l) | BresenhamIntersection::Line(l) => *l,
    //         BresenhamIntersection::Point(p) => Line::new(*p, *p),
    //     };

    //     let second_line = match second {
    //         BresenhamIntersection::Colinear(l) | BresenhamIntersection::Line(l) => *l,
    //         BresenhamIntersection::Point(p) => Line::new(*p, *p),
    //     };

    //     if second_line.start != first_line.end {
    //         return None;
    //     }

    //     // Some(BresenhamIntersection::Colinear(Line::new(
    //     //     first_line.start,
    //     //     second_line.end,
    //     // )))
    //     Some(Line::new(first_line.start, second_line.end))
    // }
}

impl<'a> Iterator for ScanlineIntersections<'a> {
    type Item = Line;

    #[allow(unused)]
    fn next(&mut self) -> Option<Self::Item> {
        println!("xxx");

        // dbg!(self.i, self.i % 2);
        let prev_end = self.start;

        let start = self
            .start
            .or_else(|| self.next_intersection().map(|l| l.start))?;

        let curr = self.next_intersection()?;

        let end = curr.end;

        let line = Line::new(start, end);

        self.start = Some(end);

        dbg!(line, prev_end, self.start);

        // match prev_end {
        //     Some(prev_end) if prev_end == line.start => {
        //         // self.i += 1;
        //     }
        //     _ => self.i += 1,
        // }

        self.i += 1;

        // Skip outside iteration
        if self.i % 2 == 0 {
            // dbg!("skipping");
            return self.next();
        }

        Some(line)
    }
}
