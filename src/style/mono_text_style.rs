use crate::{
    draw_target::DrawTarget,
    fonts::{MonoCharPixels, MonoFont},
    geometry::{Point, Size},
    pixelcolor::{BinaryColor, PixelColor},
    primitives::Rectangle,
    style::TextStyle,
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
/// [`Text`]: ../fonts/struct.Text.html
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

    /// Horizontal alignment.
    pub horizontal_alignment: HorizontalAlignment,

    /// Vertical alignment.
    pub vertical_alignment: VerticalAlignment,

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

    /// Calculates the line width in pixels.
    fn line_width(&self, text: &str) -> u32 {
        (text.len() as u32 * (F::CHARACTER_SIZE.width + F::CHARACTER_SPACING))
            .saturating_sub(F::CHARACTER_SPACING)
    }

    /// Calculates the offset between the line position and the top left corner of the bounding
    /// box.
    fn position_offset(&self, text: &str) -> Point {
        let x = match self.horizontal_alignment {
            HorizontalAlignment::Left => 0,
            HorizontalAlignment::Right => self.line_width(text).saturating_sub(1),
            HorizontalAlignment::Center => self.line_width(text).saturating_sub(1) / 2,
        }
        .saturating_cast();

        let y = match self.vertical_alignment {
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

        Point::new(x, y)
    }
}

impl<C, F> TextStyle for MonoTextStyle<C, F>
where
    C: PixelColor,
    F: MonoFont,
{
    type Color = C;

    fn render_line<D>(&self, text: &str, position: Point, target: &mut D) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let mut first = true;

        let mut position = position - self.position_offset(text);

        for c in text.chars() {
            if first {
                first = false;
            } else if F::CHARACTER_SPACING > 0 {
                // Fill space between characters if background color is set.
                if let Some(background_color) = self.background_color {
                    target.fill_solid(
                        &Rectangle::new(
                            position,
                            Size::new(F::CHARACTER_SPACING, F::CHARACTER_SIZE.height),
                        ),
                        background_color,
                    )?;
                }

                position += Size::new(F::CHARACTER_SPACING, 0);
            }

            let pixels = MonoCharPixels::<F>::new(c);

            match (self.text_color, self.background_color) {
                (Some(text_color), Some(background_color)) => {
                    let bounding_box = Rectangle::new(position, F::CHARACTER_SIZE);

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
                            .map(|Pixel(p, _)| Pixel(p + position, text_color)),
                    )?;
                }
                (None, Some(background_color)) => {
                    target.draw_iter(
                        pixels
                            .filter(|Pixel(_, c)| *c == BinaryColor::Off)
                            .map(|Pixel(p, _)| Pixel(p + position, background_color)),
                    )?;
                }
                (None, None) => {}
            }

            position += F::CHARACTER_SIZE.x_axis();
        }

        Ok(Point::zero() + F::CHARACTER_SIZE.y_axis())
    }

    fn line_bounding_box(&self, text: &str, position: Point) -> (Rectangle, Point) {
        let next_line_delta = Point::zero() + F::CHARACTER_SIZE.y_axis();

        let position = position - self.position_offset(text);

        // If a piece of text is completely transparent, return an empty bounding box
        if self.text_color.is_none() && self.background_color.is_none() {
            return (Rectangle::new(position, Size::zero()), next_line_delta);
        }

        let size = Size::new(self.line_width(text), F::CHARACTER_SIZE.height);

        (Rectangle::new(position, size), next_line_delta)
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
///     fonts::{Font6x8, Text},
///     pixelcolor::Rgb565,
///     prelude::*,
///     style::{MonoTextStyle, MonoTextStyleBuilder},
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
///     fonts::{Font6x8, Text},
///     pixelcolor::Rgb565,
///     prelude::*,
///     style::{MonoTextStyle, MonoTextStyleBuilder},
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
///     fonts::{Font6x8, Font12x16, Text},
///     pixelcolor::Rgb565,
///     prelude::*,
///     style::{MonoTextStyle, MonoTextStyleBuilder},
/// };
///
/// let style = MonoTextStyle::new(Font6x8, Rgb565::YELLOW);
///
/// let style_larger = MonoTextStyleBuilder::from(&style)
///     .font(Font12x16)
///     .build();
/// ```
///
/// [`Font`]: ../fonts/trait.Font.html
/// [`Font6x8`]: ../fonts/struct.Font6x8.html
/// [`Font8x16`]: ../fonts/struct.Font8x16.html
/// [other fonts]: ../fonts/index.html
/// [`Text`]: ../fonts/struct.Text.html
/// [`MonoTextStyle`]: ./struct.MonoTextStyle.html
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
                horizontal_alignment: HorizontalAlignment::Left,
                vertical_alignment: VerticalAlignment::Baseline,
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
            vertical_alignment: self.style.vertical_alignment,
            horizontal_alignment: self.style.horizontal_alignment,
        };

        MonoTextStyleBuilder { style }
    }

    /// Sets the horizontal alignment.
    pub fn horizontal_alignment(mut self, horizontal_alignment: HorizontalAlignment) -> Self {
        self.style.horizontal_alignment = horizontal_alignment;

        self
    }

    /// Sets the vertical alignment.
    pub fn vertical_alignment(mut self, vertical_alignment: VerticalAlignment) -> Self {
        self.style.vertical_alignment = vertical_alignment;

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

/// Vertical text alignment.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum VerticalAlignment {
    /// Top.
    Top,
    /// Bottom.
    Bottom,
    /// Center.
    Center,
    /// Baseline.
    Baseline,
}

/// Horizontal text alignment.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum HorizontalAlignment {
    /// Left.
    Left,
    /// Center.
    Center,
    /// Right.
    Right,
}

