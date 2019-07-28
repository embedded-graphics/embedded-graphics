use embedded_graphics::context::ContextExt;
use embedded_graphics::fonts::{Font12x16, Font6x12, Font6x8, Font8x16};
use embedded_graphics::icoord;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::text_6x8;
use embedded_graphics_simulator::DisplayBuilder;
use std::thread;
use std::time::Duration;

fn main() {
    let mut display = DisplayBuilder::new()
        .title("Fonts")
        .size(256, 128)
        .scale(3)
        .build_binary();

    let mut ctx = display.context();

    ctx.set_stroke_color(BinaryColor::On);
    ctx.text::<Font6x8<_>>("Hello World! - default style 6x8", icoord!(15, 15));

    ctx.set_stroke_color(BinaryColor::Off);
    ctx.set_fill_color(BinaryColor::On);
    ctx.text::<Font6x8<_>>("Hello World! - inverse 6x8", icoord!(15, 30));

    //Show smallest font with white font on black background using a macro
    display.draw(
        text_6x8!(
            "Hello world! - inverse 6x8 with macro",
            stroke = Some(BinaryColor::Off),
            fill = Some(BinaryColor::On)
        )
        .translate(icoord!(15, 40)),
    );

    let mut ctx = display.context();
    ctx.set_stroke_color(BinaryColor::On);
    ctx.text::<Font6x12<_>>("Hello 6x12!", icoord!(15, 55));
    ctx.text::<Font8x16<_>>("Hello 8x16!", icoord!(15, 80));
    ctx.text::<Font12x16<_>>("Hello 12x16!", icoord!(15, 105));

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
