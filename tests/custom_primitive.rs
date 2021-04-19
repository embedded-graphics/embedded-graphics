use embedded_graphics::{
    mock_display::MockDisplay,
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle, StyledDimensions, StyledDrawable},
};

struct Square {
    top_left: Point,
    size: u32,
}

impl Square {
    fn new(top_left: Point, size: u32) -> Self {
        Self { top_left, size }
    }

    fn to_rectangle(&self) -> Rectangle {
        Rectangle::new(self.top_left, Size::new_equal(self.size))
    }
}

impl Primitive for Square {}

impl Dimensions for Square {
    fn bounding_box(&self) -> Rectangle {
        self.to_rectangle()
    }
}

impl<C: PixelColor> StyledDrawable<PrimitiveStyle<C>> for Square {
    type Color = C;
    type Output = ();

    fn draw_styled<D>(
        &self,
        style: &PrimitiveStyle<C>,
        target: &mut D,
    ) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        self.to_rectangle().draw_styled(style, target)
    }
}

impl<C: PixelColor> StyledDimensions<PrimitiveStyle<C>> for Square {
    fn styled_bounding_box(&self, style: &PrimitiveStyle<C>) -> Rectangle {
        self.to_rectangle().styled_bounding_box(style)
    }
}

#[test]
fn draw_custom_primitive() {
    let mut display = MockDisplay::new();
    Square::new(Point::new(1, 0), 2)
        .into_styled(PrimitiveStyle::with_fill(Rgb888::RED))
        .draw(&mut display)
        .unwrap();
    display.assert_pattern(&[
        " RR", //
        " RR", //
    ]);
}

#[test]
fn custom_primitive_dimensions() {
    let styled_square =
        Square::new(Point::new(1, 0), 2).into_styled(PrimitiveStyle::with_stroke(Rgb888::RED, 2));

    assert_eq!(
        styled_square.bounding_box(),
        Rectangle::new(Point::new(1, 0), Size::new_equal(2)).offset(1)
    );
}
