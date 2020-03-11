extern crate embedded_graphics;
extern crate embedded_graphics_simulator;

use embedded_graphics::{
    egtext, fonts::Font6x8, pixelcolor::Rgb888, prelude::*, primitive_style, primitives::ThickLine,
    text_style,
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use sdl2::keyboard::Keycode;

const BACKGROUND_COLOR: Rgb888 = Rgb888::BLACK;
const FOREGROUND_COLOR: Rgb888 = Rgb888::RED;

fn draw(
    display: &mut SimulatorDisplay<Rgb888>,
    position: Point,
    stroke_width: u32,
    draw_extra: bool,
    offs: i32,
) -> Result<(), core::convert::Infallible> {
    display.clear(BACKGROUND_COLOR)?;

    let start = Point::new(
        display.size().width as i32 / 2,
        display.size().height as i32 / 2,
    );

    egtext!(
        text = &format!("W: {}", stroke_width),
        top_left = Point::zero(),
        style = text_style!(font = Font6x8, text_color = Rgb888::MAGENTA)
    )
    .into_iter()
    .chain(
        egtext!(
            text = &format!("DX {}, DY {}", position.x - start.x, position.y - start.y),
            top_left = Point::new(0, 8),
            style = text_style!(font = Font6x8, text_color = Rgb888::MAGENTA)
        )
        .into_iter(),
    )
    .draw(display)?;

    ThickLine::new(
        start,
        position,
        primitive_style!(
            stroke_width = stroke_width,
            stroke_color = Rgb888::new(0x80, 0xf2, 0x91),
        ),
        draw_extra,
        offs,
    )
    .into_iter()
    .draw(display)?;

    Ok(())
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(200, 200));
    let output_settings = OutputSettingsBuilder::new()
        .scale(8)
        .pixel_spacing(1)
        .build();
    let mut window = Window::new("Line thickness", &output_settings);

    let mut position = Point::new(150, 120);
    let mut stroke_width = 5;
    let mut draw_extra = true;
    let mut mouse_down = false;
    let mut offs = 0;

    draw(&mut display, position, stroke_width, draw_extra, offs)?;

    'running: loop {
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::KeyDown { keycode, .. } => {
                    match keycode {
                        Keycode::Up => stroke_width += 1,
                        Keycode::Down => stroke_width = (stroke_width as i32 - 1).max(0) as u32,
                        Keycode::Space => draw_extra = !draw_extra,
                        Keycode::O => offs += 1,
                        Keycode::L => offs -= 1,
                        _ => (),
                    }

                    draw(&mut display, position, stroke_width, draw_extra, offs)?;
                }
                SimulatorEvent::MouseButtonDown { point, .. } => {
                    mouse_down = true;
                    position = point;

                    draw(&mut display, position, stroke_width, draw_extra, offs)?;
                }
                SimulatorEvent::MouseButtonUp { .. } => mouse_down = false,
                SimulatorEvent::MouseMove { point, .. } => {
                    if mouse_down {
                        position = point;
                        draw(&mut display, position, stroke_width, draw_extra, offs)?;
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}
