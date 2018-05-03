//! `Drawable` trait and helpers

/// 2D coordinate type
pub type Coord = (u32, u32);

// TODO: Refactor to use both with monochrome and multicolour displays
/// Monochrome colour type
pub type Color = u8;

/// A single pixel
pub type Pixel = (Coord, Color);

/// Marks an object as "drawable". Must be implemented for all graphics objects
pub trait Drawable {}
