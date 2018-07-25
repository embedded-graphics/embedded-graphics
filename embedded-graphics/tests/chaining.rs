extern crate embedded_graphics;

use embedded_graphics::coord::Coord;
use embedded_graphics::drawable::Pixel;
use embedded_graphics::primitives::{Circle, Rect};
use embedded_graphics::Drawing;

struct FakeDisplay {}

impl Drawing for FakeDisplay {
    fn draw<T>(&mut self, _item_pixels: T)
    where
        T: Iterator<Item = Pixel>,
    {
        // Noop
    }
}

#[test]
fn it_supports_chaining() {
    let mut disp = FakeDisplay {};

    let chained = Rect::new(Coord::new(0, 0), Coord::new(1, 1), 1)
        .into_iter()
        .chain(Circle::new(Coord::new(2, 2), 1, 1).into_iter());

    disp.draw(chained);
}
