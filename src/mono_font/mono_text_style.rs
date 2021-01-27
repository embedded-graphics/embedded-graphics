use crate::{
    draw_target::DrawTarget,
    geometry::{Point, Size},
    mono_font::{MonoCharPixels, MonoFont},
    pixelcolor::{BinaryColor, PixelColor},
    primitives::Rectangle,
    text::{CharacterStyle, DecorationColor, TextMetrics, TextRenderer, VerticalAlignment},
    Pixel, SaturatingCast,
};

/// Style properties for text using a monospaced font.
///
/// A `MonoTextStyle` can be applied to a [`Text`] object to define how the text is drawn.
///
/// Because `MonoTextStyle` has the [`non_exhaustive`] attribute, it cannot be created using a
/// struct literal. To create a `MonoTextStyle` with a given text color and transparent
/// background, use the [`new`] method. For more complex text styles, use the
/// [`MonoTextStyleBuilder`].
///
/// [`Text`]: ../text/struct.Text.html
/// [`non_exhaustive`]: https://blog.rust-lang.org/2019/12/19/Rust-1.40.0.html#[non_exhaustive]-structs,-enums,-and-variants
/// [`MonoTextStyleBuilder`]: ./struct.MonoTextStyleBuilder.html
/// [`new`]: #method.new
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[non_exhaustive]
pub struct MonoTextStyle<C, F> {
    /// Text color.
    pub text_color: Option<C>,

    /// Background color.
    pub background_color: Option<C>,

    /// Underline color.
    pub underline_color: DecorationColor<C>,

    /// Strikethrough color.
    pub strikethrough_color: DecorationColor<C>,

    /// Font.
    pub font: F,
}

impl<C, F> MonoTextStyle<C, F>
where
    C: PixelColor,
    F: MonoFont,
{
    /// Creates a text style with transparent background.
    pub fn new(font: F, text_color: C) -> Self {
        MonoTextStyleBuilder::new()
            .font(font)
            .text_color(text_color)
            .build()
    }

    /// Resolves a decoration color.
    fn resolve_decoration_color(&self, color: DecorationColor<C>) -> Option<C> {
        match color {
            DecorationColor::None => None,
            DecorationColor::TextColor => self.text_color,
            DecorationColor::Custom(c) => Some(c),
        }
    }

    fn draw_background<D>(
        &self,
        width: u32,
        position: Point,
        target: &mut D,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        if width == 0 {
            return Ok(());
        }

        if let Some(background_color) = self.background_color {
            target.fill_solid(
                &Rectangle::new(position, Size::new(width, F::CHARACTER_SIZE.height)),
                background_color,
            )?;
        }

        Ok(())
    }

    fn draw_strikethrough<D>(
        &self,
        width: u32,
        position: Point,
        target: &mut D,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        if let Some(strikethrough_color) = self.resolve_decoration_color(self.strikethrough_color) {
            let top_left = position + Point::new(0, F::STRIKETHROUGH_OFFSET);
            let size = Size::new(width, F::STRIKETHROUGH_HEIGHT);

            target.fill_solid(&Rectangle::new(top_left, size), strikethrough_color)?;
        }

        Ok(())
    }

    fn draw_underline<D>(&self, width: u32, position: Point, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        if let Some(underline_color) = self.resolve_decoration_color(self.underline_color) {
            let top_left = position + Point::new(0, F::UNDERLINE_OFFSET);
            let size = Size::new(width, F::UNDERLINE_HEIGHT);

            target.fill_solid(&Rectangle::new(top_left, size), underline_color)?;
        }

        Ok(())
    }
}

