use crate::primitives::Rectangle;

/// Styled dimensions.
pub trait StyledDimensions<S> {
    /// Returns the bounding box using the given style.
    fn styled_bounding_box(&self, style: &S) -> Rectangle;
}

impl<S, T, const N: usize> StyledDimensions<S> for [T; N]
where
    T: StyledDimensions<S>,
{
    fn styled_bounding_box(&self, style: &S) -> Rectangle {
        self.iter()
            .map(|t| t.styled_bounding_box(style))
            .reduce(|bb, t| bb.envelope(&t))
            .unwrap_or(Rectangle::zero())
    }
}

impl<S, T> StyledDimensions<S> for [T]
where
    T: StyledDimensions<S>,
{
    fn styled_bounding_box(&self, style: &S) -> Rectangle {
        self.iter()
            .map(|t| t.styled_bounding_box(style))
            .reduce(|bb, t| bb.envelope(&t))
            .unwrap_or(Rectangle::zero())
    }
}

macro_rules! tuple {
    ($a:ident,) => {
        #[doc = "This trait is implemented for tuples up to twelve items long."]
        impl<S, $a:StyledDimensions<S>> StyledDimensions<S> for ($a,)
        {
            fn styled_bounding_box(&self, style: &S) -> Rectangle {
                self.0.styled_bounding_box(style)
            }
        }
    };
    ($a:ident, $($rest:ident,)+) => {
        #[doc(hidden)]
        impl<S, $a:StyledDimensions<S>, $($rest:StyledDimensions<S>),+> StyledDimensions<S> for ($a, $($rest,)+)
        {
            #[allow(non_snake_case)]
            fn styled_bounding_box(&self, style: &S) -> Rectangle {
                let (a, $($rest),+) = self;
                let bb = a.styled_bounding_box(style);
                $(let bb = $rest.styled_bounding_box(style).envelope(&bb);)+
                bb
            }
        }
        tuple! { $($rest,)+ }
    }
}

tuple!(L, K, J, I, H, G, F, E, D, C, B, A,);
