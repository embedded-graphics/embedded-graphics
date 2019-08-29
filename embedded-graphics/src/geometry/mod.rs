//! Geometry module.

mod point;
mod size;

pub use point::Point;
pub use size::Size;

/// Creates a new `Point`.
pub const fn point(x: i32, y: i32) -> Point {
    Point::new(x, y)
}

/// Creates a new `Size`.
pub const fn size(width: u32, height: u32) -> Size {
    Size::new(width, height)
}
