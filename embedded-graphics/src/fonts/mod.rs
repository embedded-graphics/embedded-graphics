//! Pixel based fonts

mod font12x16;
mod font6x12;
mod font6x8;
mod font8x16;

pub use self::font12x16::Font12x16;
pub use self::font6x12::Font6x12;
pub use self::font6x8::Font6x8;
pub use self::font8x16::Font8x16;
use pixelcolor::PixelColor;

/// Common methods for all fonts
pub trait Font<'a, C>
where
    C: PixelColor,
{
    /// Render a string in the implementing font's typeface.
    ///
    /// ```rust
    /// # use embedded_graphics::fonts::{Font, Font6x8};
    /// # use embedded_graphics::transform::Transform;
    /// # use embedded_graphics::drawable::Pixel;
    /// # use embedded_graphics::pixelcolor::PixelColorU8;
    /// #
    /// # struct Display {}
    /// # impl Display {
    /// #     pub fn draw<T>(&self, item_pixels: T) -> Result<(), ()>
    /// #     where
    /// #         T: Iterator<Item = Pixel<PixelColorU8>>,
    /// #     {
    /// #         Ok(())
    /// #     }
    /// # }
    /// #
    /// fn main() {
    ///     let disp: Display = Display {};
    ///     // Render a string with a 8bit color
    ///     let text = Font6x8::render_str("Hello world", 1u8.into());
    ///
    ///     disp.draw(text.into_iter());
    /// }
    /// ```
    fn render_str(chars: &'a str, color: C) -> Self;
}
