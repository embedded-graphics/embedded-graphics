extern crate embedded_graphics;

use embedded_graphics::coord::Coord;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Rect};
use embedded_graphics::Drawing;

struct FakeDisplay {}

#[derive(Copy, Clone)]
pub struct FakePixelColor(pub bool);

impl PixelColor for FakePixelColor {}

impl From<u8> for FakePixelColor {
    fn from(other: u8) -> Self {
        FakePixelColor(other != 0)
    }
}

impl Drawing<FakePixelColor> for FakeDisplay {
    fn draw<T>(&mut self, _item_pixels: T)
    where
        T: Iterator<Item = Pixel<FakePixelColor>>,
    {
        // Noop
    }
}

#[test]
fn it_supports_chaining() {
    let mut disp = FakeDisplay {};

    let chained = Rect::new(Coord::new(0, 0), Coord::new(1, 1), 1.into())
        .into_iter()
        .chain(Circle::new(Coord::new(2, 2), 1, 1.into()).into_iter());

    disp.draw(chained);
}
