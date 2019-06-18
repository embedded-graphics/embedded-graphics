use embedded_graphics::icoord;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rect;
use embedded_graphics_simulator::DisplayBuilder;
use std::thread;
use std::time::Duration;

fn main() {
    let mut display = DisplayBuilder::new().size(32, 32).scale(4).build();

    // Outline
    display.draw(
        Rect::new(icoord!(0, 0), icoord!(16, 16))
            .stroke(Some(1u8.into()))
            .translate(icoord!(-8, -8)),
    );

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
