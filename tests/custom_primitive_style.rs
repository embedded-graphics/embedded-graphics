use embedded_graphics::{
    common::ColorType,
    mock_display::MockDisplay,
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Rectangle, StyledDrawable},
};

struct CheckerboardStyle<C>(C, C);

impl<C: PixelColor> ColorType for CheckerboardStyle<C> {
    type Color = C;
}

impl<C: PixelColor> StyledDrawable<CheckerboardStyle<C>> for Rectangle {
    type Output = ();

    fn draw_styled<D>(
        &self,
        style: &CheckerboardStyle<C>,
        target: &mut D,
    ) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        target.fill_contiguous(
            self,
            self.points().map(|p| {
                if (p.x % 2 == 0) ^ (p.y % 2 == 0) {
                    style.1
                } else {
                    style.0
                }
            }),
        )
    }
}

#[test]
fn custom_primitive_style() {
    let style = CheckerboardStyle(Rgb888::RED, Rgb888::GREEN);

    let mut display = MockDisplay::new();
    Rectangle::new(Point::zero(), Size::new(4, 3))
        .into_styled(style)
        .draw(&mut display)
        .unwrap();
    display.assert_pattern(&[
        "RGRG", //
        "GRGR", //
        "RGRG", //
    ]);
}
