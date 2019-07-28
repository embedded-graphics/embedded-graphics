use embedded_graphics::context::{Context, ContextExt};
use embedded_graphics::icoord;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::DisplayBuilder;
use std::thread;
use std::time::Duration;

static CIRCLE_SIZE: i32 = 32;

fn draw_shapes<C, D>(ctx: &mut Context<C, D>)
where
    C: PixelColor + From<BinaryColor>,
    D: Drawing<C>,
{
    ctx.circle(icoord!(CIRCLE_SIZE, CIRCLE_SIZE), CIRCLE_SIZE as u32);
    ctx.translate(icoord!(128, 0));
    ctx.rectangle(icoord!(0, 0), icoord!(64, 64));
    ctx.translate(icoord!(128, 0));
    ctx.triangle(icoord!(32, 0), icoord!(0, 64), icoord!(64, 64));
}

fn main() {
    let mut display = DisplayBuilder::new()
        .title("Filled primitives using context")
        .size(400, 128)
        .scale(2)
        .build_binary();

    let mut ctx = display.context();

    ctx.set_stroke_color(BinaryColor::On);
    draw_shapes(&mut ctx);

    ctx.reset_translate();
    ctx.translate(icoord!(16, 16));
    ctx.set_stroke_color(BinaryColor::Off);
    ctx.set_fill_color(BinaryColor::On);
    draw_shapes(&mut ctx);

    ctx.reset_translate();
    ctx.translate(icoord!(32, 32));
    ctx.set_fill_color(BinaryColor::Off);
    draw_shapes(&mut ctx);

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
