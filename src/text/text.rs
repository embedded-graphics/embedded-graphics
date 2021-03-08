use crate::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point, Size},
    pixelcolor::PixelColor,
    primitives::Rectangle,
    text::{
        renderer::{TextMetrics, TextRenderer},
        Alignment, Baseline, TextStyle,
    },
    transform::Transform,
    Drawable, SaturatingCast, Styled,
};

/// A text object.
///
/// The `Text` struct represents a string that can be drawn onto a display.
///
/// The text object only contains the string and position and no additional styling information,
/// like the font or color. To draw a text object it is necessary to attach a style to it by using
/// the [`into_styled`] method to create a [`Styled`] object.
///
/// See the [module-level documentation] for examples how to use text objects.
///
/// [`into_styled`]: #method.into_styled
/// [`Styled`]: ../struct.Styled.html
/// [module-level documentation]: index.html
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Text<'a> {
    /// The string.
    pub text: &'a str,

    /// The position.
    ///
    /// Defines the top-left starting pixel of the text object.
    pub position: Point,
}

impl<'a> Text<'a> {
    /// Creates a text.
    pub const fn new(text: &'a str, position: Point) -> Self {
        Self { text, position }
    }

    /// Attaches a text style to the text object.
    pub fn into_styled<S>(self, style: S) -> Styled<Self, S> {
        Styled::new(self, style)
    }
}

impl Transform for Text<'_> {
    fn translate(&self, by: Point) -> Self {
        Self {
            position: self.position + by,
            ..*self
        }
    }

    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.position += by;

        self
    }
}

impl<C, S> Styled<Text<'_>, S>
where
    C: PixelColor,
    S: TextRenderer<Color = C>,
{
    fn lines(&self) -> impl Iterator<Item = (&str, Point)> {
        let mut position = self.primitive.position;

        self.primitive.text.lines().map(move |line| {
            let p = position;

            position.y += self.style.line_height().saturating_cast();

            (line, p)
        })
    }
}

impl<C, S> Styled<Text<'_>, TextStyle<S>>
where
    C: PixelColor,
    S: TextRenderer<Color = C>,
{
    fn lines(&self) -> impl Iterator<Item = (&str, Point)> {
        let mut position = self.primitive.position;

        self.primitive.text.lines().map(move |line| {
            let p = match self.style.alignment {
                Alignment::Left => position,
                Alignment::Right => {
                    let metrics = self.style.character_style.measure_string(
                        line,
                        Point::zero(),
                        self.style.baseline,
                    );
                    position - (metrics.next_position - Point::new(1, 0))
                }
                Alignment::Center => {
                    let metrics = self.style.character_style.measure_string(
                        line,
                        Point::zero(),
                        self.style.baseline,
                    );
                    position - (metrics.next_position - Point::new(1, 0)) / 2
                }
            };

            position.y += self.style.character_style.line_height().saturating_cast();

            (line, p)
        })
    }
}

impl<C, S> Drawable for Styled<Text<'_>, S>
where
    C: PixelColor,
    S: TextRenderer<Color = C>,
{
    type Color = C;
    type Output = Point;

    fn draw<D>(&self, target: &mut D) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        let mut next_position = self.primitive.position;

        for (line, position) in self.lines() {
            next_position = self
                .style
                .draw_string(line, position, Baseline::Alphabetic, target)?;
        }

        Ok(next_position)
    }
}

impl<C, S> Drawable for Styled<Text<'_>, TextStyle<S>>
where
    C: PixelColor,
    S: TextRenderer<Color = C>,
{
    type Color = C;
    type Output = Point;

    fn draw<D>(&self, target: &mut D) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        let mut next_position = self.primitive.position;

        for (line, position) in self.lines() {
            next_position = self.style.character_style.draw_string(
                line,
                position,
                self.style.baseline,
                target,
            )?;
        }

        Ok(next_position)
    }
}

fn update_min_max(min_max: &mut Option<(Point, Point)>, metrics: &TextMetrics) {
    if let Some(bottom_right) = metrics.bounding_box.bottom_right() {
        if let Some((min, max)) = min_max {
            min.x = min.x.min(metrics.bounding_box.top_left.x);
            min.y = min.y.min(metrics.bounding_box.top_left.y);
            max.x = max.x.max(bottom_right.x);
            max.y = max.y.max(bottom_right.y);
        } else {
            *min_max = Some((metrics.bounding_box.top_left, bottom_right));
        }
    }
}

