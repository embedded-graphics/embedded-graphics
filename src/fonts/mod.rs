//! Pixel based fonts

mod font6x8;

pub use self::font6x8::Font6x8;

// TODO: Add to crate prelude
/// Common methods for all fonts
pub trait Font<'a> {
    /// Render a string in the implementing font's typeface.
    ///
    /// ```rust
    /// # use embedded_graphics::fonts::{Font, Font6x8};
    /// # use embedded_graphics::transform::Transform;
    /// # use embedded_graphics::drawable::Pixel;
    /// #
    /// # struct Display {}
    /// # impl Display {
    /// #     pub fn draw<T>(&self, item_pixels: T) -> Result<(), ()>
    /// #     where
    /// #         T: Iterator<Item = Pixel>,
    /// #     {
    /// #         Ok(())
    /// #     }
    /// # }
    ///
    /// fn main() {
    ///     let disp = Display {};
    ///
    ///     let text = Font6x8::render_str("Hello world");
    ///
    ///     disp.draw(text.into_iter());
    /// }
    /// ```
    fn render_str(chars: &'a str) -> Self;
}
