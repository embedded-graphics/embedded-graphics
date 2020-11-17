mod closed_thick_segment_iter;
mod line_join;
mod linear_equation;
mod plane_sector;
mod thick_segment;
mod thick_segment_iter;

pub use closed_thick_segment_iter::ClosedThickSegmentIter;
pub use line_join::{JoinKind, LineJoin};
pub use linear_equation::{LineSide, LinearEquation};
pub use plane_sector::{PlaneSector, PlaneSectorIterator};
pub use thick_segment::{bresenham_scanline_intersection, ThickSegment};
pub use thick_segment_iter::ThickSegmentIter;

use crate::style::StrokeAlignment;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum StrokeOffset {
    /// Stroke is centered around the line skeleton.
    None,

    /// Stroke is offset to the left of the line.
    Left,

    /// Stroke is offset to the right of the line.
    Right,
}

impl From<StrokeAlignment> for StrokeOffset {
    fn from(alignment: StrokeAlignment) -> Self {
        match alignment {
            StrokeAlignment::Inside => Self::Right,
            StrokeAlignment::Outside => Self::Left,
            StrokeAlignment::Center => Self::None,
        }
    }
}