/// Marker type to improve compiler errors if no font was set in builder.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct UndefinedFont;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        fonts::Font12x16, fonts::Font6x8, fonts::Text, geometry::Dimensions,
        mock_display::MockDisplay, pixelcolor::BinaryColor, Drawable,
    };

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
                horizontal_alignment: HorizontalAlignment::Left,
                vertical_alignment: VerticalAlignment::Baseline,
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
    fn builder_alignments() {
        let style = MonoTextStyleBuilder::<BinaryColor, _>::new()
            .font(Font12x16)
            .horizontal_alignment(HorizontalAlignment::Right)
            .vertical_alignment(VerticalAlignment::Top)
            .build();

        assert_eq!(style.horizontal_alignment, HorizontalAlignment::Right);
        assert_eq!(style.vertical_alignment, VerticalAlignment::Top);
    }

    #[test]
    fn horizontal_alignment_left() {
        let style = MonoTextStyleBuilder::new()
            .font(Font6x8)
            .text_color(BinaryColor::On)
            .horizontal_alignment(HorizontalAlignment::Left)
            .build();

        let mut display = MockDisplay::new();
        Text::new("A\nBC", Point::new(0, 6))
            .into_styled(style)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            " ###        ",
            "#   #       ",
            "#   #       ",
            "#####       ",
            "#   #       ",
            "#   #       ",
            "#   #       ",
            "            ",
            "####   ###  ",
            "#   # #   # ",
            "#   # #     ",
            "####  #     ",
            "#   # #     ",
            "#   # #   # ",
            "####   ###  ",
            "            ",
        ]);
    }

    #[test]
    fn horizontal_alignment_center() {
        let style = MonoTextStyleBuilder::new()
            .font(Font6x8)
            .text_color(BinaryColor::On)
            .horizontal_alignment(HorizontalAlignment::Center)
            .build();

        let mut display = MockDisplay::new();
        Text::new("A\nBC", Point::new(5, 6))
            .into_styled(style)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "    ###     ",
            "   #   #    ",
            "   #   #    ",
            "   #####    ",
            "   #   #    ",
            "   #   #    ",
            "   #   #    ",
            "            ",
            "####   ###  ",
            "#   # #   # ",
            "#   # #     ",
            "####  #     ",
            "#   # #     ",
            "#   # #   # ",
            "####   ###  ",
            "            ",
        ]);
    }

    #[test]
    fn horizontal_alignment_right() {
        let style = MonoTextStyleBuilder::new()
            .font(Font6x8)
            .text_color(BinaryColor::On)
            .horizontal_alignment(HorizontalAlignment::Right)
            .build();

        let mut display = MockDisplay::new();
        Text::new("A\nBC", Point::new(11, 6))
            .into_styled(style)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "       ###  ",
            "      #   # ",
            "      #   # ",
            "      ##### ",
            "      #   # ",
            "      #   # ",
            "      #   # ",
            "            ",
            "####   ###  ",
            "#   # #   # ",
            "#   # #     ",
            "####  #     ",
            "#   # #     ",
            "#   # #   # ",
            "####   ###  ",
            "            ",
        ]);
    }

    #[test]
    fn vertical_alignment() {
        let mut display = MockDisplay::new();

        let style_top = MonoTextStyleBuilder::new()
            .font(Font6x8)
            .text_color(BinaryColor::On)
            .vertical_alignment(VerticalAlignment::Top)
            .build();
        let style_center = MonoTextStyleBuilder::from(&style_top)
            .vertical_alignment(VerticalAlignment::Center)
            .build();
        let style_bottom = MonoTextStyleBuilder::from(&style_top)
            .vertical_alignment(VerticalAlignment::Bottom)
            .build();
        let style_baseline = MonoTextStyleBuilder::from(&style_top)
            .vertical_alignment(VerticalAlignment::Baseline)
            .build();

        Text::new("t", Point::new(0, 8))
            .into_styled(style_top)
            .draw(&mut display)
            .unwrap();
        Text::new("c", Point::new(6, 8))
            .into_styled(style_center)
            .draw(&mut display)
            .unwrap();
        Text::new("b", Point::new(12, 8))
            .into_styled(style_bottom)
            .draw(&mut display)
            .unwrap();
        Text::new("B", Point::new(18, 8))
            .into_styled(style_baseline)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "                       ",
            "            #          ",
            "            #     #### ",
            "            # ##  #   #",
            "            ##  # #   #",
            "            #   # #### ",
            "            #   # #   #",
            "       ###  ####  #   #",
            " #    #           #### ",
            " #    #                ",
            "###   #   #            ",
            " #     ###             ",
            " #                     ",
            " #  #                  ",
            "  ##                   ",
        ]);
    }

    #[test]
    fn bounding_box() {
        for &vertical_alignment in &[
            VerticalAlignment::Top,
            VerticalAlignment::Center,
            VerticalAlignment::Bottom,
            VerticalAlignment::Baseline,
        ] {
            for &horizontal_alignment in &[
                HorizontalAlignment::Left,
                HorizontalAlignment::Center,
                HorizontalAlignment::Right,
            ] {
                let style = MonoTextStyleBuilder::new()
                    .font(Font6x8)
                    .text_color(BinaryColor::On)
                    .background_color(BinaryColor::Off)
                    .horizontal_alignment(horizontal_alignment)
                    .vertical_alignment(vertical_alignment)
                    .build();

                let text = Text::new("1\n23", Point::new_equal(20)).into_styled(style);

                let mut display = MockDisplay::new();
                text.draw(&mut display).unwrap();

                assert_eq!(display.affected_area(), text.bounding_box());
            }
        }
    }
}
