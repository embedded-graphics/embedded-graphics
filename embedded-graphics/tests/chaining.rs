extern crate embedded_graphics;

use embedded_graphics::geometry::Point;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, Rectangle};
use embedded_graphics::DrawTarget;

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

impl DrawTarget<TestPixelColor> for FakeDisplay {
    fn draw_pixel(&mut self, _pixel: Pixel<TestPixelColor>)
    { // Noop 
    }
}

#[test]
fn it_supports_chaining() {
    let mut disp = FakeDisplay {};

    let chained = Rectangle::new(Point::new(0, 0), Point::new(1, 1))
        .into_iter()
        .chain(Circle::new(Point::new(2, 2), 1).into_iter());

    chained.draw(&mut disp);
}

fn multi() -> impl Iterator<Item = Pixel<TestPixelColor>> {
    let line = Line::new(Point::new(0, 1), Point::new(2, 3))
        .stroke_color(Some(1u8.into()))
        .into_iter();

    let circle = Circle::new(Point::new(5, 5), 3)
        .stroke_color(Some(1u8.into()))
        .into_iter();

    line.chain(circle)
}

#[test]
fn return_from_fn() {
    let mut disp = FakeDisplay {};

    let chained = multi();

    chained.draw(&mut disp);
}

#[test]
fn implicit_into_iter() {
    let mut disp = FakeDisplay {};

    let chained = Rectangle::new(Point::new(0, 0), Point::new(1, 1))
        .into_iter()
        .chain(Circle::new(Point::new(2, 2), 1));

    chained.draw(&mut disp);
}