impl<C, F> TextRenderer for MonoTextStyle<C, F>
where
    C: PixelColor,
    F: MonoFont,
{
    type Color = C;

    fn draw_string<D>(&self, text: &str, position: Point, target: &mut D) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let mut first = true;
        let mut p = position;

        let mut width = 0;

        for c in text.chars() {
            if first {
                first = false;
            } else if F::CHARACTER_SPACING > 0 {
                // Fill space between characters if background color is set.
                self.draw_background(F::CHARACTER_SPACING, p, target)?;
                p += Size::new(F::CHARACTER_SPACING, 0);
                width += F::CHARACTER_SPACING;
            }

            let pixels = MonoCharPixels::<F>::new(c);

            match (self.text_color, self.background_color) {
                (Some(text_color), Some(background_color)) => {
                    let bounding_box = Rectangle::new(p, F::CHARACTER_SIZE);

                    // The glyph is opaque if both colors are set and `fill_contiguous` can be used.
                    target.fill_contiguous(
                        &bounding_box,
                        pixels.map(|Pixel(_, c)| match c {
                            BinaryColor::Off => background_color,
                            BinaryColor::On => text_color,
                        }),
                    )?;
                }
                (Some(text_color), None) => {
                    target.draw_iter(
                        pixels
                            .filter(|Pixel(_, c)| *c == BinaryColor::On)
                            .map(|Pixel(delta_p, _)| Pixel(p + delta_p, text_color)),
                    )?;
                }
                (None, Some(background_color)) => {
                    target.draw_iter(
                        pixels
                            .filter(|Pixel(_, c)| *c == BinaryColor::Off)
                            .map(|Pixel(delta_p, _)| Pixel(p + delta_p, background_color)),
                    )?;
                }
                (None, None) => {}
            }

            p += F::CHARACTER_SIZE.x_axis();
            width += F::CHARACTER_SIZE.width;
        }

        self.draw_strikethrough(width, position, target)?;
        self.draw_underline(width, position, target)?;

        Ok(p)
    }

    fn draw_whitespace<D>(
        &self,
        width: u32,
        position: Point,
        target: &mut D,
    ) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        self.draw_background(width, position, target)?;
        self.draw_strikethrough(width, position, target)?;
        self.draw_underline(width, position, target)?;

        Ok(position + Size::new(width, 0))
    }

    fn measure_string(&self, text: &str, position: Point) -> TextMetrics {
        let width = (text.len() as u32 * (F::CHARACTER_SIZE.width + F::CHARACTER_SPACING))
            .saturating_sub(F::CHARACTER_SPACING);
        let size = Size::new(width, F::CHARACTER_SIZE.height);

        // Return a zero sized bounding box if the text is completely transparent.
        let bb_size = if self.text_color.is_some() || self.background_color.is_some() {
            size
        } else {
            Size::zero()
        };

        TextMetrics {
            bounding_box: Rectangle::new(position, bb_size),
            next_position: position + size.x_axis(),
        }
    }

    /// Calculates the offset between the line position and the top edge of the bounding box.
    fn vertical_offset(&self, position: Point, vertical_alignment: VerticalAlignment) -> Point {
        let dy = match vertical_alignment {
            VerticalAlignment::Top => 0,
            VerticalAlignment::Bottom => {
                F::CHARACTER_SIZE.height.saturating_sub(1).saturating_cast()
            }
            VerticalAlignment::Center => {
                (F::CHARACTER_SIZE.height.saturating_sub(1) / 2).saturating_cast()
            }
            VerticalAlignment::Baseline => F::BASELINE
                .unwrap_or_else(|| F::CHARACTER_SIZE.height.saturating_sub(1).saturating_cast()),
        };

        position - Point::new(0, dy)
    }

    fn line_height(&self) -> u32 {
        F::CHARACTER_SIZE.height
    }
}

impl<C, F> CharacterStyle for MonoTextStyle<C, F>
where
    C: PixelColor,
    F: MonoFont,
{
    type Color = C;

    fn set_text_color(&mut self, text_color: Option<Self::Color>) {
        self.text_color = text_color;
    }

    fn set_background_color(&mut self, background_color: Option<Self::Color>) {
        self.background_color = background_color;
    }

    fn set_underline_color(&mut self, underline_color: DecorationColor<Self::Color>) {
        self.underline_color = underline_color;
    }

    fn set_strikethrough_color(&mut self, strikethrough_color: DecorationColor<Self::Color>) {
        self.strikethrough_color = strikethrough_color;
    }
}

