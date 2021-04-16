extern crate embedded_graphics;

use embedded_graphics::{
    prelude::*,
    primitives::{Circle, Line, Primitive, PrimitiveStyle, Rectangle},
};

struct FakeDisplay {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TestPixelColor(pub bool);

impl PixelColor for TestPixelColor {
    type Raw = ();
}

impl From<u8> for TestPixelColor {
    fn from(other: u8) -> Self {
        TestPixelColor(other != 0)
    }
}

impl DrawTarget for FakeDisplay {
    type Color = TestPixelColor;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, _pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        Ok(())
    }
}

impl OriginDimensions for FakeDisplay {
    fn size(&self) -> Size {
        Size::zero()
    }
}

#[test]
fn it_supports_chaining() -> Result<(), core::convert::Infallible> {
    let mut display = FakeDisplay {};

    let chained = Rectangle::new(Point::new(0, 0), Size::new(1, 1))
        .into_styled(PrimitiveStyle::default())
        .pixels()
        .chain(
            Circle::new(Point::new(1, 1), 3)
                .into_styled(PrimitiveStyle::default())
                .pixels(),
        );

    chained.draw(&mut display)
}

fn multi() -> impl Iterator<Item = Pixel<TestPixelColor>> {
    let line = Line::new(Point::new(0, 1), Point::new(2, 3))
        .into_styled(PrimitiveStyle::with_stroke(1u8.into(), 1))
        .pixels();

    let circle = Circle::new(Point::new(2, 2), 7)
        .into_styled(PrimitiveStyle::with_stroke(1u8.into(), 1))
        .pixels();

    line.chain(circle)
}

#[test]
fn return_from_fn() -> Result<(), core::convert::Infallible> {
    let mut display = FakeDisplay {};

    let chained = multi();

    chained.draw(&mut display)
}
