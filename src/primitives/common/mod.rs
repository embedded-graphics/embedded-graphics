mod closed_thick_segment_iter;
mod distance_iterator;
mod line_join;
mod linear_equation;
mod plane_sector;
mod scanline;
mod styled_scanline;
mod thick_segment;
mod thick_segment_iter;

pub use closed_thick_segment_iter::ClosedThickSegmentIter;
pub use distance_iterator::DistanceIterator;
pub use line_join::{JoinKind, LineJoin};
pub use linear_equation::{LinearEquation, OriginLinearEquation, NORMAL_VECTOR_SCALE};
pub use plane_sector::PlaneSector;
pub use scanline::Scanline;
pub use styled_scanline::StyledScanline;
pub use thick_segment::ThickSegment;
pub use thick_segment_iter::ThickSegmentIter;

use crate::primitives::StrokeAlignment;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
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

/// Which side of the center line to draw on.
///
/// Imagine standing on `start`, looking ahead to where `end` is. `Left` is to your left, `Right` to
/// your right.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub enum LineSide {
    /// Left side of the line
    Left,

    /// Right side of the line
    Right,
}

impl LineSide {
    /// Swap side.
    pub const fn swap(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

/// Point type.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub enum PointType {
    /// Represents part of the stroke.
    Stroke,

    /// Represents the interior of the shape.
    Fill,
}
