use crate::{
    draw_target::DrawTarget,
    geometry::{Point, Size},
    image::Image,
    mono_font::{
        draw_target::{Background, Both, Foreground, MonoFontDrawTarget},
        MonoFont,
    },
    pixelcolor::{BinaryColor, PixelColor},
    primitives::Rectangle,
    text::{
        renderer::{CharacterStyle, TextMetrics, TextRenderer},
        Baseline, DecorationColor,
    },
    Drawable,
};
use az::SaturatingAs;

/// Style properties for text using a monospaced font.
///
/// A `MonoTextStyle` can be applied to a [`Text`] object to define how the text is drawn.
///
/// Because `MonoTextStyle` has the [`non_exhaustive`] attribute, it cannot be created using a
/// struct literal. To create a `MonoTextStyle` with a given text color and transparent
/// background, use the [`new`] method. For more complex text styles, use the
/// [`MonoTextStyleBuilder`].
///
/// [`Text`]: crate::text::Text
/// [`non_exhaustive`]: https://blog.rust-lang.org/2019/12/19/Rust-1.40.0.html#[non_exhaustive]-structs,-enums,-and-variants
/// [`new`]: MonoTextStyle::new()
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
#[non_exhaustive]
pub struct MonoTextStyle<'a, C> {
    /// Text color.
    pub text_color: Option<C>,

    /// Background color.
    pub background_color: Option<C>,

    /// Underline color.
    pub underline_color: DecorationColor<C>,

    /// Strikethrough color.
    pub strikethrough_color: DecorationColor<C>,

    /// Font.
    pub font: &'a MonoFont<'a>,
}

impl<'a, C: PixelColor> MonoTextStyle<'a, C> {
    /// Creates a text style with transparent background.
    pub fn new(font: &'a MonoFont<'a>, text_color: C) -> Self {
        MonoTextStyleBuilder::new()
            .font(font)
            .text_color(text_color)
            .build()
    }

    /// Returns `true` if the style is transparent.
    ///
    /// Drawing a `Text` with a transparent `MonoTextStyle` will not draw any pixels.
    ///
    /// [`Text`]: super::text::Text
    pub fn is_transparent(&self) -> bool {
        self.text_color.is_none()
            && self.background_color.is_none()
            && self.underline_color.is_none()
            && self.strikethrough_color.is_none()
    }

    fn line_elements<'t>(
        &self,
        mut position: Point,
        text: &'t str,
    ) -> impl Iterator<Item = (Point, LineElement)> + 't {
        let char_width = self.font.character_size.width as i32;
        let spacing_width = self.font.character_spacing as i32;

        let mut chars = text.chars();
        let mut next_char = chars.next();
        let mut add_spacing = false;

        core::iter::from_fn(move || {
            if add_spacing {
                let p = position;
                position.x += spacing_width;

                add_spacing = false;

                Some((p, LineElement::Spacing))
            } else if let Some(c) = next_char {
                let p = position;
                position.x += char_width;

                next_char = chars.next();
                add_spacing = next_char.is_some();

                Some((p, LineElement::Char(c)))
            } else {
                Some((position, LineElement::Done))
            }
        })
    }

    fn draw_decorations<D>(
        &self,
        width: u32,
        position: Point,
        target: &mut D,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        if let Some(color) = self.strikethrough_color.to_color(self.text_color) {
            let rect = self.font.strikethrough.to_rectangle(position, width);
            target.fill_solid(&rect, color)?;
        }

        if let Some(color) = self.underline_color.to_color(self.text_color) {
            let rect = self.font.underline.to_rectangle(position, width);
            target.fill_solid(&rect, color)?;
        }

        Ok(())
    }

    fn draw_string_binary<D>(
        &self,
        text: &str,
        position: Point,
        mut target: D,
    ) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        for (p, element) in self.line_elements(position, text) {
            match element {
                LineElement::Char(c) => {
                    let glyph = self.font.glyph(c);
                    Image::new(&glyph, p).draw(&mut target)?;
                }
                // Fill space between characters if background color is set.
                LineElement::Spacing if self.font.character_spacing > 0 => {
                    if self.background_color.is_some() {
                        target.fill_solid(
                            &Rectangle::new(
                                p,
                                Size::new(
                                    self.font.character_spacing,
                                    self.font.character_size.height,
                                ),
                            ),
                            BinaryColor::Off,
                        )?;
                    }
                }
                LineElement::Spacing => {}
                LineElement::Done => return Ok(p),
            }
        }

        Ok(position)
    }

    /// Returns the vertical offset between the line position and the top edge of the bounding box.
    fn baseline_offset(&self, baseline: Baseline) -> i32 {
        match baseline {
            Baseline::Top => 0,
            Baseline::Bottom => self
                .font
                .character_size
                .height
                .saturating_sub(1)
                .saturating_as(),
            Baseline::Middle => {
                (self.font.character_size.height.saturating_sub(1) / 2).saturating_as()
            }
            Baseline::Alphabetic => self.font.baseline.saturating_as(),
        }
    }
}

