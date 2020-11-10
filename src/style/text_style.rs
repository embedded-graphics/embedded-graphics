use crate::{fonts::MonospacedFont, pixelcolor::PixelColor};

/// Style properties for text.
///
/// A `TextStyle` can be applied to a [`Text`] object to define how the text is drawn.
///
/// Because `TextStyle` has the [`non_exhaustive`] attribute, it cannot be created using a struct
/// literal. To create a `TextStyle` with a given text color and transparent background, use the
/// [`new`] method. For more complex text styles, use the [`TextStyleBuilder`].
///
/// [`Text`]: ../fonts/struct.Text.html
/// [`non_exhaustive`]: https://blog.rust-lang.org/2019/12/19/Rust-1.40.0.html#[non_exhaustive]-structs,-enums,-and-variants
/// [`TextStyleBuilder`]: ./struct.TextStyleBuilder.html
/// [`new`]: #method.new
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[non_exhaustive]
pub struct TextStyle<C, F>
where
    C: PixelColor,
    F: MonospacedFont,
{
    /// Text color.
    pub text_color: Option<C>,

    /// Background color.
    pub background_color: Option<C>,

    /// Font.
    pub font: F,
}

impl<C, F> TextStyle<C, F>
where
    C: PixelColor,
    F: MonospacedFont,
{
    /// Creates a text style with transparent background.
    pub fn new(font: F, text_color: C) -> Self {
        Self {
            font,
            text_color: Some(text_color),
            background_color: None,
        }
    }
}

/// Text style builder.
///
/// Use this builder to create [`TextStyle`]s for [`Text`].
///
/// # Examples
///
/// ## Render yellow text on a blue background
///
/// This uses the [`Font6x8`] font, but [other fonts] can also be used.
///
/// ```rust
/// use embedded_graphics::{
///     fonts::{Font6x8, Text},
///     pixelcolor::Rgb565,
///     prelude::*,
///     style::{TextStyle, TextStyleBuilder},
/// };
///
/// let style: TextStyle<Rgb565, Font6x8> = TextStyleBuilder::new(Font6x8)
///     .text_color(Rgb565::YELLOW)
///     .background_color(Rgb565::BLUE)
///     .build();
///
/// let text = Text::new("Hello Rust!", Point::new(0, 0)).into_styled(style);
/// ```
///
/// ## Transparent background
///
/// If a property is ommitted, it will remain at its default value in the resulting `TextStyle`
/// returned by `.build()`. This example draws white text with no background at all.
///
/// ```rust
/// use embedded_graphics::{
///     fonts::{Font6x8, Text},
///     pixelcolor::Rgb565,
///     prelude::*,
///     style::{TextStyle, TextStyleBuilder},
/// };
///
/// let style: TextStyle<Rgb565, Font6x8> = TextStyleBuilder::new(Font6x8)
///     .text_color(Rgb565::WHITE)
///     .build();
///
/// let text = Text::new("Hello Rust!", Point::new(0, 0)).into_styled(style);
/// ```
///
/// [`Font`]: ../fonts/trait.Font.html
/// [`Font6x8`]: ../fonts/struct.Font6x8.html
/// [`Font8x16`]: ../fonts/struct.Font8x16.html
/// [other fonts]: ../fonts/index.html
/// [`Text`]: ../fonts/struct.Text.html
/// [`TextStyle`]: ./struct.TextStyle.html
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct TextStyleBuilder<C, F>
where
    C: PixelColor,
    F: MonospacedFont + Clone,
{
    style: TextStyle<C, F>,
}

impl<C, F> TextStyleBuilder<C, F>
where
    C: PixelColor,
    F: MonospacedFont + Clone,
{
    /// Creates a new text style builder with a given font.
    pub fn new(font: F) -> Self {
        Self {
            style: TextStyle {
                font,
                background_color: None,
                text_color: None,
            },
        }
    }

    /// Sets the text color.
    pub fn text_color(mut self, text_color: C) -> Self {
        self.style.text_color = Some(text_color);

        self
    }

    /// Sets the background color.
    pub fn background_color(mut self, background_color: C) -> Self {
        self.style.background_color = Some(background_color);

        self
    }

    /// Builds the text style.
    pub fn build(self) -> TextStyle<C, F> {
        self.style
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{fonts::Font12x16, pixelcolor::BinaryColor};

    #[test]
    fn builder_default() {
        assert_eq!(
            TextStyleBuilder::<BinaryColor, _>::new(Font12x16).build(),
            TextStyle {
                font: Font12x16,
                text_color: None,
                background_color: None
            }
        );
    }

    #[test]
    fn builder_text_color() {
        assert_eq!(
            TextStyleBuilder::new(Font12x16)
                .text_color(BinaryColor::On)
                .build(),
            TextStyle::new(Font12x16, BinaryColor::On)
        );
    }

    #[test]
    fn builder_background_color() {
        assert_eq!(
            TextStyleBuilder::new(Font12x16)
                .background_color(BinaryColor::On)
                .build(),
            {
                let mut style = TextStyleBuilder::new(Font12x16).build();

                style.text_color = None;
                style.background_color = Some(BinaryColor::On);

                style
            }
        );
    }
}
