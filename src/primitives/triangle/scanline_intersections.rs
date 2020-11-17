//! Triangle scanline intersections iterator.

use crate::{
    geometry::Point,
    primitives::{
        common::{LineJoin, StrokeOffset, ThickSegment},
        polyline::scanline_intersections::{extend, touches},
        Line, Triangle,
    },
};

/// Type of scanline.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum PointType {
    /// Represents part of the stroke.
    Stroke,

    /// Represents the interior of the shape.
    Fill,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
struct LineConfig {
    first: Option<Line>,
    second: Option<Line>,
    internal: Option<(Line, PointType)>,
}

/// Triangle scanline intersections iterator.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(in crate::primitives::triangle) struct ScanlineIntersections {
    lines: LineConfig,
    triangle: Triangle,
    stroke_width: u32,
    stroke_offset: StrokeOffset,
    has_fill: bool,
}

static EMPTY: Triangle = Triangle::new(Point::zero(), Point::zero(), Point::zero());

impl ScanlineIntersections {
    /// Create a new thick segments iterator.
    pub fn new(
        triangle: &Triangle,
        stroke_width: u32,
        stroke_offset: StrokeOffset,
        has_fill: bool,
        scanline_y: i32,
    ) -> Self {
        if let Some(lines) =
            generate_lines(triangle, stroke_width, stroke_offset, has_fill, scanline_y)
        {
            Self {
                lines,
                has_fill,
                triangle: *triangle,
                stroke_width,
                stroke_offset,
            }
        } else {
            Self::empty()
        }
    }

    /// Empty.
    pub fn empty() -> Self {
        Self {
            lines: LineConfig::default(),
            has_fill: false,
            triangle: EMPTY,
            stroke_width: 0,
            stroke_offset: StrokeOffset::None,
        }
    }

    /// Reset with a new scanline.
    pub fn reset_with_new_scanline(&mut self, scanline_y: i32) {
        if let Some(lines) = generate_lines(
            &self.triangle,
            self.stroke_width,
            self.stroke_offset,
            self.has_fill,
            scanline_y,
        ) {
            self.lines = lines
        }
    }
}

fn generate_lines(
    triangle: &Triangle,
    stroke_width: u32,
    stroke_offset: StrokeOffset,
    has_fill: bool,
    scanline_y: i32,
) -> Option<LineConfig> {
    let mut edge_intersections = {
        let mut idx = 0;
        let mut left = None;
        let mut right = None;

        let t = triangle;

        core::iter::from_fn(move || {
            if stroke_width == 0 {
                return None;
            }

            while idx < 3 {
                let start = LineJoin::from_points(
                    t.vertex(idx),
                    t.vertex(idx + 1),
                    t.vertex(idx + 2),
                    stroke_width,
                    stroke_offset,
                );
                let end = LineJoin::from_points(
                    t.vertex(idx + 1),
                    t.vertex(idx + 2),
                    t.vertex(idx + 3),
                    stroke_width,
                    stroke_offset,
                );

                idx += 1;

                if let Some(next_segment) = ThickSegment::new(start, end).intersection(scanline_y) {
                    if let Some(line) = left.as_mut() {
                        if touches(*line, next_segment) {
                            *line = extend(*line, next_segment);
                            continue;
                        }
                    } else {
                        left = Some(next_segment);
                        continue;
                    }

                    if let Some(line) = right.as_mut() {
                        if touches(*line, next_segment) {
                            *line = extend(*line, next_segment);
                        }
                    } else {
                        right = Some(next_segment);
                    }
                }
            }

            // Merge any overlap between final left/right results
            // MSRV: Use Option::zip once we upgrade to 1.46.0 or greater.
            if let (Some(l), Some(r)) = (left, right) {
                if touches(l, r) {
                    left = Some(extend(l, r));
                    right = None;
                }
            }

            left.take().or_else(|| right.take())
        })
    };

    // Special case: If thick strokes completely fill the triangle interior and the stroke is
    // inside the triangle, the normal triangle shape can be used to detect the intersection,
    // with the line type being marked as Border so, when rendered, the correct color is used.
    if triangle.is_collapsed(stroke_width, stroke_offset) && stroke_offset == StrokeOffset::Right {
        Some(LineConfig {
            internal: triangle
                .scanline_intersection(scanline_y)
                .map(|l| (l, PointType::Stroke)),
            ..LineConfig::default()
        })
    } else {
        let first = edge_intersections.next();

        // For scanlines that are parallel with and are inside one edge, this should be None.
        let second = edge_intersections.next();

        // If there are two intersections, this must mean we've traversed across the center of the
        // triangle (assuming the edge line merging logic is correct). In this case, we need a
        // scanline between the two edge intersections.
        let internal = if has_fill {
            match (first, second) {
                // Triangle stroke is non-zero, so the fill line is between the insides of each
                // stroke.
                (Some(first), Some(second)) => {
                    let start_x = first.end.x.min(second.end.x);
                    let end_x = first.start.x.max(second.start.x);

                    // Line needs to be shrunk by 1px off each end to prevent overdraw.
                    // This can only happen if there's enough room to do so.
                    if (end_x - start_x) > 1 {
                        Some(Line::new(
                            Point::new(start_x + 1, scanline_y),
                            Point::new(end_x - 1, scanline_y),
                        ))
                    } else {
                        None
                    }
                }
                // Triangles with no stroke intersections and a fill color.
                (None, None) => triangle.scanline_intersection(scanline_y),
                // Because a triangle is a closed shape, a single intersection here likely means
                // we're inside one of the borders, so no fill should be returned for this
                // scanline.
                _ => None,
            }
        } else {
            None
        };

        Some(LineConfig {
            first,
            second,
            internal: internal.map(|l| (l, PointType::Fill)),
        })
    }
}

impl Iterator for ScanlineIntersections {
    type Item = (Line, PointType);

    fn next(&mut self) -> Option<Self::Item> {
        self.lines
            .internal
            .take()
            .or_else(|| self.lines.first.take().map(|l| (l, PointType::Stroke)))
            .or_else(|| self.lines.second.take().map(|l| (l, PointType::Stroke)))
    }
}
