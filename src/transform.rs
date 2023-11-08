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

impl<T, const N: usize> Transform for [T; N]
where
    T: Transform + Sized,
{
    fn translate(&self, by: Point) -> Self {
        core::array::from_fn(|i| self[i].translate(by))
    }

    fn translate_mut(&mut self, by: Point) -> &mut Self {
        for val in self.iter_mut() {
            val.translate_mut(by);
        }
        self
    }
}

macro_rules! tuple {
    ($a:ident,) => {
        #[doc = "This trait is implemented for tuples up to twelve items long."]
        impl<$a:Transform,> Transform for ($a,)
        {
            fn translate(&self, by: Point) -> Self {
                (self.0.translate(by),)
            }

            fn translate_mut(&mut self, by: Point) -> &mut Self {
                self.0.translate_mut(by);
                self
            }
        }
    };
    ($a:ident, $($rest:ident,)+) => {
        #[doc(hidden)]
        impl<$a:Transform, $($rest:Transform),+> Transform for ($a, $($rest,)+)
        {
            #[allow(non_snake_case)]
            fn translate(&self, by: Point) -> Self {
                let (a, $($rest),+) = self;
                (a.translate(by), $($rest.translate(by)),+)
            }

            #[allow(non_snake_case)]
            fn translate_mut(&mut self, by: Point) -> &mut Self {
                let (a, $($rest),+) = self;
                a.translate_mut(by);
                $($rest.translate_mut(by);)+

                self
            }
        }
        tuple! { $($rest,)+ }
    }
}

tuple!(L, K, J, I, H, G, F, E, D, C, B, A,);

#[cfg(test)]
mod tests {
    use embedded_graphics_core::prelude::Size;
    use embedded_graphics_core::primitives::Rectangle;

    use super::*;

    #[test]
    fn translate_slice() {
        let translate_dist = Point::new(5, 7);

        let rects = [
            Rectangle::new(Point::zero(), Size::new_equal(0)),
            Rectangle::new(Point::zero(), Size::new_equal(1)),
            Rectangle::new(Point::zero(), Size::new_equal(2)),
        ];

        let expected = [
            Rectangle::new(translate_dist, Size::new_equal(0)),
            Rectangle::new(translate_dist, Size::new_equal(1)),
            Rectangle::new(translate_dist, Size::new_equal(2)),
        ];

        assert_eq!(rects.translate(translate_dist), expected);
    }

    #[test]
    fn translate_tuple() {
        let translate_dist = Point::new(5, 7);

        let rects = (
            Rectangle::new(Point::zero(), Size::new_equal(0)),
            Rectangle::new(Point::zero(), Size::new_equal(1)),
            Rectangle::new(Point::zero(), Size::new_equal(2)),
        );

        let expected = (
            Rectangle::new(translate_dist, Size::new_equal(0)),
            Rectangle::new(translate_dist, Size::new_equal(1)),
            Rectangle::new(translate_dist, Size::new_equal(2)),
        );

        assert_eq!(rects.translate(translate_dist), expected);
    }
}
