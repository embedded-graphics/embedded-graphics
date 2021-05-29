use crate::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point, Size},
    primitives::Rectangle,
    text::{
        renderer::{TextMetrics, TextRenderer},
        Alignment, Baseline, TextStyle,
    },
    transform::Transform,
    Drawable,
};
use az::SaturatingAs;

use super::TextStyleBuilder;
/// Text drawable.
///
/// A text drawable can be used to draw text to a draw target.
///
/// See the [module-level documentation] for more information about text drawables and examples.
///
/// [module-level documentation]: index.html
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Text<'a, S> {
    /// The string.
    pub text: &'a str,

    /// The position.
    pub position: Point,

    /// The character style.
    pub character_style: S,

    /// The text style.
    pub text_style: TextStyle,
}

impl<'a, S> Text<'a, S> {
    /// Creates a text drawable with the default text style.
    pub const fn new(text: &'a str, position: Point, character_style: S) -> Self {
        Self {
            text,
            position,
            character_style,
            text_style: TextStyleBuilder::new().build(),
        }
    }

    /// Creates a text drawable with the given text style.
    pub const fn with_text_style(
        text: &'a str,
        position: Point,
        character_style: S,
        text_style: TextStyle,
    ) -> Self {
        Self {
            text,
            position,
            character_style,
            text_style,
        }
    }

    /// Creates a text drawable with the given baseline.
    pub const fn with_baseline(
        text: &'a str,
        position: Point,
        character_style: S,
        baseline: Baseline,
    ) -> Self {
        Self {
            text,
            position,
            character_style,
            text_style: TextStyle::with_baseline(baseline),
        }
    }

    /// Creates a text drawable with the given alignment.
    pub const fn with_alignment(
        text: &'a str,
        position: Point,
        character_style: S,
        alignment: Alignment,
    ) -> Self {
        Self {
            text,
            position,
            character_style,
            text_style: TextStyle::with_alignment(alignment),
        }
    }
}

impl<S: Clone> Transform for Text<'_, S> {
    fn translate(&self, by: Point) -> Self {
        Self {
            position: self.position + by,
            ..self.clone()
        }
    }

    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.position += by;

        self
    }
}

impl<S: TextRenderer> Text<'_, S> {
    fn lines(&self) -> impl Iterator<Item = (&str, Point)> {
        let mut position = self.position;

        self.text.lines().map(move |line| {
            let p = match self.text_style.alignment {
                Alignment::Left => position,
                Alignment::Right => {
                    let metrics = self.character_style.measure_string(
                        line,
                        Point::zero(),
                        self.text_style.baseline,
                    );
                    position - (metrics.next_position - Point::new(1, 0))
                }
                Alignment::Center => {
                    let metrics = self.character_style.measure_string(
                        line,
                        Point::zero(),
                        self.text_style.baseline,
                    );
                    position - (metrics.next_position - Point::new(1, 0)) / 2
                }
            };

            position.y += self
                .text_style
                .line_height
                .to_absolute(self.character_style.line_height())
                .saturating_as::<i32>();

            (line, p)
        })
    }
}

