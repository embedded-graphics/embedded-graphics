//! Triangle scanline intersections iterator.

use crate::{
    geometry::Point,
    primitives::{
        common::{LineJoin, PointType, Scanline, StrokeOffset, ThickSegment},
        Triangle,
    },
};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
struct LineConfig {
    first: Scanline,
    second: Scanline,
    internal: Scanline,
    internal_type: PointType,
}

/// Triangle scanline intersections iterator.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct ScanlineIntersections {
    lines: LineConfig,
    triangle: Triangle,
    stroke_width: u32,
    stroke_offset: StrokeOffset,
    has_fill: bool,
    is_collapsed: bool,
}

impl ScanlineIntersections {
    /// Create a new thick segments iterator.
    pub fn new(
        triangle: &Triangle,
        stroke_width: u32,
        stroke_offset: StrokeOffset,
        has_fill: bool,
        scanline_y: i32,
    ) -> Self {
        // Special case: If thick strokes completely fill the triangle interior and the stroke is
        // inside the triangle, the normal triangle shape can be used to detect the intersection,
        // with the line type being marked as Border so, when rendered, the correct color is used.
        let is_collapsed = triangle.is_collapsed(stroke_width, stroke_offset)
            && stroke_offset == StrokeOffset::Right;

        let mut self_ = Self {
            has_fill,
            triangle: *triangle,
            stroke_offset,
            stroke_width,
            is_collapsed,
            ..Self::empty()
        };

        self_.reset_with_new_scanline(scanline_y);

        self_
    }

    /// Empty.
    pub const fn empty() -> Self {
        Self {
            lines: LineConfig {
                first: Scanline::new_empty(0),
                second: Scanline::new_empty(0),
                internal: Scanline::new_empty(0),
                internal_type: PointType::Fill,
            },
            has_fill: false,
            triangle: Triangle::new(Point::zero(), Point::zero(), Point::zero()),
            stroke_width: 0,
            stroke_offset: StrokeOffset::None,
            is_collapsed: false,
        }
    }

    /// Reset with a new scanline.
    pub fn reset_with_new_scanline(&mut self, scanline_y: i32) {
        if let Some(lines) = self.generate_lines(scanline_y) {
            self.lines = lines
        }
    }

    fn edge_intersections(&self, scanline_y: i32) -> impl Iterator<Item = Scanline> + '_ {
        let mut idx = 0;
        let mut left = Scanline::new_empty(scanline_y);
        let mut right = Scanline::new_empty(scanline_y);

        core::iter::from_fn(move || {
            if self.stroke_width == 0 {
                return None;
            }

            while idx < 3 {
                let start = LineJoin::from_points(
                    self.triangle.vertices[idx % 3],
                    self.triangle.vertices[(idx + 1) % 3],
                    self.triangle.vertices[(idx + 2) % 3],
                    self.stroke_width,
                    self.stroke_offset,
                );
                let end = LineJoin::from_points(
                    self.triangle.vertices[(idx + 1) % 3],
                    self.triangle.vertices[(idx + 2) % 3],
                    self.triangle.vertices[(idx + 3) % 3],
                    self.stroke_width,
                    self.stroke_offset,
                );

                idx += 1;

                let scanline = ThickSegment::new(start, end).intersection(scanline_y);

                if !left.is_empty() {
                    if left.try_extend(&scanline) {
                        continue;
                    }
                } else {
                    left = scanline;
                    continue;
                }

                if !right.is_empty() {
                    right.try_extend(&scanline);
                } else {
                    right = scanline;
                }
            }

            // Merge any overlap between final left/right results
            if left.try_extend(&right) {
                right = Scanline::new_empty(scanline_y);
            }

            left.try_take().or_else(|| right.try_take())
        })
    }

    fn generate_lines(&self, scanline_y: i32) -> Option<LineConfig> {
        let mut edge_intersections = self.edge_intersections(scanline_y);

        if self.is_collapsed {
            Some(LineConfig {
                internal: self.triangle.scanline_intersection(scanline_y),
                internal_type: PointType::Stroke,
                first: Scanline::new_empty(0),
                second: Scanline::new_empty(0),
            })
        } else {
            let first = edge_intersections.next();

            // For scanlines that are parallel with and are inside one edge, this should be None.
            let second = edge_intersections.next();

            // If there are two intersections, this must mean we've traversed across the center of the
            // triangle (assuming the edge line merging logic is correct). In this case, we need a
            // scanline between the two edge intersections.
            let internal = if self.has_fill {
                match (&first, &second) {
                    // Triangle stroke is non-zero, so the fill line is between the insides of each
                    // stroke.
                    (Some(first), Some(second)) => {
                        let start_x = first.x.end.min(second.x.end);
                        let end_x = first.x.start.max(second.x.start);

                        Scanline {
                            x: start_x..end_x,
                            y: scanline_y,
                        }
                    }
                    // Triangles with no stroke intersections and a fill color.
                    (None, None) => self.triangle.scanline_intersection(scanline_y),
                    // Because a triangle is a closed shape, a single intersection here likely means
                    // we're inside one of the borders, so no fill should be returned for this
                    // scanline.
                    _ => Scanline::new_empty(scanline_y),
                }
            } else {
                Scanline::new_empty(scanline_y)
            };

            Some(LineConfig {
                first: first.unwrap_or(Scanline::new_empty(scanline_y)),
                second: second.unwrap_or(Scanline::new_empty(scanline_y)),
                internal,
                internal_type: PointType::Fill,
            })
        }
    }
}

impl Iterator for ScanlineIntersections {
    type Item = (Scanline, PointType);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(internal) = self.lines.internal.try_take() {
            Some((internal, self.lines.internal_type))
        } else if let Some(first) = self.lines.first.try_take() {
            Some((first, PointType::Stroke))
        } else if let Some(second) = self.lines.second.try_take() {
            Some((second, PointType::Stroke))
        } else {
            None
        }
    }
}
