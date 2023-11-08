//! `Drawable` trait and helpers
use crate::{draw_target::DrawTarget, geometry::Point, pixelcolor::PixelColor};

/// A single pixel.
///
/// `Pixel` objects are used to specify the position and color of drawn pixels.
///
/// # Examples
///
/// The [`Drawable`] trait is implemented for `Pixel` which allows single pixels
/// to be drawn to a [`DrawTarget`]:
/// ```
/// use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::new();
///
/// Pixel(Point::new(1, 2), BinaryColor::On).draw(&mut display)?;
/// # Ok::<(), core::convert::Infallible>(())
/// ```
///
/// Iterators with `Pixel` items can also be drawn:
///
/// ```
/// use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::new();
///
/// (0..32)
///     .map(|i| Pixel(Point::new(i, i * 2), BinaryColor::On))
///     .draw(&mut display)?;
/// # Ok::<(), core::convert::Infallible>(())
/// ```
///
/// [`DrawTarget`]: crate::draw_target::DrawTarget
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct Pixel<C>(pub Point, pub C)
where
    C: PixelColor;

impl<C> Drawable for Pixel<C>
where
    C: PixelColor,
{
    type Color = C;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        target.draw_iter(core::iter::once(*self))
    }
}

/// Marks an object as "drawable". Must be implemented for all graphics objects
///
/// The `Drawable` trait describes how a particular graphical object is drawn. A `Drawable` object
/// can define its `draw` method as a collection of graphical primitives or as an iterator
/// over pixels being rendered with [`DrawTarget`]'s [`draw_iter`] method.
///
/// ```rust
/// use embedded_graphics::{
///     mono_font::{ascii::FONT_6X9, MonoTextStyle},
///     pixelcolor::{BinaryColor, PixelColor, Rgb888},
///     prelude::*,
///     primitives::{Rectangle, PrimitiveStyle},
///     text::Text,
/// };
///
/// struct Button<'a, C: PixelColor> {
///     top_left: Point,
///     size: Size,
///     bg_color: C,
///     fg_color: C,
///     text: &'a str,
/// }
///
/// impl<C> Drawable for Button<'_, C>
/// where
///     C: PixelColor + From<BinaryColor>,
/// {
///     type Color = C;
///     type Output = ();
///
///     fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
///     where
///         D: DrawTarget<Color = C>,
///     {
///         Rectangle::new(self.top_left, self.size)
///             .into_styled(PrimitiveStyle::with_fill(self.bg_color))
///             .draw(target)?;
///
///         let style = MonoTextStyle::new(&FONT_6X9, self.fg_color);
///
///         Text::new(self.text, Point::new(6, 13), style).draw(target)?;
///
///         Ok(())
///     }
/// }
///
/// let mut button = Button {
///     top_left: Point::zero(),
///     size: Size::new(60, 20),
///     bg_color: Rgb888::RED,
///     fg_color: Rgb888::BLUE,
///     text: "Click me!",
/// };
///
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::default();
/// # display.set_allow_overdraw(true);
/// button.draw(&mut display)?;
/// # Ok::<(), core::convert::Infallible>(())
/// ```
///
/// [`DrawTarget`]: crate::draw_target::DrawTarget
/// [`draw_iter`]: crate::draw_target::DrawTarget::draw_iter
pub trait Drawable {
    /// The pixel color type.
    type Color: PixelColor;

    /// The return type of the `draw` method.
    ///
    /// The `Output` type can be used to return results and values produced from the drawing of the
    /// current item. For example, rendering two differently styled text items next to each other
    /// can make use of a returned value, allowing the next text to be positioned after the first:
    ///
    /// ```
    /// use embedded_graphics::{
    ///     mono_font::{
    ///         ascii::{FONT_10X20, FONT_6X10},
    ///         MonoTextStyle,
    ///     },
    ///     pixelcolor::BinaryColor,
    ///     prelude::*,
    ///     text::Text,
    /// };
    ///
    /// # let mut display = embedded_graphics::mock_display::MockDisplay::new();
    /// # display.set_allow_out_of_bounds_drawing(true);
    /// let label_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    /// let value_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    ///
    /// let next_point = Text::new("Label ", Point::new(10, 20), label_style)
    ///     .draw(&mut display)?;
    ///
    /// Text::new("1234", next_point, value_style).draw(&mut display)?;
    /// # Ok::<(), core::convert::Infallible>(())
    /// ```
    ///
    /// Use `()` if no value should be returned.
    type Output;

    /// Draw the graphics object using the supplied DrawTarget.
    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>;
}

impl<T: Drawable> Drawable for &T {
    type Color = T::Color;
    type Output = T::Output;

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        (**self).draw(target)
    }
}

impl<T: Drawable> Drawable for &mut T {
    type Color = T::Color;
    type Output = T::Output;

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        (**self).draw(target)
    }
}

impl<T: Drawable, const N: usize> Drawable for [T; N] {
    type Color = T::Color;
    type Output = [T::Output; N];

    // TODO: rewrite this funciton to use `slice::try_from_fn` once it is stabilized
    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let mut error = None;

        let mut outputs: [_; N] = core::array::from_fn(|i| {
            if error.is_some() {
                None
            } else {
                match self[i].draw(target) {
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
        impl<$a> Drawable for ($a,)
        where
            $a: Drawable,
        {
            type Color = $a::Color;
            type Output = ($a::Output,);

            fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
            where
                D: DrawTarget<Color = Self::Color>,
            {
                Ok((
                    self.0.draw(target)?,
                ))
            }
        }
    };
    ($a:ident, $($rest:ident,)+) => {
        #[doc(hidden)]
        impl<$a, $($rest,)+> Drawable for ($a, $($rest,)+)
        where
            $a: Drawable,
            $($rest: Drawable<Color = $a::Color>,)+
        {
            type Color = $a::Color;
            type Output = ($a::Output, $($rest::Output,)+);

            #[allow(non_snake_case)]
            fn draw<DIS>(&self, target: &mut DIS) -> Result<Self::Output, DIS::Error>
            where
                DIS: DrawTarget<Color = Self::Color>,
            {
                let ($a, $($rest,)+) = self;
                Ok((
                    $a.draw(target)?,
                    $($rest.draw(target)?,)+
                ))
            }
        }
        tuple! { $($rest,)+ }
    }
}

tuple!(L, K, J, I, H, G, F, E, D, C, B, A,);

#[cfg(test)]
mod tests {
    // NOTE: `crate` cannot be used here due to circular dependency resolution behavior.
    use embedded_graphics::{
        geometry::Point, mock_display::MockDisplay, pixelcolor::BinaryColor, Drawable, Pixel,
    };

    #[test]
    fn draw_pixel() {
        let mut display = MockDisplay::new();
        Pixel(Point::new(0, 0), BinaryColor::On)
            .draw(&mut display)
            .unwrap();
        Pixel(Point::new(2, 1), BinaryColor::On)
            .draw(&mut display)
            .unwrap();
        Pixel(Point::new(1, 2), BinaryColor::On)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "#  ", //
            "  #", //
            " # ", //
        ]);
    }
}