impl<C: PixelColor> TextRenderer for MonoTextStyle<'_, C> {
    type Color = C;

    fn draw_string<D>(
        &self,
        text: &str,
        position: Point,
        baseline: Baseline,
        target: &mut D,
    ) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let position = position - Point::new(0, self.baseline_offset(baseline));

        let next = match (self.text_color, self.background_color) {
            (Some(text_color), Some(background_color)) => self.draw_string_binary(
                text,
                position,
                MonoFontDrawTarget::new(target, Both(text_color, background_color)),
            )?,
            (Some(text_color), None) => self.draw_string_binary(
                text,
                position,
                MonoFontDrawTarget::new(target, Foreground(text_color)),
            )?,
            (None, Some(background_color)) => self.draw_string_binary(
                text,
                position,
                MonoFontDrawTarget::new(target, Background(background_color)),
            )?,
            (None, None) => {
                let dx = (self.font.character_size.width + self.font.character_spacing)
                    * text.chars().count() as u32;

                position + Size::new(dx, 0)
            }
        };

        if next.x > position.x {
            let width = (next.x - position.x) as u32;
            self.draw_decorations(width, position, target)?;
        }

        Ok(next + Point::new(0, self.baseline_offset(baseline)))
    }

    fn draw_whitespace<D>(
        &self,
        width: u32,
        position: Point,
        baseline: Baseline,
        target: &mut D,
    ) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let position = position - Point::new(0, self.baseline_offset(baseline));

        if width != 0 {
            if let Some(background_color) = self.background_color {
                target.fill_solid(
                    &Rectangle::new(position, Size::new(width, self.font.character_size.height)),
                    background_color,
                )?;
            }

            self.draw_decorations(width, position, target)?;
        }

        Ok(position + Point::new(width.saturating_as(), self.baseline_offset(baseline)))
    }

    fn measure_string(&self, text: &str, position: Point, baseline: Baseline) -> TextMetrics {
        let bb_position = position - Point::new(0, self.baseline_offset(baseline));

        let bb_width = (text.chars().count() as u32
            * (self.font.character_size.width + self.font.character_spacing))
            .saturating_sub(self.font.character_spacing);

        let bb_height = if self.underline_color != DecorationColor::None {
            self.font.underline.height + self.font.underline.offset
        } else {
            self.font.character_size.height
        };

        let bb_size = Size::new(bb_width, bb_height);

        TextMetrics {
            bounding_box: Rectangle::new(bb_position, bb_size),
            next_position: position + bb_size.x_axis(),
        }
    }

    fn line_height(&self) -> u32 {
        self.font.character_size.height
    }
}

impl<C: PixelColor> CharacterStyle for MonoTextStyle<'_, C> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
enum LineElement {
    Char(char),
    Spacing,
    Done,
}

