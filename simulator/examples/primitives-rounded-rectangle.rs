//! TODO: Doc

use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::*,
    style::{PrimitiveStyleBuilder, StrokeAlignment},
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use sdl2::keyboard::Keycode;

fn draw(
    top_left: Point,
    size: Size,
    radius: Size,
    stroke_width: u32,
    align: StrokeAlignment,
    display: &mut SimulatorDisplay<Rgb888>,
) {
    display.clear(Rgb888::BLACK).unwrap();

    RoundedRectangle::with_equal_corners(Rectangle::new(top_left, size), radius)
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_width(stroke_width)
                .stroke_color(Rgb888::RED)
                .stroke_alignment(align)
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
    let mut window = Window::new("Rounded rectangle debugger", &output_settings);

    let top_left = Point::new(20, 20);

    let mut mouse_down = false;

    let mut bounding_rect = Rectangle::with_corners(top_left, Point::new(100, 100));

    let mut stroke_width = 5;

    let mut radius = 20;

    let mut align = StrokeAlignment::Center;

    draw(
        bounding_rect.top_left,
        bounding_rect.size,
        Size::new(radius, radius),
        stroke_width,
        align,
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
                }
                SimulatorEvent::KeyDown { keycode, .. } => match keycode {
                    Keycode::Up => stroke_width += 1,
                    Keycode::Down => stroke_width = (stroke_width as i32 - 1).max(0) as u32,

                    Keycode::Right => radius += 1,
                    Keycode::Left => radius = (radius as i32 - 1).max(0) as u32,

                    Keycode::Space => {
                        align = match align {
                            StrokeAlignment::Center => StrokeAlignment::Outside,
                            StrokeAlignment::Outside => StrokeAlignment::Inside,
                            StrokeAlignment::Inside => StrokeAlignment::Center,
                        }
                    }
                    _ => (),
                },
                SimulatorEvent::MouseButtonUp { .. } => mouse_down = false,
                SimulatorEvent::MouseMove { point, .. } => {
                    if mouse_down {
                        bounding_rect = Rectangle::with_corners(top_left, point);
                    }
                }
                _ => {}
            }

            draw(
                bounding_rect.top_left,
                bounding_rect.size,
                Size::new(radius, radius),
                stroke_width,
                align,
                &mut display,
            );
        }
    }

    Ok(())
}
