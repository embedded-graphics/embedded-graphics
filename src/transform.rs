//! Transformations for graphics objects

use crate::geometry::Point;

/// Transform operations
pub trait Transform {
    /// Move the origin of an object by a given number of (x, y) pixels, returning a new object
    fn translate(&self, by: Point) -> Self;

    /// Move the origin of an object by a given number of (x, y) pixels, mutating the object
    /// in place
    fn translate_mut(&mut self, by: Point) -> &mut Self;
}
