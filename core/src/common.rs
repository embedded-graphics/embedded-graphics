//! Common traits and error types.

use crate::{geometry::Point, pixelcolor::PixelColor};

/// Trait to set the color type.
pub trait ColorType {
    /// The color type.
    type Color: PixelColor;
}

/// Trait to get the color of a pixel.
pub trait GetPixel: ColorType {
    /// Gets the color of a pixel.
    ///
    /// Returns `None` if `point` is outside the bounding box.
    fn pixel(&self, point: Point) -> Option<Self::Color>;
}

/// Trait to set the color of a pixel.
pub trait SetPixel: ColorType {
    /// Tries to set the color of a pixel.
    ///
    /// Returns an error if the point is outside the bounding box.
    fn try_set_pixel(&mut self, point: Point, color: Self::Color) -> Result<(), OutOfBoundsError>;

    /// Sets the color of a pixel.
    ///
    /// Trying to set the color of a point outside the bounding box is a noop. Use [`Self::try_set_pixel`]
    /// if you need to detect if the point was out of bounds.
    #[inline]
    fn set_pixel(&mut self, point: Point, color: Self::Color) {
        self.try_set_pixel(point, color).ok();
    }
}

/// Out of bounds error.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct OutOfBoundsError;
