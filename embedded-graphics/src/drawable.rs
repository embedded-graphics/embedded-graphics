//! `Drawable` trait and helpers

use crate::geometry::Point;
use crate::pixelcolor::PixelColor;

/// A single pixel
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Pixel<C: PixelColor>(pub Point, pub C);

/// Marks an object as "drawable". Must be implemented for all graphics objects
pub trait Drawable {}
