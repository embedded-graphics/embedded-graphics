//! Transformations for graphics objects

use super::drawable::Coord;

/// Transform operations
pub trait Transform {
    /// Move the origin of an object by a given number of (x, y) pixels
    fn translate(&self, by: Coord) -> Self;
}
