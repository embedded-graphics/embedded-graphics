//! # Example: Triangles and `contains()`
//!
//! Shows multiple triangles with different properties in green and a crosshair under the mouse
//! cursor in cyan. If the cursor enters a triangle, that triangle will turn red.

use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Line, Triangle},
    style::PrimitiveStyle,
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use sdl2::keyboard::Keycode;

const PAD: i32 = 10;
const SPACING: i32 = 66;

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
    display: &mut SimulatorDisplay<Rgb888>,
    point: Point,
    show_outlines: bool,
) -> Result<(), core::convert::Infallible> {
    let style = PrimitiveStyle::with_fill(Rgb888::GREEN);
    let touch_style = PrimitiveStyle::with_fill(Rgb888::RED);

    display.clear(Rgb888::BLACK)?;

    let triangles = [
        Triangle::new(Point::new(5, 10), Point::new(15, 20), Point::new(10, 15)),
        Triangle::new(Point::new(0, 0), Point::new(64, 10), Point::new(15, 64)),
        // Straight top
        Triangle::new(Point::new(5, 0), Point::new(30, 64), Point::new(64, 0)),
        Triangle::new(Point::new(0, 0), Point::new(0, 64), Point::new(64, 30)),
        Triangle::new(Point::new(22, 0), Point::new(0, 64), Point::new(64, 64)),
        Triangle::new(Point::new(0, 22), Point::new(64, 0), Point::new(64, 64)),
    ];

    for (i, triangle) in triangles.iter().enumerate() {
        let triangle = triangle.translate(Point::new(PAD + (SPACING * i as i32), PAD));

        let contains = triangle.contains(point);

        if show_outlines {
            triangle
                .into_styled(if contains { touch_style } else { style })
                .draw(display)?;
        } else {
            triangle
                .bounding_box()
                .points()
                .filter(|p| triangle.contains(*p))
                .map(|p| Pixel(p, if contains { Rgb888::RED } else { Rgb888::GREEN }))
                .draw(display)?;
        }

        // if show_outlines {
        //     Line::new(triangle.p1, triangle.p2)
        //         .into_styled(PrimitiveStyle::with_stroke(Rgb888::YELLOW, 1))
        //         .draw(display)?;
        //     Line::new(triangle.p2, triangle.p3)
        //         .into_styled(PrimitiveStyle::with_stroke(Rgb888::YELLOW, 1))
        //         .draw(display)?;
        //     Line::new(triangle.p3, triangle.p1)
        //         .into_styled(PrimitiveStyle::with_stroke(Rgb888::YELLOW, 1))
        //         .draw(display)?;

        //     Pixel(triangle.p1, Rgb888::CYAN).draw(display)?;
        //     Pixel(triangle.p2, Rgb888::CYAN).draw(display)?;
        //     Pixel(triangle.p3, Rgb888::CYAN).draw(display)?;
        // }
    }

    // Cursor position
    empty_crosshair(point, Rgb888::CYAN, display);

    Ok(())
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(450, 128));

    let output_settings = OutputSettingsBuilder::new()
        .scale(4)
        .pixel_spacing(1)
        .build();
    let mut window = Window::new("Triangle collision", &output_settings);

    let mut show_outlines = true;
    let mut position = Point::zero();

    draw(&mut display, position, show_outlines)?;

    'running: loop {
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::MouseMove { point, .. } => {
                    position = point;
                }
                SimulatorEvent::KeyDown { keycode, .. } => match keycode {
                    Keycode::Space => show_outlines = !show_outlines,
                    _ => (),
                },
                _ => {}
            }

            draw(&mut display, position, show_outlines)?
        }
    }

    Ok(())
}
