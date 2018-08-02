//! `Drawable` trait and helpers

use super::unsignedcoord::UnsignedCoord;

/// A single pixel
#[derive(Copy, Clone, Debug)]
pub struct Pixel<C: PixelColor + Clone>(pub UnsignedCoord, pub C);

/// Color trait
pub trait PixelColor {}

/// Marks an object as "drawable". Must be implemented for all graphics objects
pub trait Drawable {}
