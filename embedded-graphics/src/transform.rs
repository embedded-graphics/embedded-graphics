//! Transformations for graphics objects

use crate::coord::Coord;

/// Transform operations
pub trait Transform {
    /// Move the origin of an object by a given number of (x, y) pixels, returning a new object
    fn translate(&self, by: Coord) -> Self;

    /// Move the origin of an object by a given number of (x, y) pixels, mutating the object
    /// in place
    fn translate_mut(&mut self, by: Coord) -> &mut Self;
}
