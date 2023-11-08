/// Styled drawable.
pub trait StyledPixels<S> {
    /// Iterator type.
    type Iter: Iterator;

    /// Returns an iterator over all pixels in this styled primitive.
    fn pixels(&self, style: &S) -> Self::Iter;
}

impl<S, T: StyledPixels<S>> StyledPixels<S> for &T {
    type Iter = T::Iter;

    fn pixels(&self, style: &S) -> Self::Iter {
        (**self).pixels(style)
    }
}

impl<S, T: StyledPixels<S>> StyledPixels<S> for &mut T {
    type Iter = T::Iter;

    fn pixels(&self, style: &S) -> Self::Iter {
        (**self).pixels(style)
    }
}

impl<S, T: StyledPixels<S>, const N: usize> StyledPixels<S> for [T; N] {
    type Iter = core::array::IntoIter<T::Iter, N>;

    fn pixels(&self, style: &S) -> Self::Iter {
        core::array::from_fn(|i| self[i].pixels(style)).into_iter()
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
    ($style:ident, $a:expr, $b:expr,) => {
        $a.pixels($style).chain($b.pixels($style))
    };
    ($style:ident, $a:expr, $($rest:expr,)+) => {
        $a.pixels($style).chain(chain_impl!($style, $($rest,)+))
    }
}

macro_rules! tuple {
    ($a:ident,) => {
        #[doc = "This trait is implemented for tuples up to twelve items long."]
        impl<S, $a> StyledPixels<S> for ($a,)
        where
            $a: StyledPixels<S>,
        {
            type Iter = $a::Iter;

            fn pixels(&self, style: &S) -> Self::Iter {
                self.0.pixels(style)
            }
        }
    };
    ($a:ident, $($rest:ident,)+) => {
        #[doc(hidden)]
        impl<S, $a, $($rest,)+> StyledPixels<S> for ($a, $($rest,)+)
        where
            $a: StyledPixels<S>,
            $($rest: StyledPixels<S>,
            <$rest as StyledPixels<S>>::Iter: Iterator<Item = <<$a as StyledPixels<S>>::Iter as Iterator>::Item>,)+
        {
            type Iter = chain_type!($a, $($rest,)+);

            #[allow(non_snake_case)]
            fn pixels(&self, style: &S) -> Self::Iter {
                let ($a, $($rest),+) = self;
                chain_impl!(style, $a, $($rest,)+)
            }
        }
        tuple! { $($rest,)+ }
    }
}

tuple!(L, K, J, I, H, G, F, E, D, C, B, A,);
