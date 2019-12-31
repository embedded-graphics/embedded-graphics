use crate::fonts::Font;
use crate::pixelcolor::PixelColor;

/// Style properties for texts.
///
/// `TextStyle` can be applied to a [`Text`] object to define how a text is drawn.
///
/// Because `TextStyle` has the [`non_exhaustive` attribute], it cannot be created using a struct
/// expression. To create a `TextStyle`, the [`text_style!()` macro] or [`TextStyleBuilder` builder]
/// can be used instead.
///
/// [`Text`]: ../fonts/struct.Text.html
/// [`non_exhaustive` attribute]: https://blog.rust-lang.org/2019/12/19/Rust-1.40.0.html#[non_exhaustive]-structs,-enums,-and-variants
/// [`text_style!()` macro]: ../macro.text_style.html
/// [`TextStyleBuilder` builder]: ./struct.TextStyleBuilder.html
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct TextStyle<C, F>
where
    C: PixelColor,
    F: Font,
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
    F: Font,
{
    /// Creates a text style with transparent background.
    pub fn new(font: F, text_color: C) -> Self {
        Self {
            font,
            text_color: Some(text_color),
            background_color: None,
        }
    }

    /// Create a text style with only the font defined
    pub fn with_font(font: F) -> Self {
        Self {
            font,
            text_color: None,
            background_color: None,
        }
    }
}

/// Primitive style builder.
#[derive(Debug, PartialEq, Eq)]
pub struct TextStyleBuilder<C, F>
where
    C: PixelColor,
    F: Font,
{
    style: TextStyle<C, F>,
}

impl<C, F> TextStyleBuilder<C, F>
where
    C: PixelColor,
    F: Font,
{
    /// Creates a new primitive style builder with a given font.
    pub fn new(font: F) -> Self {
        Self {
            style: TextStyle::with_font(font),
        }
    }

    /// Sets the text color.
    pub fn text_color(&mut self, text_color: C) -> &mut Self {
        self.style.text_color = Some(text_color);

        self
    }

    /// Sets the background color.
    pub fn background_color(&mut self, background_color: C) -> &mut Self {
        self.style.background_color = Some(background_color);

        self
    }

    /// Builds the primitive style.
    pub fn build(&self) -> TextStyle<C, F> {
        self.style
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fonts::Font12x16;
    use crate::pixelcolor::BinaryColor;

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
    fn builder_fill() {
        assert_eq!(
            TextStyleBuilder::new(Font12x16)
                .background_color(BinaryColor::On)
                .build(),
            {
                let mut style = TextStyle::with_font(Font12x16);

                style.text_color = None;
                style.background_color = Some(BinaryColor::On);

                style
            }
        );
    }
}
