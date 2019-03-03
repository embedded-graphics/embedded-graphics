//! `Drawable` trait and helpers

use crate::coord::Coord;
use crate::pixelcolor::PixelColor;
use crate::unsignedcoord::UnsignedCoord;

/// A single pixel
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Pixel<C: PixelColor>(pub UnsignedCoord, pub C);

/// Marks an object as "drawable". Must be implemented for all graphics objects
pub trait Drawable {}

/// Adds the ability to get the dimensions/position of a graphics object
///
/// This **should** be implemented for all builtin embedded-graphics primitives and fonts. Third party
/// implementations do not have to implement this trait as an object may not have a known size. If
/// the object _does_ have a known size, this trait **should** be implemented.
pub trait Dimensions {
    /// Get the top left corner of the bounding box for an object
    fn top_left(&self) -> Coord;

    /// Get the bottom right corner of the bounding box for an object
    fn bottom_right(&self) -> Coord;

    /// Get the width and height for an object
    fn size(&self) -> UnsignedCoord;
}
