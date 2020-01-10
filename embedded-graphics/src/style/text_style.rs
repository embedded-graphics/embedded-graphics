use crate::{fonts::Font, pixelcolor::PixelColor};

/// Style properties for texts.
///
/// `TextStyle` can be applied to a [`Text`] object to define how a text is drawn.
///
/// Because `TextStyle` has the [`non_exhaustive`] attribute, it cannot be created using a struct
/// literal. To create a `TextStyle`, the [`text_style!`] macro or [`TextStyleBuilder`] can be
/// used instead.
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
}

/// Text style builder.
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
    pub fn text_color(&mut self, text_color: C) -> &mut Self {
        self.style.text_color = Some(text_color);

        self
    }

    /// Sets the background color.
    pub fn background_color(&mut self, background_color: C) -> &mut Self {
        self.style.background_color = Some(background_color);

        self
    }

    /// Builds the text style.
    pub fn build(&self) -> TextStyle<C, F> {
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
