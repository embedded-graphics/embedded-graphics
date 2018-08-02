//! `Drawable` trait and helpers

use super::unsignedcoord::UnsignedCoord;
use super::color::Color;

/// A single pixel
pub type Pixel<C> = (UnsignedCoord, Color<C>);

/// Marks an object as "drawable". Must be implemented for all graphics objects
pub trait Drawable {}