/// Text style builder for monospaced fonts.
///
/// Use this builder to create [`MonoTextStyle`]s for [`Text`].
///
/// # Examples
///
/// ## Render yellow text on a blue background
///
/// This uses the [`FONT_6X9`] font, but [other fonts] can also be used.
///
/// ```rust
/// use embedded_graphics::{
///     mono_font::{ascii::FONT_6X9, MonoTextStyle, MonoTextStyleBuilder},
///     pixelcolor::Rgb565,
///     prelude::*,
///     text::Text,
/// };
///
/// let style = MonoTextStyleBuilder::new()
///     .font(&FONT_6X9)
///     .text_color(Rgb565::YELLOW)
///     .background_color(Rgb565::BLUE)
///     .build();
///
/// let text = Text::new("Hello Rust!", Point::new(0, 0), style);
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
///     mono_font::{ascii::FONT_6X9, MonoTextStyle, MonoTextStyleBuilder},
///     pixelcolor::Rgb565,
///     prelude::*,
///     text::Text,
/// };
///
/// let style = MonoTextStyleBuilder::new()
///     .font(&FONT_6X9)
///     .text_color(Rgb565::WHITE)
///     .build();
///
/// let text = Text::new("Hello Rust!", Point::new(0, 0), style);
/// ```
///
/// ## Modifying an existing style
///
/// The builder can also be used to modify an existing style.
///
/// ```
/// use embedded_graphics::{
///     mono_font::{ascii::{FONT_6X9, FONT_10X20}, MonoTextStyle, MonoTextStyleBuilder},
///     pixelcolor::Rgb565,
///     prelude::*,
///     text::Text,
/// };
///
/// let style = MonoTextStyle::new(&FONT_6X9, Rgb565::YELLOW);
///
/// let style_larger = MonoTextStyleBuilder::from(&style)
///     .font(&FONT_10X20)
///     .build();
/// ```
///
/// [`FONT_6X9`]: crate::mono_font::ascii::FONT_6X9
/// [other fonts]: super
/// [`Text`]: crate::text::Text
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct MonoTextStyleBuilder<'a, C> {
    style: MonoTextStyle<'a, C>,
}

impl<C> MonoTextStyleBuilder<'_, C> {
    /// Creates a new text style builder.
    pub const fn new() -> Self {
        Self {
            style: MonoTextStyle {
                font: &super::NULL_FONT,
                background_color: None,
                text_color: None,
                underline_color: DecorationColor::None,
                strikethrough_color: DecorationColor::None,
            },
        }
    }
}

impl<'a, C> MonoTextStyleBuilder<'a, C> {
    /// Sets the font.
    pub fn font<'b>(self, font: &'b MonoFont<'b>) -> MonoTextStyleBuilder<'b, C> {
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

    /// Resets the text color to transparent.
    pub fn reset_text_color(mut self) -> Self {
        self.style.text_color = None;

        self
    }

    /// Resets the background color to transparent.
    pub fn reset_background_color(mut self) -> Self {
        self.style.background_color = None;

        self
    }

    /// Removes the underline decoration.
    pub fn reset_underline(mut self) -> Self {
        self.style.underline_color = DecorationColor::None;

        self
    }

    /// Removes the strikethrough decoration.
    pub fn reset_strikethrough(mut self) -> Self {
        self.style.strikethrough_color = DecorationColor::None;

        self
    }
}

impl<C: PixelColor> MonoTextStyleBuilder<'_, C> {
    /// Sets the text color.
    pub const fn text_color(mut self, text_color: C) -> Self {
        self.style.text_color = Some(text_color);

        self
    }

    /// Sets the background color.
    pub const fn background_color(mut self, background_color: C) -> Self {
        self.style.background_color = Some(background_color);

        self
    }

    /// Enables underline with a custom color.
    pub const fn underline_with_color(mut self, underline_color: C) -> Self {
        self.style.underline_color = DecorationColor::Custom(underline_color);

        self
    }

    /// Enables strikethrough with a custom color.
    pub const fn strikethrough_with_color(mut self, strikethrough_color: C) -> Self {
        self.style.strikethrough_color = DecorationColor::Custom(strikethrough_color);

        self
    }
}

impl<'a, C: PixelColor> MonoTextStyleBuilder<'a, C> {
    /// Builds the text style.
    ///
    /// This method can only be called after a font was set by using the [`font`] method. All other
    /// settings are optional and they will be set to their default value if they are missing.
    ///
    /// [`font`]: MonoTextStyleBuilder::font()
    pub const fn build(self) -> MonoTextStyle<'a, C> {
        self.style
    }
}

