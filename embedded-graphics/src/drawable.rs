//! `Drawable` trait and helpers
use crate::{geometry::Point, pixelcolor::PixelColor, DrawTarget};

/// Marks an object as "drawable". Must be implemented for all graphics objects
///
/// The `Drawable` trait describes how a particular graphical object is drawn. A `Drawable` object
/// can define its `draw` method as a collection of graphical primitives or as an iterator
/// over pixels being rendered with [`DrawTarget`]'s [`draw_iter`] method.
///
/// ```rust
/// use embedded_graphics::{
///     egrectangle, egtext,
///     fonts::Font6x8,
///     geometry::Point,
///     pixelcolor::{BinaryColor, PixelColor, Rgb888},
///     prelude::*,
///     primitive_style, text_style,
/// };
///
/// struct Button<'a, C: PixelColor> {
///     top_left: Point,
///     bottom_right: Point,
///     bg_color: C,
///     fg_color: C,
///     text: &'a str,
/// }
///
/// impl<'a, C: 'a> Drawable<C> for &Button<'a, C>
/// where
///     C: PixelColor + From<BinaryColor>,
/// {
///     fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
///         egrectangle!(
///             top_left = self.top_left,
///             bottom_right = self.bottom_right,
///             style = primitive_style!(fill_color = self.bg_color)
///         )
///         .draw(display);
///         egtext!(
///             text = self.text,
///             top_left = (20, 20),
///             style = text_style!(font = Font6x8, text_color = self.fg_color)
///         )
///         .draw(display)
///     }
/// }
///
/// fn main() {
///     let mut button = Button {
///         top_left: Point::zero(),
///         bottom_right: Point::new(100, 50),
///         bg_color: Rgb888::RED,
///         fg_color: Rgb888::BLUE,
///         text: "Click me!",
///     };
///     # use embedded_graphics::mock_display::MockDisplay;
///     # let mut display = MockDisplay::default();
///     button.draw(&mut display);
/// }
/// ```
///
/// [`DrawTarget`]: ../trait.DrawTarget.html
/// [`draw_iter`]: ../trait.DrawTarget.html#method.draw_iter
pub trait Drawable<C>
where
    C: PixelColor,
{
    /// Draw the graphics object using the supplied DrawTarget.
    fn draw<T: DrawTarget<C>>(self, display: &mut T) -> Result<(), T::Error>;
}

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
/// Pixel(Point::new(1, 2), BinaryColor::On).draw(&mut display);
/// ```
///
/// Iterators with `Pixel` items can also be drawn:
///
/// ```
/// use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::new();
///
/// (0..100)
///     .map(|i| Pixel(Point::new(i, i * 2), BinaryColor::On))
///     .draw(&mut display);
/// ```
///
/// [`Drawable`]: trait.Drawable.html
/// [`DrawTarget`]: ../trait.DrawTarget.html
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Pixel<C: PixelColor>(pub Point, pub C);

impl<C> Drawable<C> for Pixel<C>
where
    C: PixelColor,
{
    fn draw<T: DrawTarget<C>>(self, display: &mut T) -> Result<(), T::Error> {
        display.draw_pixel(self)
    }
}

impl<C, T> Drawable<C> for &mut T
where
    C: PixelColor,
    T: Iterator<Item = Pixel<C>>,
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        display.draw_iter(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{mock_display::MockDisplay, pixelcolor::BinaryColor};

    #[test]
    fn draw_pixel() -> Result<(), core::convert::Infallible> {
        let mut display = MockDisplay::new();
        Pixel(Point::new(0, 0), BinaryColor::On).draw(&mut display)?;
        Pixel(Point::new(2, 1), BinaryColor::On).draw(&mut display)?;
        Pixel(Point::new(1, 2), BinaryColor::On).draw(&mut display)?;

        #[rustfmt::skip]
        assert_eq!(
            MockDisplay::from_pattern(&[
                "#  ",
                "  #",
                " # ",
            ]),
            display
        );

        Ok(())
    }
}
