use embedded_graphics::{
    pixelcolor::Rgb888, prelude::*, primitives::*, style::PrimitiveStyleBuilder,
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use sdl2::keyboard::Keycode;

fn draw(
    points: &[Point],
    width: u32,
    _mouse_pos: Point,
    display: &mut SimulatorDisplay<Rgb888>,
) -> Result<(), core::convert::Infallible> {
    display.clear(Rgb888::BLACK)?;

    Polyline::new(points)
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_width(width)
                .stroke_color(Rgb888::RED)
                .build(),
        )
        .draw(display)
}

const PADDING: i32 = 16;

fn main() -> Result<(), core::convert::Infallible> {
    let w = 320i32;
    let h = 180i32;

    let mut display: SimulatorDisplay<Rgb888> =
        SimulatorDisplay::new(Size::new(w as u32 + 100, h as u32));
    let output_settings = OutputSettingsBuilder::new().scale(4).build();
    let mut window = Window::new("Polyline joints debugger", &output_settings);

    let mut end_point = Point::new(363, 39);

    let mut width = 15u32;

    let mut mouse_down = false;

    let mut points = [
        Point::new(PADDING, h / 2),
        Point::new(100, h / 2),
        Point::new(120, h / 2 - 20),
        Point::new(140, h / 2),
        Point::new(160, h / 2),
        Point::new(180, h / 2 + 10),
        Point::new(200, PADDING),
        Point::new(220, h / 2 + 20),
        Point::new(240, h / 2),
        Point::new(w - PADDING, h / 2),
        end_point,
    ];

    let mut num_points = points.len();

    draw(&points, width, end_point, &mut display)?;

    'running: loop {
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::MouseButtonDown { point, .. } => {
                    mouse_down = true;

                    end_point = point;
                }
                SimulatorEvent::KeyDown { keycode, .. } => match keycode {
                    Keycode::Up => width += 1,
                    Keycode::Down => width = width.saturating_sub(1),
                    Keycode::Left => num_points = num_points.saturating_sub(1),
                    Keycode::Right => num_points = (num_points + 1).min(points.len()),
                    _ => (),
                },
                SimulatorEvent::MouseButtonUp { .. } => mouse_down = false,
                SimulatorEvent::MouseMove { point, .. } => {
                    if mouse_down {
                        *points.get_mut(10).unwrap() = end_point;
                    }
                    end_point = point;
                }
                _ => {}
            }

            // *points.get_mut(10).unwrap() = end_point;

            draw(&points, width, end_point, &mut display)?;
        }
    }

    Ok(())
}
