extern crate embedded_graphics;
extern crate embedded_graphics_simulator;

use embedded_graphics::{
    egtext, fonts::Font6x8, pixelcolor::Rgb888, prelude::*, primitives::Line,
    style::PrimitiveStyle, text_style,
};
use embedded_graphics_simulator::{SimulatorDisplay, SimulatorEvent, WindowBuilder};
use sdl2::keyboard::Keycode;

const BACKGROUND_COLOR: Rgb888 = Rgb888::BLACK;
const FOREGROUND_COLOR: Rgb888 = Rgb888::RED;

fn draw(
    display: &mut SimulatorDisplay<Rgb888>,
    position: Point,
    stroke_width: u32,
) -> Result<(), core::convert::Infallible> {
    display.clear(BACKGROUND_COLOR)?;

    egtext!(
        text = &format!("W: {}", stroke_width),
        top_left = Point::zero(),
        style = text_style!(font = Font6x8, text_color = Rgb888::MAGENTA)
    )
    .draw(display)?;

    Line::new(
        Point::new(
            display.size().width as i32 / 2,
            display.size().height as i32 / 2,
        ),
        position,
    )
    .into_styled(PrimitiveStyle::with_stroke(FOREGROUND_COLOR, stroke_width))
    .draw(display)?;

    Ok(())
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(200, 200));
    let mut window = WindowBuilder::new(&display)
        .scale(4)
        .title("Click to move circle")
        .build();

    let mut position = Point::new(50, 50);
    let mut stroke_width = 1;

    draw(&mut display, position, stroke_width)?;

    'running: loop {
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::KeyDown { keycode, .. } => {
                    match keycode {
                        Keycode::Up => stroke_width += 1,
                        Keycode::Down => stroke_width = (stroke_width as i32 - 1).max(0) as u32,
                        _ => (),
                    }

                    draw(&mut display, position, stroke_width)?;
                }
                SimulatorEvent::MouseButtonUp { point, .. } => {
                    position = point;
                    draw(&mut display, position, stroke_width)?;
                }
                _ => {}
            }
        }
    }

    Ok(())
}
