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
use std::cmp::Ordering;
use triangle::sort_clockwise;

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

fn empty_crosshair(
    point: Point,
    color: Rgb888,
    display: &mut SimulatorDisplay<Rgb888>,
) -> Result<(), core::convert::Infallible> {
    let radius = Size::new_equal(4);
    let inner_radius = Size::new_equal(2);

    Line::new(point - radius.x_axis(), point - inner_radius.x_axis())
        .points()
        .chain(Line::new(point + radius.x_axis(), point + inner_radius.x_axis()).points())
        .chain(Line::new(point - radius.y_axis(), point - inner_radius.y_axis()).points())
        .chain(Line::new(point + radius.y_axis(), point + inner_radius.y_axis()).points())
        .map(|p| Pixel(p, color))
        .draw(display)
}

fn point_label(
    point: Point,
    idx: u32,
    display: &mut SimulatorDisplay<Rgb888>,
) -> Result<(), core::convert::Infallible> {
    Text::new(&format!("P{}", idx), point)
        .into_styled(
            TextStyleBuilder::new(Font6x8)
                .background_color(Rgb888::YELLOW)
                .text_color(Rgb888::BLUE)
                .build(),
        )
        .draw(display)
}

fn sort_two_yx(p1: &Point, p2: &Point) -> Ordering {
    // If p1.y is less than p2.y, return it first. Otherwise, if they have the same Y coordinate,
    // the first point becomes the one with the lesser X coordinate.
    if p1.y < p2.y || (p1.y == p2.y && p1.x < p2.x) {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

fn draw(
    mouse_pos: Point,
    corner_pos: Point,
    width: u32,
    alignment: StrokeAlignment,
    // display: &mut OverdrawDisplay,
    display: &mut SimulatorDisplay<Rgb888>,
) -> Result<(), core::convert::Infallible> {
    display.clear(Rgb888::BLACK)?;

    let scanline = Line::new(
        mouse_pos.y_axis(),
        mouse_pos.y_axis() + display.size().x_axis(),
    );

    let mut trapezium = [
        Point::new(40, 20),
        Point::new(80, 10),
        corner_pos,
        Point::new(30, 60),
        // Point::new(10, 40),
    ];

    let center = trapezium
        .iter()
        .fold(Point::zero(), |accum, point| accum + *point)
        / 4;

    empty_crosshair(center, Rgb888::CYAN, display)?;

    // trapezium.sort_by(|a, b| sort_clockwise(a, b, center));
    // trapezium.sort_by(sort_two_yx);

    for (idx, point) in trapezium.iter().enumerate() {
        point_label(*point, idx as u32, display)?;
    }

    // "inner" lines
    Line::new(trapezium[0], trapezium[1])
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::GREEN, 1))
        .draw(display)?;
    Line::new(trapezium[2], trapezium[3])
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::GREEN, 1))
        .draw(display)?;

    // Edge Bresenham lines
    Line::new(trapezium[1], trapezium[2])
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::MAGENTA, 1))
        .draw(display)?;
    Line::new(trapezium[3], trapezium[0])
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::MAGENTA, 1))
        .draw(display)?;

    // Scanline - last so it's on top
    scanline
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::RED, 1))
        .draw(display)?;

    Ok(())
}

fn main() -> Result<(), core::convert::Infallible> {
    let w = 100i32;
    let h = 100i32;

    let mut display: SimulatorDisplay<Rgb888> =
        SimulatorDisplay::new(Size::new(w as u32 + 100, h as u32));
    let output_settings = OutputSettingsBuilder::new()
        .scale(4)
        // .pixel_spacing(1)
        .build();
    let mut window = Window::new("Polyline segment debugger", &output_settings);

    // let mut overdraw_display = OverdrawDisplay::new(display.size());

    let mut corner_pos = Point::zero();
    let mut mouse_pos = Point::zero();

    let mut width = 15u32;
    let mut alignment = StrokeAlignment::Center;

    let mut mouse_down = false;

    draw(mouse_pos, corner_pos, width, alignment, &mut display)?;

    // overdraw_display.draw_to_display(&mut display)?;

    'running: loop {
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::MouseButtonDown { point, .. } => {
                    mouse_down = true;

                    corner_pos = point;
                }
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
                SimulatorEvent::MouseButtonUp { .. } => mouse_down = false,
                SimulatorEvent::MouseMove { point, .. } => {
                    if mouse_down {
                        corner_pos = point;
                    }
                    mouse_pos = point;
                }
                _ => {}
            }

            draw(mouse_pos, corner_pos, width, alignment, &mut display)?;

            // overdraw_display.draw_to_display(&mut display)?;
        }
    }

    Ok(())
}
