use crate::{
    draw_target::DrawTarget,
    fonts::{MonospacedFont, MonospacedPixels, Text},
    geometry::Size,
    pixelcolor::PixelColor,
    primitives::Rectangle,
    style::{TextStyle, TextStylePixels},
};

/// Style properties for text using a monospaced font.
///
/// A `MonospacedTextStyle` can be applied to a [`Text`] object to define how the text is drawn.
///
/// Because `MonospacedTextStyle` has the [`non_exhaustive`] attribute, it cannot be created using a
/// struct literal. To create a `MonospacedTextStyle` with a given text color and transparent
/// background, use the [`new`] method. For more complex text styles, use the
/// [`MonospacedTextStyleBuilder`].
///
/// [`Text`]: ../fonts/struct.Text.html
/// [`non_exhaustive`]: https://blog.rust-lang.org/2019/12/19/Rust-1.40.0.html#[non_exhaustive]-structs,-enums,-and-variants
/// [`MonospacedTextStyleBuilder`]: ./struct.MonospacedTextStyleBuilder.html
/// [`new`]: #method.new
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[non_exhaustive]
pub struct MonospacedTextStyle<C, F>
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

impl<C, F> MonospacedTextStyle<C, F>
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

impl<C, F> TextStyle for MonospacedTextStyle<C, F>
where
    C: PixelColor,
    F: MonospacedFont + Copy,
{
    type Color = C;

    fn render_text<D>(&self, text: &Text<'_>, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        target.draw_iter(MonospacedPixels::new(text, *self))
    }

    fn bounding_box(&self, text: &Text<'_>) -> Rectangle {
        // If a piece of text is completely transparent, return an empty bounding box
        if self.text_color.is_none() && self.background_color.is_none() {
            return Rectangle::new(text.position, Size::zero());
        }

        let width = text
            .text
            .lines()
            .map(|line| {
                (line.len() as u32 * (F::CHARACTER_SPACING + F::CHARACTER_SIZE.width))
                    .saturating_sub(F::CHARACTER_SPACING)
            })
            .max()
            .unwrap_or(0);

        let height = if width > 0 {
            F::CHARACTER_SIZE.height * text.text.lines().count() as u32
        } else {
            0
        };

        let size = Size::new(width, height);

        Rectangle::new(text.position, size)
    }
}

impl<'a, C, F> TextStylePixels<'a> for MonospacedTextStyle<C, F>
where
    C: PixelColor + 'a,
    F: MonospacedFont + Copy + 'a,
{
    type Iter = MonospacedPixels<'a, C, F>;

    fn pixels(&self, text: &Text<'a>) -> Self::Iter {
        MonospacedPixels::new(&text, self.clone())
    }
}

/// Text style builder for monospaced fonts.
///
/// Use this builder to create [`MonospacedTextStyle`]s for [`Text`].
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
///     style::{MonospacedTextStyle, MonospacedTextStyleBuilder},
/// };
///
/// let style: MonospacedTextStyle<Rgb565, Font6x8> = MonospacedTextStyleBuilder::new(Font6x8)
///     .text_color(Rgb565::YELLOW)
///     .background_color(Rgb565::BLUE)
///     .build();
///
/// let text = Text::new("Hello Rust!", Point::new(0, 0)).into_styled(style);
/// ```
///
/// ## Transparent background
///
/// If a property is omitted, it will remain at its default value in the resulting
/// `MonospacedTextStyle`returned by `.build()`. This example draws white text with no background at
/// all.
///
/// ```rust
/// use embedded_graphics::{
///     fonts::{Font6x8, Text},
///     pixelcolor::Rgb565,
///     prelude::*,
///     style::{MonospacedTextStyle, MonospacedTextStyleBuilder},
/// };
///
/// let style: MonospacedTextStyle<Rgb565, Font6x8> = MonospacedTextStyleBuilder::new(Font6x8)
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
/// [`MonospacedTextStyle`]: ./struct.MonospacedTextStyle.html
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct MonospacedTextStyleBuilder<C, F>
where
    C: PixelColor,
    F: MonospacedFont + Clone,
{
    style: MonospacedTextStyle<C, F>,
}

impl<C, F> MonospacedTextStyleBuilder<C, F>
where
    C: PixelColor,
    F: MonospacedFont + Clone,
{
    /// Creates a new text style builder with a given font.
    pub fn new(font: F) -> Self {
        Self {
            style: MonospacedTextStyle {
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
    pub fn build(self) -> MonospacedTextStyle<C, F> {
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
            MonospacedTextStyleBuilder::<BinaryColor, _>::new(Font12x16).build(),
            MonospacedTextStyle {
                font: Font12x16,
                text_color: None,
                background_color: None
            }
        );
    }

    #[test]
    fn builder_text_color() {
        assert_eq!(
            MonospacedTextStyleBuilder::new(Font12x16)
                .text_color(BinaryColor::On)
                .build(),
            MonospacedTextStyle::new(Font12x16, BinaryColor::On)
        );
    }

    #[test]
    fn builder_background_color() {
        assert_eq!(
            MonospacedTextStyleBuilder::new(Font12x16)
                .background_color(BinaryColor::On)
                .build(),
            {
                let mut style = MonospacedTextStyleBuilder::new(Font12x16).build();

                style.text_color = None;
                style.background_color = Some(BinaryColor::On);

                style
            }
        );
    }
}
