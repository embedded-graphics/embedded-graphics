use embedded_graphics::fonts::Font6x8;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::{egcircle, egrectangle, egtext};
use embedded_graphics_simulator::{SimulatorDisplay, WindowBuilder};

fn main() {
    let mut display = SimulatorDisplay::new(Size::new(256, 128));

    egcircle!(
        center = (20, 20),
        radius = 20 as u32,
        fill_color = Some(Rgb565::RED)
    )
    .into_iter()
    .chain(&egrectangle!(
        top_left = (20, 20),
        bottom_right = (100, 80),
        fill_color = Some(Rgb565::RED)
    ))
    .draw(&mut display);

    egtext!(
        text = "Hello world! - no background",
        top_left = Point::new(15, 15),
        font = Font6x8,
        text_color = Some(Rgb565::WHITE),
    )
    .draw(&mut display);

    egtext!(
        text = "Hello world! - filled background",
        top_left = Point::new(15, 30),
        font = Font6x8,
        text_color = Some(Rgb565::YELLOW),
        background_color = Some(Rgb565::BLUE)
    )
    .draw(&mut display);

    egtext!(
        text = "Hello world! - inverse background",
        top_left = Point::new(15, 45),
        font = Font6x8,
        text_color = Some(Rgb565::BLUE),
        background_color = Some(Rgb565::YELLOW)
    )
    .draw(&mut display);

    let mut window = WindowBuilder::new(&display)
        .title("Fonts with transparent background")
        .scale(3)
        .build();
    window.show_static(&display);
}
