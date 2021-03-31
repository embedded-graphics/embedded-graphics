use embedded_graphics::{
    mock_display::MockDisplay, pixelcolor::Rgb888, prelude::*, primitives::Rectangle,
};

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

struct TargetRectangle(Rectangle, Option<Rgb888>);

impl TargetSpecificDrawable<TestDisplay> for TargetRectangle {
    type Output = ();

    fn draw(&self, target: &mut TestDisplay) -> Result<Self::Output, Error> {
        for point in self.0.intersection(&target.bounding_box()).points() {
            // `set_pixel` wouldn't be accessible in a regular `Drawable`.
            target.0.set_pixel(point, self.1);
        }

        Ok(())
    }
}

#[test]
fn target_specific_drawable() {
    let mut target = TestDisplay(MockDisplay::new());

    TargetRectangle(
        Rectangle::new(Point::new(1, 1), Size::new(2, 3)),
        Some(Rgb888::RED),
    )
    .draw(&mut target)
    .unwrap();

    TargetRectangle(Rectangle::new(Point::new(2, 3), Size::new(1, 1)), None)
        .draw(&mut target)
        .unwrap();

    target.0.assert_pattern(&[
        "   ", //
        " RR", //
        " RR", //
        " R ", //
    ]);
}
