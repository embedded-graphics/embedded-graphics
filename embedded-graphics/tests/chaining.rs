extern crate embedded_graphics;

use embedded_graphics::{
    drawable::{Drawable, Pixel},
    geometry::{Point, Size},
    pixelcolor::{raw::RawData, PixelColor},
    primitives::{Circle, Line, Primitive, Rectangle},
    style::PrimitiveStyle,
    DrawTarget,
};

struct FakeDisplay {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TestPixelColor(pub bool);

impl PixelColor for TestPixelColor {
    type Raw = ();

    fn into_raw(&self) -> <Self::Raw as RawData>::Storage {
        ()
    }
}

impl From<u8> for TestPixelColor {
    fn from(other: u8) -> Self {
        TestPixelColor(other != 0)
    }
}

impl DrawTarget<TestPixelColor> for FakeDisplay {
    type Error = core::convert::Infallible;

    fn draw_pixel(&mut self, _pixel: Pixel<TestPixelColor>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn size(&self) -> Size {
        Size::zero()
    }
}

#[test]
fn it_supports_chaining() -> Result<(), core::convert::Infallible> {
    let mut display = FakeDisplay {};

    let mut chained = Rectangle::new(Point::new(0, 0), Point::new(1, 1))
        .into_styled(PrimitiveStyle::default())
        .into_iter()
        .chain(
            Circle::new(Point::new(2, 2), 1)
                .into_styled(PrimitiveStyle::default())
                .into_iter(),
        );

    chained.draw(&mut display)
}

fn multi() -> impl Iterator<Item = Pixel<TestPixelColor>> {
    let line = Line::new(Point::new(0, 1), Point::new(2, 3))
        .into_styled(PrimitiveStyle::with_stroke(1u8.into(), 1))
        .into_iter();

    let circle = Circle::new(Point::new(5, 5), 3)
        .into_styled(PrimitiveStyle::with_stroke(1u8.into(), 1))
        .into_iter();

    line.chain(circle)
}

#[test]
fn return_from_fn() -> Result<(), core::convert::Infallible> {
    let mut display = FakeDisplay {};

    let mut chained = multi();

    chained.draw(&mut display)
}

#[test]
fn implicit_into_iter() -> Result<(), core::convert::Infallible> {
    let mut display = FakeDisplay {};

    let mut chained = Rectangle::new(Point::new(0, 0), Point::new(1, 1))
        .into_styled(PrimitiveStyle::default())
        .into_iter()
        .chain(
            Circle::new(Point::new(2, 2), 1)
                .into_styled(PrimitiveStyle::default())
                .into_iter(),
        );

    chained.draw(&mut display)
}