/// Text style builder for monospaced fonts.
///
/// Use this builder to create [`MonoTextStyle`]s for [`Text`].
///
/// # Examples
///
/// ## Render yellow text on a blue background
///
/// This uses the [`Font6x8`] font, but [other fonts] can also be used.
///
/// ```rust
/// use embedded_graphics::{
///     mono_font::{Font6x8, MonoTextStyle, MonoTextStyleBuilder},
///     pixelcolor::Rgb565,
///     prelude::*,
///     text::Text,
/// };
///
/// let style = MonoTextStyleBuilder::new()
///     .font(Font6x8)
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
/// `MonoTextStyle` returned by `.build()`. This example draws white text with no background at
/// all.
///
/// ```rust
/// use embedded_graphics::{
///     mono_font::{Font6x8, MonoTextStyle, MonoTextStyleBuilder},
///     pixelcolor::Rgb565,
///     prelude::*,
///     text::Text,
/// };
///
/// let style = MonoTextStyleBuilder::new()
///     .font(Font6x8)
///     .text_color(Rgb565::WHITE)
///     .build();
///
/// let text = Text::new("Hello Rust!", Point::new(0, 0)).into_styled(style);
/// ```
///
/// ## Modifying an existing style
///
/// The builder can also be used to modify an existing style.
///
/// ```
/// use embedded_graphics::{
///     mono_font::{Font6x8, Font12x16, MonoTextStyle, MonoTextStyleBuilder},
///     pixelcolor::Rgb565,
///     prelude::*,
///     text::Text,
/// };
///
/// let style = MonoTextStyle::new(Font6x8, Rgb565::YELLOW);
///
/// let style_larger = MonoTextStyleBuilder::from(&style)
///     .font(Font12x16)
///     .build();
/// ```
///
/// [`Font6x8`]: struct.Font6x8.html
/// [`Font8x16`]: struct.Font8x16.html
/// [other fonts]: index.html
/// [`Text`]: ../text/struct.Text.html
/// [`MonoTextStyle`]: struct.MonoTextStyle.html
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct MonoTextStyleBuilder<C, F> {
    style: MonoTextStyle<C, F>,
}

impl<C> MonoTextStyleBuilder<C, UndefinedFont> {
    /// Creates a new text style builder.
    pub fn new() -> Self {
        Self {
            style: MonoTextStyle {
                font: UndefinedFont,
                background_color: None,
                text_color: None,
                underline_color: DecorationColor::None,
                strikethrough_color: DecorationColor::None,
            },
        }
    }
}

impl<C, F> MonoTextStyleBuilder<C, F> {
    /// Sets the font.
    pub fn font<Font>(self, font: Font) -> MonoTextStyleBuilder<C, Font> {
        let style = MonoTextStyle {
            font,
            background_color: self.style.background_color,
            text_color: self.style.text_color,
            underline_color: self.style.underline_color,
            strikethrough_color: self.style.strikethrough_color,
        };

        MonoTextStyleBuilder { style }
    }

    /// Enables underline using the text color.
    pub fn underline(mut self) -> Self {
        self.style.underline_color = DecorationColor::TextColor;

        self
    }

    /// Enables strikethrough using the text color.
    pub fn strikethrough(mut self) -> Self {
        self.style.strikethrough_color = DecorationColor::TextColor;

        self
    }
}

impl<C, F> MonoTextStyleBuilder<C, F>
where
    C: PixelColor,
{
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

    /// Enables underline with a custom color.
    pub fn underline_with_color(mut self, underline_color: C) -> Self {
        self.style.underline_color = DecorationColor::Custom(underline_color);

        self
    }

    /// Enables strikethrough with a custom color.
    pub fn strikethrough_with_color(mut self, strikethrough_color: C) -> Self {
        self.style.strikethrough_color = DecorationColor::Custom(strikethrough_color);

        self
    }
}

