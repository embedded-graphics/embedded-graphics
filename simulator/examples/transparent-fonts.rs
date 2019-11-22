use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::{egcircle, egrectangle, text_6x8};
use embedded_graphics_simulator::{SimulatorDisplay, WindowBuilder};

fn main() {
    let mut display = SimulatorDisplay::new(Size::new(256, 128));

    egcircle!(
        (20, 20),
        20 as u32,
        stroke_color = Some(Rgb565::RED),
        fill_color = Some(Rgb565::RED)
    )
    .into_iter()
    .chain(&egrectangle!(
        (20, 20),
        (100, 80),
        fill_color = Some(Rgb565::RED)
    ))
    .draw(&mut display);

    text_6x8!(
        "Hello world! - no background",
        text_color = Some(Rgb565::WHITE),
    )
    .translate(Point::new(15, 15))
    .draw(&mut display);

    text_6x8!(
        "Hello world! - filled background",
        text_color = Some(Rgb565::YELLOW),
        background_color = Some(Rgb565::BLUE)
    )
    .translate(Point::new(15, 30))
    .draw(&mut display);

    text_6x8!(
        "Hello world! - inverse background",
        text_color = Some(Rgb565::BLUE),
        background_color = Some(Rgb565::YELLOW)
    )
    .translate(Point::new(15, 45))
    .draw(&mut display);

    let mut window = WindowBuilder::new(&display)
        .title("Fonts with transparent background")
        .scale(3)
        .build();
    window.show_static(&display);
}
