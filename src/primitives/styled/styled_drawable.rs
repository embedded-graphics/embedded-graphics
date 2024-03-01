use crate::{draw_target::DrawTarget, pixelcolor::PixelColor};

/// Styled drawable.
pub trait StyledDrawable<S> {
    /// Color type.
    type Color: PixelColor;
    /// Output type.
    type Output;

    /// Draws the primitive using the given style.
    fn draw_styled<D>(&self, style: &S, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>;
}

impl<S, T, const N: usize> StyledDrawable<S> for [T; N]
where
    T: StyledDrawable<S>,
{
    type Color = T::Color;
    type Output = [T::Output; N];

    // TODO: rewrite this funciton to use `slice::try_from_fn` once it is stabilized
    fn draw_styled<D>(&self, style: &S, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let mut error = None;

        let mut outputs: [_; N] = core::array::from_fn(|i| {
            if error.is_some() {
                None
            } else {
                match self[i].draw_styled(style, target) {
                    Ok(o) => Some(o),
                    Err(e) => {
                        error = Some(e);
                        None
                    }
                }
            }
        });

        if let Some(e) = error {
            Err(e)
        } else {
            Ok(core::array::from_fn(|i| outputs[i].take().unwrap()))
        }
    }
}

macro_rules! tuple {
    ($a:ident,) => {
        #[doc = "This trait is implemented for tuples up to twelve items long."]
        impl<S, $a> StyledDrawable<S> for ($a,)
        where
            $a: StyledDrawable<S>,
        {
            type Color = $a::Color;
            type Output = ($a::Output,);

            fn draw_styled<D>(&self, style: &S, target: &mut D) -> Result<Self::Output, D::Error>
            where
                D: DrawTarget<Color = Self::Color>,
            {
                Ok((
                    self.0.draw_styled(style, target)?,
                ))
            }
        }
    };
    ($a:ident, $($rest:ident,)+) => {
        #[doc(hidden)]
        impl<S, $a, $($rest,)+> StyledDrawable<S> for ($a, $($rest,)+)
        where
            $a: StyledDrawable<S>,
            $($rest: StyledDrawable<S, Color = $a::Color>,)+
        {
            type Color = $a::Color;
            type Output = ($a::Output, $($rest::Output,)+);

            #[allow(non_snake_case)]
            fn draw_styled<DIS>(&self, style: &S, target: &mut DIS) -> Result<Self::Output, DIS::Error>
            where
                DIS: DrawTarget<Color = Self::Color>,
            {
                let ($a, $($rest,)+) = self;
                Ok((
                    $a.draw_styled(style, target)?,
                    $($rest.draw_styled(style, target)?,)+
                ))
            }
        }
        tuple! { $($rest,)+ }
    }
}

tuple!(L, K, J, I, H, G, F, E, D, C, B, A,);
