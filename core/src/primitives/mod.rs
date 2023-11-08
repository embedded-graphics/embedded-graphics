//! Core primitives.
//!
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

impl<T: PointsIter> PointsIter for &T {
    type Iter = T::Iter;

    fn points(&self) -> Self::Iter {
        (**self).points()
    }
}

impl<T: PointsIter> PointsIter for &mut T {
    type Iter = T::Iter;

    fn points(&self) -> Self::Iter {
        (**self).points()
    }
}

impl<T: PointsIter, const N: usize> PointsIter for [T; N] {
    type Iter = core::iter::Flatten<core::array::IntoIter<T::Iter, N>>;

    fn points(&self) -> Self::Iter {
        core::array::from_fn(|i| self[i].points())
            .into_iter()
            .flatten()
    }
}

macro_rules! chain_type {
    ($a:ident, $b:ident,) => {
        core::iter::Chain<$a::Iter, $b::Iter>
    };
    ($a:ident, $($rest:ident,)+) => {
        core::iter::Chain<$a::Iter, chain_type!($($rest,)+)>
    }
}

macro_rules! chain_impl {
    ($a:expr, $b:expr,) => {
        $a.points().chain($b.points())
    };
    ($a:expr, $($rest:expr,)+) => {
        $a.points().chain(chain_impl!($($rest,)+))
    }
}

macro_rules! tuple {
    ($a:ident,) => {
        #[doc = "This trait is implemented for tuples up to twelve items long."]
        impl<$a:PointsIter> PointsIter for ($a,)
        {
            type Iter = $a::Iter;

            fn points(&self) -> Self::Iter {
                self.0.points()
            }
        }
    };
    ($a:ident, $($rest:ident,)+) => {
        #[doc(hidden)]
        impl<$a:PointsIter, $($rest:PointsIter),+> PointsIter for ($a, $($rest,)+)
        {
            type Iter = chain_type!($a, $($rest,)+);

            #[allow(non_snake_case)]
            fn points(&self) -> Self::Iter {
                let ($a, $($rest),+) = self;
                chain_impl!($a, $($rest,)+)
            }
        }
        tuple! { $($rest,)+ }
    }
}

tuple!(L, K, J, I, H, G, F, E, D, C, B, A,);