impl<'a, C: PixelColor> From<&MonoTextStyle<'a, C>> for MonoTextStyleBuilder<'a, C> {
    fn from(style: &MonoTextStyle<'a, C>) -> Self {
        Self { style: *style }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::Dimensions,
        image::ImageRaw,
        mock_display::MockDisplay,
        mono_font::{
            ascii::{FONT_10X20, FONT_6X9},
            iso_8859_1::FONT_6X9 as FONT_6X9_LATIN1,
            mapping,
            tests::*,
            DecorationDimensions,
        },
        pixelcolor::{BinaryColor, Rgb888, RgbColor},
        text::Text,
        Drawable,
    };

    const SPACED_FONT: MonoFont = MonoFont {
        character_spacing: 5,
        underline: DecorationDimensions::new(9, 1),
        ..FONT_6X9
    };

    #[test]
    fn builder_default() {
        assert_eq!(
            MonoTextStyleBuilder::<BinaryColor>::new()
                .font(&FONT_10X20)
                .build(),
            MonoTextStyle {
                font: &FONT_10X20,
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
                .font(&FONT_10X20)
                .text_color(BinaryColor::On)
                .build(),
            MonoTextStyle::new(&FONT_10X20, BinaryColor::On)
        );
    }

    #[test]
    fn builder_background_color() {
        assert_eq!(
            MonoTextStyleBuilder::new()
                .font(&FONT_10X20)
                .background_color(BinaryColor::On)
                .build(),
            {
                let mut style = MonoTextStyleBuilder::new().font(&FONT_10X20).build();

                style.text_color = None;
                style.background_color = Some(BinaryColor::On);

                style
            }
        );
    }

    #[test]
    fn builder_resets() {
        let base = MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(BinaryColor::On)
            .background_color(BinaryColor::On)
            .underline()
            .strikethrough();

        assert_eq!(
            base.clone().reset_text_color().build(),
            MonoTextStyleBuilder::new()
                .font(&FONT_10X20)
                .background_color(BinaryColor::On)
                .underline()
                .strikethrough()
                .build()
        );

        assert_eq!(
            base.clone().reset_background_color().build(),
            MonoTextStyleBuilder::new()
                .font(&FONT_10X20)
                .text_color(BinaryColor::On)
                .underline()
                .strikethrough()
                .build()
        );

        assert_eq!(
            base.clone().reset_underline().build(),
            MonoTextStyleBuilder::new()
                .font(&FONT_10X20)
                .text_color(BinaryColor::On)
                .background_color(BinaryColor::On)
                .strikethrough()
                .build()
        );

        assert_eq!(
            base.clone().reset_strikethrough().build(),
            MonoTextStyleBuilder::new()
                .font(&FONT_10X20)
                .text_color(BinaryColor::On)
                .background_color(BinaryColor::On)
                .underline()
                .build()
        );
    }

    #[test]
    fn underline_text_color() {
        let style = MonoTextStyleBuilder::new()
            .font(&FONT_6X9)
            .text_color(Rgb888::WHITE)
            .underline()
            .build();

        let mut display = MockDisplay::new();
        Text::new("ABC", Point::new(0, 6), style)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "                  ",
            "  W   WWWW    WW  ",
            " W W  W   W  W  W ",
            "W   W WWWW   W    ",
            "WWWWW W   W  W    ",
            "W   W W   W  W  W ",
            "W   W WWWW    WW  ",
            "                  ",
            "WWWWWWWWWWWWWWWWWW",
        ]);
    }

    #[test]
    fn underline_text_color_with_alignment() {
        let character_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X9)
            .text_color(Rgb888::WHITE)
            .underline()
            .build();

        let mut display = MockDisplay::new();
        Text::with_baseline("ABC", Point::new(0, 6), character_style, Baseline::Middle)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "                  ",
            "                  ",
            "                  ",
            "  W   WWWW    WW  ",
            " W W  W   W  W  W ",
            "W   W WWWW   W    ",
            "WWWWW W   W  W    ",
            "W   W W   W  W  W ",
            "W   W WWWW    WW  ",
            "                  ",
            "WWWWWWWWWWWWWWWWWW",
        ]);
    }

    #[test]
    fn underline_custom_color() {
        let style = MonoTextStyleBuilder::new()
            .font(&FONT_6X9)
            .text_color(Rgb888::WHITE)
            .underline_with_color(Rgb888::RED)
            .build();

        let mut display = MockDisplay::new();
        Text::new("ABC", Point::new(0, 6), style)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "                  ",
            "  W   WWWW    WW  ",
            " W W  W   W  W  W ",
            "W   W WWWW   W    ",
            "WWWWW W   W  W    ",
            "W   W W   W  W  W ",
            "W   W WWWW    WW  ",
            "                  ",
            "RRRRRRRRRRRRRRRRRR",
        ]);
    }

    #[test]
    fn strikethrough_text_color() {
        let style = MonoTextStyleBuilder::new()
            .font(&FONT_6X9)
            .text_color(Rgb888::WHITE)
            .strikethrough()
            .build();

        let mut display = MockDisplay::new();
        display.set_allow_overdraw(true);

        Text::new("ABC", Point::new(0, 6), style)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "                  ",
            "  W   WWWW    WW  ",
            " W W  W   W  W  W ",
            "W   W WWWW   W    ",
            "WWWWWWWWWWWWWWWWWW",
            "W   W W   W  W  W ",
            "W   W WWWW    WW  ",
        ]);
    }

    #[test]
    fn strikethrough_custom_color() {
        let style = MonoTextStyleBuilder::new()
            .font(&FONT_6X9)
            .text_color(Rgb888::WHITE)
            .strikethrough_with_color(Rgb888::RED)
            .build();

        let mut display = MockDisplay::new();
        display.set_allow_overdraw(true);

        Text::new("ABC", Point::new(0, 6), style)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "                  ",
            "  W   WWWW    WW  ",
            " W W  W   W  W  W ",
            "W   W WWWW   W    ",
            "RRRRRRRRRRRRRRRRRR",
            "W   W W   W  W  W ",
            "W   W WWWW    WW  ",
        ]);
    }

    #[test]
    fn whitespace_background() {
        let style = MonoTextStyleBuilder::new()
            .font(&FONT_6X9)
            .text_color(Rgb888::YELLOW)
            .background_color(Rgb888::WHITE)
            .build();

        let mut display = MockDisplay::new();
        style
            .draw_whitespace(4, Point::zero(), Baseline::Top, &mut display)
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
            "WWWW", //
        ]);
    }

    #[test]
    fn whitespace_decorations() {
        let style = MonoTextStyleBuilder::new()
            .font(&FONT_6X9)
            .text_color(Rgb888::YELLOW)
            .underline_with_color(Rgb888::GREEN)
            .strikethrough_with_color(Rgb888::RED)
            .build();

        let mut display = MockDisplay::new();
        style
            .draw_whitespace(3, Point::zero(), Baseline::Top, &mut display)
            .unwrap();

        display.assert_pattern(&[
            "   ", //
            "   ", //
            "   ", //
            "   ", //
            "RRR", //
            "   ", //
            "   ", //
            "   ", //
            "GGG", //
        ]);
    }

    #[test]
    fn whitespace_background_and_decorations() {
        let style = MonoTextStyleBuilder::new()
            .font(&FONT_6X9)
            .text_color(Rgb888::YELLOW)
            .background_color(Rgb888::WHITE)
            .underline_with_color(Rgb888::GREEN)
            .strikethrough_with_color(Rgb888::RED)
            .build();

        let mut display = MockDisplay::new();
        display.set_allow_overdraw(true);

        style
            .draw_whitespace(8, Point::zero(), Baseline::Top, &mut display)
            .unwrap();

        display.assert_pattern(&[
            "WWWWWWWW", //
            "WWWWWWWW", //
            "WWWWWWWW", //
            "WWWWWWWW", //
            "RRRRRRRR", //
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
            &SPACED_FONT,
            &[
                "                 ",
                " # #        # #  ",
                " # #        # #  ",
                "#####      ##### ",
                " # #        # #  ",
                "#####      ##### ",
                " # #        # #  ",
                " # #        # #  ",
            ],
        );
    }

    #[test]
    fn character_spacing_with_background() {
        let character_style = MonoTextStyleBuilder::new()
            .font(&SPACED_FONT)
            .text_color(BinaryColor::On)
            .background_color(BinaryColor::Off)
            .build();

        let mut display = MockDisplay::new();
        Text::with_baseline("##", Point::zero(), character_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            ".................",
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
            .font(&SPACED_FONT)
            .text_color(Rgb888::WHITE)
            .underline_with_color(Rgb888::GREEN)
            .strikethrough_with_color(Rgb888::RED)
            .build();

        let mut display = MockDisplay::new();
        display.set_allow_overdraw(true);

        Text::with_baseline("##", Point::zero(), character_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "                 ",
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
        let style = MonoTextStyleBuilder::new()
            .font(&SPACED_FONT)
            .text_color(BinaryColor::On)
            .build();

        assert_eq!(
            Text::with_baseline("#", Point::zero(), style, Baseline::Top).bounding_box(),
            Rectangle::new(Point::zero(), Size::new(6, 9)),
        );

        assert_eq!(
            Text::with_baseline("##", Point::zero(), style, Baseline::Top).bounding_box(),
            Rectangle::new(Point::zero(), Size::new(6 * 2 + 5, 9)),
        );
        assert_eq!(
            Text::with_baseline("###", Point::zero(), style, Baseline::Top).bounding_box(),
            Rectangle::new(Point::zero(), Size::new(6 * 3 + 5 * 2, 9)),
        );
    }

    #[test]
    fn underlined_character_dimensions() {
        let style = MonoTextStyleBuilder::new()
            .font(&SPACED_FONT)
            .text_color(BinaryColor::On)
            .underline()
            .build();

        assert_eq!(
            Text::with_baseline("#", Point::zero(), style, Baseline::Top).bounding_box(),
            Rectangle::new(Point::zero(), Size::new(6, 10)),
        );
    }

    #[test]
    fn control_characters() {
        let style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);

        let mut display = MockDisplay::new();
        style
            .draw_string("A\t\n\rB", Point::zero(), Baseline::Top, &mut display)
            .unwrap();

        let mut expected = MockDisplay::new();
        style
            .draw_string("A???B", Point::zero(), Baseline::Top, &mut expected)
            .unwrap();

        display.assert_eq(&expected);
    }

    #[test]
    fn character_style() {
        let mut style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);
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
                font: &FONT_6X9,
            }
        );
    }

    #[test]
    fn draw_string_return_value() {
        let style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);
        let start = Point::new(10, 20);
        let expected_next = start + Point::new(2 * 6, 0);

        for baseline in [
            Baseline::Top,
            Baseline::Middle,
            Baseline::Alphabetic,
            Baseline::Bottom,
        ]
        .iter()
        {
            let mut display = MockDisplay::new();
            let next = style
                .draw_string("AB", start, *baseline, &mut display)
                .unwrap();

            assert_eq!(
                next, expected_next,
                "Unexpected next point for {:?}: {:?} (expected {:?})",
                baseline, next, expected_next
            );
        }
    }

    #[test]
    fn draw_whitespace_return_value() {
        let style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);
        let start = Point::new(10, 20);
        let expected_next = start + Point::new(15, 0);

        for baseline in [
            Baseline::Top,
            Baseline::Middle,
            Baseline::Alphabetic,
            Baseline::Bottom,
        ]
        .iter()
        {
            let mut display = MockDisplay::new();
            let next = style
                .draw_whitespace(15, start, *baseline, &mut display)
                .unwrap();

            assert_eq!(
                next, expected_next,
                "Unexpected next point for {:?}: {:?} (expected {:?})",
                baseline, next, expected_next
            );
        }
    }

    #[test]
    fn latin1_text_dimensions_one_line() {
        let position = Point::new(5, 5);

        let style = MonoTextStyleBuilder::<BinaryColor>::new()
            .font(&FONT_6X9_LATIN1)
            .build();
        let text = Text::with_baseline("123°§£", position, style, Baseline::Top);

        assert_eq!(
            text.bounding_box(),
            Rectangle::new(
                position,
                FONT_6X9_LATIN1
                    .character_size
                    .component_mul(Size::new(6, 1))
            )
        );

        let mut display = MockDisplay::new();
        let next = text.draw(&mut display).unwrap();

        assert_eq!(next, position + FONT_6X9_LATIN1.character_size.x_axis() * 6);
    }

    #[test]
    fn transparent_text_dimensions_one_line() {
        let position = Point::new(5, 5);

        let style = MonoTextStyleBuilder::<BinaryColor>::new()
            .font(&FONT_6X9)
            .build();
        let text = Text::with_baseline("123", position, style, Baseline::Top);

        assert_eq!(
            text.bounding_box(),
            Rectangle::new(
                position,
                FONT_6X9.character_size.component_mul(Size::new(3, 1))
            )
        );

        let mut display = MockDisplay::new();
        let next = text.draw(&mut display).unwrap();

        assert_eq!(next, position + FONT_6X9.character_size.x_axis() * 3);
    }

    #[test]
    fn transparent_text_dimensions_one_line_spaced() {
        let position = Point::new(5, 5);

        let style = MonoTextStyleBuilder::<BinaryColor>::new()
            .font(&SPACED_FONT)
            .build();
        let text = Text::with_baseline("123", position, style, Baseline::Top);

        assert_eq!(
            text.bounding_box(),
            Rectangle::new(
                position,
                SPACED_FONT.character_size.component_mul(Size::new(3, 1))
                    + Size::new(SPACED_FONT.character_spacing, 0) * 2
            )
        );

        let mut display = MockDisplay::new();
        let next = text.draw(&mut display).unwrap();

        assert_eq!(
            next,
            position
                + (SPACED_FONT.character_size.x_axis()
                    + Size::new(SPACED_FONT.character_spacing, 0))
                    * 3
        );
    }

    #[test]
    fn transparent_text_dimensions_two_lines() {
        let position = Point::new(5, 5);

        let style = MonoTextStyleBuilder::<BinaryColor>::new()
            .font(&FONT_6X9)
            .build();
        let text = Text::with_baseline("123\n1", position, style, Baseline::Top);

        assert_eq!(
            text.bounding_box(),
            Rectangle::new(
                position,
                FONT_6X9.character_size.component_mul(Size::new(3, 2))
            )
        );

        let mut display = MockDisplay::new();
        let next = text.draw(&mut display).unwrap();

        assert_eq!(next, position + FONT_6X9.character_size);
    }

    #[test]
    fn elements_iter() {
        let style = MonoTextStyle::new(&SPACED_FONT, BinaryColor::On);

        let mut iter = style.line_elements(Point::new(10, 20), "");
        assert_eq!(iter.next(), Some((Point::new(10, 20), LineElement::Done)));

        let mut iter = style.line_elements(Point::new(10, 20), "a");
        assert_eq!(
            iter.next(),
            Some((Point::new(10, 20), LineElement::Char('a')))
        );
        assert_eq!(iter.next(), Some((Point::new(16, 20), LineElement::Done)));

        let mut iter = style.line_elements(Point::new(10, 20), "abc");
        assert_eq!(
            iter.next(),
            Some((Point::new(10, 20), LineElement::Char('a')))
        );
        assert_eq!(
            iter.next(),
            Some((Point::new(16, 20), LineElement::Spacing))
        );
        assert_eq!(
            iter.next(),
            Some((Point::new(21, 20), LineElement::Char('b')))
        );
        assert_eq!(
            iter.next(),
            Some((Point::new(27, 20), LineElement::Spacing))
        );
        assert_eq!(
            iter.next(),
            Some((Point::new(32, 20), LineElement::Char('c')))
        );
        assert_eq!(iter.next(), Some((Point::new(38, 20), LineElement::Done)));
    }

    #[test]
    fn builder_change_font() {
        let _style = {
            let font = MonoFont {
                image: ImageRaw::new(&[1, 2, 3], 1),
                character_size: Size::new(1, 2),
                character_spacing: 0,
                baseline: 0,
                strikethrough: DecorationDimensions::default_strikethrough(2),
                underline: DecorationDimensions::default_underline(2),
                glyph_mapping: &mapping::ASCII,
            };

            let style = MonoTextStyleBuilder::new()
                .font(&font)
                .text_color(BinaryColor::On)
                .build();

            // `style` cannot be returned from this block, because if is limited by the lifetime of
            // `font`. But it should be possible to return `style2` because `FONT_6X9` is a const.
            let style2 = MonoTextStyleBuilder::from(&style).font(&FONT_6X9).build();

            style2
        };
    }
}
