//! `Drawable` trait and helpers
use crate::geometry::Point;
use crate::pixelcolor::PixelColor;
use crate::DrawTarget;

/// A single pixel
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Pixel<C: PixelColor>(pub Point, pub C);

/// Marks an object as "drawable". Must be implemented for all graphics objects
///
/// The `Drawable` trait describes how a particular graphical object is drawn. A `Drawable` object
/// can define its `draw` method as a collection of graphical primitives or as an iterator
/// over pixels being rendered with [`DrawTarget`]'s [`draw_iter`] method.
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::{egrectangle, text_6x8};
/// use embedded_graphics::geometry::Point;
/// use embedded_graphics::pixelcolor::{PixelColor, BinaryColor, Rgb888};
///
/// struct Button<'a, C: PixelColor> {
///     top_left: Point,
///     bottom_right: Point,
///     bg_color: C,
///     fg_color: C,
///     text: &'a str
/// }
///
/// impl<'a, C: 'a> Drawable<C> for &Button<'a, C>
/// where
///     C: PixelColor + From<BinaryColor>,
/// {
///     fn draw<D: DrawTarget<C>>(self, display: &mut D) {
///         egrectangle!(self.top_left, self.bottom_right, fill_color = Some(self.bg_color)).draw(display);
///         text_6x8!(self.text, stroke_color = Some(self.fg_color))
///             .translate(Point::new(20, 20))
///             .draw(display);
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
///
/// ```
///
/// [`DrawTarget`]: ../trait.DrawTarget.html
/// [`draw_iter`]: ../trait.DrawTarget.html#method.draw_iter
pub trait Drawable<C>
where
    C: PixelColor,
{
    /// Draw the graphics object using the supplied DrawTarget.
    fn draw<T: DrawTarget<C>>(self, display: &mut T);
}

impl<C, T> Drawable<C> for &mut T
where
    C: PixelColor,
    T: Iterator<Item = Pixel<C>>,
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) {
        display.draw_iter(self);
    }
}