impl<C, S> Dimensions for Styled<Text<'_>, S>
where
    C: PixelColor,
    S: TextRenderer<Color = C>,
{
    fn bounding_box(&self) -> Rectangle {
        let mut min_max: Option<(Point, Point)> = None;

        for (line, position) in self.lines() {
            let metrics = self
                .style
                .measure_string(line, position, Baseline::Alphabetic);
            update_min_max(&mut min_max, &metrics);
        }

        if let Some((min, max)) = min_max {
            Rectangle::with_corners(min, max)
        } else {
            Rectangle::new(self.primitive.position, Size::zero())
        }
    }
}

impl<C, S> Dimensions for Styled<Text<'_>, TextStyle<S>>
where
    C: PixelColor,
    S: TextRenderer<Color = C>,
{
    fn bounding_box(&self) -> Rectangle {
        let mut min_max: Option<(Point, Point)> = None;

        for (line, position) in self.lines() {
            let metrics =
                self.style
                    .character_style
                    .measure_string(line, position, self.style.baseline);
            update_min_max(&mut min_max, &metrics);
        }

        if let Some((min, max)) = min_max {
            Rectangle::with_corners(min, max)
        } else {
            Rectangle::new(self.primitive.position, Size::zero())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::Size,
        mock_display::MockDisplay,
        mono_font::{
            ascii::{Font6x13, Font6x9},
            tests::assert_text_from_pattern,
            MonoTextStyle, MonoTextStyleBuilder,
        },
        pixelcolor::BinaryColor,
        primitives::{Primitive, PrimitiveStyle},
        text::{Alignment, Baseline, TextStyleBuilder},
    };

    const HELLO_WORLD: &'static str = "Hello World!";

    #[test]
    fn constructor() {
        let text = Text::new("Hello e-g", Point::new(10, 11));

        assert_eq!(
            text,
            Text {
                text: "Hello e-g",
                position: Point::new(10, 11),
            }
        );
    }

    #[test]
    fn multiline() {
        assert_text_from_pattern(
            "AB\nC",
            Font6x9,
            &[
                "            ",
                "  #   ####  ",
                " # #  #   # ",
                "#   # ####  ",
                "##### #   # ",
                "#   # #   # ",
                "#   # ####  ",
                "            ",
                "            ",
                "            ",
                "  ##        ",
                " #  #       ",
                " #          ",
                " #          ",
                " #  #       ",
                "  ##        ",
            ],
        );
    }

    #[test]
    fn multiline_empty_line() {
        assert_text_from_pattern(
            "A\n\nBC",
            Font6x9,
            &[
                "            ",
                "  #         ",
                " # #        ",
                "#   #       ",
                "#####       ",
                "#   #       ",
                "#   #       ",
                "            ",
                "            ",
                "            ",
                "            ",
                "            ",
                "            ",
                "            ",
                "            ",
                "            ",
                "            ",
                "            ",
                "            ",
                "####    ##  ",
                "#   #  #  # ",
                "####   #    ",
                "#   #  #    ",
                "#   #  #  # ",
                "####    ##  ",
                "            ",
            ],
        );
    }

    #[test]
    fn multiline_dimensions() {
        let character_style = MonoTextStyleBuilder::new()
            .font(Font6x9)
            .text_color(BinaryColor::On)
            .build();

        let text_style = TextStyleBuilder::new()
            .character_style(character_style)
            .baseline(Baseline::Top)
            .build();

        let text = Text::new("AB\nC", Point::zero()).into_styled(text_style);

        assert_eq!(
            text.bounding_box(),
            Rectangle::new(Point::zero(), Size::new(2 * 6, 2 * 9))
        );
    }

    #[test]
    fn position_and_translate() {
        let style = MonoTextStyle::new(Font6x9, BinaryColor::On);

        let hello = Text::new(HELLO_WORLD, Point::zero()).into_styled(style);

        let hello_translated = hello.translate(Point::new(5, -20));
        assert_eq!(
            hello.bounding_box().size,
            hello_translated.bounding_box().size
        );

        let mut hello_with_point = Text::new(HELLO_WORLD, Point::new(5, -20)).into_styled(style);
        assert_eq!(hello_translated, hello_with_point);

        hello_with_point.translate_mut(Point::new(-5, 20));
        assert_eq!(hello, hello_with_point);
    }

    #[test]
    fn inverted_text() {
        let mut display_inverse = MockDisplay::new();
        let style_inverse = MonoTextStyleBuilder::new()
            .font(Font6x9)
            .text_color(BinaryColor::Off)
            .background_color(BinaryColor::On)
            .build();
        Text::new("Mm", Point::new(0, 7))
            .into_styled(style_inverse)
            .draw(&mut display_inverse)
            .unwrap();

        let mut display_normal = MockDisplay::new();
        let style_normal = MonoTextStyleBuilder::new()
            .font(Font6x9)
            .text_color(BinaryColor::On)
            .background_color(BinaryColor::Off)
            .build();
        Text::new("Mm", Point::new(0, 7))
            .into_styled(style_normal)
            .draw(&mut display_normal)
            .unwrap();

        display_inverse.assert_eq(&display_normal.map(|c| c.invert()));
    }

    #[test]
    fn no_fill_does_not_hang() {
        let mut display = MockDisplay::new();
        Text::new(" ", Point::zero())
            .into_styled(MonoTextStyle::new(Font6x9, BinaryColor::On))
            .draw(&mut display)
            .unwrap();

        display.assert_eq(&MockDisplay::new());
    }

    #[test]
    fn transparent_text_color_does_not_overwrite_background() {
        let character_style = MonoTextStyleBuilder::new()
            .font(Font6x9)
            .background_color(BinaryColor::On)
            .build();

        let text_style = TextStyleBuilder::new()
            .character_style(character_style)
            .baseline(Baseline::Top)
            .build();

        let mut display = MockDisplay::new();
        display.set_allow_overdraw(true);

        // Draw a background for the first character
        Rectangle::new(Point::zero(), Size::new(6, 8))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
            .draw(&mut display)
            .unwrap();

        Text::new("AA", Point::zero())
            .into_styled(text_style)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "############",
            "##.##### ###",
            "#.#.### # ##",
            ".###.# ### #",
            ".....#     #",
            ".###.# ### #",
            ".###.# ### #",
            "############",
            "############",
        ]);
    }

    #[test]
    #[ignore]
    fn transparent_text_has_zero_size_but_retains_position() {
        let style = MonoTextStyleBuilder::<BinaryColor, _>::new()
            .font(Font6x9)
            .build();

        let styled = Text::new(" A", Point::new(7, 11)).into_styled(style);

        assert_eq!(
            styled.bounding_box(),
            Rectangle::new(Point::new(7, 11), Size::zero()),
            "Transparent text is expected to have a zero sized bounding box with the top left corner at the text position",
        );
    }

    #[test]
    fn alignment_left() {
        let character_style = MonoTextStyleBuilder::new()
            .font(Font6x9)
            .text_color(BinaryColor::On)
            .build();

        let text_style = TextStyleBuilder::new()
            .character_style(character_style)
            .alignment(Alignment::Left)
            .baseline(Baseline::Top)
            .build();

        let mut display = MockDisplay::new();
        Text::new("A\nBC", Point::new(0, 0))
            .into_styled(text_style)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "            ",
            "  #         ",
            " # #        ",
            "#   #       ",
            "#####       ",
            "#   #       ",
            "#   #       ",
            "            ",
            "            ",
            "            ",
            "####    ##  ",
            "#   #  #  # ",
            "####   #    ",
            "#   #  #    ",
            "#   #  #  # ",
            "####    ##  ",
            "            ",
        ]);
    }

    #[test]
    fn alignment_center() {
        let character_style = MonoTextStyleBuilder::new()
            .font(Font6x9)
            .text_color(BinaryColor::On)
            .build();

        let text_style = TextStyleBuilder::new()
            .character_style(character_style)
            .alignment(Alignment::Center)
            .baseline(Baseline::Top)
            .build();

        let mut display = MockDisplay::new();
        Text::new("A\nBC", Point::new(5, 0))
            .into_styled(text_style)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "            ",
            "     #      ",
            "    # #     ",
            "   #   #    ",
            "   #####    ",
            "   #   #    ",
            "   #   #    ",
            "            ",
            "            ",
            "            ",
            "####    ##  ",
            "#   #  #  # ",
            "####   #    ",
            "#   #  #    ",
            "#   #  #  # ",
            "####    ##  ",
            "            ",
        ]);
    }

    #[test]
    fn horizontal_alignment_right() {
        let character_style = MonoTextStyleBuilder::new()
            .font(Font6x9)
            .text_color(BinaryColor::On)
            .build();

        let text_style = TextStyleBuilder::new()
            .character_style(character_style)
            .alignment(Alignment::Right)
            .baseline(Baseline::Top)
            .build();

        let mut display = MockDisplay::new();
        Text::new("A\nBC", Point::new(11, 0))
            .into_styled(text_style)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "            ",
            "        #   ",
            "       # #  ",
            "      #   # ",
            "      ##### ",
            "      #   # ",
            "      #   # ",
            "            ",
            "            ",
            "            ",
            "####    ##  ",
            "#   #  #  # ",
            "####   #    ",
            "#   #  #    ",
            "#   #  #  # ",
            "####    ##  ",
            "            ",
        ]);
    }

    #[test]
    fn baseline() {
        let mut display = MockDisplay::new();

        let character_style = MonoTextStyleBuilder::new()
            .font(Font6x9)
            .text_color(BinaryColor::On)
            .build();

        let style_top = TextStyleBuilder::new()
            .character_style(character_style)
            .baseline(Baseline::Top)
            .build();
        let style_middle = TextStyleBuilder::new()
            .character_style(character_style)
            .baseline(Baseline::Middle)
            .build();
        let style_bottom = TextStyleBuilder::new()
            .character_style(character_style)
            .baseline(Baseline::Bottom)
            .build();
        let style_baseline = TextStyleBuilder::new()
            .character_style(character_style)
            .baseline(Baseline::Alphabetic)
            .build();

        Text::new("t", Point::new(0, 8))
            .into_styled(style_top)
            .draw(&mut display)
            .unwrap();
        Text::new("m", Point::new(6, 8))
            .into_styled(style_middle)
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
            "             #         ",
            "             #         ",
            "             ###  #### ",
            "             #  # #   #",
            "             #  # #### ",
            "             ###  #   #",
            "      ## #        #   #",
            "      # # #       #### ",
            "  #   # # #            ",
            "  #   #   #            ",
            " ###                   ",
            "  #                    ",
            "  # #                  ",
            "   #                   ",
        ]);
    }

    #[test]
    fn bounding_box() {
        for &baseline in &[
            Baseline::Top,
            Baseline::Middle,
            Baseline::Bottom,
            Baseline::Alphabetic,
        ] {
            for &alignment in &[Alignment::Left, Alignment::Center, Alignment::Right] {
                let character_style = MonoTextStyleBuilder::new()
                    .font(Font6x9)
                    .text_color(BinaryColor::On)
                    .background_color(BinaryColor::Off)
                    .build();

                let text_style = TextStyleBuilder::new()
                    .character_style(character_style)
                    .alignment(alignment)
                    .baseline(baseline)
                    .build();

                let text = Text::new("1\n23", Point::new_equal(20)).into_styled(text_style);

                let mut display = MockDisplay::new();
                text.draw(&mut display).unwrap();

                assert_eq!(
                    display.affected_area(),
                    text.bounding_box(),
                    "alignment: {:?}, baseline: {:?}",
                    alignment,
                    baseline
                );
            }
        }
    }

    #[test]
    fn chained_text_drawing() {
        let character_style1 = MonoTextStyleBuilder::new()
            .font(Font6x9)
            .text_color(BinaryColor::On)
            .build();

        let character_style2 = MonoTextStyleBuilder::new()
            .font(Font6x13)
            .text_color(BinaryColor::Off)
            .build();

        let mut display = MockDisplay::new();
        let next = Text::new("AB", Point::new(0, 8))
            .into_styled(character_style1)
            .draw(&mut display)
            .unwrap();
        Text::new("C", next)
            .into_styled(character_style2)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "             ...  ",
            "            .   . ",
            "            .     ",
            "  #   ####  .     ",
            " # #  #   # .     ",
            "#   # ####  .     ",
            "##### #   # .     ",
            "#   # #   # .   . ",
            "#   # ####   ...  ",
        ]);
    }
}
