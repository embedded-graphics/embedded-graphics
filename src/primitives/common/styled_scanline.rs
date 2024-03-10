use core::ops::Range;

use crate::{draw_target::DrawTarget, primitives::common::Scanline};

/// Scanline with stroke and fill regions.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct StyledScanline {
    y: i32,
    stroke_range: Range<i32>,
    fill_range: Range<i32>,
}

impl StyledScanline {
    /// Creates a new styled scanline.
    pub fn new(y: i32, stroke_range: Range<i32>, fill_range: Option<Range<i32>>) -> Self {
        let fill_range = fill_range.unwrap_or_else(|| stroke_range.end..stroke_range.end);

        Self {
            y,
            stroke_range,
            fill_range,
        }
    }

    /// Returns the stroke region on the left side.
    ///
    /// If the scanline contains no fill region the entire scanline will be returned.
    pub const fn stroke_left(&self) -> Scanline {
        Scanline::new(self.y, self.stroke_range.start..self.fill_range.start)
    }

    /// Returns the stroke region on the right side.
    ///
    /// If the scanline contains no fill region an empty scanline will be returned.
    pub const fn stroke_right(&self) -> Scanline {
        Scanline::new(self.y, self.fill_range.end..self.stroke_range.end)
    }

    /// Returns the fill region.
    pub fn fill(&self) -> Scanline {
        Scanline::new(self.y, self.fill_range.clone())
    }

    /// Draws the stroke regions.
    pub fn draw_stroke<T: DrawTarget>(
        &self,
        target: &mut T,
        stroke_color: T::Color,
    ) -> Result<(), T::Error> {
        self.stroke_left().draw(target, stroke_color)?;
        self.stroke_right().draw(target, stroke_color)
    }

    /// Draws the stroke and fill regions.
    pub fn draw_stroke_and_fill<T: DrawTarget>(
        &self,
        target: &mut T,
        stroke_color: T::Color,
        fill_color: T::Color,
    ) -> Result<(), T::Error> {
        self.stroke_left().draw(target, stroke_color)?;
        self.fill().draw(target, fill_color)?;
        self.stroke_right().draw(target, stroke_color)
    }
}

#[cfg(feature = "async_draw")]
use crate::draw_target::AsyncDrawTarget;

#[cfg(feature = "async_draw")]
impl StyledScanline {
    /// Draws the stroke regions.
    pub async fn draw_stroke_async<T: AsyncDrawTarget>(
        &self,
        target: &mut T,
        stroke_color: T::Color,
    ) -> Result<(), T::Error> {
        self.stroke_left().draw_async(target, stroke_color).await?;
        self.stroke_right().draw_async(target, stroke_color).await
    }

    /// Draws the stroke and fill regions.
    pub async fn draw_stroke_and_fill_async<T: AsyncDrawTarget>(
        &self,
        target: &mut T,
        stroke_color: T::Color,
        fill_color: T::Color,
    ) -> Result<(), T::Error> {
        self.stroke_left().draw_async(target, stroke_color).await?;
        self.fill().draw_async(target, fill_color).await?;
        self.stroke_right().draw_async(target, stroke_color).await
    }
}
