//! `Drawable` trait and helpers

use pixelcolor::PixelColor;
use unsignedcoord::UnsignedCoord;

/// A single pixel
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Pixel<C: PixelColor>(pub UnsignedCoord, pub C);

/// Marks an object as "drawable". Must be implemented for all graphics objects
pub trait Drawable {}
