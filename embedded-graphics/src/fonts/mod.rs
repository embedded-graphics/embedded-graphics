//! Pixel based fonts

mod font12x16;
mod font6x12;
mod font8x16;
mod font6x8;

pub use self::font12x16::Font12x16;
pub use self::font6x12::Font6x12;
pub use self::font8x16::Font8x16;
pub use self::font6x8::Font6x8;

use color::Color;

/// Common methods for all fonts
pub trait Font<'a> {
    /// Data type to store color
    type C : Clone + Copy + PartialEq;

    /// Render a string in the implementing font's typeface.
    ///
    /// ```rust
    /// # use embedded_graphics::fonts::{Font, Font6x8};
    /// # use embedded_graphics::transform::Transform;
    /// # use embedded_graphics::drawable::Pixel;
    /// # use embedded_graphics::color::Color;
    /// #
    /// #
    /// # struct Display {}
    /// # impl Display {
    /// #     pub fn draw<T, C>(&self, item_pixels: T) -> Result<(), ()>
    /// #     where
    /// #         T: Iterator<Item = Pixel<C>>,
    /// #         C: Clone + Copy + PartialEq,
    /// #     {
    /// #         Ok(())
    /// #     }
    /// # }
    ///
    /// fn main() {
    ///     let disp = Display {};
    ///     // Render a string with a 8bit color
    ///     let text = Font6x8::render_str("Hello world", Color::new(1));  
    ///
    ///     disp.draw(text.into_iter());
    /// }
    /// ```
    fn render_str(chars: &'a str, color: Color<Self::C>) -> Self;
}
