use embedded_graphics::{
    fonts::*,
    pixelcolor::Rgb888,
    prelude::*,
    primitives::line::{Intersection, Side},
    primitives::line_joint::{EdgeCorners, JointKind, LineJoint},
    primitives::*,
    style::*,
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use sdl2::keyboard::Keycode;

fn filled_tri(triangle: Triangle, color: Rgb888) -> impl Iterator<Item = Pixel<Rgb888>> {
    triangle.mathematical_points().map(move |p| Pixel(p, color))
}

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
    display: &mut SimulatorDisplay<Rgb888>,
) -> Result<(), core::convert::Infallible> {
    let skeleton_style = PrimitiveStyle::with_stroke(Rgb888::RED, 1);

    Polyline::new(points)
        .into_styled(skeleton_style)
        .draw(display)?;

    Ok(())
}

const PADDING: i32 = 16;

fn main() -> Result<(), core::convert::Infallible> {
    let (w, h) = (320i32, 256i32);

    let mut display: SimulatorDisplay<Rgb888> =
        SimulatorDisplay::new(Size::new(w as u32, h as u32));
    let output_settings = OutputSettingsBuilder::new()
        .scale(4)
        // .pixel_spacing(1)
        .build();
    let mut window = Window::new("Polyline joints debugger", &output_settings);

    // let mut end_point = Point::new(82, 110);
    let mut width = 15u32;
    let mut alignment = StrokeAlignment::Center;

    // let mut mouse_down = false;

    let points = [
        Point::new(PADDING, h / 2),
        Point::new(50, h / 2),
        Point::new(60, h / 2 - 20),
        Point::new(70, h / 2),
        Point::new(80, h / 2),
        Point::new(90, h / 2 + 10),
        Point::new(100, PADDING),
        Point::new(110, h / 2 + 20),
        Point::new(120, h / 2),
        Point::new(w - PADDING, h / 2),
    ];

    draw(&points, width, alignment, &mut display)?;

    'running: loop {
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                // SimulatorEvent::MouseButtonDown { point, .. } => {
                //     mouse_down = true;

                //     end_point = point;
                // }
                SimulatorEvent::KeyDown { keycode, .. } => match keycode {
                    Keycode::Up => width += 1,
                    Keycode::Down => width = width.saturating_sub(1),
                    Keycode::Space => {
                        alignment = match alignment {
                            StrokeAlignment::Center => StrokeAlignment::Outside,
                            StrokeAlignment::Outside => StrokeAlignment::Inside,
                            StrokeAlignment::Inside => StrokeAlignment::Center,
                        }
                    }
                    _ => (),
                },
                // SimulatorEvent::MouseButtonUp { .. } => mouse_down = false,
                // SimulatorEvent::MouseMove { point, .. } => {
                //     if mouse_down {
                //         end_point = point;
                //     }
                // }
                _ => {}
            }

            draw(&points, width, alignment, &mut display)?;
        }
    }

    Ok(())
}