impl<C, F> MonoTextStyleBuilder<C, F>
where
    C: PixelColor,
    F: MonoFont,
{
    /// Builds the text style.
    ///
    /// This method can only be called after a font was set by using the [`font`] method. All other
    /// settings are optional and they will be set to their default value if they are missing.
    ///
    /// [`font`]: #method.font
    pub fn build(self) -> MonoTextStyle<C, F> {
        self.style
    }
}

impl<C, F> From<&MonoTextStyle<C, F>> for MonoTextStyleBuilder<C, F>
where
    C: PixelColor,
    F: MonoFont,
{
    fn from(style: &MonoTextStyle<C, F>) -> Self {
        Self { style: *style }
    }
}

/// Marker type to improve compiler errors if no font was set in builder.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct UndefinedFont;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::Dimensions,
        mock_display::MockDisplay,
        mono_font::{tests::*, Font12x16, Font6x8},
        pixelcolor::{BinaryColor, Rgb888, RgbColor},
        text::{Text, TextStyleBuilder},
        Drawable,
    };

    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
    struct SpacedFont;

    impl MonoFont for SpacedFont {
        const FONT_IMAGE: &'static [u8] = Font6x8::FONT_IMAGE;
        const FONT_IMAGE_WIDTH: u32 = Font6x8::FONT_IMAGE_WIDTH;
        const CHARACTER_SIZE: Size = Font6x8::CHARACTER_SIZE;
        const CHARACTER_SPACING: u32 = 5;
        const STRIKETHROUGH_OFFSET: i32 = Font6x8::STRIKETHROUGH_OFFSET;

        fn char_offset(c: char) -> u32 {
            Font6x8::char_offset(c)
        }
    }

    #[test]
    fn builder_default() {
        assert_eq!(
            MonoTextStyleBuilder::<BinaryColor, _>::new()
                .font(Font12x16)
                .build(),
            MonoTextStyle {
                font: Font12x16,
                text_color: None,
                background_color: None,
                underline_color: DecorationColor::None,
                strikethrough_color: DecorationColor::None,
            }
        );
    }

    #[test]
    fn builder_text_color() {
        assert_eq!(
            MonoTextStyleBuilder::new()
                .font(Font12x16)
                .text_color(BinaryColor::On)
                .build(),
            MonoTextStyle::new(Font12x16, BinaryColor::On)
        );
    }

    #[test]
    fn builder_background_color() {
        assert_eq!(
            MonoTextStyleBuilder::new()
                .font(Font12x16)
                .background_color(BinaryColor::On)
                .build(),
            {
                let mut style = MonoTextStyleBuilder::new().font(Font12x16).build();

                style.text_color = None;
                style.background_color = Some(BinaryColor::On);

                style
            }
        );
    }

    #[test]
    fn underline_text_color() {
        let style = MonoTextStyleBuilder::new()
            .font(Font6x8)
            .text_color(Rgb888::WHITE)
            .underline()
            .build();

        let mut display = MockDisplay::new();
        Text::new("ABC", Point::new(0, 6))
            .into_styled(style)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            " WWW  WWWW   WWW  ",
            "W   W W   W W   W ",
            "W   W W   W W     ",
            "WWWWW WWWW  W     ",
            "W   W W   W W     ",
            "W   W W   W W   W ",
            "W   W WWWW   WWW  ",
            "                  ",
            "WWWWWWWWWWWWWWWWWW",
        ]);
    }

    #[test]
    fn underline_custom_color() {
        let style = MonoTextStyleBuilder::new()
            .font(Font6x8)
            .text_color(Rgb888::WHITE)
            .underline_with_color(Rgb888::RED)
            .build();

        let mut display = MockDisplay::new();
        Text::new("ABC", Point::new(0, 6))
            .into_styled(style)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            " WWW  WWWW   WWW  ",
            "W   W W   W W   W ",
            "W   W W   W W     ",
            "WWWWW WWWW  W     ",
            "W   W W   W W     ",
            "W   W W   W W   W ",
            "W   W WWWW   WWW  ",
            "                  ",
            "RRRRRRRRRRRRRRRRRR",
        ]);
    }

    #[test]
    fn strikethrough_text_color() {
        let style = MonoTextStyleBuilder::new()
            .font(Font6x8)
            .text_color(Rgb888::WHITE)
            .strikethrough()
            .build();

        let mut display = MockDisplay::new();
        display.set_allow_overdraw(true);

        Text::new("ABC", Point::new(0, 6))
            .into_styled(style)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            " WWW  WWWW   WWW  ",
            "W   W W   W W   W ",
            "W   W W   W W     ",
            "WWWWWWWWWWWWWWWWWW",
            "W   W W   W W     ",
            "W   W W   W W   W ",
            "W   W WWWW   WWW  ",
        ]);
    }

    #[test]
    fn strikethrough_custom_color() {
        let style = MonoTextStyleBuilder::new()
            .font(Font6x8)
            .text_color(Rgb888::WHITE)
            .strikethrough_with_color(Rgb888::RED)
            .build();

        let mut display = MockDisplay::new();
        display.set_allow_overdraw(true);

        Text::new("ABC", Point::new(0, 6))
            .into_styled(style)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            " WWW  WWWW   WWW  ",
            "W   W W   W W   W ",
            "W   W W   W W     ",
            "RRRRRRRRRRRRRRRRRR",
            "W   W W   W W     ",
            "W   W W   W W   W ",
            "W   W WWWW   WWW  ",
        ]);
    }

    #[test]
    fn whitespace_background() {
        let style = MonoTextStyleBuilder::new()
            .font(Font6x8)
            .text_color(Rgb888::YELLOW)
            .background_color(Rgb888::WHITE)
            .build();

        let mut display = MockDisplay::new();
        style
            .draw_whitespace(4, Point::zero(), &mut display)
            .unwrap();

        display.assert_pattern(&[
            "WWWW", //
            "WWWW", //
            "WWWW", //
            "WWWW", //
            "WWWW", //
            "WWWW", //
            "WWWW", //
            "WWWW", //
        ]);
    }

    #[test]
    fn whitespace_decorations() {
        let style = MonoTextStyleBuilder::new()
            .font(Font6x8)
            .text_color(Rgb888::YELLOW)
            .underline_with_color(Rgb888::GREEN)
            .strikethrough_with_color(Rgb888::RED)
            .build();

        let mut display = MockDisplay::new();
        style
            .draw_whitespace(3, Point::zero(), &mut display)
            .unwrap();

        display.assert_pattern(&[
            "   ", //
            "   ", //
            "   ", //
            "RRR", //
            "   ", //
            "   ", //
            "   ", //
            "   ", //
            "GGG", //
        ]);
    }

    #[test]
    fn whitespace_background_and_decorations() {
        let style = MonoTextStyleBuilder::new()
            .font(Font6x8)
            .text_color(Rgb888::YELLOW)
            .background_color(Rgb888::WHITE)
            .underline_with_color(Rgb888::GREEN)
            .strikethrough_with_color(Rgb888::RED)
            .build();

        let mut display = MockDisplay::new();
        display.set_allow_overdraw(true);

        style
            .draw_whitespace(8, Point::zero(), &mut display)
            .unwrap();

        display.assert_pattern(&[
            "WWWWWWWW", //
            "WWWWWWWW", //
            "WWWWWWWW", //
            "RRRRRRRR", //
            "WWWWWWWW", //
            "WWWWWWWW", //
            "WWWWWWWW", //
            "WWWWWWWW", //
            "GGGGGGGG", //
        ]);
    }

    #[test]
    fn character_spacing() {
        assert_text_from_pattern(
            "##",
            SpacedFont,
            &[
                " # #        # #  ",
                " # #        # #  ",
                "#####      ##### ",
                " # #        # #  ",
                "#####      ##### ",
                " # #        # #  ",
                " # #        # #  ",
                "                 ",
            ],
        );
    }

    #[test]
    fn character_spacing_with_background() {
        let character_style = MonoTextStyleBuilder::new()
            .font(SpacedFont)
            .text_color(BinaryColor::On)
            .background_color(BinaryColor::Off)
            .build();

        let text_style = TextStyleBuilder::new()
            .character_style(character_style)
            .vertical_alignment(VerticalAlignment::Top)
            .build();

        let mut display = MockDisplay::new();
        Text::new("##", Point::zero())
            .into_styled(text_style)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            ".#.#........#.#..",
            ".#.#........#.#..",
            "#####......#####.",
            ".#.#........#.#..",
            "#####......#####.",
            ".#.#........#.#..",
            ".#.#........#.#..",
            ".................",
        ]);
    }

    #[test]
    fn character_spacing_decorations() {
        let character_style = MonoTextStyleBuilder::new()
            .font(SpacedFont)
            .text_color(Rgb888::WHITE)
            .underline_with_color(Rgb888::GREEN)
            .strikethrough_with_color(Rgb888::RED)
            .build();

        let text_style = TextStyleBuilder::new()
            .character_style(character_style)
            .vertical_alignment(VerticalAlignment::Top)
            .build();

        let mut display = MockDisplay::new();
        display.set_allow_overdraw(true);

        Text::new("##", Point::zero())
            .into_styled(text_style)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            " W W        W W  ",
            " W W        W W  ",
            "WWWWW      WWWWW ",
            "RRRRRRRRRRRRRRRRR",
            "WWWWW      WWWWW ",
            " W W        W W  ",
            " W W        W W  ",
            "                 ",
            "GGGGGGGGGGGGGGGGG",
        ]);
    }

    #[test]
    fn character_spacing_dimensions() {
        let character_style = MonoTextStyleBuilder::new()
            .font(SpacedFont)
            .text_color(BinaryColor::On)
            .build();

        let text_style = TextStyleBuilder::new()
            .character_style(character_style)
            .vertical_alignment(VerticalAlignment::Top)
            .build();

        assert_eq!(
            Text::new("#", Point::zero())
                .into_styled(text_style)
                .bounding_box(),
            Rectangle::new(Point::zero(), Size::new(6, 8)),
        );

        assert_eq!(
            Text::new("##", Point::zero())
                .into_styled(text_style)
                .bounding_box(),
            Rectangle::new(Point::zero(), Size::new(6 * 2 + 5, 8)),
        );
        assert_eq!(
            Text::new("###", Point::zero())
                .into_styled(text_style)
                .bounding_box(),
            Rectangle::new(Point::zero(), Size::new(6 * 3 + 5 * 2, 8)),
        );
    }

    #[test]
    fn control_characters() {
        let style = MonoTextStyle::new(Font6x8, BinaryColor::On);

        let mut display = MockDisplay::new();
        style
            .draw_string("A\t\n\rB", Point::zero(), &mut display)
            .unwrap();

        let mut expected = MockDisplay::new();
        style
            .draw_string("A???B", Point::zero(), &mut expected)
            .unwrap();

        display.assert_eq(&expected);
    }

    #[test]
    fn character_style() {
        let mut style = MonoTextStyle::new(Font6x8, BinaryColor::On);
        CharacterStyle::set_text_color(&mut style, None);
        CharacterStyle::set_background_color(&mut style, Some(BinaryColor::On));
        CharacterStyle::set_underline_color(&mut style, DecorationColor::TextColor);
        CharacterStyle::set_strikethrough_color(
            &mut style,
            DecorationColor::Custom(BinaryColor::On),
        );

        assert_eq!(
            style,
            MonoTextStyle {
                text_color: None,
                background_color: Some(BinaryColor::On),
                underline_color: DecorationColor::TextColor,
                strikethrough_color: DecorationColor::Custom(BinaryColor::On),
                font: Font6x8,
            }
        );
    }
}
