//! A debugging tool for thick lines
//!
//! Use the up/down arrow keys to increase or decrease the line thickness. Click and drag to move
//! the end point of the line around.
//!
//! The thickness, DX and DY components of the line are displayed in the top right corner of the
//! window.

extern crate embedded_graphics;
extern crate embedded_graphics_simulator;

use embedded_graphics::{
    egtext, fonts::Font6x8, pixelcolor::Rgb888, prelude::*, primitive_style, primitives::Line,
    text_style,
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use sdl2::keyboard::Keycode;

const BACKGROUND_COLOR: Rgb888 = Rgb888::BLACK;

fn draw(
    display: &mut SimulatorDisplay<Rgb888>,
    position: Point,
    stroke_width: u32,
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

    Line::new(start, position)
        .into_styled(primitive_style!(
            stroke_width = stroke_width,
            stroke_color = Rgb888::new(0x80, 0xf2, 0x91),
        ))
        .draw(display)?;

    Ok(())
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(200, 200));
    let output_settings = OutputSettingsBuilder::new()
        .scale(4)
        .pixel_spacing(1)
        .build();
    let mut window = Window::new("Line thickness debugger", &output_settings);

    let mut position = Point::new(150, 120);
    let mut stroke_width = 5;
    let mut mouse_down = false;

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
                SimulatorEvent::MouseButtonDown { point, .. } => {
                    mouse_down = true;
                    position = point;

                    draw(&mut display, position, stroke_width)?;
                }
                SimulatorEvent::MouseButtonUp { .. } => mouse_down = false,
                SimulatorEvent::MouseMove { point, .. } => {
                    if mouse_down {
                        position = point;
                        draw(&mut display, position, stroke_width)?;
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}
