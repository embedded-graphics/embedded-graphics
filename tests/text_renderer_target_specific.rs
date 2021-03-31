use embedded_graphics::{mock_display::MockDisplay, pixelcolor::Rgb888, prelude::*, primitives::Rectangle, text::{Alignment, Baseline, Text, TextStyleBuilder, renderer::{CharacterStyle, TargetSpecificTextRenderer, TextMetrics}}};

type Error = <MockDisplay<Rgb888> as DrawTarget>::Error;

struct TestDisplay(MockDisplay<Rgb888>);

impl DrawTarget for TestDisplay {
    type Color = Rgb888;
    type Error = Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.0.draw_iter(pixels)
    }
}

impl OriginDimensions for TestDisplay {
    fn size(&self) -> Size {
        Size::new_equal(128)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct DummyTargetTextStyle<T>(T);

impl<T: PixelColor> CharacterStyle for DummyTargetTextStyle<T> {
    fn measure_string(&self, text: &str, position: Point, _baseline: Baseline) -> TextMetrics {
        // The `baseline` is ignored in this test, but must be used in a real impl.

        let width = text.len() as u32 * 4;

        TextMetrics {
            bounding_box: Rectangle::new(position, Size::new(width, 4)),
            next_position: position + Size::new(width, 0),
        }
    }

    fn line_height(&self) -> u32 {
        5
    }
}

impl TargetSpecificTextRenderer<TestDisplay> for DummyTargetTextStyle<Rgb888> {
    fn draw_string(
        &self,
        text: &str,
        position: Point,
        baseline: Baseline,
        target: &mut TestDisplay,
    ) -> Result<Point, Error> {
        let metrics = self.measure_string(text, position, baseline);

        target.fill_solid(&metrics.bounding_box, self.0)?;

        Ok(metrics.next_position)
    }

    fn draw_whitespace(
        &self,
        _width: u32,
        _position: Point,
        _baseline: Baseline,
        _target: &mut TestDisplay,
    ) -> Result<Point, Error> {
        todo!()
    }
}

#[test]
fn target_specific_text_renderer() {
    let mut target = TestDisplay(MockDisplay::new());

    Text::new("ab\nc", Point::zero())
        .into_styled(DummyTargetTextStyle(Rgb888::GREEN))
        .draw(&mut target)
        .unwrap();

    target.0.assert_pattern(&[
        "GGGGGGGG", //
        "GGGGGGGG", //
        "GGGGGGGG", //
        "GGGGGGGG", //
        "        ", //
        "GGGG    ", //
        "GGGG    ", //
        "GGGG    ", //
        "GGGG    ", //
    ]);
}

#[test]
fn target_specific_text_renderer_with_text_style() {
    let mut target = TestDisplay(MockDisplay::new());

    let character_style = DummyTargetTextStyle(Rgb888::BLUE);
    let text_style = TextStyleBuilder::new()
        .character_style(character_style)
        .alignment(Alignment::Right)
        .build();

    Text::new("ab\nc", Point::new(7, 0))
        .into_styled(text_style)
        .draw(&mut target)
        .unwrap();

    target.0.assert_pattern(&[
        "BBBBBBBB", //
        "BBBBBBBB", //
        "BBBBBBBB", //
        "BBBBBBBB", //
        "        ", //
        "    BBBB", //
        "    BBBB", //
        "    BBBB", //
        "    BBBB", //
    ]);
}
