//! Intersection between Bresenham line and scanline

use embedded_graphics::{
    fonts::*,
    pixelcolor::{Gray8, Rgb888},
    prelude::*,
    primitives::line_joint::{EdgeCorners, LineJoint},
    primitives::*,
    style::*,
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, OverdrawDisplay, SimulatorDisplay, SimulatorEvent, Window,
};
use sdl2::keyboard::Keycode;
use std::cmp::Ordering;

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

fn sort_two_yx_cmp(p1: &Point, p2: &Point) -> Ordering {
    // If p1.y is less than p2.y, return it first. Otherwise, if they have the same Y coordinate,
    // the first point becomes the one with the lesser X coordinate.
    if p1.y < p2.y || (p1.y == p2.y && p1.x < p2.x) {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

// Flag will be true if pair was swapped
fn sort_two_yx(p1: Point, p2: Point) -> (Point, Point, bool) {
    // If p1.y is less than p2.y, return it first. Otherwise, if they have the same Y coordinate,
    // the first point becomes the one with the lesser X coordinate.
    if p1.y < p2.y || (p1.y == p2.y && p1.x < p2.x) {
        (p1, p2, false)
    } else {
        (p2, p1, true)
    }
}

fn draw(
    mouse_pos: Point,
    corner_pos: Point,
    // display: &mut OverdrawDisplay,
    display: &mut SimulatorDisplay<Rgb888>,
) -> Result<(), core::convert::Infallible> {
    display.clear(Rgb888::BLACK)?;

    let scanline = Line::new(
        mouse_pos.y_axis(),
        mouse_pos.y_axis() + display.size().x_axis(),
    );

    // Scanline
    scanline
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::RED, 1))
        .draw(display)?;

    let line = Line::new(Point::new(10, 10), Point::new(20, 30));

    if let Some(intersection_point) = line.bresenham_scanline_intersection(mouse_pos.y) {
        empty_crosshair(intersection_point.as_line().start, Rgb888::CYAN, display)?;
    }

    line.into_styled(PrimitiveStyle::with_stroke(Rgb888::YELLOW, 1))
        .draw(display)?;

    Ok(())
}

fn main() -> Result<(), core::convert::Infallible> {
    let w = 150i32;
    let h = 100i32;

    let mut display: SimulatorDisplay<Rgb888> =
        SimulatorDisplay::new(Size::new(w as u32, h as u32));
    let output_settings = OutputSettingsBuilder::new()
        .scale(8)
        // .pixel_spacing(1)
        .build();
    let mut window = Window::new("Bresenham scanline intersection", &output_settings);

    // let mut overdraw_display = OverdrawDisplay::new(display.size());

    let mut corner_pos = Point::zero();
    let mut mouse_pos = Point::zero();

    let mut width = 15u32;
    let mut alignment = StrokeAlignment::Center;

    let mut mouse_down = false;

    draw(mouse_pos, corner_pos, &mut display)?;

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
                // SimulatorEvent::KeyDown { keycode, .. } => match keycode {
                //     Keycode::Up => width += 1,
                //     Keycode::Down => width = width.saturating_sub(1),
                //     Keycode::Space => {
                //         alignment = match alignment {
                //             StrokeAlignment::Center => StrokeAlignment::Outside,
                //             StrokeAlignment::Outside => StrokeAlignment::Inside,
                //             StrokeAlignment::Inside => StrokeAlignment::Center,
                //         }
                //     }
                //     _ => (),
                // },
                SimulatorEvent::MouseButtonUp { .. } => mouse_down = false,
                SimulatorEvent::MouseMove { point, .. } => {
                    if mouse_down {
                        corner_pos = point;
                    }
                    mouse_pos = point;
                }
                _ => {}
            }

            draw(mouse_pos, corner_pos, &mut display)?;

            // overdraw_display.draw_to_display(&mut display)?;
        }
    }

    Ok(())
}
