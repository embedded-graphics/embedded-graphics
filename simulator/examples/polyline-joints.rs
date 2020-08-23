use embedded_graphics::{
    fonts::*,
    pixelcolor::{Gray8, Rgb888},
    prelude::*,
    primitives::line_joint::{EdgeCorners, LineJoint},
    primitives::triangle::MathematicalPoints,
    primitives::*,
    style::*,
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, OverdrawDisplay, SimulatorDisplay, SimulatorEvent, Window,
};
use sdl2::keyboard::Keycode;

fn crosshair(point: Point, color: Rgb888, display: &mut SimulatorDisplay<Rgb888>) {
    let radius = Size::new(4, 4);

    Line::new(point - radius.x_axis(), point + radius.x_axis())
        .into_styled(PrimitiveStyle::with_stroke(color, 1))
        .draw(display)
        .unwrap();

    Line::new(point - radius.y_axis(), point + radius.y_axis())
        .into_styled(PrimitiveStyle::with_stroke(color, 1))
        .draw(display)
        .unwrap();
}

fn empty_crosshair(point: Point, color: Rgb888, display: &mut SimulatorDisplay<Rgb888>) {
    let radius = Size::new_equal(4);
    let inner_radius = Size::new_equal(2);

    Line::new(point - radius.x_axis(), point - inner_radius.x_axis())
        .points()
        .chain(Line::new(point + radius.x_axis(), point + inner_radius.x_axis()).points())
        .chain(Line::new(point - radius.y_axis(), point - inner_radius.y_axis()).points())
        .chain(Line::new(point + radius.y_axis(), point + inner_radius.y_axis()).points())
        .map(|p| Pixel(p, color))
        .draw(display)
        .unwrap();
}

fn draw(
    points: &[Point],
    width: u32,
    alignment: StrokeAlignment,
    display: &mut OverdrawDisplay,
) -> Result<(), core::convert::Infallible> {
    display.clear(Gray8::BLACK)?;

    Text::new(&format!("Points {}", points.len()), Point::zero())
        .into_styled(
            TextStyleBuilder::new(Font6x8)
                .background_color(Gray8::WHITE)
                .text_color(Gray8::WHITE)
                .build(),
        )
        .draw(display)?;

    Polyline::new(points)
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_width(width)
                .stroke_alignment(alignment)
                .stroke_color(Gray8::WHITE)
                .build(),
        )
        .draw(display)?;

    Ok(())
}

const PADDING: i32 = 16;

fn main() -> Result<(), core::convert::Infallible> {
    // let (w, h) = (320i32, 256i32);

    let w = 320i32;
    // 16:9 aspect ratio for Twitter
    let h = 180i32;

    let mut display: SimulatorDisplay<Rgb888> =
        SimulatorDisplay::new(Size::new(w as u32 + 100, h as u32));
    let output_settings = OutputSettingsBuilder::new()
        .scale(4)
        // .pixel_spacing(1)
        .build();
    let mut window = Window::new("Polyline joints debugger", &output_settings);

    let mut overdraw_display = OverdrawDisplay::new(display.size());

    let mut end_point = Point::new(202, 113);

    let mut width = 15u32;
    let mut alignment = StrokeAlignment::Center;

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

    draw(
        &points[0..num_points],
        width,
        alignment,
        &mut overdraw_display,
    )?;

    overdraw_display.draw_to_display(&mut display)?;

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
                    Keycode::Space => {
                        alignment = match alignment {
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
                        end_point = point;
                    }
                }
                _ => {}
            }

            *points.last_mut().unwrap() = end_point;

            draw(
                &points[0..num_points],
                width,
                alignment,
                &mut overdraw_display,
            )?;

            overdraw_display.draw_to_display(&mut display)?;
        }
    }

    Ok(())
}
