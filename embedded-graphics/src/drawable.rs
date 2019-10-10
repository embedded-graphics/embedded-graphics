//! `Drawable` trait and helpers
use crate::geometry::Point;
use crate::pixelcolor::PixelColor;
use crate::DrawTarget;

/// A single pixel
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Pixel<C: PixelColor>(pub Point, pub C);

/// Marks an object as "drawable". Must be implemented for all graphics objects
///
/// The `Drawable` trait describes how a particular graphical object is drawn. For an object to be
/// drawable, it must implement `IntoIterator` for a mutable reference to itself; this is because
/// by default, a drawable object uses the [`draw_iter`][draw_iter] render method.
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::{egrectangle, text_6x8};
/// use embedded_graphics::geometry::Point;
/// use embedded_graphics::pixelcolor::{PixelColor, BinaryColor, Rgb888};
/// use embedded_graphics::primitives::rectangle::{Rectangle, RectangleIterator};
/// use embedded_graphics::fonts::Font6x8Conf;
/// use embedded_graphics::fonts::font_builder::FontBuilderIterator;
///
/// struct Button<'a, C: PixelColor> {
///     p1: Point,
///     p2: Point,
///     bg_color: C,
///     fg_color: C,
///     text: &'a str
/// }
///
/// impl<'a, C: 'a> IntoIterator for &'a mut Button<'a, C>
/// where
///     C: PixelColor + From<BinaryColor>,
/// {
///     type Item = Pixel<C>;
///     type IntoIter = core::iter::Chain<RectangleIterator<C>, FontBuilderIterator<'a, C, Font6x8Conf>>;
///
///     fn into_iter(self) -> Self::IntoIter {
///         egrectangle!(self.p1, self.p2, stroke_color=Some(self.bg_color)).into_iter()
///             .chain(text_6x8!(self.text, fill_color = Some(self.fg_color)).translate(Point::new(20, 16)).into_iter())
///     }
/// }
///
/// impl<'a, C: 'a> Drawable<'a, C> for Button<'a, C>
/// where
///     C: PixelColor + From<BinaryColor>,
/// { }
///
/// fn main() {
///     let mut button = Button {
///         p1: Point::new(0, 0),
///         p2: Point::new(10, 10),
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
/// [draw_iter]: ../trait.DrawTarget.html#method.draw_iter
pub trait Drawable<'a, C>
where
    C: PixelColor + 'a,
    Self: 'a,
    &'a mut Self: IntoIterator<Item = Pixel<C>>,
{
    /// Draw the graphics object. Override this method with primitive drawing methods as
    /// applicable.
    fn draw<T: DrawTarget<C>>(&'a mut self, display: &mut T) {
        display.draw_iter(self);
    }
}

impl<'a, C, T> Drawable<'a, C> for T
where
    C: PixelColor + 'a,
    T: 'a + Iterator<Item = Pixel<C>>,
{
}
