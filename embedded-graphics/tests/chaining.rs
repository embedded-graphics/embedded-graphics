extern crate embedded_graphics;

use embedded_graphics::coord::Coord;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Rect};
use embedded_graphics::Drawing;

struct FakeDisplay {}

#[derive(Copy, Clone)]
pub struct TestPixelColor(pub bool);

impl PixelColor for TestPixelColor {}

impl From<u8> for TestPixelColor {
    fn from(other: u8) -> Self {
        TestPixelColor(other != 0)
    }
}

impl Drawing<TestPixelColor> for FakeDisplay {
    fn draw<T>(&mut self, _item_pixels: T)
    where
        T: IntoIterator<Item = Pixel<TestPixelColor>>,
    {
        // Noop
    }
}

#[test]
fn it_supports_chaining() {
    let mut disp = FakeDisplay {};

    let chained = Rect::new(Coord::new(0, 0), Coord::new(1, 1))
        .into_iter()
        .chain(Circle::new(Coord::new(2, 2), 1).into_iter());

    disp.draw(chained);
}
