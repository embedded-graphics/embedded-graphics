use embedded_graphics::{
    mock_display::MockDisplay,
    pixelcolor::Rgb888,
    prelude::*,
    primitives::Rectangle,
    text::{
        renderer::{TextMetrics, TextRenderer},
        Baseline, Text,
    },
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct GenericTextStyle<C>(C);

impl<C: PixelColor> TextRenderer for GenericTextStyle<C> {
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
        let metrics = self.measure_string(text, position, baseline);

        target.fill_solid(&metrics.bounding_box, self.0)?;

        Ok(metrics.next_position)
    }

    fn draw_whitespace<D>(
        &self,
        _width: u32,
        _position: Point,
        _baseline: Baseline,
        _target: &mut D,
    ) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        todo!()
    }

    fn measure_string(&self, text: &str, position: Point, _baseline: Baseline) -> TextMetrics {
        // TODO: use baseline

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

#[test]
fn generic_text_renderer() {
    let mut target = MockDisplay::new();

    Text::new("ab\nc", Point::zero(), GenericTextStyle(Rgb888::RED))
        .draw(&mut target)
        .unwrap();

    target.assert_pattern(&[
        "RRRRRRRR", //
        "RRRRRRRR", //
        "RRRRRRRR", //
        "RRRRRRRR", //
        "        ", //
        "RRRR    ", //
        "RRRR    ", //
        "RRRR    ", //
        "RRRR    ", //
    ]);
}
