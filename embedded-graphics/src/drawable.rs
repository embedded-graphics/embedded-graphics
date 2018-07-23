//! `Drawable` trait and helpers

use super::coord::Coord;

// TODO: Refactor to use both with monochrome and multicolour displays
/// Monochrome colour type
pub type Color = u8;

/// A single pixel
pub type Pixel = (Coord, Color);

/// Marks an object as "drawable". Must be implemented for all graphics objects
pub trait Drawable {}
