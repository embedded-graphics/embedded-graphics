//! Common traits and error types.

use crate::{geometry::Point, pixelcolor::PixelColor};

/// Trait to get the color of a pixel.
pub trait GetPixel<C: PixelColor> {
    /// Gets the color of a pixel.
    ///
    /// Returns `None` if `point` is outside the bounding box.
    fn pixel(&self, point: Point) -> Option<C>;
}

/// Trait to set the color of a pixel.
pub trait SetPixel<C: PixelColor> {
    /// Tries to set the color of a pixel.
    ///
    /// Returns an error if the point is outside the bounding box.
    fn try_set_pixel(&mut self, point: Point, color: C) -> Result<(), OutOfBoundsError>;

    /// Sets the color of a pixel.
    ///
    /// Trying to set the color of a point outside the bounding box is a noop. Use [`Self::try_set_pixel`]
    /// if you need to detect if the point was out of bounds.
    #[inline]
    fn set_pixel(&mut self, point: Point, color: C) {
        self.try_set_pixel(point, color).ok();
    }
}

/// Out of bounds error.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct OutOfBoundsError;
