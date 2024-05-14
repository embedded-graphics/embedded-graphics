//! Core primitives.

pub mod rectangle;

use crate::geometry::Point;
pub use rectangle::Rectangle;

/// Create an iterator over all points in the primitive.
pub trait PointsIter {
    /// Iterator over all points inside the primitive.
    type Iter: Iterator<Item = Point>;

    /// Returns an iterator over all points inside the primitive.
    fn points(&self) -> Self::Iter;
}

impl<T: PointsIter + ?Sized> PointsIter for &T {
    type Iter = T::Iter;

    fn points(&self) -> Self::Iter {
        (**self).points()
    }
}

impl<T: PointsIter + ?Sized> PointsIter for &mut T {
    type Iter = T::Iter;

    fn points(&self) -> Self::Iter {
        (**self).points()
    }
}