impl<S: TextRenderer> Drawable for Text<'_, S> {
    type Color = S::Color;
    type Output = Point;

    fn draw<D>(&self, target: &mut D) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let mut next_position = self.position;

        for (line, position) in self.lines() {
            next_position = self.character_style.draw_string(
                line,
                position,
                self.text_style.baseline,
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

impl<S: TextRenderer> Dimensions for Text<'_, S> {
    fn bounding_box(&self) -> Rectangle {
        let mut min_max: Option<(Point, Point)> = None;

        for (line, position) in self.lines() {
            let metrics =
                self.character_style
                    .measure_string(line, position, self.text_style.baseline);
            update_min_max(&mut min_max, &metrics);
        }

        if let Some((min, max)) = min_max {
            Rectangle::with_corners(min, max)
        } else {
            Rectangle::new(self.position, Size::zero())
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
            ascii::{FONT_6X13, FONT_6X9},
            tests::assert_text_from_pattern,
            MonoTextStyle, MonoTextStyleBuilder,
        },
        pixelcolor::BinaryColor,
        primitives::{Primitive, PrimitiveStyle},
        text::{Alignment, Baseline, LineHeight, TextStyleBuilder},
    };

    const HELLO_WORLD: &'static str = "Hello World!";

    #[test]
    fn constructor() {
        let character_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);

        let text = Text::new("Hello e-g", Point::new(10, 11), character_style);

        assert_eq!(
            text,
            Text {
                text: "Hello e-g",
                position: Point::new(10, 11),
                character_style,
                text_style: TextStyle::default(),
            }
        );
    }

    #[test]
    fn multiline() {
        assert_text_from_pattern(
            "AB\nC",
            &FONT_6X9,
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
            &FONT_6X9,
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
            .font(&FONT_6X9)
            .text_color(BinaryColor::On)
            .build();

        let text = Text::with_baseline("AB\nC", Point::zero(), character_style, Baseline::Top);

        assert_eq!(
            text.bounding_box(),
            Rectangle::new(Point::zero(), Size::new(2 * 6, 2 * 9))
        );
    }

    #[test]
    fn position_and_translate() {
        let style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);

        let hello = Text::new(HELLO_WORLD, Point::zero(), style);

        let hello_translated = hello.translate(Point::new(5, -20));
        assert_eq!(
            hello.bounding_box().size,
            hello_translated.bounding_box().size
        );

        let mut hello_with_point = Text::new(HELLO_WORLD, Point::new(5, -20), style);
        assert_eq!(hello_translated, hello_with_point);

        hello_with_point.translate_mut(Point::new(-5, 20));
        assert_eq!(hello, hello_with_point);
    }

    #[test]
    fn inverted_text() {
        let mut display_inverse = MockDisplay::new();
        let style_inverse = MonoTextStyleBuilder::new()
            .font(&FONT_6X9)
            .text_color(BinaryColor::Off)
            .background_color(BinaryColor::On)
            .build();
        Text::new("Mm", Point::new(0, 7), style_inverse)
            .draw(&mut display_inverse)
            .unwrap();

        let mut display_normal = MockDisplay::new();
        let style_normal = MonoTextStyleBuilder::new()
            .font(&FONT_6X9)
            .text_color(BinaryColor::On)
            .background_color(BinaryColor::Off)
            .build();
        Text::new("Mm", Point::new(0, 7), style_normal)
            .draw(&mut display_normal)
            .unwrap();

        display_inverse.assert_eq(&display_normal.map(|c| c.invert()));
    }

    #[test]
    fn no_fill_does_not_hang() {
        let mut display = MockDisplay::new();
        Text::new(
            " ",
            Point::zero(),
            MonoTextStyle::new(&FONT_6X9, BinaryColor::On),
        )
        .draw(&mut display)
        .unwrap();

        display.assert_eq(&MockDisplay::new());
    }

    #[test]
    fn transparent_text_color_does_not_overwrite_background() {
        let character_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X9)
            .background_color(BinaryColor::On)
            .build();

        let mut display = MockDisplay::new();
        display.set_allow_overdraw(true);

        // Draw a background for the first character
        Rectangle::new(Point::zero(), Size::new(6, 8))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
            .draw(&mut display)
            .unwrap();

        Text::with_baseline("AA", Point::zero(), character_style, Baseline::Top)
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
        let style = MonoTextStyleBuilder::<BinaryColor>::new()
            .font(&FONT_6X9)
            .build();

        let styled = Text::new(" A", Point::new(7, 11), style);

        assert_eq!(
            styled.bounding_box(),
            Rectangle::new(Point::new(7, 11), Size::zero()),
            "Transparent text is expected to have a zero sized bounding box with the top left corner at the text position",
        );
    }

    #[test]
    fn alignment_left() {
        let character_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X9)
            .text_color(BinaryColor::On)
            .build();

        let text_style = TextStyleBuilder::new()
            .alignment(Alignment::Left)
            .baseline(Baseline::Top)
            .build();

        let mut display = MockDisplay::new();
        Text::with_text_style("A\nBC", Point::new(0, 0), character_style, text_style)
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
            .font(&FONT_6X9)
            .text_color(BinaryColor::On)
            .build();

        let text_style = TextStyleBuilder::new()
            .alignment(Alignment::Center)
            .baseline(Baseline::Top)
            .build();

        let mut display = MockDisplay::new();
        Text::with_text_style("A\nBC", Point::new(5, 0), character_style, text_style)
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
            .font(&FONT_6X9)
            .text_color(BinaryColor::On)
            .build();

        let text_style = TextStyleBuilder::new()
            .alignment(Alignment::Right)
            .baseline(Baseline::Top)
            .build();

        let mut display = MockDisplay::new();
        Text::with_text_style("A\nBC", Point::new(11, 0), character_style, text_style)
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
            .font(&FONT_6X9)
            .text_color(BinaryColor::On)
            .build();

        Text::with_baseline("t", Point::new(0, 8), character_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();
        Text::with_baseline("m", Point::new(6, 8), character_style, Baseline::Middle)
            .draw(&mut display)
            .unwrap();
        Text::with_baseline("b", Point::new(12, 8), character_style, Baseline::Bottom)
            .draw(&mut display)
            .unwrap();
        Text::with_baseline(
            "B",
            Point::new(18, 8),
            character_style,
            Baseline::Alphabetic,
        )
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
                    .font(&FONT_6X9)
                    .text_color(BinaryColor::On)
                    .background_color(BinaryColor::Off)
                    .build();

                let text_style = TextStyleBuilder::new()
                    .alignment(alignment)
                    .baseline(baseline)
                    .build();

                let text = Text::with_text_style(
                    "1\n23",
                    Point::new_equal(20),
                    character_style,
                    text_style,
                );

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
            .font(&FONT_6X9)
            .text_color(BinaryColor::On)
            .build();

        let character_style2 = MonoTextStyleBuilder::new()
            .font(&FONT_6X13)
            .text_color(BinaryColor::Off)
            .build();

        let mut display = MockDisplay::new();
        let next = Text::new("AB", Point::new(0, 8), character_style1)
            .draw(&mut display)
            .unwrap();
        Text::new("C", next, character_style2)
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

    #[test]
    fn line_height_pixels() {
        let character_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X9)
            .text_color(BinaryColor::On)
            .build();

        let text_style = TextStyleBuilder::new()
            .line_height(LineHeight::Pixels(7))
            .build();

        let mut display = MockDisplay::new();
        Text::with_text_style("A\nB", Point::new(0, 5), character_style, text_style)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "  #  ", //
            " # # ", //
            "#   #", //
            "#####", //
            "#   #", //
            "#   #", //
            "     ", //
            "#### ", //
            "#   #", //
            "#### ", //
            "#   #", //
            "#   #", //
            "#### ", //
        ]);
    }

    #[test]
    fn line_height_percent() {
        let character_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X9)
            .text_color(BinaryColor::On)
            .build();

        let text_style = TextStyleBuilder::new()
            .baseline(Baseline::Top)
            .line_height(LineHeight::Percent(200))
            .build();

        let mut display = MockDisplay::new();
        Text::with_text_style("A\nBC", Point::zero(), character_style, text_style)
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
        ]);
    }
}
