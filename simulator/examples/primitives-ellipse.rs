//! # Example: `Ellipse` primitive.
//!
//! Click and drag to move the bottom right corner of the ellipse's bounding box around the screen.
//!
//! The stroke size can be increased or decreased using the up and down arrow keys.
//!
//! This example is not particularly useful on it's own, but is helpful when debugging ellipse
//! rendering.

use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::*,
    style::{PrimitiveStyle, PrimitiveStyleBuilder, TextStyle},
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use sdl2::keyboard::Keycode;

fn draw_ellipse(
    top_left: Point,
    size: Size,
    stroke_width: u32,
    display: &mut SimulatorDisplay<Rgb888>,
) {
    display.clear(Rgb888::BLACK).unwrap();

    Text::new(&format!("S: {}\n{:?}", stroke_width, size), Point::zero())
        .into_styled(TextStyle::new(Font6x8, Rgb888::MAGENTA))
        .draw(display)
        .unwrap();

    // Bounding rect
    Rectangle::new(top_left, size)
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::WHITE, 1))
        .draw(display)
        .unwrap();

    Ellipse::new(top_left, size)
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_width(stroke_width)
                .stroke_color(Rgb888::RED)
                .fill_color(Rgb888::GREEN)
                .build(),
        )
        .draw(display)
        .unwrap();
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(200, 200));
    let output_settings = OutputSettingsBuilder::new()
        .scale(2)
        .pixel_spacing(1)
        .build();
    let mut window = Window::new("Ellipse debugger", &output_settings);

    let top_left = Point::new(50, 50);

    let mut mouse_down = false;

    let mut bounding_rect = Rectangle::with_corners(top_left, Point::new(100, 100));

    let mut stroke_width = 5;

    draw_ellipse(
        bounding_rect.top_left,
        bounding_rect.size,
        stroke_width,
        &mut display,
    );

    'running: loop {
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::MouseButtonDown { point, .. } => {
                    mouse_down = true;

                    bounding_rect = Rectangle::with_corners(top_left, point);

                    draw_ellipse(
                        bounding_rect.top_left,
                        bounding_rect.size,
                        stroke_width,
                        &mut display,
                    );
                }
                SimulatorEvent::KeyDown { keycode, .. } => {
                    match keycode {
                        Keycode::Up => stroke_width += 1,
                        Keycode::Down => stroke_width = (stroke_width as i32 - 1).max(0) as u32,
                        _ => (),
                    }

                    draw_ellipse(
                        bounding_rect.top_left,
                        bounding_rect.size,
                        stroke_width,
                        &mut display,
                    );
                }
                SimulatorEvent::MouseButtonUp { .. } => mouse_down = false,
                SimulatorEvent::MouseMove { point, .. } => {
                    if mouse_down {
                        bounding_rect = Rectangle::with_corners(top_left, point);

                        draw_ellipse(
                            bounding_rect.top_left,
                            bounding_rect.size,
                            stroke_width,
                            &mut display,
                        );
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}
