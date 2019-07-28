use embedded_graphics::context::ContextExt;
use embedded_graphics::fonts::Font6x8;
use embedded_graphics::icoord;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics_simulator::{BinaryColorTheme, DisplayBuilder};
use std::thread;
use std::time::Duration;

fn main() {
    let mut display = DisplayBuilder::new()
        .title("Hello World")
        .theme(BinaryColorTheme::OledBlue)
        .build_binary();

    let mut ctx = display.context();
    ctx.set_stroke_color(BinaryColor::On);

    // Outline
    ctx.circle(icoord!(64, 64), 64);

    // Clock hands
    ctx.line(icoord!(64, 64), icoord!(0, 64));
    ctx.line(icoord!(64, 64), icoord!(80, 80));

    ctx.text::<Font6x8<_>>("Hello World!", icoord!(5, 50));

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
